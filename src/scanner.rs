use core::fmt;
use std::num::NonZeroUsize;

use crate::Lox;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: NonZeroUsize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: unsafe { NonZeroUsize::new_unchecked(1) },
        }
    }

    pub fn scan_tokens(&mut self) -> &[Token] {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        macro_rules! match_second {
            ($c:expr, $if_true:ident, $if_false:ident) => {
                if self._match($c) {
                    TokenType::$if_true
                } else {
                    TokenType::$if_false
                }
            };
        }

        match c {
            b'(' => self.add_token(TokenType::LeftParen, None),
            b')' => self.add_token(TokenType::RightParen, None),
            b'{' => self.add_token(TokenType::LeftBrace, None),
            b'}' => self.add_token(TokenType::RightBrace, None),
            b',' => self.add_token(TokenType::Comma, None),
            b'.' => self.add_token(TokenType::Dot, None),
            b'-' => self.add_token(TokenType::Minus, None),
            b'+' => self.add_token(TokenType::Plus, None),
            b';' => self.add_token(TokenType::Semicolon, None),
            b'*' => self.add_token(TokenType::Star, None),
            b'!' => {
                let t = match_second!(b'=', BangEqual, Bang);
                self.add_token(t, None);
            }
            b'=' => {
                let t = match_second!(b'=', EqualEqual, Equal);
                self.add_token(t, None);
            }
            b'<' => {
                let t = match_second!(b'=', LessEqual, Less);
                self.add_token(t, None);
            }
            b'>' => {
                let t = match_second!(b'=', GreaterEqual, Greater);
                self.add_token(t, None);
            }
            t => {
                Lox::error(self.line, format!("Unexpected character: {t}"));
            }
        }
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;

        self.source_current_char()
    }

    // NOTE: This isn't in the book; it's a shortcut for `source.charAt()`.
    //
    fn source_current_char(&self) -> u8 {
        *self
            .source
            .as_bytes()
            .get(self.current)
            .expect("`current` exceeded the length of `source`")
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Object>) {
        let text = self
            .source
            .get(self.start..self.current)
            .expect("`start` and/or `current` are invalid `source` bounds");

        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line))
    }

    fn _match(&mut self, c: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source_current_char() != c {
            return false;
        }

        self.current += 1;

        true
    }
}

#[derive(Debug)]
pub struct Token {
    _type: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: NonZeroUsize,
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Object;

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Implement Display for Object")
    }
}
