use tokenizer::{ParseResult};
use tokenizer::TomlFragment::Table;

pub fn parse_table(s: &str) -> Option<ParseResult> {
    //the shortest table you could have 
    //is 3 chars: [a]
    if s.len() < 3 {
        return None;
    }

    if s.char_at(0) != '[' {
        return None;
    }

    let mut index = 0;
    for c in (&s[1..]).chars() {
        //'=', '#', '[', ']', and '.' aren't allowed in table names
        //don't check for '.' here because a higher level will be responsible
        //for dealing with nested table names
        if c == '=' || 
           c == '#' ||
           c == '[' || 
           c.is_whitespace() {
            return None;
        } else if c == ']' {
            index = index + 1;
            break;
        } else {
            index = index + 1;
        }
    }

    //the loop ended before we saw the closing bracket
    if s.char_at(index) != ']' {
        return None;
    } 

    let fragment = Table(&s[1..index]);
    let remainder = &s[index + 1..];

    Some(ParseResult { fragment: fragment, remainder: remainder })
}

#[test]
fn parse_table_empty_str() {
    assert_eq!(None, parse_table(""));
}

#[test]
fn parse_table_incomplete_table() {
    assert_eq!(None, parse_table("["));
    assert_eq!(None, parse_table("[t"));
    assert_eq!(None, parse_table("[table"));
}

#[test]
fn parse_table_simple_name() {
    let expected = ParseResult { fragment: Table("table_name"), remainder: "" };
    assert_eq!(Some(expected), parse_table("[table_name]"));
}
