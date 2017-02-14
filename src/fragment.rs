use std;

///Describes where is lexeme
pub struct Fragment<'a>{
    pub line_number:usize,
    pub fragment:&'a str,
}

impl<'a> Fragment<'a>{
    pub fn new(line_number:usize,fragment:&'a str) -> Fragment{
        Fragment{
            line_number:line_number,
            fragment:fragment,
        }
    }
}

impl<'a> std::fmt::Display for Fragment<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Line:{}\n{}", self.line_number, self.fragment)
    }
}
