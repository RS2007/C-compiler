use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    IntType,
    BooleanType,
    LParen,
    RParen,
    Integer(i32),
    Identifier(Box<String>),
    String(Box<String>),
    Comma,
    Plus,
    Minus,
    Asterik,
    Illegal,
}

pub struct Token {
    token_type: TokenType,
    line: u64,
    col: u64,
}

pub struct Lexer {
    pub input: &'static str,
    pub tokens: Vec<Token>,
}

trait PeekChar {
    fn peek_char(&mut self) -> Option<char>;
}

impl PeekChar for Peekable<Chars<'_>> {
    fn peek_char(&mut self) -> Option<char> {
        self.peek().copied()
    }
}

#[cfg(test)]
mod tests {}
