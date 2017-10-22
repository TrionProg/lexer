use std;
use std::str::CharIndices;

use super::Lexeme;
use Error;
use Fragment;
use Line;

///The cursor over text, calling method next_lex you will get lexemes one by one, you can not get previous lexemes.
///If you want have access to previous lexemes, you should use iterator over Vec<Lexeme>. It will be added later.
pub struct Cursor<'a>{
    pub text:&'a str,

    //current positions of cursor
    cur_it:CharIndices<'a>,
    cur_pos:usize,//position of current char in bytes
    cur_char:char,
    cur_line_number:usize,
    cur_line_pos:usize,//position of line begining in bytes
    cur_column:usize,//position of char from beginning of line

    //lexeme description
    pub line_number:usize, //number of line
    pub line_pos:usize, //position of line
    pub column:usize,
    pub pos:usize, //position of lexeme in bytes
    pub lex:Lexeme<'a>,
}

impl<'a>Cursor<'a>{
    ///Creates Cursor for given text. Lexemes can not live more then text!
    pub fn new( text:&'a str ) -> Cursor<'a>{
        let mut cursor=Cursor {
            text:text,

            cur_it:text.char_indices(),
            cur_pos:0,
            cur_char:'\0',
            cur_line_number:1,
            cur_line_pos:0,
            cur_column:0,

            line_number:0,
            line_pos:0,
            column:0,
            pos:0,
            lex:Lexeme::EOF,
        };

        cursor.next_char();

        cursor
    }

    ///Returns fragment of text between line,where current(!) lexeme begins and end of line where lexeme ends
    pub fn get_fragment(&self) -> Fragment<'a>{
        //NOTE:Lexeme end is cur_pos

        let (a,bc)=self.text.split_at(self.line_pos);
        let line_char_pos=self.cur_pos-self.line_pos;

        let mut fragment_end=None;

        for (i,c) in bc.char_indices(){

            if c=='\n' && i>=line_char_pos {
                fragment_end=Some(i);
                break;
            }
        }

        let fragment_text=match fragment_end{
            Some( fragment_end ) => {
                let (b,c) = bc.split_at( fragment_end );
                b
            },
            None => bc,
        };

        Fragment::new(self.line_number,fragment_text)
    }

    ///Returns current line. It must be called if error occurs
    pub fn get_line(&self) -> Line{
        let (a,bc)=self.text.split_at(self.cur_line_pos);

        let line_char_pos=self.cur_pos-self.cur_line_pos;

        let mut line_end=None;

        for (i,c) in bc.char_indices() {
            if c=='\n' {
                line_end=Some(i);
                break;
            }
        }

        let line=match line_end{
            Some( line_end ) => {
                let (b,c) = bc.split_at( line_end );
                b
            },
            None => bc,
        };

        Line::new(self.line_number, self.column, line)
    }

    //Return fragment of text between start and end
    fn get_part(&self, start:usize, end:usize) -> &'a str{
        let (a,bc)=self.text.split_at( start );
        let (b,c) = bc.split_at( end-start );
        b
    }

    fn next_char(&mut self) -> char{
        match self.cur_it.next(){
            None => {
                if self.cur_char!='\0' {
                    self.cur_column+=1;
                    self.cur_pos+=1;//size of '\0' is 1 byte
                    self.cur_char='\0';
                }
            },
            Some( (pos, ch) ) => {
                self.cur_column+=1;
                self.cur_pos=pos;
                self.cur_char=ch;
            },
        }

        self.cur_char
    }

    ///Returns next lexeme, after each call cursor.cur_char is next char after lexeme
    pub fn next_lex(&mut self) -> Result<Lexeme<'a>, Error>{
        //NOTE: processing of \n must be later then next_char to save current line
        //NOTE: no shielding
        //IDEA: for optimization this perations may executed with local copy of cursor

        loop{
            //skip spaces
            loop{
                match self.cur_char {
                    '\n' => {
                        self.cur_line_pos=self.cur_pos+1;
                        self.cur_line_number+=1;
                        self.cur_column=0;
                    },
                    ' ' | '\t' | '\r' => {},
                    _ => break,//non space symbal has been found
                }

                self.next_char();
            }

            //save beginning of location
            self.line_pos=self.cur_line_pos;
            self.line_number=self.cur_line_number;
            self.pos=self.cur_pos;
            self.column=self.cur_column;

            if self.cur_char.is_digit(10) {

                while self.next_char().is_digit(10) {}

                match self.cur_char{
                    'x' => {
                        if self.get_part(self.pos,self.cur_pos)!="0" {
                            return Err( Error::Other(self.get_line(), String::from("Format of hexadecimal must be like 0xAF")) );
                        }

                        let mut length=0;
                        let mut value:u64=0;
                        loop{
                            while self.next_char().is_digit(16){
                                value*=16;
                                if self.cur_char.is_alphabetic(){
                                    value+=self.cur_char.to_lowercase().next().unwrap() as u64 - 'a' as u64 + 10;
                                }else{
                                    value+=self.cur_char as u64 - '0' as u64;
                                }

                                length+=1;
                            }

                            if self.cur_char != '_' && self.cur_char != ' ' {
                                break;
                            }
                        }

                        if length==0 {
                            return Err( Error::Other(self.get_line(), String::from("After 0x of hecademical must be letter 0 to F, write 0x0 if you want zero value")) );
                        }else{
                            self.lex=Lexeme::Number(value);
                            return Ok( self.lex.clone() );
                        }
                    },
                    _ => {
                        let value=match self.get_part(self.pos,self.cur_pos).parse::<u64>() {
                            Ok( v ) => v,
                            Err( e ) => return Err( Error::ParseNumberError(self.get_line(), String::from(self.get_part(self.pos,self.cur_pos)), e) ),
                        };

                        self.lex=Lexeme::Number(value);
                        return Ok( self.lex.clone() );
                    },
                }
            }else if self.cur_char.is_alphabetic() || self.cur_char=='_'{
                while self.next_char().is_alphabetic() || self.cur_char.is_digit(10) || self.cur_char=='_' {}

                self.lex=Lexeme::Ident(self.get_part(self.pos,self.cur_pos));
                return Ok( self.lex.clone() );
            }else{
                match self.cur_char{
                    '/'=>{
                        match self.next_char(){
                            '/'=>{
                                loop{
                                    match self.next_char(){
                                        '\0'=>{
                                            self.lex=Lexeme::EOF;
                                            return Ok( self.lex.clone() );
                                        },
                                        '\n'=>{
                                            self.cur_line_pos=self.cur_pos+1;
                                            self.cur_line_number+=1;
                                            self.cur_column=0;
                                            break;
                                        }
                                        _=>{},
                                    }
                                }

                                self.next_char();
                            },
                            '*'=>{
                                let mut counter=1;
                                loop{
                                    match self.next_char(){
                                        '\0'=>return Err( Error::UnexpectedEOF(self.get_line(),"*/") ),
                                        '\n'=>{
                                            self.cur_line_pos=self.cur_pos+1;
                                            self.cur_line_number+=1;
                                            self.cur_column=0;
                                        },
                                        '/'=>{
                                            if self.next_char()=='*' {
                                                counter+=1;
                                            }
                                        },
                                        '*'=>{
                                            if self.next_char()=='/' {
                                                counter-=1;

                                                if counter==0 {
                                                    break;
                                                }
                                            }
                                        },
                                        _=>{}
                                    }
                                }

                                self.next_char();
                            },
                            _=>{
                                self.lex=Lexeme::Operator('/');
                                return Ok( self.lex.clone() );
                            },
                        }
                    },
                    '-' | '+' | '*' | '%' | '^' | '=' | '<' | '>' | '!' | '|' | '&' =>{
                        self.lex=Lexeme::Operator(self.cur_char);
                        self.next_char();
                        return Ok( self.lex.clone() );
                    },
                    '['|']'|'{'|'}'|'('|')'=>{
                        self.lex=Lexeme::Bracket(self.cur_char);
                        self.next_char();
                        return Ok( self.lex.clone() );
                    },
                    ','=>{
                        self.lex=Lexeme::Comma;
                        self.next_char();
                        return Ok( self.lex.clone() );
                    },
                    ':'=>{
                        self.lex=Lexeme::Colon;
                        self.next_char();
                        return Ok( self.lex.clone() );
                    }
                    ';'=>{
                        self.lex=Lexeme::Semicolon;
                        self.next_char();
                        return Ok( self.lex.clone() );
                    },
                    '"' | '\''=>{
                        let finising_quote=self.cur_char;
                        let mut is_shielding=false;

                        loop{
                            self.next_char();

                            if self.cur_char=='\0' {
                                return Err( Error::UnexpectedEOF(self.get_line(),
                                    match finising_quote{
                                        '\'' => "\\",
                                        '"'  => "\"",
                                        _ => {unreachable!();}
                                    }
                                ) );
                            }else if is_shielding {
                                is_shielding=false;
                            }else{
                                match self.cur_char {
                                    '"'=>{
                                        if self.cur_char=='"' {
                                            break;
                                        }
                                    },
                                    '\''=>{
                                        if self.cur_char=='\'' {
                                            break;
                                        }
                                    },
                                    '\\'=>is_shielding=true,
                                    _=>{},
                                }
                            }
                        }

                        self.lex=Lexeme::String(self.get_part(self.pos+1, self.cur_pos));
                        self.next_char();
                        return Ok( self.lex.clone() );
                    },
                    '.'=>{
                        self.lex=Lexeme::Dot;
                        self.next_char();
                        return Ok( self.lex.clone() );
                    },
                    '\0' => {
                        self.lex=Lexeme::EOF;
                        return Ok( self.lex.clone() );
                    },
                    _ => return Err( Error::UnexpectedSymbal(self.get_line(), self.cur_char) ),
                }
            }
        }
    }
}


//TODO:add iterator:struct Iter for cursor(to avoid moving)
/*
impl<'a> std::iter::Iterator for Cursor<'a>{
    type Item=Result<Lexeme<'a>,Error>;

    fn next(&mut self) -> Option<Self::Item>{
        match self.next_lex(){
            Ok( lex ) => {
                if lex==Lexeme::EOF {
                    None
                }else{
                    Some(Ok(lex))
                }
            },
            Err( e ) => Some( Err(e) ),
        }
    }
}
*/
