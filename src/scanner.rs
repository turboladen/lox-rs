#[macro_use]
pub mod token;

use std::num::NonZeroUsize;

use token::{Object, Token, TokenType};

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

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));

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
            b'/' => {
                // Need to handle comments (which are double-slash)
                if self._match(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
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

    /// Consumes the next character in the source file and returns it. Where `advance()` is for
    /// input, `add_token()` is for output.
    ///
    fn advance(&mut self) -> u8 {
        // NOTE: Java's `++` increments, but returns the old value!
        // ex:
        // int i = 3;
        // int a = i++; // a = 3, i = 4
        let old_current = self.current;
        self.current += 1;

        self.unchecked_char_at(old_current)
    }

    /// Like `advance()` but doesn't consume a character.
    ///
    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }

        self.unchecked_char_at(self.current)
    }

    // NOTE: This isn't in the book; it's a shortcut for `source.charAt()`.
    //
    fn unchecked_char_at(&self, offset: usize) -> u8 {
        *self.source.as_bytes().get(offset).expect(&format!(
            "`offset` ({}) exceeded the length of `source` ({})",
            self.current,
            self.source.len()
        ))
    }

    /// Graps the text of the current lexeme and creates a new token for it.
    ///
    fn add_token(&mut self, token_type: TokenType, literal: Option<Object>) {
        let text = self
            .source
            .get(self.start..self.current)
            .expect("`start` and/or `current` are invalid `source` bounds");

        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line))
    }

    /// Like a conditional `advance()`. We only consume the current character if it's what we're
    /// looking for.
    ///
    fn _match(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.unchecked_char_at(self.current) != expected {
            return false;
        }

        self.current += 1;

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_at() {
        let token = "(";
        let scanner = Scanner::new(token.to_string());
        assert_eq!(scanner.unchecked_char_at(0), b'(');

        let token = "()";
        let scanner = Scanner::new(token.to_string());
        assert_eq!(scanner.unchecked_char_at(1), b')');
    }

    #[test]
    fn test_scan_single_tokens() {
        macro_rules! test_single_token {
            ($lexeme:expr, $variant:ident) => {
                let mut scanner = Scanner::new($lexeme.to_string());
                let tokens = scanner.scan_tokens();
                assert_eq!(
                    &[token!($variant, $lexeme, None, 1), token!(Eof, "", None, 1),],
                    tokens
                );
            };
        }
        test_single_token!("(", LeftParen);
        test_single_token!(")", RightParen);
        test_single_token!("{", LeftBrace);
        test_single_token!("}", RightBrace);
        test_single_token!(",", Comma);
        test_single_token!(".", Dot);
        test_single_token!("-", Minus);
        test_single_token!("+", Plus);
        test_single_token!(";", Semicolon);
        test_single_token!("/", Slash);
        test_single_token!("*", Star);

        test_single_token!("!", Bang);
        test_single_token!("=", Equal);
        test_single_token!(">", Greater);
        test_single_token!("<", Less);
    }

    #[test]
    fn test_scan_double_tokens() {
        macro_rules! test_double_token {
            ($lexeme:expr, $variant:ident) => {
                let mut scanner = Scanner::new($lexeme.to_string());
                let tokens = scanner.scan_tokens();
                assert_eq!(
                    &[token!($variant, $lexeme, None, 1), token!(Eof, "", None, 1)],
                    tokens
                );
            };
        }
        test_double_token!("!=", BangEqual);
        test_double_token!("==", EqualEqual);
        test_double_token!(">=", GreaterEqual);
        test_double_token!("<=", LessEqual);
    }
}
