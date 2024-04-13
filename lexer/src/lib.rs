use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    IntType,
    BooleanType,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Integer(i32),
    Identifier(Box<String>),
    String(Box<String>),
    Return,
    Comma,
    Plus,
    Minus,
    Semicolon,
    Multiply,
    Illegal,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    token_type: TokenType,
    line: u64,
    col: u64,
}

pub struct Lexer {
    pub input: &'static str,
    pub tokens: Vec<Token>,
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

fn omit_whitespace(input: &mut Peekable<Chars>, col: &mut u64, line: &mut u64) {
    while let Some(&character) = input.peek() {
        match character {
            ' ' | '\t' => {
                input.next();
                *col += 1;
            }
            '\n' => {
                input.next();
                *col = 1;
                *line += 1;
            }
            '\r' => unreachable!(),
            _ => break,
        }
    }
}

impl Lexer {
    pub fn new(input: &'static str) -> Self {
        Lexer {
            input: input.trim(),
            tokens: vec![],
        }
    }

    pub fn lex(self: &mut Self) {
        let mut input_iter = self.input.chars().peekable();
        let mut line = 1;
        let mut col = 1;
        while let Some(&character) = input_iter.peek() {
            match character {
                '0'..='9' => {
                    let old_col = col;
                    let number = lex_number(&mut input_iter, &mut col);
                    self.tokens.push(Token {
                        token_type: TokenType::Integer(number as i32),
                        col: old_col,
                        line,
                    });
                }
                'a'..='z' | 'A'..='Z' => {
                    let old_col = col;
                    let identifier = lex_identifier(&mut input_iter, &mut col);
                    match (*identifier).as_str() {
                        "return" => {
                            self.tokens.push(Token {
                                token_type: TokenType::Return,
                                line,
                                col: old_col,
                            });
                        }
                        "int" => {
                            self.tokens.push(Token {
                                token_type: TokenType::IntType,
                                line,
                                col: old_col,
                            });
                        }
                        _ => {
                            self.tokens.push(Token {
                                token_type: TokenType::Identifier(identifier),
                                line,
                                col: old_col,
                            });
                        }
                    }
                }
                '(' => {
                    self.tokens.push(Token {
                        token_type: TokenType::LParen,
                        line,
                        col,
                    });
                    input_iter.next();
                    col += 1;
                }
                ')' => {
                    self.tokens.push(Token {
                        token_type: TokenType::RParen,
                        line,
                        col,
                    });
                    input_iter.next();
                    col += 1;
                }
                '{' => {
                    self.tokens.push(Token {
                        token_type: TokenType::LBrace,
                        line,
                        col,
                    });
                    input_iter.next();
                    col += 1;
                }
                '}' => {
                    self.tokens.push(Token {
                        token_type: TokenType::RBrace,
                        line,
                        col,
                    });
                    input_iter.next();
                    col += 1;
                }
                '+' => {
                    self.tokens.push(Token {
                        token_type: TokenType::Plus,
                        line,
                        col,
                    });
                    input_iter.next();
                    col += 1;
                }
                '-' => {
                    let old_col = col;
                    input_iter.next();
                    if ('0'..='9').contains(input_iter.peek().unwrap()) {
                        let number = lex_number(&mut input_iter, &mut col);
                        self.tokens.push(Token {
                            token_type: TokenType::Integer(-1 * number as i32),
                            line,
                            col: old_col,
                        });
                    } else {
                        self.tokens.push(Token {
                            token_type: TokenType::Minus,
                            line,
                            col,
                        });
                    }
                    col += 1;
                }
                ';' => {
                    self.tokens.push(Token {
                        token_type: TokenType::Semicolon,
                        line,
                        col,
                    });
                    input_iter.next();
                    col += 1;
                }
                '*' => {
                    self.tokens.push(Token {
                        token_type: TokenType::Multiply,
                        line,
                        col,
                    });
                    input_iter.next();
                    col += 1;
                }
                _ => {
                    println!("char is {}", character);
                    unreachable!();
                }
            }
            omit_whitespace(&mut input_iter, &mut col, &mut line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_parens_arith() {
        let mut lexer = Lexer::new("( )+ -* ");
        lexer.lex();
        let expected_tokens = vec![
            Token {
                token_type: TokenType::LParen,
                line: 1,
                col: 1,
            },
            Token {
                token_type: TokenType::RParen,
                line: 1,
                col: 3,
            },
            Token {
                token_type: TokenType::Plus,
                line: 1,
                col: 4,
            },
            Token {
                token_type: TokenType::Minus,
                line: 1,
                col: 6,
            },
            Token {
                token_type: TokenType::Multiply,
                line: 1,
                col: 7,
            },
        ];
        assert!(lexer
            .tokens
            .iter()
            .zip(expected_tokens.iter())
            .all(|(a, b)| *a == *b));
    }

    #[test]
    fn test_lex_neg_number() {
        let mut lexer = Lexer::new("-1 - 3");
        lexer.lex();
        let expected_tokens = vec![
            Token {
                token_type: TokenType::Integer(-1),
                line: 1,
                col: 1,
            },
            Token {
                token_type: TokenType::Minus,
                line: 1,
                col: 4,
            },
            Token {
                token_type: TokenType::Integer(3),
                line: 1,
                col: 6,
            },
        ];
        assert!(lexer
            .tokens
            .iter()
            .zip(expected_tokens)
            .all(|(a, b)| *a == b));
    }

    #[test]
    fn test_basic_main() {
        let mut lexer = Lexer::new(
            "int main(){
            return -32;
        }",
        );
        lexer.lex();
        let expected_token_types = vec![
            TokenType::IntType,
            TokenType::Identifier(Box::new("main".to_string())),
            TokenType::LParen,
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::Return,
            TokenType::Integer(-32),
            TokenType::Semicolon,
            TokenType::RBrace,
        ];
        assert!(lexer
            .tokens
            .iter()
            .zip(expected_token_types.iter())
            .all(|(a, b)| a.token_type == *b));
    }
}
