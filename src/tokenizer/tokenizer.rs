use tokenizer::TomlFragment;
use super::parser_whitespace::parse_whitespace;
use super::parser_comment::parse_comment;
use self::TokenizeResult::{Success, Error};

#[derive(PartialEq, Show)]
pub enum TokenizeResult<'a> {
    Success(Vec<TomlFragment<'a>>),
    Error(&'a str),
}

pub fn tokenize(s : &str) -> TokenizeResult {
    let mut tokens = Vec::with_capacity(20);

    let mut rest = s;

    while !rest.is_empty() {
        match parse_whitespace(rest) {
            None => {},
            Some(result) => { tokens.push(result.fragment); rest = result.remainder; continue; }
        }

        match parse_comment(rest) {
            None => {},
            Some(result) => { tokens.push(result.fragment); rest = result.remainder; continue; }
        } 

        return Error("Tokenize failed");
    }

    Success(tokens)
}

#[test]
fn tokenize_error() {
    assert_eq!(Error("Tokenize failed"), tokenize("!23121312awefa"));
}

#[test]
fn tokenize_empty_str() {
    assert_eq!(Success(vec![]), tokenize(""));
} 

#[test]
fn tokenize_whitespace() {
    let tokens = vec![TomlFragment::Whitespace(" ")];
    assert_eq!(Success(tokens), tokenize(" "));
}

#[test]
fn tokenize_comment() {
    let tokens = vec![TomlFragment::Comment("this is a comment")];
    assert_eq!(Success(tokens), tokenize("#this is a comment"));
}

#[test]
fn tokenize_whitespace_comment() {
    let tokens = vec![
        TomlFragment::Whitespace("    \t"),
        TomlFragment::Comment("just some whitespace"),
    ];
    assert_eq!(Success(tokens), tokenize("    \t#just some whitespace"));
}