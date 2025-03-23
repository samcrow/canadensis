extern crate canadensis_dsdl_parser;

use canadensis_dsdl_parser::{parse, Config};

#[test]
fn comments() {
    fn try_parse_comment(text: &str) {
        let config = Config::default();
        match parse(text, &config) {
            Ok(_pairs) => {}
            Err(e) => panic!("{}", e),
        }
    }

    try_parse_comment("#");
    try_parse_comment("# ");
    try_parse_comment("# Comment text!")
}

#[test]
fn simple1() {
    try_parse("");

    try_parse(
        r"
        # Comment at the beginning of the line
# Next comment
    # Whitespace before the comment",
    );
    try_parse("#Comment at beginning");

    try_parse(
        r"Name.1.0 name
# The name of the accessed register. Shall not be empty.
# Use the List service to obtain the list of registers on the node.

Value.1.0 value
# Value to be written. Empty if no write is required.

@sealed

---

uavcan.time.SynchronizedTimestamp.1.0 timestamp
bool mutable
bool persistent
void6
Value.1.0 value

@sealed
",
    );
    fn try_parse(text: &str) {
        let config = Config::default();
        match parse(text, &config) {
            Ok(_pairs) => {}
            Err(e) => panic!("{}", e),
        }
    }
}
