extern crate lexer;
use lexer::stream_lexer::{Cursor,Lexeme};





#[test]
fn print_lexemes_with_fragments_of_code(){
    let text="let mut cursor=Cursor::new(text.as_ref());

    loop{
        let lex=cursor.next_lex().unwrap();

        if lex==Lexeme::EOF {
            break;
        }

        println!(\"{} \n {}\",lex,cursor.get_fragment());
    }
    ";

    let mut cursor=Cursor::new(text.as_ref());

    loop{
        let lex=cursor.next_lex().unwrap();

        if lex==Lexeme::EOF {
            break;
        }

        println!("{} \n {}",lex,cursor.get_fragment());
    }
} 
