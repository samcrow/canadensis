use once_cell::sync::Lazy;
use regex::Regex;

/// Determines if the provided identifier is a reserved keyword, as defined in table 3.5 of the
/// specification
///
/// Reserved keywords are case-insensitive.
pub fn is_reserved_keyword(identifier: &str) -> bool {
    RESERVED.is_match(identifier)
}

pub fn is_valid_identifier(identifier: &str) -> bool {
    IDENTIFIER.is_match(identifier)
}

static IDENTIFIER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*$").unwrap());

/// A regular expression that matches reserved keywords
static RESERVED: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        // This expression has case-insensitive matching.
        // Whitespace in the expression literal is not significant.
        r"^((?xi)
truncated
| saturated
| true
| false
| bool
| u?int\d*
| float\d*
| u?q\d+_\d+
| void\d*
| optional
| aligned
| const
| struct
| super
| template
| enum
| self
| and
| or
| not
| auto
| type
| con
| prn
| aux
| nul
| com\d
| lpt\d
| _.*_
)$",
    )
    .unwrap()
});

#[cfg(test)]
mod test {
    use super::{is_reserved_keyword, is_valid_identifier};

    #[test]
    fn identifiers() {
        assert!(!is_valid_identifier(""));
        assert!(is_valid_identifier("a"));
        assert!(is_valid_identifier("A"));
        assert!(is_valid_identifier("_"));
        assert!(!is_valid_identifier("3"));
        assert!(is_valid_identifier("a3"));
        assert!(is_valid_identifier("_a"));
        assert!(is_valid_identifier("_0118_999_88_999_911_9725_3"));
    }

    #[test]
    fn keywords() {
        assert!(is_reserved_keyword("truncated"));
        assert!(is_reserved_keyword("TRUNCATED"));
        assert!(is_reserved_keyword("TrUnCaTeD"));

        assert!(!is_reserved_keyword("contains_truncated"));
        assert!(!is_reserved_keyword("truncated1"));

        assert!(is_reserved_keyword("int3"));
        assert!(is_reserved_keyword("uint9999999999999"));
        assert!(is_reserved_keyword("_a_"));
        assert!(is_reserved_keyword("_offset_"));
    }
}
