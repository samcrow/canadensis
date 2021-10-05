use crate::compiled::{
    CompiledDsdl, DsdlKind, Extent, Field, Message, MessageKind, Struct, Union, Variant,
};
use crate::error::Error;
use crate::package::DsdlFile;
use crate::type_key::{TypeFullName, TypeKey};
use crate::types::constant::Constant;
use crate::types::directive::evaluate_directive;
use crate::types::expression::convert_type;
use crate::types::{array_length_bits, PrimitiveType, ResolvedType};
use canadensis_bit_length_set::BitLengthSet;
use canadensis_dsdl_parser::{Identifier, Span, Statement};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs;
use std::mem;
use std::path::PathBuf;

/// The minimum number of variants in a union
const UNION_MIN_VARIANTS: usize = 2;
/// Alignment of a composite type, in bits
const COMPOSITE_ALIGNMENT: usize = 8;

pub(crate) fn compile(
    files: BTreeMap<TypeKey, DsdlFile>,
) -> Result<BTreeMap<TypeKey, CompiledDsdl>, Error> {
    PersistentContext {
        pending: files,
        done: BTreeMap::new(),
    }
    .compile()
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
    /// This is useful for the _offset_ magic variable.
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

        static BIT_LENGTH_ZERO: Lazy<BitLengthSet> = Lazy::new(|| BitLengthSet::single(0));

        match self.current_file.state.as_ref().expect("No state") {
            // Before any field or @union, the length is zero
            State::Message | State::Response(_) => &BIT_LENGTH_ZERO,
            // Normal struct length
            State::MessageStruct(_, length) => length,
            State::ResponseStruct(_, _, length) => length,
            // Unions, length already calculated
            State::MessageUnion(UnionState::UsedOffset(_, length))
            | State::MessageUnion(UnionState::End(_, _, length))
            | State::ResponseUnion(_, UnionState::UsedOffset(_, length))
            | State::ResponseUnion(_, UnionState::End(_, _, length)) => length,
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
    pub fn get_by_key(&mut self, key: &TypeKey) -> Result<&CompiledDsdl, Error> {
        // Look in the current package if the package is not specified
        if key.name().path().is_empty() {
            let local_key = TypeKey::new(
                TypeFullName::new(self.current_file.path.clone(), key.name().name().to_owned()),
                key.version().clone(),
            );
            self.persistent.get_by_key(&local_key)
        } else {
            self.persistent.get_by_key(key)
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
            State::MessageStruct(StructState::Collecting(_), _)
            | State::MessageStruct(StructState::End(_, _), _)
            | State::MessageUnion(UnionState::UsedOffset(_, _))
            | State::MessageUnion(UnionState::End(_, _, _)) => Err(span_error!(
                span,
                "@deprecated is not allowed after a composite type attribute definition",
            )),
            State::Response(_) | State::ResponseUnion(_, _) | State::ResponseStruct(_, _, _) => {
                Err(span_error!(
                    span,
                    "@deprecated is not allowed in the response section of a service type",
                ))
            }
        }
    }
}

/// A convenience function to make a CompileContext
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
}

impl PersistentContext {
    fn compile(mut self) -> Result<BTreeMap<TypeKey, CompiledDsdl>, Error> {
        while let Some(key) = self.pending.keys().next().cloned() {
            let input = self.pending.remove(&key).unwrap();
            let output = self.compile_one(&key, input)?;
            let existing = self.done.insert(key, output);
            assert!(existing.is_none(), "Duplicate type in done");
        }
        Ok(self.done)
    }

    fn compile_one(&mut self, key: &TypeKey, input: DsdlFile) -> Result<CompiledDsdl, Error> {
        let input_path: PathBuf = input.path().into();
        self.compile_one_inner(key, input)
            .map_err(|e| Error::CompileFile {
                path: input_path,
                inner: Box::new(e),
            })
    }

    fn compile_one_inner(&mut self, key: &TypeKey, input: DsdlFile) -> Result<CompiledDsdl, Error> {
        // Create a new state for this file
        let mut state = FileState::new(key.name().path());

        let text = fs::read_to_string(input.path())?;
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
                    } else {
                        let name_str = name.name;
                        let new_constant =
                            Constant::evaluate(&mut ctx(self, &mut state), ty, name, value)?;
                        state.constants.insert(name_str.to_owned(), new_constant);
                    }
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

                    state.add_field(ty, name, total_length, total_alignment, span)?;
                }
                Statement::PaddingField { bits, span } => {
                    state.add_padding_field(bits, span)?;
                }
                Statement::ServiceResponseMarker(span) => {
                    state.handle_service_response_marker(span)?
                }
            }
        }
        // End of file, check that everything is here
        state.finish(ast.eof_span, input.fixed_port_id())
    }

    /// Looks up a type by its name and version
    ///
    /// If the type has already been compiled, this function returns it. Otherwise, this function
    /// attempts to compile it and then returns it.
    pub fn get_by_key(&mut self, key: &TypeKey) -> Result<&CompiledDsdl, Error> {
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
}

impl Default for FileState {
    fn default() -> Self {
        FileState {
            path: vec![],
            constants: BTreeMap::new(),
            deprecated: false,
            state: Some(State::Message),
        }
    }
}

impl FileState {
    fn new(path: &[String]) -> Self {
        FileState {
            path: path.to_vec(),
            constants: BTreeMap::new(),
            deprecated: false,
            state: Some(State::Message),
        }
    }

    fn handle_service_response_marker(&mut self, span: Span<'_>) -> Result<(), Error> {
        match self.state.take().expect("No state") {
            // Struct message completed
            State::MessageStruct(StructState::End(fields, extent), length) => {
                let message = Message {
                    deprecated: mem::take(&mut self.deprecated),
                    extent,
                    kind: MessageKind::Struct(Struct { fields }),
                    bit_length: length,
                };
                // Clear constants
                self.constants.clear();
                self.state = Some(State::Response(message));
                Ok(())
            }
            // Union message completed
            State::MessageUnion(UnionState::End(variants, extent, length)) => {
                let message = Message {
                    deprecated: mem::take(&mut self.deprecated),
                    extent,
                    kind: MessageKind::Union(Union { variants }),
                    bit_length: length,
                };
                // Clear constants
                self.constants.clear();
                self.state = Some(State::Response(message));
                Ok(())
            }
            // Incomplete message, no @sealed or @extent
            State::Message
            | State::MessageStruct(StructState::Collecting(_), _)
            | State::MessageUnion(UnionState::Collecting(_))
            | State::MessageUnion(UnionState::UsedOffset(_, _)) => Err(span_error!(
                span,
                "Expected @sealed or @extent before the end of the request type"
            )),
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
                    StructState::Collecting(vec![Field::Padding(bits)]),
                    bits_added,
                ));
                Ok(())
            }
            State::MessageStruct(StructState::Collecting(mut fields), length) => {
                fields.push(Field::Padding(bits));
                self.state = Some(State::MessageStruct(
                    StructState::Collecting(fields),
                    length.concatenate([bits_added]),
                ));
                Ok(())
            }
            State::Response(req) => {
                self.state = Some(State::ResponseStruct(
                    req,
                    StructState::Collecting(vec![Field::Padding(bits)]),
                    bits_added,
                ));
                Ok(())
            }
            State::ResponseStruct(req, StructState::Collecting(mut fields), length) => {
                fields.push(Field::Padding(bits));
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
        total_alignment: usize,
        span: Span<'_>,
    ) -> Result<(), Error> {
        // TODO: Check for an existing field/variant with the same name
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
                    StructState::Collecting(vec![Field::data(ty, name)]),
                    // Initial bit length matches the first field
                    update_struct_length(BitLengthSet::single(0)),
                ));
                Ok(())
            }
            State::Response(req) => {
                self.state = Some(State::ResponseStruct(
                    req,
                    StructState::Collecting(vec![Field::data(ty, name)]),
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

                fields.push(Field::data(ty, name));
                self.state = Some(State::MessageStruct(
                    StructState::Collecting(fields),
                    update_struct_length(length),
                ));
                Ok(())
            }
            State::ResponseStruct(req, StructState::Collecting(mut fields), length) => {
                fields.push(Field::data(ty, name));
                self.state = Some(State::ResponseStruct(
                    req,
                    StructState::Collecting(fields),
                    update_struct_length(length),
                ));
                Ok(())
            }
            // Add a variant to a union
            State::MessageUnion(UnionState::Collecting(mut variants)) => {
                variants.push(Variant::new(ty, name));
                self.state = Some(State::MessageUnion(UnionState::Collecting(variants)));
                Ok(())
            }
            State::ResponseUnion(req, UnionState::Collecting(mut variants)) => {
                variants.push(Variant::new(ty, name));
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

    fn finish(self, eof_span: Span<'_>, fixed_port_id: Option<u32>) -> Result<CompiledDsdl, Error> {
        match self.state.expect("No state") {
            State::MessageStruct(StructState::End(fields, extent), length) => {
                let message = Message {
                    deprecated: self.deprecated,
                    extent,
                    kind: MessageKind::Struct(Struct { fields }),
                    bit_length: length.pad_to_alignment(COMPOSITE_ALIGNMENT),
                };
                Ok(CompiledDsdl {
                    fixed_port_id,
                    kind: DsdlKind::Message {
                        message,
                        constants: self.constants,
                    },
                })
            }
            State::ResponseStruct(request, StructState::End(fields, extent), length) => {
                let response = Message {
                    deprecated: self.deprecated,
                    extent,
                    kind: MessageKind::Struct(Struct { fields }),
                    bit_length: length.pad_to_alignment(COMPOSITE_ALIGNMENT),
                };
                Ok(CompiledDsdl {
                    fixed_port_id,
                    kind: DsdlKind::Service { request, response },
                })
            }
            State::MessageUnion(UnionState::End(variants, extent, length)) => {
                let message = Message {
                    deprecated: self.deprecated,
                    extent,
                    kind: MessageKind::Union(Union { variants }),
                    bit_length: length.pad_to_alignment(COMPOSITE_ALIGNMENT),
                };
                Ok(CompiledDsdl {
                    fixed_port_id,
                    kind: DsdlKind::Message {
                        message,
                        constants: self.constants,
                    },
                })
            }
            State::ResponseUnion(request, UnionState::End(variants, extent, length)) => {
                let response = Message {
                    deprecated: self.deprecated,
                    extent,
                    kind: MessageKind::Union(Union { variants }),
                    bit_length: length.pad_to_alignment(COMPOSITE_ALIGNMENT),
                };
                Ok(CompiledDsdl {
                    fixed_port_id,
                    kind: DsdlKind::Service { request, response },
                })
            }
            State::Message
            | State::MessageStruct(StructState::Collecting(_), _)
            | State::MessageUnion(UnionState::Collecting(_))
            | State::MessageUnion(UnionState::UsedOffset(_, _))
            | State::Response(_)
            | State::ResponseStruct(_, StructState::Collecting(_), _)
            | State::ResponseUnion(_, UnionState::Collecting(_))
            | State::ResponseUnion(_, UnionState::UsedOffset(_, _)) => Err(span_error!(
                eof_span,
                "Expected @extent or @sealed before end of file"
            )),
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

/// Creates a bit length set for a union, which includes the implicit discriminant and all
/// variants
fn make_union_bit_length(variants: &[Variant]) -> BitLengthSet {
    let discriminant_bits = array_length_bits(variants.len());
    let discriminant_length = BitLengthSet::single(
        discriminant_bits
            .try_into()
            .expect("Can't convert discriminant bits to usize"),
    );
    let variant_lengths = variants
        .iter()
        .map(|variant| variant.ty.size())
        .fold1(|size1, size2| size1.unite([size2]))
        .unwrap_or_else(|| BitLengthSet::single(0));
    // Concatenate the discriminant and the variant lengths
    discriminant_length.concatenate([variant_lengths])
}
