
#[derive(PartialEq, Show)]
pub enum TomlFragment<'a> {
    Whitespace(&'a str),
    
    //doesn't include the leading '#'
    Comment(&'a str),

    Boolean(&'a str),

    //the tokenizer doesn't check if this will fit into an i64
    Integer(&'a str),
}

#[derive(PartialEq, Show)]
pub struct ParseResult<'a> {
    pub fragment : TomlFragment<'a>,
    pub remainder : &'a str,
}
