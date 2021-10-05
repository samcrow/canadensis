use crate::compiled::{CompiledDsdl, Extent, Field, Message};
use crate::package::{DsdlFile, Error};
use crate::type_key::TypeKey;
use crate::types::constant::Constant;
use crate::types::evaluate_directive;
use canadensis_bit_length_set::BitLengthSet;
use canadensis_dsdl_parser::{make_error, Statement};
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

pub(crate) fn compile(
    files: BTreeMap<TypeKey, DsdlFile>,
) -> Result<BTreeMap<TypeKey, CompiledDsdl>, Error> {
    CompileContext {
        pending: files,
        done: BTreeMap::new(),
        file_states: Vec::new(),
    }
    .compile()
}

pub(crate) struct CompileContext2 {
    persistent: CompileContext,
    current_file: FileState,
}

/// A context used during the compilation process
pub(crate) struct CompileContext {
    /// Files that have not been compiled
    pending: BTreeMap<TypeKey, DsdlFile>,
    /// Files that have been compiled
    done: BTreeMap<TypeKey, CompiledDsdl>,
    /// The parsing state for the current file(s)
    ///
    /// The top (last element) corresponds to the current file.
    file_states: Vec<FileState>,
}

impl CompileContext {
    fn compile(mut self) -> Result<BTreeMap<TypeKey, CompiledDsdl>, Error> {
        while let Some(key) = self.pending.keys().next().cloned() {
            let input = self.pending.remove(&key).unwrap();
            let output = self.compile_one(&key, input)?;
            self.done
                .insert(key, output)
                .expect("Duplicate type key in done");
        }
        Ok(self.done)
    }

    fn compile_one(&mut self, key: &TypeKey, input: DsdlFile) -> Result<CompiledDsdl, Error> {
        let input_path: PathBuf = input.path().into();
        println!("Compiling {} from {}", key, input.path().display());
        self.compile_one_inner(key, input)
            .map_err(|e| Error::CompileFile {
                path: input_path,
                inner: Box::new(e),
            })
    }

    fn compile_one_inner(&mut self, key: &TypeKey, input: DsdlFile) -> Result<CompiledDsdl, Error> {
        // Create a new state for this file
        self.file_states.push(FileState::default());

        let text = fs::read_to_string(input.path())?;
        let ast = canadensis_dsdl_parser::parse(&text)?;

        for statement in ast.statements {
            match statement {
                Statement::Directive { name, value } => evaluate_directive(self, name, value)?,
                Statement::Constant { ty, name, value } => {
                    if self.constants.contains_key(name.name) {
                        return Err(make_error(
                            format!(
                                "A constant attribute named {} has already been defined",
                                name.name
                            ),
                            name.span,
                        )
                        .into());
                    } else {
                        let name_str = name.name;
                        let new_constant = Constant::evaluate(self, ty, name, value)?;
                        println!(
                            "Calculated constant {} = {}",
                            name_str,
                            new_constant.value()
                        );
                        self.constants.insert(name_str.to_owned(), new_constant);
                    }
                }
                Statement::Field { .. } => {}
                Statement::PaddingField { .. } => {}
                Statement::ServiceResponseMarker => {}
            }
        }

        todo!()
    }

    /// Looks up a type by its name and version
    ///
    /// If the type has already been compiled, this function returns it. Otherwise, this function
    /// attempts to compile it and then returns it.
    pub fn get_by_key(&mut self, key: &TypeKey) -> Result<&CompiledDsdl, Error> {
        if self.done.contains_key(key) {
            Ok(self.done.get(key).unwrap())
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

    pub fn constants(&self) -> &BTreeMap<String, Constant> {
        let state = self.file_states.last().expect("No active file state");
        &state.constants
    }
}

/// All state needed when parsing a file
#[derive(Debug)]
struct FileState {
    constants: BTreeMap<String, Constant>,
    bit_length: BitLengthSet,
    state: Option<State>,
}

impl Default for FileState {
    fn default() -> Self {
        FileState {
            constants: BTreeMap::new(),
            bit_length: BitLengthSet::default(),
            state: Some(State::Message),
        }
    }
}

#[derive(Debug)]
enum State {
    /// Initial state, expecting @union or the first field
    Message,
    /// Assembling a sequence of non-union fields
    MessageStruct(FieldState),
    /// Assembling a sequence of union fields
    MessageUnion(FieldState),
    /// Initial state for a response, expecting @union or the first field
    Response(Message),
    /// Assembling a sequence of non-union fields
    ResponseStruct(Message, FieldState),
    /// Assembling a sequence of union fields
    ResponseUnion(Message, FieldState),
}

impl Default for State {
    fn default() -> Self {
        State::Message
    }
}

/// The state of collecting fields of a struct or variants of an enum
#[derive(Debug)]
enum FieldState {
    /// Waiting for the first field/variant
    Initial,
    /// Collecting fields/variants
    Middle(Vec<Field>),
    /// Got an @extent, @sealed, or (for a union) use of __offset__, which can't be followed by any
    /// more fields/variants
    End(Vec<Field>, Option<Extent>),
}
