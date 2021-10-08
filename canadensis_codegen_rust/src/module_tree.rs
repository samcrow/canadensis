use crate::GeneratedType;
use std::collections::BTreeMap;
use std::iter::FromIterator;

pub(crate) struct ModuleTree {
    /// Structs at this level
    pub structs: Vec<GeneratedType>,
    /// Submodules
    pub children: BTreeMap<String, ModuleTree>,
}

impl Default for ModuleTree {
    fn default() -> Self {
        ModuleTree {
            structs: Vec::new(),
            children: BTreeMap::new(),
        }
    }
}

impl ModuleTree {
    fn add_struct(&mut self, path: &[String], generated: GeneratedType) {
        match path {
            [] => {
                // It goes here
                self.structs.push(generated);
            }
            [submodule, rest_of_path @ ..] => {
                let subtree = self.children.entry(submodule.clone()).or_default();
                subtree.add_struct(rest_of_path, generated);
            }
        }
    }
}

impl FromIterator<GeneratedType> for ModuleTree {
    fn from_iter<T: IntoIterator<Item = GeneratedType>>(iter: T) -> Self {
        let mut tree = ModuleTree::default();

        for generated_struct in iter {
            let path = generated_struct.name.path.clone();
            tree.add_struct(&path, generated_struct);
        }

        tree
    }
}

mod fmt_impl {
    use super::ModuleTree;
    use std::fmt::{Display, Formatter, Result};

    impl Display for ModuleTree {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            for generated_struct in &self.structs {
                writeln!(f, "{}", generated_struct)?;
            }
            for (sub_name, submodule) in &self.children {
                writeln!(f, "pub mod {} {{", sub_name)?;
                Display::fmt(submodule, f)?;
                writeln!(f, "}}")?;
            }
            Ok(())
        }
    }
}
