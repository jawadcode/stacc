# Stacc

A dynamically typed, interpreted, stack-based language.

## How does it work?

Each call-frame/scope has its own variables and stack, so you can `get`/`set` variables and `push`/`pop` from the stack.

Here are all the different types of statements in the language:

- Function definition - `begin fn_name: param1 param2 <newline> <statements> end`
- Variable declaration (sets variable in current scope) - `set var_name 123`
- Push (pushes value onto current stack) - `push 123`
- Pop (pops value from stack and discards it) - `pop` / `set a pop`
- Print - `print "Hello World!"`
- Function call (pops argument values from parent stack, and pushes result onto parent stack) - `call do_thing`

Here are all the different types of expressions in the language:

- Identifier - gets value
- Literal - integer, float or string literal
- Binary operation - arithmetic (+, -, \*, /) or comparison (<, >, <=, >=, ==, !=) expression
- Unary operation - prefix `-` or prefix `not`
- Pop (pops the value at the top of the stack and returns it) - `pop`

Note: the result of a function is either the value at the end of its stack or if the stack is empty, nothing and statements must be proceeded by newlines

## Code Examples?

In the aptly named `examples` directory of the repo.

## How do I use the executable?

- `stacc` to get a repl
- `stacc <input file>` to run the interpreter on a file
