use std::{fmt, num::NonZeroUsize};

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    _type: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: NonZeroUsize,
}

#[cfg(test)]
macro_rules! token {
    ($variant:ident, $lexeme:expr, $literal:expr, $line:expr) => {
        Token::new(
            TokenType::$variant,
            $lexeme.to_string(),
            $literal,
            $line.try_into().unwrap(),
        )
    };
}

impl Token {
    pub fn new(
        _type: TokenType,
        lexeme: String,
        literal: Option<Object>,
        line: NonZeroUsize,
    ) -> Self {
        Self {
            _type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.literal.as_ref() {
            Some(literal) => {
                write!(f, "{:?} {} {}", self._type, self.lexeme, literal)
            }
            None => {
                write!(f, "{:?} {}", self._type, self.lexeme)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Object;

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Implement Display for Object")
    }
}
