use tokenizer::{ParseResult, TomlFragment};

pub fn parse_whitespace(s : &str) -> Option<ParseResult> {
    if s.is_empty() {
        return None;
    }

    if !s.char_at(0).is_whitespace() {
        return None;
    }

    let mut index : uint = 0;

    for c in s.chars() {
        if !c.is_whitespace() {
            break;
        }

        index = index + 1;
    } 

    let fragment = TomlFragment::Whitespace(s.slice(0, index));
    let remainder = s.slice_from(index);

    return Some(ParseResult { fragment: fragment, remainder: remainder });
}

#[test]
fn parse_whitespace_empty_string() {
    assert_eq!(None, parse_whitespace(""));
}

#[test]
fn parse_whitespace_no_whitespace_str() {
    assert_eq!(None, parse_whitespace("abc"));
}

#[test]
fn parse_whitespace_one_space() {
    let fragment = TomlFragment::Whitespace(" ");
    let remainder = "";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), parse_whitespace(" "));
}

#[test]
fn parse_whitespace_two_spaces() {
    let fragment = TomlFragment::Whitespace("  ");
    let remainder = "";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), parse_whitespace("  "));
}

#[test]
fn parse_whitespace_one_space_one_letter() {
    let fragment = TomlFragment::Whitespace(" ");
    let remainder = "w";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), parse_whitespace(" w"));
}

#[test]
fn parse_whitespace_unix_newline() {
    let fragment = TomlFragment::Whitespace("\n");
    let remainder = "";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), parse_whitespace("\n"));
}

#[test]
fn parse_whitespace_windows_newline() {
    let fragment = TomlFragment::Whitespace("\r\n");
    let remainder = "";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), parse_whitespace("\r\n"));
} 

