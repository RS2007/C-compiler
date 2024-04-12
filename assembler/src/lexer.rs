use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Semicolon,
    AddInst,
    SubInst,
    MulInst,
    DivInst,
    MovInst,
    LdrInst,
    StrInst,
    BInst,
    BlInst,
    CbzInst,
    CbnzInst,
    LSquare,
    RSquare,
    Hash,
    Integer(i64),
    Identifier(Box<String>),
    Comma,
    Bang,
}

#[derive(Debug, PartialEq)]
struct Token {
    token_type: TokenType,
    line: u64,
    col: u64,
}

pub struct Lexer {
    input: &'static str,
    tokens: Vec<Token>,
}

fn lex_identifier(iter: &mut Peekable<Chars<'static>>, col: &mut u64) -> Box<String> {
    let mut string = String::new();
    string += &iter.next().unwrap().to_string();
    *col = *col + 1;
    while let Some(&character) = iter.peek() {
        if character.is_alphanumeric() {
            string += &character.to_string();
            *col = *col + 1;
            iter.next();
        } else {
            break;
        }
    }
    Box::new(string)
}

fn lex_number(iter: &mut Peekable<Chars<'static>>, col: &mut u64) -> i64 {
    let mut accum: u64 = 0;
    let is_neg = if *iter.peek().unwrap() == '-' {
        iter.next();
        true
    } else {
        false
    };
    while let Some(&character) = iter.peek() {
        println!("{}", character);
        if character.is_numeric() {
            accum = accum * 10 + character.to_digit(10).unwrap() as u64;
            *col = *col + 1;
            iter.next();
        } else {
            break;
        }
    }
    if is_neg {
        -1 * accum as i64
    } else {
        accum as i64
    }
}

impl Lexer {
    pub fn new(input: &'static str) -> Self {
        return Lexer {
            input,
            tokens: vec![],
        };
    }

    pub fn lex(self: &mut Self) {
        let mut input_iter = self.input.chars().peekable();
        let mut line = 1;
        let mut col = 1;
        while let Some(&character) = input_iter.peek() {
            match character {
                ',' => {
                    let token = Token {
                        token_type: TokenType::Comma,
                        line,
                        col,
                    };
                    col = col + 1;
                    self.tokens.push(token);
                    input_iter.next();
                }
                'a'..='z' | 'A'..='Z' => {
                    let prev_col = col.clone();
                    let identifier = lex_identifier(&mut input_iter, &mut col);
                    let token = Token {
                        token_type: TokenType::Identifier(identifier),
                        line,
                        col: prev_col,
                    };
                    self.tokens.push(token);
                }
                '0'..='9' | '-' => {
                    let prev_col = col.clone();
                    let num = lex_number(&mut input_iter, &mut col);
                    let token = Token {
                        token_type: TokenType::Integer(num),
                        line,
                        col: prev_col,
                    };
                    self.tokens.push(token);
                }
                '\n' => line += 1,
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lex_number_comma() {
        let mut lexer = Lexer::new("32,-43");
        lexer.lex();
        let expected_toks = vec![
            Token {
                token_type: TokenType::Integer(32),
                line: 1,
                col: 1,
            },
            Token {
                token_type: TokenType::Comma,
                line: 1,
                col: 3,
            },
            Token {
                token_type: TokenType::Integer(-43),
                line: 1,
                col: 4,
            },
        ];
        assert!(lexer
            .tokens
            .iter()
            .zip(expected_toks)
            .map(|(a, b)| { *a == b })
            .all(|x| x));
    }

    #[test]
    fn lex_identifier_number() {
        let mut lexer = Lexer::new("hello,-12");
        lexer.lex();
        let expected_toks = vec![
            Token {
                token_type: TokenType::Identifier(Box::new("hello".to_string())),
                line: 1,
                col: 1,
            },
            Token {
                token_type: TokenType::Comma,
                line: 1,
                col: 6,
            },
            Token {
                token_type: TokenType::Integer(-12),
                line: 1,
                col: 7,
            },
        ];
        println!("{:?}", lexer.tokens);
        assert!(lexer
            .tokens
            .iter()
            .zip(expected_toks)
            .map(|(a, b)| { *a == b })
            .all(|x| x));
    }
}
