
#[derive(PartialEq, Debug)]
pub enum TomlFragment<'a> {
    Whitespace(&'a str),
    
    //doesn't include the leading '#'
    Comment(&'a str),

    Boolean(&'a str),

    //the tokenizer doesn't check if this will fit into an i64
    Integer(&'a str),

    //may contain doted names (ex, "test.abc")
    Table(&'a str),

    BracketOpen,

    BracketClose,

    Comma,
}

#[derive(PartialEq, Debug)]
pub struct ParseResult<'a> {
    pub fragment : TomlFragment<'a>,
    pub remainder : &'a str,
}
