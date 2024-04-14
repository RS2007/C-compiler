# A functional C compiler toolchain

## Why?

Why would anyone write a c compiler, isn't that a solved problem, just use GCC or clang or if you want something lightweight just use TCC.
The answer is that these are "black boxes", thus you have zero clue what is happening internally. I stil can't make sense of why my linker takes an enormous amount of time to link LLVM or why clangd keeps asking me to do forward declarations.

Also if you haven't browsed through my github, I like making compilers. Everytime I make a new one, there's this nagging feeling of incompleteness, either because the language is too simple(for sake of parsing) or I offload most of the backend work to LLVM. Thus such an ambitious attempt, write a minimal compiler toolchain for the C programming language(or a reasonable subset):

```
    Source code -> Compiler frontend ->  TAC IR -> Optimizing backend -> Generating ASM -> Assembler (ASM -> Machine instructions) -> .o file -> Linker -> Executable
```

## Why rust?

I don't have a good answer for this question, primarily memory safety(duh!!), but also because a significant chunk of writing the compiler is parsing and having Algebraic Data Types would be definitely helpful. Towards the latter parts of the compiler(like the linker), I would prefer a language that has an easy interop between its structs and their binary representations, but I guess thats a reasonable tradeoff in this case.

## When will this be complete?

- I have zero clue,but hopefully in a span of an year.

## Plan as of now:
* Parser/Lexer for a C subset 
* SSA design + build (Design is a strong word, probably a salad of design decisions from existing SSA bsaed IRs)
* SSA based optimizations: (Sparse conditional constant propogation + Constant Folding +  Comman SubExpression Elimination)
* Function inling with an `inline` directive 
* Liveness Analysis + Register allocation + Peephole
* Codegen to ARM -> Assembler -> .o file
* .o file -> Linker -> Elf executable


## References
-  https://github.com/sysprog21/shecc