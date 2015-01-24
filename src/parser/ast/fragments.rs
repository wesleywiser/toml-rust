use tokenizer::TomlFragment;

#[derive(PartialEq, Show)]
pub enum AstFragment {
    Boolean(bool),
}

pub fn parse_fragment(tokens: Vec<TomlFragment>) -> Result<Vec<AstFragment>, &str> {
    let mut fragments = Vec::with_capacity(tokens.len() / 2);

    for token in tokens.iter() {
        fragments.push(
            match token {
                &TomlFragment::Boolean("true") => AstFragment::Boolean(true),
                &TomlFragment::Boolean("false") => AstFragment::Boolean(false),
                _ => return Err("not implemented".as_slice()),
            });
    }

    Ok(fragments)
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
