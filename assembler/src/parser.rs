use crate::lexer::{Token, TokenType};
pub struct Parser {
    tokens: Vec<Token>,
}

pub struct Program {
    pub(crate) instructions: Vec<Instruction>,
}
pub struct Instruction {
    pub(crate) label: Option<Token>,
    pub(crate) opcode: Token,
    pub(crate) operands: Vec<Token>,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub(crate) fn parse_program(self: &mut Self) -> Result<Program, ()> {
        let mut instructions: Vec<Instruction> = vec![];
        while self.tokens.len() > 0 {
            let instruction = self.parse_instruction()?;
            instructions.push(instruction);
        }
        Ok(Program { instructions })
    }

    fn parse_optional_label(self: &mut Self) -> Result<Option<Token>, ()> {
        if let Some(token) = self.tokens.get(0) {
            // TODO: this seems to be weird
            return Ok(match &token.token_type {
                TokenType::Identifier(_) => Some((*token).clone()),
                _ => None,
            });
        }
        Err(eprintln!("Error during parsing"))
    }

    fn parse_opcode(self: &mut Self) -> Result<Token, ()> {
        if let Some(token) = self.tokens.get(0) {
            return match token.token_type {
                TokenType::AddInst => Ok(self.tokens.remove(0)),
                _ => Err(eprintln!(
                    "Expected an instruction at line: {} and col: {}, got {:?}",
                    token.line, token.col, token.token_type
                )),
            };
        }
        Err(eprintln!("Error during parsing"))
    }

    fn parse_operands_rest(self: &mut Self) -> Result<Vec<Token>, ()> {
        //TODO: reimplement(incorrect)
        let mut operands: Vec<Token> = vec![];
        while self.tokens.len() > 0 {
            if let Some(token) = self.tokens.get(0) {
                match token.token_type {
                    TokenType::Comma => {
                        self.tokens.remove(0);
                        let operand = self.parse_operand()?;
                        operands.push(operand);
                    }
                    _ => break,
                }
            }
        }
        Ok(operands)
    }

    fn parse_operand(self: &mut Self) -> Result<Token, ()> {
        let first_tok = self.tokens.remove(0);
        match first_tok.token_type {
            TokenType::Register(_) => Ok(first_tok),
            TokenType::Hash => unimplemented!(),
            TokenType::LSquare => unimplemented!(),
            _ => {
                return Err(eprintln!(
                    "At line {} and col {}: Expected either a register, immediate or mem access got {:?}",
                    first_tok.line,first_tok.col,first_tok.token_type
                ))
            }
        }
    }

    fn parse_operands(self: &mut Self) -> Result<Vec<Token>, ()> {
        let operand = self.parse_operand()?;
        let mut operands = self.parse_operands_rest()?;
        operands.insert(0, operand);
        Ok(operands)
    }

    fn parse_instruction(self: &mut Self) -> Result<Instruction, ()> {
        let label = self.parse_optional_label()?;
        let opcode = self.parse_opcode()?;
        let operands = self.parse_operands()?;
        Ok(Instruction {
            label,
            opcode,
            operands,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn test_basic_parse() {
        let input = "add x3,x2,x1";
        let mut lexer = Lexer::new(input);
        lexer.lex();
        let mut parser = Parser::new(lexer.tokens);
        let program = parser.parse_program();
        assert!(program.is_ok());
        assert_eq!(program.as_ref().unwrap().instructions.len(), 1);
        assert_eq!(program.as_ref().unwrap().instructions[0].label, None);
        assert_eq!(
            program.as_ref().unwrap().instructions[0].opcode.token_type,
            TokenType::AddInst
        );
        assert_eq!(program.as_ref().unwrap().instructions[0].operands.len(), 3);
        assert_eq!(
            program.as_ref().unwrap().instructions[0].operands[0].token_type,
            TokenType::Register(Box::new("x3".to_string()))
        );
        assert_eq!(
            program.as_ref().unwrap().instructions[0].operands[1].token_type,
            TokenType::Register(Box::new("x2".to_string()))
        );
        assert_eq!(
            program.as_ref().unwrap().instructions[0].operands[2].token_type,
            TokenType::Register(Box::new("x1".to_string()))
        );
    }
}
