use tokenizer::{ParseResult, TomlFragment};

pub fn parse_comment(s : &str) -> Option<ParseResult> {
    if s.is_empty() {
        return None;
    }

    match (s.chars().nth(0), s.lines().nth(0)) {
        (Some('#'), Some(comment)) => {
            let fragment = TomlFragment::Comment(&comment[1..]);
            let remainder = &s[comment.len()..];
            
            Some(ParseResult { fragment: fragment, remainder: remainder })
        }
        _ => None
    }
}

#[test]
fn parse_comment_empty_str() {
    assert_eq!(None, parse_comment(""));
}

#[test]
fn parse_comment_non_comment_str() {
    assert_eq!(None, parse_comment("test"));
}

#[test]
fn parse_comment_only_comment_char() {
    let fragment = TomlFragment::Comment("");
    let remainder = "";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), parse_comment("#"));
}

#[test]
fn parse_comment_unix_newline() {
    let fragment = TomlFragment::Comment("");
    let remainder = "\n";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), parse_comment("#\n"));
}

#[test]
fn parse_comment_windows_newline() {
    let fragment = TomlFragment::Comment("\r");
    let remainder = "\n";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), parse_comment("#\r\n"));
}

#[test]
fn parse_comment_text() {
    let fragment = TomlFragment::Comment("some comment");
    let remainder= "";
    
    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), parse_comment("#some comment"));
}

#[test]
fn parse_comment_content_after() {
    let fragment = TomlFragment::Comment("test 123! :)");
    let remainder = "\ncontent";

    assert_eq!(Some(ParseResult { fragment: fragment, remainder: remainder }), parse_comment("#test 123! :)\ncontent"));
}

