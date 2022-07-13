use crate::GeneratedItem;
use std::collections::BTreeMap;
use std::iter::FromIterator;

/// A tree of Rust modules that may contain items
#[derive(Default)]
pub(crate) struct ModuleTree {
    /// Structs at this level
    pub items: Vec<GeneratedItem>,
    /// Submodules
    pub children: BTreeMap<String, ModuleTree>,
}

impl ModuleTree {
    fn add_item(&mut self, path: &[String], generated: GeneratedItem) {
        match path {
            [] => {
                // It goes here
                self.items.push(generated);
            }
            [submodule, rest_of_path @ ..] => {
                let subtree = self.children.entry(submodule.clone()).or_default();
                subtree.add_item(rest_of_path, generated);
            }
        }
    }
}

impl FromIterator<GeneratedItem> for ModuleTree {
    fn from_iter<T: IntoIterator<Item = GeneratedItem>>(iter: T) -> Self {
        let mut tree = ModuleTree::default();

        for generated_item in iter {
            let path = generated_item.name().path.clone();
            tree.add_item(&path, generated_item);
        }

        tree
    }
}

mod fmt_impl {
    use super::ModuleTree;
    use crate::GeneratedItem;
    use std::fmt::{Display, Formatter, Result};

    impl Display for ModuleTree {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            for generated_item in &self.items {
                writeln!(f, "{}", generated_item)?;
            }
            for (sub_name, submodule) in &self.children {
                // If the submodule has no child modules and all its items are deprecated,
                // mark the submodule as deprecated
                let deprecated = submodule.children.is_empty()
                    && !submodule.items.is_empty()
                    && submodule.items.iter().all(GeneratedItem::deprecated);

                if deprecated {
                    // Allow use of the deprecated type in this module only
                    writeln!(f, "#[allow(deprecated)]")?;
                    writeln!(f, "#[deprecated]")?;
                }
                writeln!(f, "pub mod {} {{", sub_name)?;
                Display::fmt(submodule, f)?;
                writeln!(f, "}}")?;
            }
            Ok(())
        }
    }
}
