use core::error;
use std::fs::read_to_string;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f\r]+")] // Ignore this regex pattern between tokens
enum Token {
    // Identifiers and literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex("[0-9]+")]
    Integer,

    #[regex(r"[0-9]+\.[0-9]+f")]
    Float,

    // #[regex(r"\[.*?\]")]
    // Array,

    #[regex("(?i)(true|false)(?-i)")]
    Bool,

    // Operators and symbols
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("=")]
    Equals,

    #[token(";")]
    Semicolon,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,
    
    #[token(".")]
    Dot,

    
    // String literals
    #[regex(r#""([^"\\]|\\.)*""#)]
    String,

    // Comments (ignoring them)
    #[regex(r"#.*")]
    Comment,
}

fn main() {
    let file = get_input_file();

    let mut lex = Token::lexer(&file);

    while let Some(token) = lex.next() {
        match token {
            Ok(Token::String) => {
                let span = lex.span();
                eprintln!("Unexpected token: {:?}", &file[span]);
            }
            _ => println!("{:?}: {}", token, &file[lex.span()]),
        }
    }
}
// returns file of multiple idenifiers
fn get_input_file() -> String {
    let file_endings = ["zen", "z", "zenyth", "znyth"];
    let mut file_contents: Result<String, std::io::Error> = Ok("".to_string());
    for ending in file_endings {
        let file_name = format!("input.{}", ending);
        file_contents = read_to_string(file_name);
        if file_contents.is_ok() {
            println!("{:?}", file_contents);
            break;
        }
    }
    return file_contents.unwrap();
}