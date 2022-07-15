use canadensis_dsdl_frontend::compiled::Extent;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result};

use crate::{GeneratedType, MessageRole};

/// Implements DataType and Message, Request, or Response as appropriate
pub(crate) struct ImplementDataType<'t, 'c>(pub &'t GeneratedType<'c>);

impl Display for ImplementDataType<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Part 1: DataType and extent
        writeln!(
            f,
            "impl ::canadensis_encoding::DataType for {} {{",
            self.0.name.type_name
        )?;
        match self.0.extent {
            Extent::Sealed => {
                writeln!(f, "/// This type is sealed.")?;
                writeln!(f, "const EXTENT_BYTES: Option<u32> = None;")?;
            }
            Extent::Delimited(extent_bits) => {
                let extent_bytes = extent_bits / 8;
                let extent_bytes = u32::try_from(extent_bytes).expect("Extent too large for u32");

                writeln!(
                    f,
                    "/// This type is delimited with an extent of {} bytes.",
                    extent_bytes
                )?;
                writeln!(
                    f,
                    "const EXTENT_BYTES: Option<u32> = Some({});",
                    extent_bytes
                )?;
            }
        }
        writeln!(f, "}}")?;

        // Part 2: Role
        match &self.0.role {
            MessageRole::Message => writeln!(
                f,
                "impl ::canadensis_encoding::Message for {} {{}}",
                self.0.name.type_name
            )?,
            MessageRole::Request => writeln!(
                f,
                "impl ::canadensis_encoding::Request for {} {{}}",
                self.0.name.type_name
            )?,
            MessageRole::Response => writeln!(
                f,
                "impl ::canadensis_encoding::Response for {} {{}}",
                self.0.name.type_name
            )?,
        }

        Ok(())
    }
}
