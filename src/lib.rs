
#[derive(PartialEq, Show)]
enum TomlFragment<'a> {
    Whitespace(&'a str),
    
    //doesn't include the leading '#'
    Comment(&'a str),
}

#[derive(PartialEq, Show)]
struct ParseResult<'a> {
    fragment : TomlFragment<'a>,
    remainder : &'a str,
}

fn parse_comment(s : &str) -> Option<ParseResult> {
    if s.is_empty() {
        return None;
    }

    if s.char_at(0) != '#' {
        return None;
    }

    match s.lines().nth(0) {
        Some(comment) => {
            let fragment = TomlFragment::Comment(comment.slice_from(1));
            let remainder = s.slice_from(comment.len());
            
            Some(ParseResult { fragment: fragment, remainder: remainder })
        }
        None => None
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

fn parse_whitespace(s : &str) -> Option<ParseResult> {
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
