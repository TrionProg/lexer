use std;

///Describes where error has been occured
#[derive(Debug)]
pub struct Line{
    pub line_number:usize,
    pub column:usize,
    pub line:String,
}

impl Line{
    pub fn new(line_number:usize,column:usize,line:&str) -> Line{
        Line{
            line_number:line_number,
            column:column,
            line:String::from(line),
        }
    }
}

impl std::fmt::Display for Line{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Line:{}, col:{}\n{}", self.line_number, self.column, self.line)
    }
}
