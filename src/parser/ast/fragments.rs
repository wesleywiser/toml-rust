use tokenizer::TomlFragment;

#[derive(PartialEq, Debug)]
pub enum AstFragment<'a> {
    Boolean(bool),
    Whitespace(&'a str),
    Array(Vec<AstFragment<'a>>),
    Comma,
}

#[derive(PartialEq, Debug)]
pub struct ParseResult<'a> {
    pub number_of_consumed_tokens : usize,

    pub parse_result : Vec<AstFragment<'a>>,
}

pub fn parse_token<'a>(tokens: &'a[TomlFragment]) -> Result<ParseResult<'a>, &'a str> {
    let token = &tokens[0];

    let ast_fragment = 
        match token {
            &TomlFragment::Boolean("true") => AstFragment::Boolean(true),
            &TomlFragment::Boolean("false") => AstFragment::Boolean(false),
            &TomlFragment::Whitespace(s) => AstFragment::Whitespace(s),
            &TomlFragment::Comma => AstFragment::Comma,
            &TomlFragment::BracketOpen => return parse_array(tokens),
            _ => return Err("not implemented"),
        };

    Ok(ParseResult { number_of_consumed_tokens: 1, parse_result: vec![ast_fragment] })
}

pub fn parse_array<'a>(tokens: &'a[TomlFragment]) -> Result<ParseResult<'a>, &'a str> {
    let mut elements = Vec::new();

    let mut seen_opening_bracket  = false;

    let mut index = 0;
    
    let token_count = tokens.iter().count();

    while index < token_count {
        let token = &tokens[index];
        match token {
            &TomlFragment::BracketOpen => 
                if seen_opening_bracket {
                    return Err("Recursive arrays not implemented");
                } else {
                    seen_opening_bracket = true;
                    index = index + 1;
                },
            &TomlFragment::BracketClose =>
                if seen_opening_bracket {
                    let result = vec![AstFragment::Array(elements)];
                    return Ok(ParseResult { number_of_consumed_tokens: index + 1, parse_result: result });
                } else {
                    return Err("Too many closing brackets");
                },
            _ => 
                match parse_token(&tokens[index..]) {
                    Ok(result) => {
                            elements.extend(result.parse_result);
                            index = index + result.number_of_consumed_tokens;
                        },
                    Err(msg) => return Err(msg),
                },
        }
    }

    Err("Unterminated array")
}

#[cfg(test)]
mod test {
    use tokenizer::TomlFragment;
    use super::*;

    #[test]
    fn parse_fragment_whitespace() {
        let input = vec![TomlFragment::Whitespace("   ")];
        let expected = 
            Ok(
                ParseResult {
                    number_of_consumed_tokens: 1,
                    parse_result: vec![AstFragment::Whitespace("   ")]
                });

        assert_eq!(expected, parse_token(&input));
    }

    #[test]
    fn parse_fragment_boolean_true() {
        let input = vec![TomlFragment::Boolean("true")];
        let expected = 
            Ok(
                ParseResult {
                    number_of_consumed_tokens: 1,
                    parse_result: vec![AstFragment::Boolean(true)]
                });

        assert_eq!(expected, parse_token(&input));
    }
    
    #[test]
    fn parse_fragment_boolean_false() {
        let input = vec![TomlFragment::Boolean("false")];
        let expected = 
            Ok(
                ParseResult {
                    number_of_consumed_tokens: 1,
                    parse_result: vec![AstFragment::Boolean(false)]
                });

        assert_eq!(expected, parse_token(&input));
    }

    #[test]
    fn parse_empty_array() {
        let input = vec![TomlFragment::BracketOpen, TomlFragment::BracketClose];
        
        let expected = 
            Ok(
                ParseResult { 
                    number_of_consumed_tokens: 2,
                    parse_result: vec![AstFragment::Array(Vec::new())]
                });

        assert_eq!(expected, parse_array(&input));
    }

    #[test]
    fn parse_empty_array_whitespace() {
        let input = vec![
            TomlFragment::BracketOpen, 
            TomlFragment::Whitespace(" "), 
            TomlFragment::BracketClose
        ];

        let expected = 
            Ok(
                ParseResult {
                    number_of_consumed_tokens: 3,
                    parse_result: vec![AstFragment::Array(vec![AstFragment::Whitespace(" ")])]
                });

        assert_eq!(expected, parse_array(&input));
    }

    #[test]
    fn parse_single_element_array_bool_true() {
        let input = vec![
            TomlFragment::BracketOpen, 
            TomlFragment::Boolean("true"), 
            TomlFragment::BracketClose
        ];

        let expected = 
            Ok(
                ParseResult {
                    number_of_consumed_tokens: 3,
                    parse_result: vec![AstFragment::Array(vec![AstFragment::Boolean(true)])]
                });

        assert_eq!(expected, parse_array(&input));
    }

    #[test]
    fn parse_single_element_array_bool_false() {
        let input = vec![
            TomlFragment::BracketOpen, 
            TomlFragment::Boolean("false"), 
            TomlFragment::BracketClose
        ];

        let expected = 
            Ok(
                ParseResult {
                    number_of_consumed_tokens: 3,
                    parse_result: vec![AstFragment::Array(vec![AstFragment::Boolean(false)])]
                });

        assert_eq!(expected, parse_array(&input));
    }

    #[test]
    fn parse_two_element_array_1() {
        let input = vec![
            TomlFragment::BracketOpen, 
            TomlFragment::Boolean("true"),
            TomlFragment::Comma,
            TomlFragment::Boolean("false"), 
            TomlFragment::BracketClose
        ];

        let expected = 
            Ok(
                ParseResult {
                    number_of_consumed_tokens: 5,
                    parse_result: 
                        vec![
                            AstFragment::Array(
                                vec![
                                    AstFragment::Boolean(true),
                                    AstFragment::Comma,
                                    AstFragment::Boolean(false)
                                ])
                        ]
                });

        assert_eq!(expected, parse_array(&input));
    }
}
