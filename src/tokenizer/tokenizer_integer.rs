use tokenizer::{ParseResult};
use tokenizer::TomlFragment::Integer;

pub fn tokenize_integer(s : &str) -> Option<ParseResult> {
    let index : usize = 
        match s.chars().nth(0) {
            Some('+') | Some('-') => 1,
            Some(c) if c.is_digit(10) => 0,
            _ => return None
        };

    match get_end_of_number(&s[index..]) {
        None => None,
        Some(end_index) => {
            let fragment = Integer(&s[..index + end_index]);
            let remainder = &s[index + end_index..];

            //if the next char is a '.', then this isn't an Integer
            match remainder.chars().nth(0) {
                Some('.') => None,
                _ => Some(ParseResult { fragment: fragment, remainder: remainder })
            }
        }
    }
}

pub fn get_end_of_number(s : &str) -> Option<usize> {
    if s.is_empty() {
        return None;
    }

    let mut index : usize = 0;

    for c in s.chars() {
        if !c.is_digit(10) {
            break;
        }

        index = index + 1;
    }

    match index {
        0 => None,
        n => Some(n)
    }
}

#[test]
fn tokenize_integer_empty_str() {
    assert_eq!(None, tokenize_integer(""));
}

#[test]
fn tokenize_integer_text() {
    assert_eq!(None, tokenize_integer("gw"));
}

#[test]
fn tokenize_integer_one_digit() {
    let expected = ParseResult { fragment: Integer("9"), remainder: "" };
    assert_eq!(Some(expected), tokenize_integer("9"));
}

#[test]
fn tokenize_integer_two_digits() {
    let expected = ParseResult { fragment: Integer("55"), remainder: "" };
    assert_eq!(Some(expected), tokenize_integer("55"));
}

#[test]
fn tokenize_integer_positive() {
    let expected = ParseResult { fragment: Integer("+123"), remainder: "" };
    assert_eq!(Some(expected), tokenize_integer("+123"));
}

#[test]
fn tokenize_integer_negative() {
    let expected = ParseResult { fragment: Integer("-987"), remainder: "" };
    assert_eq!(Some(expected), tokenize_integer("-987"));
}

#[test]
fn tokenize_integer_number_trailing_whitespace() {
    let expected = ParseResult { fragment: Integer("33"), remainder: " " };
    assert_eq!(Some(expected), tokenize_integer("33 "));
}

#[test]
fn tokenize_integer_decimal() {
    assert_eq!(None, tokenize_integer("123.456"));
}

#[test]
fn get_end_of_number_empty_str() {
    assert_eq!(None, get_end_of_number(""));
}

#[test]
fn get_end_of_number_text() {
    assert_eq!(None, get_end_of_number("awf"));
}

#[test]
fn get_end_of_number_one_digit() {
    assert_eq!(Some(1), get_end_of_number("5"));
}

#[test]
fn get_end_of_number_two_digits() {
    assert_eq!(Some(2), get_end_of_number("12"));
}
