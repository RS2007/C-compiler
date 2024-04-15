use crate::{
    lexer::TokenType,
    parser::{Instruction, Program},
};

struct Encoder {
    program: Program,
}

impl TokenType {
    fn encode(self: &Self) -> u8 {
        match self {
            TokenType::Register(reg) => match (*reg).as_str() {
                "x0" => 0,
                "x1" => 1,
                "x2" => 2,
                "x3" => 3,
                "x4" => 4,
                "x5" => 5,
                "x6" => 6,
                "x7" => 7,
                "x8" => 8,
                "x9" => 9,
                "x10" => 10,
                "x11" => 11,
                "x12" => 12,
                "x13" => 13,
                "x14" => 14,
                "x15" => 15,
                "x16" => 16,
                "x17" => 17,
                "x18" => 18,
                "x19" => 19,
                "x20" => 20,
                "x21" => 21,
                "x22" => 22,
                "x23" => 23,
                "x24" => 24,
                "x25" => 25,
                "x26" => 26,
                "x27" => 27,
                "x28" => 28,
                "x29" => 29,
                "x30" => 30,
                "x31" => 31,
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}

impl Instruction {
    pub fn encode(self: &mut Self) -> u32 {
        match self.opcode.token_type {
            TokenType::AddInst => {
                // TODO: Errors cause bit shift goes out of range, right thing to do is to pick a unsigned int size
                // slightly bigger than the size of the part of the instruction, bit shift to the right till the last bit
                // of the relevant part is at index 0 (right most) and then take that smaller uint
                // left shift by whatever correct amount (probably construct helpers for this, so that its less verbose)
                let rd = self.operands.get(0).unwrap().token_type.encode();
                let rs1 = self.operands.get(1).unwrap().token_type.encode();
                let rs2 = self.operands.get(2).unwrap().token_type.encode();
                let rd_masked = (rd & 0b00011111) as u32;
                let rs1_masked = ((rs1 & 0b00011111) << 5) as u32;
                let imm6 = 0 << 10 as u32;
                let rs2_masked = ((rs2 & 0b00011111) as u32) << 16;
                let shift_and_amt = 0 << 21 as u32;
                let opcode_encoded = (0b100010110 << 23) as u32;
                let mut encoded_inst = 0 as u32;
                encoded_inst |= opcode_encoded;
                encoded_inst |= rd_masked;
                encoded_inst |= rs1_masked;
                encoded_inst |= imm6;
                encoded_inst |= rs2_masked;
                encoded_inst |= shift_and_amt;
                encoded_inst as u32
            }
            _ => unimplemented!(),
        }
    }
}

impl Encoder {
    fn new(program: Program) -> Self {
        Encoder { program }
    }

    pub(crate) fn encode_program(self: &mut Self) -> Vec<u32> {
        let mut bytes: Vec<u32> = vec![];
        for instruction in self.program.instructions.iter_mut() {
            let encoded_inst = instruction.encode();
            bytes.push(encoded_inst);
        }
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_encode_add_register() {
        let mut lexer = Lexer::new("add x3,x2,x1");
        lexer.lex();
        let mut parser = Parser::new(lexer.tokens);
        let program = parser.parse_program().expect("Failed to parse program");
        let mut encoder = Encoder::new(program);
        let bytes = encoder.encode_program();
        assert_eq!(bytes.len(), 1);
        println!("Value of bytes: {:b}", bytes.get(0).unwrap());
        assert_eq!(*bytes.get(0).unwrap(), 0x8b010043 as u32);
    }
}
