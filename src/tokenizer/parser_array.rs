use tokenizer::{ParseResult};
use tokenizer::TomlFragment;
use tokenizer::TomlFragment::{BracketOpen, BracketClose, Comma};

pub fn parse_bracket_open(s: &str) -> Option<ParseResult> {
    parse_single_char('[', BracketOpen, s)
}

pub fn parse_bracket_close(s: &str) -> Option<ParseResult> {
    parse_single_char(']', BracketClose, s)
}

pub fn parse_comma(s: &str) -> Option<ParseResult> {
    parse_single_char(',', Comma, s)
}

fn parse_single_char<'a>(c : char, fragment: TomlFragment<'a>, s: &'a str) -> Option<ParseResult<'a>> {
    if s.chars().nth(0) != Some(c) {
        return None;
    }

    let remainder = &s[1..];

    Some(ParseResult { fragment: fragment, remainder: remainder })
}

#[test]
fn parse_single_char_empty_str() {
    assert_eq!(None, parse_single_char('[', BracketOpen, ""));
}

#[test]
fn parse_bracket_open_success() {
    let expected = ParseResult { fragment: BracketOpen, remainder: "" };
    assert_eq!(Some(expected), parse_bracket_open("["));

    let expected = ParseResult { fragment: BracketOpen, remainder: "test" };
    assert_eq!(Some(expected), parse_bracket_open("[test"));
}

#[test]
fn parse_bracket_close_success() {
    let expected = ParseResult { fragment: BracketClose, remainder: "" };
    assert_eq!(Some(expected), parse_bracket_close("]"));

    let expected = ParseResult { fragment: BracketClose, remainder: "test" };
    assert_eq!(Some(expected), parse_bracket_close("]test"));
}

#[test]
fn parse_comma_success() {
    let expected = ParseResult { fragment: Comma, remainder: "" };
    assert_eq!(Some(expected), parse_comma(","));

    let expected = ParseResult { fragment: Comma, remainder: "test" };
    assert_eq!(Some(expected), parse_comma(",test"));
}
