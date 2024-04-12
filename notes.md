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