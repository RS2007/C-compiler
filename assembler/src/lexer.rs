use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Semicolon,
    AddInst,
    AsrInst,
    SubInst,
    MulInst,
    DivInst,
    CmpInst,
    MovInst,
    LdrInst,
    StrInst,
    StpInst,
    BInst,
    BlInst,
    CbzInst,
    CbnzInst,
    LSquare,
    RSquare,
    Hash,
    Integer(i64),
    Register(Box<String>),
    Identifier(Box<String>),
    Comma,
    Bang,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) line: u64,
    pub(crate) col: u64,
}

pub struct Lexer {
    input: &'static str,
    pub(crate) tokens: Vec<Token>,
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

fn omit_whitespace(iter: &mut Peekable<Chars<'static>>, col: &mut u64, line: &mut u64) {
    while let Some(&character) = iter.peek() {
        if character.is_whitespace() {
            if character == '\n' {
                *line = *line + 1;
                *col = 1;
            } else {
                *col = *col + 1;
            }
            iter.next();
        } else {
            break;
        }
    }
}

impl Lexer {
    pub fn new(input: &'static str) -> Self {
        return Lexer {
            input: input.trim(),
            tokens: vec![],
        };
    }

    pub fn lex(self: &mut Self) {
        let mut input_iter = self.input.chars().peekable();
        let mut line = 1;
        let mut col = 1;
        'lex_loop: while let Some(&character) = input_iter.peek() {
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
                '[' => {
                    let token = Token {
                        token_type: TokenType::LSquare,
                        line,
                        col,
                    };
                    col = col + 1;
                    self.tokens.push(token);
                    input_iter.next();
                }
                ']' => {
                    let token = Token {
                        token_type: TokenType::RSquare,
                        line,
                        col,
                    };
                    col = col + 1;
                    self.tokens.push(token);
                    input_iter.next();
                }
                '#' => {
                    let token = Token {
                        token_type: TokenType::Hash,
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
                    let token_type = match identifier.as_str() {
                        "add" => TokenType::AddInst,
                        "sub" => TokenType::SubInst,
                        "mul" => TokenType::MulInst,
                        "div" => TokenType::DivInst,
                        "cmp" => TokenType::CmpInst,
                        "mov" => TokenType::MovInst,
                        "ldr" => TokenType::LdrInst,
                        "str" => TokenType::StrInst,
                        "stp" => TokenType::StpInst,
                        "b" => TokenType::BInst,
                        "bl" => TokenType::BlInst,
                        "cbz" => TokenType::CbzInst,
                        "cbnz" => TokenType::CbnzInst,
                        "asr" => TokenType::AsrInst,
                        "x0" => TokenType::Register(Box::new("x0".to_string())),
                        "x1" => TokenType::Register(Box::new("x1".to_string())),
                        "x2" => TokenType::Register(Box::new("x2".to_string())),
                        "x3" => TokenType::Register(Box::new("x3".to_string())),
                        "x4" => TokenType::Register(Box::new("x4".to_string())),
                        "x5" => TokenType::Register(Box::new("x5".to_string())),
                        "x6" => TokenType::Register(Box::new("x6".to_string())),
                        "x7" => TokenType::Register(Box::new("x7".to_string())),
                        "x8" => TokenType::Register(Box::new("x8".to_string())),
                        "x9" => TokenType::Register(Box::new("x9".to_string())),
                        "x10" => TokenType::Register(Box::new("x10".to_string())),
                        "x11" => TokenType::Register(Box::new("x11".to_string())),
                        "x12" => TokenType::Register(Box::new("x12".to_string())),
                        "x13" => TokenType::Register(Box::new("x13".to_string())),
                        "x14" => TokenType::Register(Box::new("x14".to_string())),
                        "x15" => TokenType::Register(Box::new("x15".to_string())),
                        "x16" => TokenType::Register(Box::new("x16".to_string())),
                        "x17" => TokenType::Register(Box::new("x17".to_string())),
                        "x18" => TokenType::Register(Box::new("x18".to_string())),
                        "x19" => TokenType::Register(Box::new("x19".to_string())),
                        "x20" => TokenType::Register(Box::new("x20".to_string())),
                        "x21" => TokenType::Register(Box::new("x21".to_string())),
                        "x22" => TokenType::Register(Box::new("x22".to_string())),
                        "x23" => TokenType::Register(Box::new("x23".to_string())),
                        "x24" => TokenType::Register(Box::new("x24".to_string())),
                        "x25" => TokenType::Register(Box::new("x25".to_string())),
                        "x26" => TokenType::Register(Box::new("x26".to_string())),
                        "x27" => TokenType::Register(Box::new("x27".to_string())),
                        "x28" => TokenType::Register(Box::new("x28".to_string())),
                        "x29" => TokenType::Register(Box::new("x29".to_string())),
                        "x30" => TokenType::Register(Box::new("x30".to_string())),
                        "sp" => TokenType::Register(Box::new("sp".to_string())),
                        "xzr" => TokenType::Register(Box::new("xzr".to_string())),
                        // TODO: is there a better way to structure this to avoid cloning?
                        _ => TokenType::Identifier(identifier),
                    };
                    let token = Token {
                        token_type,
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
                _ => {}
            }
            omit_whitespace(&mut input_iter, &mut col, &mut line);
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
        assert!(lexer
            .tokens
            .iter()
            .zip(expected_toks)
            .map(|(a, b)| { *a == b })
            .all(|x| x));
    }

    #[test]
    fn lex_with_newline() {
        let mut lexer = Lexer::new("32\n-43");
        lexer.lex();
        let expected_toks = vec![
            Token {
                token_type: TokenType::Integer(32),
                line: 1,
                col: 1,
            },
            Token {
                token_type: TokenType::Integer(-43),
                line: 2,
                col: 1,
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
    fn lex_basic_inst() {
        let mut lexer = Lexer::new("add x3,x1,x2");
        lexer.lex();
        let expected_toks = vec![
            Token {
                token_type: TokenType::AddInst,
                line: 1,
                col: 1,
            },
            Token {
                token_type: TokenType::Register(Box::new("x3".to_string())),
                line: 1,
                col: 5,
            },
            Token {
                token_type: TokenType::Comma,
                line: 1,
                col: 7,
            },
            Token {
                token_type: TokenType::Register(Box::new("x1".to_string())),
                line: 1,
                col: 8,
            },
            Token {
                token_type: TokenType::Comma,
                line: 1,
                col: 10,
            },
            Token {
                token_type: TokenType::Register(Box::new("x2".to_string())),
                line: 1,
                col: 11,
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
    fn test_lexing_multiple_inst() {
        let input = "stp     x23, x24, [sp, #48]
mov     x23, x1
mov     x24, x2
cmp     xzr, x20, asr #3
                    ";
        let mut lexer = Lexer::new(input);
        lexer.lex();
        let expected_toks = vec![
            TokenType::StpInst,
            TokenType::Register(Box::new("x23".to_string())),
            TokenType::Comma,
            TokenType::Register(Box::new("x24".to_string())),
            TokenType::Comma,
            TokenType::LSquare,
            TokenType::Register(Box::new("sp".to_string())),
            TokenType::Comma,
            TokenType::Hash,
            TokenType::Integer(48),
            TokenType::RSquare,
            TokenType::MovInst,
            TokenType::Register(Box::new("x23".to_string())),
            TokenType::Comma,
            TokenType::Register(Box::new("x1".to_string())),
            TokenType::MovInst,
            TokenType::Register(Box::new("x24".to_string())),
            TokenType::Comma,
            TokenType::Register(Box::new("x2".to_string())),
            TokenType::CmpInst,
            TokenType::Register(Box::new("xzr".to_string())),
            TokenType::Comma,
            TokenType::Register(Box::new("x20".to_string())),
            TokenType::Comma,
            TokenType::AsrInst,
            TokenType::Hash,
            TokenType::Integer(3),
        ];
        assert!(lexer
            .tokens
            .iter()
            .zip(expected_toks)
            .map(|(a, b)| { a.token_type == b })
            .all(|x| x));
    }
}
