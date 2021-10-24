//! String literal unescaping

use crate::ast::error;
use crate::Rule;
use pest::error::Error;
use pest::Span;

// Unescapes a string with the escape sequences described in table 3.4 of the specification
pub fn unescape_string(s: &str, span: Span<'_>) -> Result<String, Error<Rule>> {
    let mut unescaped = String::with_capacity(s.len());
    let mut state = State::Idle;
    for c in s.chars() {
        state = match state {
            State::Idle => {
                if c == '\\' {
                    // Starting an escape sequence
                    State::Backslash
                } else {
                    // Just pass through
                    unescaped.push(c);
                    State::Idle
                }
            }
            State::Backslash => {
                // This is the character after the backslash, which determines
                // the unescaped character
                match c {
                    '\\' => {
                        // Literal backslash
                        unescaped.push('\\');
                        State::Idle
                    }
                    'r' => {
                        // Carriage return
                        unescaped.push('\r');
                        State::Idle
                    }
                    'n' => {
                        // Line feed
                        unescaped.push('\n');
                        State::Idle
                    }
                    't' => {
                        // Horizontal tab
                        unescaped.push('\t');
                        State::Idle
                    }
                    '\'' => {
                        // Single quote
                        unescaped.push('\'');
                        State::Idle
                    }
                    '"' => {
                        // Double quote
                        unescaped.push('"');
                        State::Idle
                    }
                    'u' => {
                        // Beginning of a 4-hex-digit unicode value
                        State::Unicode0of4
                    }
                    'U' => {
                        // Beginning of an 8-hex-digit unicode value
                        State::Unicode0of8
                    }
                    _ => {
                        return Err(error(
                            format!("Unexpected character '{}' after \\ in string literal", c),
                            span.clone(),
                        ))
                    }
                }
            }
            State::Unicode0of4 => {
                let mut escape_chars = String::with_capacity(4);
                escape_chars.push(c);
                State::Unicode1of4(escape_chars)
            }
            State::Unicode1of4(mut escape_chars) => {
                escape_chars.push(c);
                State::Unicode2of4(escape_chars)
            }
            State::Unicode2of4(mut escape_chars) => {
                escape_chars.push(c);
                State::Unicode3of4(escape_chars)
            }
            State::Unicode3of4(mut escape_chars) => {
                escape_chars.push(c);
                let value = u16::from_str_radix(&escape_chars, 16).map_err(|e| {
                    error(
                        format!(
                            "Invalid 4-digit unicode escape sequence \"\\u{}\": {}",
                            escape_chars, e
                        ),
                        span.clone(),
                    )
                })?;
                let constructed = char::from_u32(value.into()).ok_or_else(||
                    error(format!("Unicode escape sequence \\u{:04x} does not represent a valid scalar value", value), span.clone())
                )?;
                unescaped.push(constructed);
                State::Idle
            }
            State::Unicode0of8 => {
                let mut escape_chars = String::with_capacity(8);
                escape_chars.push(c);
                State::Unicode1of8(escape_chars)
            }
            State::Unicode1of8(mut escape_chars) => {
                escape_chars.push(c);
                State::Unicode2of8(escape_chars)
            }
            State::Unicode2of8(mut escape_chars) => {
                escape_chars.push(c);
                State::Unicode3of8(escape_chars)
            }
            State::Unicode3of8(mut escape_chars) => {
                escape_chars.push(c);
                State::Unicode4of8(escape_chars)
            }
            State::Unicode4of8(mut escape_chars) => {
                escape_chars.push(c);
                State::Unicode5of8(escape_chars)
            }
            State::Unicode5of8(mut escape_chars) => {
                escape_chars.push(c);
                State::Unicode6of8(escape_chars)
            }
            State::Unicode6of8(mut escape_chars) => {
                escape_chars.push(c);
                State::Unicode7of8(escape_chars)
            }
            State::Unicode7of8(mut escape_chars) => {
                escape_chars.push(c);
                let value = u32::from_str_radix(&escape_chars, 16).map_err(|e| {
                    error(
                        format!(
                            "Invalid 8-digit unicode escape sequence \"\\U{}\": {}",
                            escape_chars, e
                        ),
                        span.clone(),
                    )
                })?;
                let constructed = char::from_u32(value).ok_or_else(||
                    error(format!("Unicode escape sequence \\U{:08x} does not represent a valid scalar value", value), span.clone())
                )?;
                unescaped.push(constructed);
                State::Idle
            }
        };
    }
    if state == State::Idle {
        Ok(unescaped)
    } else {
        Err(error("Unexpected end of escape sequence".to_owned(), span))
    }
}

#[derive(PartialEq)]
enum State {
    Idle,
    Backslash,
    Unicode0of4,
    Unicode1of4(String),
    Unicode2of4(String),
    Unicode3of4(String),
    Unicode0of8,
    Unicode1of8(String),
    Unicode2of8(String),
    Unicode3of8(String),
    Unicode4of8(String),
    Unicode5of8(String),
    Unicode6of8(String),
    Unicode7of8(String),
}

#[cfg(test)]
mod test {
    use super::unescape_string;
    use crate::Rule;
    use pest::error::Error;
    use pest::Span;

    #[test]
    fn no_escapes() -> Result<(), Error<Rule>> {
        check_unescape("", "")?;
        check_unescape("a", "a")?;
        check_unescape("aaAAAAAAaaAAaaaaaaaaa", "aaAAAAAAaaAAaaaaaaaaa")?;
        check_unescape(r#"""#, r#"""#)?;
        check_unescape("'", "'")?;
        Ok(())
    }

    #[test]
    fn escapes() -> Result<(), Error<Rule>> {
        check_unescape(r#"\\\r\n\t\'\""#, "\\\r\n\t'\"")?;
        check_unescape(r#"\"ain\'t\""#, r#""ain't""#)?;
        check_unescape(r#"oh,\u0020hi\U0000000aMark"#, "oh, hi\nMark")?;

        check_unescape(r#"\U0001f63a"#, "😺")?;
        check_unescape(r#"\U0001F63A"#, "😺")?;
        Ok(())
    }

    #[test]
    fn basic_errors() {
        check_unescape(r#"\"#, "").unwrap_err();
        check_unescape(r#"\ "#, "").unwrap_err();
        check_unescape(r#"\Z"#, "").unwrap_err();
        check_unescape(r#"//\/\/\/\"#, "").unwrap_err();
        check_unescape(r#"Text\"#, "").unwrap_err();
    }

    #[test]
    fn unicode_errors() {
        check_unescape(r#"\u"#, "").unwrap_err();
        check_unescape(r#"\uz"#, "").unwrap_err();
        check_unescape(r#"\uzzzz"#, "").unwrap_err();
        check_unescape(r#"\ufffg"#, "").unwrap_err();

        check_unescape(r#"\U"#, "").unwrap_err();
        check_unescape(r#"\U1234"#, "").unwrap_err();
        check_unescape(r#"\Uabcdefgh"#, "").unwrap_err();
        check_unescape(r#"\Uffffffff"#, "").unwrap_err();
    }

    fn check_unescape(original: &str, expected: &str) -> Result<(), Error<Rule>> {
        let dummy_span = Span::new("a", 0, 1).unwrap();
        let unescaped = unescape_string(original, dummy_span)?;
        assert_eq!(unescaped, expected);
        Ok(())
    }
}
