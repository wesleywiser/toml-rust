use tokenizer::TomlFragment;

#[derive(PartialEq, Debug)]
pub enum AstFragment<'a> {
    Boolean(bool),
    Whitespace(&'a str),
}

pub fn parse_fragment(tokens: Vec<TomlFragment>) -> Result<Vec<AstFragment>, &str> {
    let mut fragments = Vec::with_capacity(tokens.len() / 2);

    for token in tokens.iter() {
        fragments.push(
            match token {
                &TomlFragment::Boolean("true") => AstFragment::Boolean(true),
                &TomlFragment::Boolean("false") => AstFragment::Boolean(false),
                &TomlFragment::Whitespace(s) => AstFragment::Whitespace(s),
                _ => return Err("not implemented"),
            });
    }

    Ok(fragments)
}

#[cfg(test)]
mod test {
    use tokenizer::TomlFragment;
    use super::*;

    #[test]
    fn parse_fragment_whitespace() {
        let expected = Ok(vec![AstFragment::Whitespace("   ")]);
        let input = vec![TomlFragment::Whitespace("   ")];
        assert_eq!(expected, parse_fragment(input));
    }

    #[test]
    fn parse_fragment_boolean_true() {
        let expected = Ok(vec![AstFragment::Boolean(true)]);
        let input = vec![TomlFragment::Boolean("true")];
        assert_eq!(expected, parse_fragment(input));
    }
    
    #[test]
    fn parse_fragment_boolean_false() {
        let expected = Ok(vec![AstFragment::Boolean(false)]);
        let input = vec![TomlFragment::Boolean("false")];
        assert_eq!(expected, parse_fragment(input));
    }
}
