use tokenizer::{ParseResult};
use tokenizer::TomlFragment::Boolean;

pub fn tokenize_boolean(s : &str) -> Option<ParseResult> {
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
        
    match remainder.chars().nth(0) {
        Some(c) if c.is_whitespace() => 
            Some(ParseResult { fragment: Boolean(fragment), remainder: remainder }),
        None => 
            Some(ParseResult { fragment: Boolean(fragment), remainder: remainder }),
        _ => None
    }
}

#[test]
fn tokenize_boolean_empty_str() {
    assert_eq!(None, tokenize_boolean(""));
}

#[test]
fn tokenize_boolean_not_a_boolean() {
    assert_eq!(None, tokenize_boolean("awf1123"));
}

#[test]
fn tokenize_boolean_almost_true() {
    assert_eq!(None, tokenize_boolean("true_but_not_quite"));
}

#[test]
fn tokenize_boolean_almost_false() {
    assert_eq!(None, tokenize_boolean("false_but_not_quite"));
}

#[test]
fn tokenize_boolean_true() {
    assert_eq!(Some(ParseResult { fragment: Boolean("true"), remainder: "" }), tokenize_boolean("true"));
}

#[test]
fn tokenize_boolean_false() {
    assert_eq!(Some(ParseResult { fragment: Boolean("false"), remainder: "" }), tokenize_boolean("false"));
}

#[test]
fn tokenize_boolean_true_newline() {
    assert_eq!(Some(ParseResult { fragment: Boolean("true"), remainder: "\n" }), tokenize_boolean("true\n"));
}

#[test]
fn tokenize_boolean_false_newline() {
    assert_eq!(Some(ParseResult { fragment: Boolean("false"), remainder: "\n" }), tokenize_boolean("false\n"));
}
