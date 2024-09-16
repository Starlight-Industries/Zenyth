use zenyth::{
    Token,
};
use logos::Logos;

use crate::split_into_chunks;
#[cfg(test)]
#[test]
fn test_lexer_identifiers() {

    let input = "variable_name x y123";
    let tokens: Vec<_> = Token::lexer(input).collect::<Result<_, _>>().unwrap();
    assert_eq!(tokens, vec![Token::Identifier, Token::Identifier, Token::Identifier]);
}

#[test]
fn test_lexer_literals() {
    let input = "42 3.14f true false \"hello\"";
    let tokens: Vec<_> = Token::lexer(input).collect::<Result<_, _>>().unwrap();
    assert_eq!(tokens, vec![Token::Integer, Token::Float, Token::Bool, Token::Bool, Token::String]);
}

#[test]
fn test_lexer_operators() {
    let input = "+ - * / = ; ( ) { } [ ] . , :";
    let tokens: Vec<_> = Token::lexer(input).collect::<Result<_, _>>().unwrap();
    assert_eq!(tokens, vec![
        Token::Plus, Token::Minus, Token::Multiply, Token::Divide, Token::Equals,
        Token::Semicolon, Token::OpenParen, Token::CloseParen, Token::OpenBrace,
        Token::CloseBrace, Token::OpenBracket, Token::CloseBracket, Token::Dot,
        Token::Comma, Token::Colon
    ]);
}

#[test]
fn test_lexer_comments() {
    let input = "x = 5 # This is a comment\ny = 10";
    let tokens: Vec<_> = Token::lexer(input).collect::<Result<_, _>>().unwrap();
    assert_eq!(tokens, vec![
        Token::Identifier, Token::Equals, Token::Integer,
        Token::Comment, Token::Identifier, Token::Equals, Token::Integer
    ]);
}

#[test]
fn test_split_into_chunks() {
    let input = "Hello, World! This is a test.";
    let chunks = split_into_chunks(input, 10);
    assert_eq!(chunks, vec!["Hello, Wor", "ld! This i", "s a test."]);
}
#[cfg(test)]
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

fn simple_random(seed: u64) -> u64 {
        (seed ^ 0x5DEECE66D) & ((1 << 48) - 1)
}

fn random_char(rng: &mut u64) -> char {
    *rng = simple_random(*rng);
    (*rng % 256) as u8 as char
}

    #[test]
fn fuzz_lexer() {
    for i in 0..999 {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos() as u64;

        let mut rng = simple_random(seed);

        let length = (rng % 100) as usize;

        let input: String = (0..length)
            .map(|_| random_char(&mut rng))
            .collect();

        println!("Test {}: Input: {:?}", i, input);

        let tokens = Token::lexer(&input).collect::<Result<Vec<_>, _>>();

        match tokens {
            Ok(t) => {
                println!("Tokens: {:?}", t);
                assert!(true, "Lexer successfully processed input");
            },
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false, "Lexer failed to process input: {:?}", e);
            },
        }
    }
}
