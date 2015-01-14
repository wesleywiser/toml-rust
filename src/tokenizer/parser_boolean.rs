use tokenizer::{ParseResult};
use tokenizer::TomlFragment::Boolean;

pub fn parse_boolean(s : &str) -> Option<ParseResult> {
    if s.is_empty() {
        return None;
    }

    let (fragment, remainder) = 
        if s.starts_with("true") {
            (&s[..4], &s[4..])
        } else if s.starts_with("false") {
            (&s[..5], &s[5..])
        } else {
            return None;
        };
        
    if remainder.is_empty() || remainder.char_at(0).is_whitespace() {
        Some(ParseResult { fragment: Boolean(fragment), remainder: remainder })
    } else { None }
}

#[test]
fn parse_boolean_empty_str() {
    assert_eq!(None, parse_boolean(""));
}

#[test]
fn parse_boolean_not_a_boolean() {
    assert_eq!(None, parse_boolean("awf1123"));
}

#[test]
fn parse_boolean_almost_true() {
    assert_eq!(None, parse_boolean("true_but_not_quite"));
}

#[test]
fn parse_boolean_almost_false() {
    assert_eq!(None, parse_boolean("false_but_not_quite"));
}

#[test]
fn parse_boolean_true() {
    assert_eq!(Some(ParseResult { fragment: Boolean("true"), remainder: "" }), parse_boolean("true"));
}

#[test]
fn parse_boolean_false() {
    assert_eq!(Some(ParseResult { fragment: Boolean("false"), remainder: "" }), parse_boolean("false"));
}

#[test]
fn parse_boolean_true_newline() {
    assert_eq!(Some(ParseResult { fragment: Boolean("true"), remainder: "\n" }), parse_boolean("true\n"));
}

#[test]
fn parse_boolean_false_newline() {
    assert_eq!(Some(ParseResult { fragment: Boolean("false"), remainder: "\n" }), parse_boolean("false\n"));
}
