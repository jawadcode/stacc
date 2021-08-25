use std::{
    env, fmt, fs,
    io::{self, Write},
    process,
};

use stacc::{interpreter::Interpreter, parser::Parser};

fn unwrap<T, E: fmt::Display + fmt::Debug>(result: Result<T, E>) -> T {
    if let Err(err) = result {
        eprintln!("{}", err);
        process::exit(1);
    }

    result.unwrap()
}

fn main() {
    let filename = env::args().nth(1);
    if filename.is_none() {
        repl();
        process::exit(1);
    }

    let filename = filename.unwrap();
    let contents = unwrap(fs::read_to_string(filename));
    let stmts = unwrap(Parser::new(&contents).parse());

    let mut interpreter = Interpreter::new();
    unwrap(interpreter.run(&stmts))
}

fn repl() {
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.push('\n');

        let stmt = Parser::new(&input).parse_stmt();
        match stmt {
            Ok(stmt) => match interpreter.run_one(&stmt) {
                Ok(_) => interpreter.print_state(),
                Err(err) => eprintln!("{}", err),
            },
            Err(err) => eprintln!("{}", err),
        }
    }
}
