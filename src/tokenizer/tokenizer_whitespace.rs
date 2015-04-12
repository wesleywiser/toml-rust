use tokenizer::{ParseResult, TomlFragment};

pub fn tokenize_whitespace(s : &str) -> Option<ParseResult> {
    if s.is_empty() {
        return None;
    }

    let mut index : usize = 0;

    for c in s.chars() {
        if !c.is_whitespace() {
            break;
        }

        index = index + 1;
    } 

    match index {
        0 => None,
        _ => {
            let fragment = TomlFragment::Whitespace(&s[0..index]);
            let remainder = &s[index..];

            Some(ParseResult { fragment: fragment, remainder: remainder })
        }
    }
}

#[test]
fn tokenize_whitespace_empty_string() {
    assert_eq!(None, tokenize_whitespace(""));
}

#[test]
fn tokenize_whitespace_no_whitespace_str() {
    assert_eq!(None, tokenize_whitespace("abc"));
}

#[test]
fn tokenize_whitespace_one_space() {
    let fragment = TomlFragment::Whitespace(" ");
    let remainder = "";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), tokenize_whitespace(" "));
}

#[test]
fn tokenize_whitespace_two_spaces() {
    let fragment = TomlFragment::Whitespace("  ");
    let remainder = "";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), tokenize_whitespace("  "));
}

#[test]
fn tokenize_whitespace_one_space_one_letter() {
    let fragment = TomlFragment::Whitespace(" ");
    let remainder = "w";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), tokenize_whitespace(" w"));
}

#[test]
fn tokenize_whitespace_unix_newline() {
    let fragment = TomlFragment::Whitespace("\n");
    let remainder = "";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), tokenize_whitespace("\n"));
}

#[test]
fn tokenize_whitespace_windows_newline() {
    let fragment = TomlFragment::Whitespace("\r\n");
    let remainder = "";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), tokenize_whitespace("\r\n"));
} 

