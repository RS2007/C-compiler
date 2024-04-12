# Design of the lexer

## Designing of the lexer state machine

- The lexer is a state machine, initially at a start state
- Depending on the character it gets it goes to another state
- If it gets an integer value, it goes to a got integer state, till a non integer is encountered in which case it resets to the start
- If it gets a comma, go back to start
- If it gets a character, then it should be an identifier, more characters loop on the identifier, when non character is observed go back to start
- start on receiving whitespaces should remain at start

## State stored in the lexer

- What state should be kept in the lexer, line number as well as the column(start of token)

# Design of the assembler

## Parsing the AArch64 assembly language

### BNF

```bnf
<program> ::= <instruction>*

<instruction> ::= <label>? <opcode> <operand>* ';'?

<label> ::= <identifier> ':'

<opcode> ::= 'ADD' | 'SUB' | 'MUL' | 'DIV' | 'MOV' | 'LDR' | 'STR' | 'B' | 'BL' | 'CBZ' | 'CBNZ' | 'CMP' | 'RET' | 'NOP'

<operand> ::= <register> | <immediate> | <memory_access>

<register> ::= 'X0' | 'X1' | 'X2' | ... | 'X30' | 'SP' | 'XZR'

<immediate> ::= '#' <integer>

<memory_access> ::= '[' <register> ']' | '[' <register> ',' <register> ']' | '[' <register> ']!' | '[' <register> ',' <register> ']!'

<identifier> ::= <letter> (<letter> | <digit>)*

<letter> ::= 'A' | 'B' | ... | 'Z' | 'a' | 'b' | ... | 'z'

<digit> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-'

<integer> ::= <digit>+
```

- LL(k) parser
  - `parse_program`: for file is not finished: parse_instruction
  - `parse_instruction`: parse_optional_label, parse_opcode for token is not semicolon parse the operand
  - `parse_opcode` is string matching
  - `parse_operand` can be either or `parse_register`, `parse_immediate` or `parse_memory_access`(just one lookahead required)

> [!NOTE]
> Might omit making a handwritten parser and using a parser generator for this is because, we do not need good error messages for the assembler(at least as of now, since we are not standalone shipping the assembler)
