use std;

///Lexeme
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum Lexeme<'a>{
    EOF,
    Ident(&'a str),
    Number(u64),
    String(&'a str),
    Operator(char),
    Bracket(char),
    Colon,
    Semicolon,
    Comma,
    Dot,
    //NewLine, ??
}

impl<'a> std::fmt::Display for Lexeme<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Lexeme::EOF => write!(f, "'EOF'"),
            Lexeme::Ident(s) => write!(f, "\"{}\"",s),
            Lexeme::Number(d) => write!(f, "{}",d),
            Lexeme::String(s) => write!(f, "\"{}\"",s),
            Lexeme::Operator(c) => write!(f, "'{}'",c),
            Lexeme::Bracket(c) => write!(f, "'{}'",c),
            Lexeme::Colon => write!(f, "':'"),
            Lexeme::Semicolon => write!(f, "';'"),
            Lexeme::Comma => write!(f, "','"),
            Lexeme::Dot => write!(f, "'.'"),
            //Lexeme::NewLine => write!(f, "'\n'"),
        }
    }
}
