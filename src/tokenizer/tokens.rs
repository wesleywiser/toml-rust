
#[derive(PartialEq, Show)]
pub enum TomlFragment<'a> {
    Whitespace(&'a str),
    
    //doesn't include the leading '#'
    Comment(&'a str),
}

#[derive(PartialEq, Show)]
pub struct ParseResult<'a> {
    pub fragment : TomlFragment<'a>,
    pub remainder : &'a str,
}
