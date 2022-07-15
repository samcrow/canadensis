use crate::compiled::{
    CompiledDsdl, DsdlKind, Extent, Field, Message, MessageKind, Struct, Union, Variant,
};
use crate::constants::Constants;
use crate::error::Error;
use crate::package::DsdlFile;
use crate::type_key::{TypeFullName, TypeKey};
use crate::types::constant::Constant;
use crate::types::directive::evaluate_directive;
use crate::types::expression::convert_type;
use crate::types::{array_length_bits, PrimitiveType, ResolvedScalarType, ResolvedType};
use crate::warning::Warnings;
use canadensis_bit_length_set::BitLengthSet;
use canadensis_dsdl_parser::{Identifier, Span, Statement};
use once_cell::sync::Lazy;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::mem;
use std::path::PathBuf;

/// The minimum number of variants in a union
const UNION_MIN_VARIANTS: usize = 2;
/// Alignment of a composite type, in bits
const COMPOSITE_ALIGNMENT: u32 = 8;
/// A bit length set containing 0
///
/// This is used to return a reference to a bit length set when no other bit length set is available.
static BIT_LENGTH_ZERO: Lazy<BitLengthSet> = Lazy::new(|| BitLengthSet::single(0));

/// Compiles a set of files
///
/// This function returns the compiled DSDL or an error. In either case, it also returns
/// a set of warnings.
pub(crate) fn compile(files: BTreeMap<TypeKey, DsdlFile>) -> CompileOutput {
    let context = PersistentContext {
        pending: files,
        done: BTreeMap::new(),
        warnings: Warnings::new(),
    };
    context.compile()
}

/// The output of a compile operation
pub(crate) struct CompileOutput {
    /// The compiled DSDL, or an error that prevented compilation
    pub dsdl: Result<BTreeMap<TypeKey, CompiledDsdl>, Error>,
    /// Any warnings reported while compiling
    pub warnings: Warnings,
}

/// The compile context is passed to other compiling-related functions.
///
/// They use its functions to collect information about the type currently being compiled, and
/// about other types.
pub(crate) struct CompileContext<'p> {
    /// Persistent context and information about other types
    persistent: &'p mut PersistentContext,
    /// Information about the type currently being compiled
    current_file: &'p mut FileState,
}

impl<'p> CompileContext<'p> {
    /// Returns the constants that have been declared in the current file
    ///
    /// If the current file defines a service type, constants declared in the request section
    /// are not available in the response section.
    pub fn constants(&self) -> &BTreeMap<String, Constant> {
        &self.current_file.constants
    }

    /// Returns a bit length set covering the fields that have been processed so far
    ///
    /// This is useful for the `_offset_` magic variable.
    pub fn bit_length_set(&mut self) -> &BitLengthSet {
        // If necessary, change the state to calculate the union bit length
        match self.current_file.state.take().expect("No state") {
            State::MessageUnion(UnionState::Collecting(fields)) => {
                // Don't check here if we have at least UNION_MIN_VARIANTS.
                // That will get checked later, and checking it here would add a lot of complexity.
                let length = make_union_bit_length(&fields);
                self.current_file.state =
                    Some(State::MessageUnion(UnionState::UsedOffset(fields, length)));
            }
            State::ResponseUnion(req, UnionState::Collecting(fields)) => {
                // Don't check here if we have at least UNION_MIN_VARIANTS.
                // That will get checked later, and checking it here would add a lot of complexity.
                let length = make_union_bit_length(&fields);
                self.current_file.state = Some(State::ResponseUnion(
                    req,
                    UnionState::UsedOffset(fields, length),
                ));
            }
            other => self.current_file.state = Some(other),
        }
        // Now extract the bit length

        match self.current_file.state.as_ref().expect("No state") {
            // Before any field or @union, the length is zero
            State::Message | State::Response(_) => &BIT_LENGTH_ZERO,
            // Normal struct length, or union length already known
            State::MessageStruct(_, length)
            | State::ResponseStruct(_, _, length)
            | State::MessageUnion(
                UnionState::UsedOffset(_, length) | UnionState::End(_, _, length),
            )
            | State::ResponseUnion(
                _,
                UnionState::UsedOffset(_, length) | UnionState::End(_, _, length),
            ) => length,
            // Unions, need to calculate length and change state
            State::MessageUnion(UnionState::Collecting(_))
            | State::ResponseUnion(_, UnionState::Collecting(_)) => {
                unreachable!("No bit length, but we just calculated it")
            }
        }
    }

    /// Looks up a type by its name and version
    ///
    /// If the type has already been compiled, this function returns it. Otherwise, this function
    /// attempts to compile it and then returns it.
    ///
    /// The provided key may be a local key (with no path), which refers to a type in the same
    /// module.
    ///
    /// The return value is the canonical version of the provided key and the corresponding compiled
    /// type.
    pub fn type_by_key(&mut self, key: TypeKey) -> Result<(TypeKey, &CompiledDsdl), Error> {
        // Look in the current package if the package is not specified
        if key.name().path().is_empty() {
            let canonical_key = TypeKey::new(
                TypeFullName::new(self.current_file.path.clone(), key.name().name().to_owned()),
                key.version().clone(),
            );
            let result = self.persistent.type_by_key(&canonical_key)?;
            Ok((canonical_key, result))
        } else {
            let result = self.persistent.type_by_key(&key)?;
            Ok((key, result))
        }
    }

    /// Handles a @union directive
    pub fn handle_union(&mut self, span: Span<'_>) -> Result<(), Error> {
        // @union may only be before the first field in a message (or request or response)
        let state = &mut self.current_file.state;
        match state.take().expect("No state") {
            State::Message => {
                *state = Some(State::MessageUnion(UnionState::Collecting(vec![])));
                Ok(())
            }
            State::Response(m) => {
                *state = Some(State::ResponseUnion(m, UnionState::Collecting(vec![])));
                Ok(())
            }
            _ => Err(span_error!(
                span,
                "A @union directive is not allowed after the first composite type attribute",
            )),
        }
    }
    /// Handles an @extent directive
    pub fn handle_extent(&mut self, span: Span<'_>, extent_bits: u64) -> Result<(), Error> {
        // @extent is after all fields/variants
        apply_sealed_or_extent(
            &mut self.current_file.state,
            Extent::Delimited(extent_bits),
            span,
        )
    }
    /// Handles a @sealed directive
    pub fn handle_sealed(&mut self, span: Span<'_>) -> Result<(), Error> {
        // @sealed is after all fields/variants
        apply_sealed_or_extent(&mut self.current_file.state, Extent::Sealed, span)
    }
    /// Handles a @deprecated directive
    pub fn handle_deprecated(&mut self, span: Span<'_>) -> Result<(), Error> {
        // Limitations:
        // * Can only appear once per file
        // * Can't appear in the response section
        // * Can't be after a field
        if self.current_file.deprecated {
            return Err(span_error!(
                span,
                "@deprecated is not allowed more than once per file"
            ));
        }
        match self.current_file.state.as_ref().expect("No state") {
            State::Message => {
                self.current_file.deprecated = true;
                Ok(())
            }
            State::MessageUnion(UnionState::Collecting(variants)) => {
                if variants.is_empty() {
                    self.current_file.deprecated = true;
                    Ok(())
                } else {
                    Err(span_error!(
                        span,
                        "@deprecated is not allowed after a composite type attribute definition",
                    ))
                }
            }
            State::MessageStruct(StructState::Collecting(_) | StructState::End(_, _), _)
            | State::MessageUnion(UnionState::UsedOffset(_, _) | UnionState::End(_, _, _)) => {
                Err(span_error!(
                    span,
                    "@deprecated is not allowed after a composite type attribute definition",
                ))
            }
            State::Response(_) | State::ResponseUnion(_, _) | State::ResponseStruct(_, _, _) => {
                Err(span_error!(
                    span,
                    "@deprecated is not allowed in the response section of a service type",
                ))
            }
        }
    }
}

/// A convenience function to make a `CompileContext`
fn ctx<'p>(p: &'p mut PersistentContext, c: &'p mut FileState) -> CompileContext<'p> {
    CompileContext {
        persistent: p,
        current_file: c,
    }
}

/// A context used during the compilation process
struct PersistentContext {
    /// Files that have not been compiled
    pending: BTreeMap<TypeKey, DsdlFile>,
    /// Files that have been compiled
    done: BTreeMap<TypeKey, CompiledDsdl>,
    /// Any reported warnings
    warnings: Warnings,
}

impl PersistentContext {
    fn compile(mut self) -> CompileOutput {
        while let Some(key) = self.pending.keys().next().cloned() {
            let input = self.pending.remove(&key).unwrap();
            match self.compile_one(&key, input) {
                Ok(output) => {
                    let existing = self.done.insert(key, output);
                    assert!(existing.is_none(), "Duplicate type in done");
                }
                Err(e) => {
                    return CompileOutput {
                        dsdl: Err(e),
                        warnings: self.warnings,
                    }
                }
            }
        }
        CompileOutput {
            dsdl: Ok(self.done),
            warnings: self.warnings,
        }
    }

    fn compile_one(&mut self, key: &TypeKey, input: DsdlFile) -> Result<CompiledDsdl, Error> {
        let input_path = input.path().map(PathBuf::from);
        self.compile_one_inner(key, input)
            .map_err(|e| Error::CompileFile {
                key: key.clone(),
                path: input_path,
                inner: Box::new(e),
            })
    }

    fn compile_one_inner(&mut self, key: &TypeKey, input: DsdlFile) -> Result<CompiledDsdl, Error> {
        self.warnings.check_pre_compile(key);

        // Create a new state for this file
        let mut state = FileState::new(key.name().path());

        let text = input.read()?;
        let ast = canadensis_dsdl_parser::parse(&text)?;

        for statement in ast.statements {
            match statement {
                Statement::Directive { name, value } => {
                    evaluate_directive(&mut ctx(self, &mut state), name, value)?
                }
                Statement::Constant { ty, name, value } => {
                    if state.constants.contains_key(name.name) {
                        return Err(span_error!(
                            name.span,
                            "A constant attribute named {} has already been defined",
                            name.name
                        ));
                    }
                    let name_str = name.name;
                    let new_constant =
                        Constant::evaluate(&mut ctx(self, &mut state), ty, name, value)?;
                    state.constants.insert(name_str.to_owned(), new_constant);
                }
                Statement::Field { ty, name, span } => {
                    let ty = convert_type(&mut ctx(self, &mut state), ty)?;
                    let ty = ty.resolve(&mut ctx(self, &mut state), span.clone())?;
                    let ty_alignment = ty.alignment();
                    let ty_length = ty.size();

                    // Calculate the total length and alignment for the implicit length or delimiter
                    // header (if any) and the field type
                    let (total_length, total_alignment) = match ty.implicit_field() {
                        Some(implicit_length) => {
                            let implicit_type = PrimitiveType::from(implicit_length);
                            let implicit_bit_length =
                                BitLengthSet::single(implicit_type.bit_length());
                            (
                                ty_length.concatenate([implicit_bit_length]),
                                std::cmp::max(ty_alignment, implicit_type.alignment()),
                            )
                        }
                        None => (ty_length, ty_alignment),
                    };

                    // If the field type is deprecated, this type must also be deprecated.
                    check_deprecated_in_non_deprecated(key, &state, &ty)?;

                    state.add_field(ty, name, total_length, total_alignment, span)?;
                }
                Statement::PaddingField { bits, span } => {
                    state.add_padding_field(bits, span)?;
                }
                Statement::ServiceResponseMarker(span) => {
                    state.handle_service_response_marker(span)?
                }
                Statement::Comment(comment) => state.handle_comment(comment.as_str().trim())?,
            }
        }
        // End of file, check that everything is here
        let compiled = state.finish(ast.eof_span, input.fixed_port_id())?;

        self.warnings.check_post_compile(key, &compiled);
        Ok(compiled)
    }

    /// Looks up a composite type by its name and version
    ///
    /// If the type has already been compiled, this function returns it. Otherwise, this function
    /// attempts to compile it and then returns it.
    pub fn type_by_key(&mut self, key: &TypeKey) -> Result<&CompiledDsdl, Error> {
        // Although types that differ only in case are prohibited, the names must match exactly
        // when looking them up.
        if self.done.contains_key_case_sensitive(key) {
            Ok(self.done.get_case_sensitive(key).unwrap())
        } else {
            // Look up and try to compile
            match self.pending.remove(key) {
                Some(pending) => {
                    // Compile it
                    let compiled = self.compile_one(key, pending)?;

                    match self.done.entry(key.clone()) {
                        Entry::Vacant(entry) => Ok(entry.insert(compiled)),
                        Entry::Occupied(_) => panic!(
                            "Compiled type {} was inserted when we didn't expect it",
                            key
                        ),
                    }
                }
                None => Err(Error::UnknownType(key.clone())),
            }
        }
    }
}

/// Checks a type about to be added as a field or variant to this type, and returns an error
/// if inner_type is deprecated and the type associated with `state` is not deprecated
fn check_deprecated_in_non_deprecated(
    outer_key: &TypeKey,
    state: &FileState,
    inner_type: &ResolvedType,
) -> Result<(), Error> {
    // Check if type is deprecated
    match inner_type.scalar() {
        ResolvedScalarType::Composite { key, inner } => {
            if !state.deprecated && inner.deprecated {
                Err(Error::DeprecatedInNonDeprecated {
                    outer: outer_key.clone(),
                    inner: key.clone(),
                })
            } else {
                Ok(())
            }
        }
        ResolvedScalarType::Primitive(_) => Ok(()),
        ResolvedScalarType::Void { .. } => Ok(()),
    }
}

trait MapExt {
    /// Checks if this map contains a mapping for the specified key, using case-sensitive comparison
    fn contains_key_case_sensitive(&self, key: &TypeKey) -> bool {
        self.get_case_sensitive(key).is_some()
    }
    /// Finds the value corresponding to the specified key, using case-sensitive comparison
    fn get_case_sensitive(&self, key: &TypeKey) -> Option<&CompiledDsdl>;
}
impl MapExt for BTreeMap<TypeKey, CompiledDsdl> {
    fn get_case_sensitive(&self, key: &TypeKey) -> Option<&CompiledDsdl> {
        let (stored_key, value) = self.get_key_value(key)?;
        if stored_key.case_sensitive_equal(key) {
            Some(value)
        } else {
            None
        }
    }
}

/// All state needed when parsing a file
#[derive(Debug)]
struct FileState {
    /// The path components of the package that the file is in
    path: Vec<String>,
    /// Constants defined in the file so far
    constants: BTreeMap<String, Constant>,
    /// True if this type is deprecated
    ///
    /// For service types, this applies to both the request and response.
    deprecated: bool,
    /// Top-level parsing state
    ///
    /// This is always Some, except during functions that match on the current state.
    state: Option<State>,
    /// Documentation comments from the top of the file
    comments: String,
}

impl Default for FileState {
    fn default() -> Self {
        FileState {
            path: vec![],
            constants: BTreeMap::new(),
            deprecated: false,
            state: Some(State::Message),
            comments: String::new(),
        }
    }
}

impl FileState {
    fn new(path: &[String]) -> Self {
        FileState {
            path: path.to_vec(),
            ..FileState::default()
        }
    }

    fn handle_service_response_marker(&mut self, span: Span<'_>) -> Result<(), Error> {
        match self.state.take().expect("No state") {
            // Struct message completed
            State::MessageStruct(StructState::End(fields, extent), length) => {
                let message = Message {
                    // self.deprecated stays the same so that the response is also deprecated
                    deprecated: self.deprecated,
                    extent,
                    kind: MessageKind::Struct(Struct { fields }),
                    bit_length: length,
                    constants: self.take_constants(),
                };
                self.state = Some(State::Response(message));
                Ok(())
            }
            // Union message completed
            State::MessageUnion(UnionState::End(variants, extent, length)) => {
                let discriminant_bits = calculate_discriminant_bits(variants.len());
                let message = Message {
                    // self.deprecated stays the same so that the response is also deprecated
                    deprecated: self.deprecated,
                    extent,
                    kind: MessageKind::Union(Union {
                        discriminant_bits,
                        variants,
                    }),
                    bit_length: length,
                    constants: self.take_constants(),
                };
                self.state = Some(State::Response(message));
                Ok(())
            }
            // Incomplete message, no @sealed or @extent
            State::Message
            | State::MessageStruct(StructState::Collecting(_), _)
            | State::MessageUnion(UnionState::Collecting(_) | UnionState::UsedOffset(_, _)) => {
                Err(span_error!(
                    span,
                    "Expected @sealed or @extent before the end of the request type"
                ))
            }
            // If already in a response, can't have another ---
            State::Response(_) | State::ResponseUnion(_, _) | State::ResponseStruct(_, _, _) => {
                Err(span_error!(
                    span,
                    "Unexpected extra service response marker"
                ))
            }
        }
    }

    fn add_padding_field(&mut self, bits: u8, span: Span<'_>) -> Result<(), Error> {
        let bits_added = BitLengthSet::single(bits.into());
        match self.state.take().expect("No state") {
            State::Message => {
                self.state = Some(State::MessageStruct(
                    StructState::Collecting(vec![Field::padding(bits, true, span)]),
                    bits_added,
                ));
                Ok(())
            }
            State::MessageStruct(StructState::Collecting(mut fields), length) => {
                fields.push(Field::padding(bits, length.is_byte_aligned(), span));
                self.state = Some(State::MessageStruct(
                    StructState::Collecting(fields),
                    length.concatenate([bits_added]),
                ));
                Ok(())
            }
            State::Response(req) => {
                self.state = Some(State::ResponseStruct(
                    req,
                    StructState::Collecting(vec![Field::padding(bits, true, span)]),
                    bits_added,
                ));
                Ok(())
            }
            State::ResponseStruct(req, StructState::Collecting(mut fields), length) => {
                fields.push(Field::padding(bits, length.is_byte_aligned(), span));
                self.state = Some(State::ResponseStruct(
                    req,
                    StructState::Collecting(fields),
                    length.concatenate([bits_added]),
                ));
                Ok(())
            }
            State::MessageStruct(StructState::End(_, _), _)
            | State::ResponseStruct(_, StructState::End(_, _), _) => Err(span_error!(
                span,
                "Padding is not allowed after @sealed or @extent"
            )),
            State::MessageUnion(_) | State::ResponseUnion(_, _) => {
                Err(span_error!(span, "Padding is not allowed in a union"))
            }
        }
    }

    fn add_field(
        &mut self,
        ty: ResolvedType,
        name: Identifier,
        total_length: BitLengthSet,
        total_alignment: u32,
        span: Span<'_>,
    ) -> Result<(), Error> {
        let name = name.name.to_owned();
        let update_struct_length = |length: BitLengthSet| {
            length
                .pad_to_alignment(total_alignment)
                .concatenate([total_length])
                .pad_to_alignment(total_alignment)
        };

        match self.state.take().expect("No state") {
            // No fields, enter struct mode and add the first one
            State::Message => {
                self.state = Some(State::MessageStruct(
                    StructState::Collecting(vec![Field::data(ty, name, true, span)]),
                    // Initial bit length matches the first field
                    update_struct_length(BitLengthSet::single(0)),
                ));
                Ok(())
            }
            State::Response(req) => {
                self.state = Some(State::ResponseStruct(
                    req,
                    StructState::Collecting(vec![Field::data(ty, name, true, span)]),
                    // Initial bit length matches the first field
                    update_struct_length(BitLengthSet::single(0)),
                ));
                Ok(())
            }
            // Add a field to a struct
            State::MessageStruct(StructState::Collecting(mut fields), length) => {
                if fields.iter().any(|existing| existing.name() == Some(&name)) {
                    return Err(span_error!(span, "A field named {} already exists", name));
                }

                fields.push(Field::data(ty, name, length.is_byte_aligned(), span));
                self.state = Some(State::MessageStruct(
                    StructState::Collecting(fields),
                    update_struct_length(length),
                ));
                Ok(())
            }
            State::ResponseStruct(req, StructState::Collecting(mut fields), length) => {
                if fields.iter().any(|existing| existing.name() == Some(&name)) {
                    return Err(span_error!(span, "A field named {} already exists", name));
                }

                fields.push(Field::data(ty, name, length.is_byte_aligned(), span));
                self.state = Some(State::ResponseStruct(
                    req,
                    StructState::Collecting(fields),
                    update_struct_length(length),
                ));
                Ok(())
            }
            // Add a variant to a union
            State::MessageUnion(UnionState::Collecting(mut variants)) => {
                if variants.iter().any(|existing| existing.name() == name) {
                    return Err(span_error!(span, "A variant named {} already exists", name));
                }

                variants.push(Variant::new(ty, name, span));
                self.state = Some(State::MessageUnion(UnionState::Collecting(variants)));
                Ok(())
            }
            State::ResponseUnion(req, UnionState::Collecting(mut variants)) => {
                if variants.iter().any(|existing| existing.name() == name) {
                    return Err(span_error!(span, "A variant named {} already exists", name));
                }

                variants.push(Variant::new(ty, name, span));
                self.state = Some(State::ResponseUnion(req, UnionState::Collecting(variants)));
                Ok(())
            }
            // Past end of union
            State::MessageUnion(UnionState::UsedOffset(_, _))
            | State::ResponseUnion(_, UnionState::UsedOffset(_, _)) => Err(span_error!(
                span,
                "Composite type attribute definition not allowed in union after use of _offset_"
            )),
            // After @sealed or @extent
            State::MessageStruct(StructState::End(_, _), _)
            | State::ResponseStruct(_, StructState::End(_, _), _)
            | State::MessageUnion(UnionState::End(_, _, _))
            | State::ResponseUnion(_, UnionState::End(_, _, _)) => Err(span_error!(
                span,
                "Composite type attribute definition not allowed after @sealed or @extent"
            )),
        }
    }

    fn handle_comment(&mut self, comment: &str) -> Result<(), Error> {
        // If state is not State::Message, the comment applies to the most recent field, variant,
        // or constant.
        // Find the most recent constant for comparison.
        let last_constant: Option<&mut Constant> =
            self.constants.values_mut().max_by_key(|c| c.end_offset());

        match self.state.as_mut().expect("No state") {
            State::Message => {
                if let Some(last_constant) = last_constant {
                    // Have at least one constant, comment belongs there
                    last_constant.append_comment(comment);
                } else {
                    // Comment applies to top-level
                    if !self.comments.is_empty() {
                        self.comments.push('\n');
                    }
                    self.comments.push_str(comment);
                }
            }
            State::MessageStruct(StructState::Collecting(fields), _) => {
                // Comment applies to most recent field or constant, whichever is later in the file
                apply_comment_to_constant_or_field(last_constant, fields.last_mut(), comment);
            }
            State::MessageUnion(
                UnionState::Collecting(variants) | UnionState::End(variants, _, _),
            ) => {
                apply_comment_to_constant_or_variant(last_constant, variants.last_mut(), comment);
            }
            State::Response(_) => {
                // Don't expect a comment here, ignore
            }
            State::ResponseStruct(_, StructState::Collecting(fields), _) => {
                apply_comment_to_constant_or_field(last_constant, fields.last_mut(), comment);
            }
            State::ResponseUnion(
                _,
                UnionState::Collecting(variants) | UnionState::UsedOffset(variants, _),
            ) => {
                apply_comment_to_constant_or_variant(last_constant, variants.last_mut(), comment);
            }
            _ => { /* Don't expect a comment here, ignore */ }
        }
        Ok(())
    }

    /// Handles an end of file and checks that the complete file is well-formed
    ///
    /// On success, this function returnes a compiled DSDL object.
    fn finish(self, eof_span: Span<'_>, fixed_port_id: Option<u32>) -> Result<CompiledDsdl, Error> {
        match self.state.expect("No state") {
            State::MessageStruct(StructState::End(fields, extent), length) => {
                let message = Message {
                    deprecated: self.deprecated,
                    extent,
                    kind: MessageKind::Struct(Struct { fields }),
                    bit_length: length.pad_to_alignment(COMPOSITE_ALIGNMENT),
                    constants: Constants::from_map(self.constants),
                };
                Ok(CompiledDsdl {
                    fixed_port_id,
                    kind: DsdlKind::Message(message),
                    comments: self.comments,
                })
            }
            State::ResponseStruct(request, StructState::End(fields, extent), length) => {
                let response = Message {
                    deprecated: self.deprecated,
                    extent,
                    kind: MessageKind::Struct(Struct { fields }),
                    bit_length: length.pad_to_alignment(COMPOSITE_ALIGNMENT),
                    constants: Constants::from_map(self.constants),
                };
                Ok(CompiledDsdl {
                    fixed_port_id,
                    kind: DsdlKind::Service { request, response },
                    comments: self.comments,
                })
            }
            State::MessageUnion(UnionState::End(variants, extent, length)) => {
                let discriminant_bits = calculate_discriminant_bits(variants.len());
                let message = Message {
                    deprecated: self.deprecated,
                    extent,
                    kind: MessageKind::Union(Union {
                        discriminant_bits,
                        variants,
                    }),
                    bit_length: length.pad_to_alignment(COMPOSITE_ALIGNMENT),
                    constants: Constants::from_map(self.constants),
                };
                Ok(CompiledDsdl {
                    fixed_port_id,
                    kind: DsdlKind::Message(message),
                    comments: self.comments,
                })
            }
            State::ResponseUnion(request, UnionState::End(variants, extent, length)) => {
                let discriminant_bits = calculate_discriminant_bits(variants.len());
                let response = Message {
                    deprecated: self.deprecated,
                    extent,
                    kind: MessageKind::Union(Union {
                        discriminant_bits,
                        variants,
                    }),
                    bit_length: length.pad_to_alignment(COMPOSITE_ALIGNMENT),
                    constants: Constants::from_map(self.constants),
                };
                Ok(CompiledDsdl {
                    fixed_port_id,
                    kind: DsdlKind::Service { request, response },
                    comments: self.comments,
                })
            }
            State::Message
            | State::MessageStruct(StructState::Collecting(_), _)
            | State::MessageUnion(UnionState::Collecting(_) | UnionState::UsedOffset(_, _))
            | State::Response(_)
            | State::ResponseStruct(_, StructState::Collecting(_), _)
            | State::ResponseUnion(_, UnionState::Collecting(_) | UnionState::UsedOffset(_, _)) => {
                Err(span_error!(
                    eof_span,
                    "Expected @extent or @sealed before end of file"
                ))
            }
        }
    }

    /// Removes and returns the current constants map, replacing it with an empty map
    #[must_use]
    fn take_constants(&mut self) -> Constants {
        Constants::from_map(mem::take(&mut self.constants))
    }
}

/// Applies a comment to last_constant or last_field, whichever is later in the file
fn apply_comment_to_constant_or_field(
    last_constant: Option<&mut Constant>,
    last_field: Option<&mut Field>,
    comment: &str,
) {
    match (last_constant, last_field) {
        (None, None) => {}
        (Some(last_constant), None) => last_constant.append_comment(comment),
        (None, Some(last_field)) => last_field.append_comment(comment),
        (Some(last_constant), Some(last_field)) => {
            if last_constant.end_offset() > last_field.end_offset() {
                last_constant.append_comment(comment)
            } else {
                last_field.append_comment(comment)
            }
        }
    }
}
/// Applies a comment to last_constant or last_variant, whichever is later in the file
fn apply_comment_to_constant_or_variant(
    last_constant: Option<&mut Constant>,
    last_variant: Option<&mut Variant>,
    comment: &str,
) {
    match (last_constant, last_variant) {
        (None, None) => {}
        (Some(last_constant), None) => last_constant.append_comment(comment),
        (None, Some(last_variant)) => last_variant.append_comment(comment),
        (Some(last_constant), Some(last_variant)) => {
            if last_constant.end_offset() > last_variant.end_offset() {
                last_constant.append_comment(comment)
            } else {
                last_variant.append_comment(comment)
            }
        }
    }
}

#[derive(Debug)]
enum State {
    /// Initial state, expecting @union or the first field
    Message,
    /// Assembling a sequence of non-union fields
    MessageStruct(StructState, BitLengthSet),
    /// Assembling a sequence of union fields
    MessageUnion(UnionState),
    /// Initial state for a response, expecting @union or the first field
    Response(Message),
    /// Assembling a sequence of non-union fields
    ResponseStruct(Message, StructState, BitLengthSet),
    /// Assembling a sequence of union fields
    ResponseUnion(Message, UnionState),
}

impl Default for State {
    fn default() -> Self {
        State::Message
    }
}

/// The state of collecting fields of a struct
#[derive(Debug)]
enum StructState {
    /// Collecting fields
    Collecting(Vec<Field>),
    /// Got an @extent or @sealed, which can't be followed by any
    /// more fields
    End(Vec<Field>, Extent),
}
/// The state of collecting variants of a union
#[derive(Debug)]
enum UnionState {
    /// Collecting variants (may not have any variants yet)
    Collecting(Vec<Variant>),
    /// Got a use of _offset_, which can't be followed by any
    /// more fields
    UsedOffset(Vec<Variant>, BitLengthSet),
    /// Got an @extent or @sealed, which can't be followed by any
    /// more fields
    End(Vec<Variant>, Extent, BitLengthSet),
}

/// Applies state transitions and checks errors to handle a @sealed or @extent directive
fn apply_sealed_or_extent(
    state: &mut Option<State>,
    extent: Extent,
    span: Span,
) -> Result<(), Error> {
    match state.take().expect("No state") {
        // Already got @extent or @sealed
        State::MessageStruct(StructState::End(_, _), _)
        | State::MessageUnion(UnionState::End(_, _, _))
        | State::ResponseStruct(_, StructState::End(_, _), _)
        | State::ResponseUnion(_, UnionState::End(_, _, _)) => Err(span_error!(span, "An @extent or @sealed directive is not allowed after another @extent or @sealed directive")),
        // Struct, no fields yet
        State::Message => {
            *state = Some(State::MessageStruct(StructState::End(
                vec![],
                extent),
                BitLengthSet::single(0)
            ));
            Ok(())
        }
        State::Response(req) => {
            *state = Some(State::ResponseStruct(
                req,
                StructState::End(vec![], extent),
             BitLengthSet::single(0)));
            Ok(())
        }
        // Struct, have some fields
        State::MessageStruct(StructState::Collecting(fields), length) => {
            *state = Some(State::MessageStruct(StructState::End(
                fields,
                extent,
            ), length));
            Ok(())
        }
        State::ResponseStruct(req, StructState::Collecting(fields), length) => {
            *state = Some(State::ResponseStruct(
                req,
                StructState::End(fields, extent),
             length));
            Ok(())
        }
        // Union
        State::MessageUnion(UnionState::Collecting(fields)) => {
            if fields.len() >= UNION_MIN_VARIANTS {
                let length = make_union_bit_length(&fields);
                *state = Some(State::MessageUnion(UnionState::End(
                    fields,
                    extent,
                    length,
                )));
                Ok(())
            } else {
                Err(span_error!(span, "Need at least two union variants before @extent or @sealed"))
            }
        }
         State::MessageUnion(UnionState::UsedOffset(fields, length)) => {
            if fields.len() >= UNION_MIN_VARIANTS {
                *state = Some(State::MessageUnion(UnionState::End(
                    fields,
                    extent,
                    length,
                )));
                Ok(())
            } else {
                Err(span_error!(span, "Need at least two union variants before @extent or @sealed"))
            }
        }
        // Response union
        State::ResponseUnion(req, UnionState::Collecting(fields)) => {
            if fields.len() >= UNION_MIN_VARIANTS {
                let length = make_union_bit_length(&fields);
                *state = Some(State::ResponseUnion(req, UnionState::End(
                    fields,
                    extent,
                    length,
                )));
                Ok(())
            } else {
                Err(span_error!(span, "Need at least two union variants before @extent or @sealed"))
            }
        }
        State::ResponseUnion(req, UnionState::UsedOffset(fields, length)) => {
            if fields.len() >= UNION_MIN_VARIANTS {
                *state = Some(State::ResponseUnion(req, UnionState::End(
                    fields,
                    extent,
                    length,
                )));
                Ok(())
            } else {
                Err(span_error!(span, "Need at least two union variants before @extent or @sealed"))
            }
        }
    }
}

/// Calculates the number of bits needed for the discriminant of a union with the provided number
/// of variants
fn calculate_discriminant_bits(num_variants: usize) -> u8 {
    array_length_bits(
        num_variants
            .try_into()
            .expect("Number of union variants too large for u64"),
    )
    .try_into()
    .expect("Discriminant bit length too large for u8")
}

/// Creates a bit length set for a union, which includes the implicit discriminant and all
/// variants
fn make_union_bit_length(variants: &[Variant]) -> BitLengthSet {
    let discriminant_bits = calculate_discriminant_bits(variants.len());
    let discriminant_length = BitLengthSet::single(discriminant_bits.into());
    let variant_lengths = variants
        .iter()
        .map(|variant| variant.ty().size())
        .reduce(|size1, size2| size1.unite([size2]))
        .unwrap_or_else(|| BitLengthSet::single(0));
    // Concatenate the discriminant and the variant lengths
    discriminant_length.concatenate([variant_lengths])
}
