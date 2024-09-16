use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f\r]+")] // Ignore this regex pattern between tokens
pub enum Token {
    // Identifiers and literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex("[0-9]+")]
    Integer,

    #[regex(r"[0-9]+\.[0-9]+f")]
    Float,

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

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,
    
    #[token(".")]
    Dot,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token("@")]
    At,

    // String literals
    #[regex(r#""([^"\\]|\\.)*""#)]
    String,

    // Comments (ignoring them)
    #[regex(r"#.*")]
    Comment,

    #[regex(r"@[a-zA-Z_][a-zA-Z0-9_]*", priority = 3)]
    Decorator,
}