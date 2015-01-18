use tokenizer::{ParseResult};
use tokenizer::TomlFragment::Integer;

pub fn parse_integer(s : &str) -> Option<ParseResult> {
    if s.is_empty() {
        return None;
    }

    let first_char = s.char_at(0);
    if !(first_char == '+' || first_char == '-' || first_char.is_digit(10)) {
        return None;
    }

    let index : usize = 
        if first_char == '+' || first_char == '-' { 1 }
        else { 0 };

    match get_end_of_number(&s[index..]) {
        None => None,
        Some(end_index) => {
            let fragment = Integer(&s[..index + end_index]);
            let remainder = &s[index + end_index..];

            //if the next char is a '.', then this isn't an Integer
            if !remainder.is_empty() && remainder.char_at(0) == '.' {
                return None;
            }

            Some(ParseResult { fragment: fragment, remainder: remainder })
        }
    }
}

pub fn get_end_of_number(s : &str) -> Option<usize> {
    if s.is_empty() {
        return None;
    }

    if !s.char_at(0).is_digit(10) {
        return None;
    }

    let mut index : usize = 0;

    for c in s.chars() {
        if !c.is_digit(10) {
            break;
        }

        index = index + 1;
    }

    Some(index)
}

#[test]
fn parse_integer_empty_str() {
    assert_eq!(None, parse_integer(""));
}

#[test]
fn parse_integer_text() {
    assert_eq!(None, parse_integer("gw"));
}

#[test]
fn parse_integer_one_digit() {
    let expected = ParseResult { fragment: Integer("9"), remainder: "" };
    assert_eq!(Some(expected), parse_integer("9"));
}

#[test]
fn parse_integer_two_digits() {
    let expected = ParseResult { fragment: Integer("55"), remainder: "" };
    assert_eq!(Some(expected), parse_integer("55"));
}

#[test]
fn parse_integer_positive() {
    let expected = ParseResult { fragment: Integer("+123"), remainder: "" };
    assert_eq!(Some(expected), parse_integer("+123"));
}

#[test]
fn parse_integer_negative() {
    let expected = ParseResult { fragment: Integer("-987"), remainder: "" };
    assert_eq!(Some(expected), parse_integer("-987"));
}

#[test]
fn parse_integer_number_trailing_whitespace() {
    let expected = ParseResult { fragment: Integer("33"), remainder: " " };
    assert_eq!(Some(expected), parse_integer("33 "));
}

#[test]
fn parse_integer_decimal() {
    assert_eq!(None, parse_integer("123.456"));
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
