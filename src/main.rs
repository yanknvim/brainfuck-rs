mod interpreter;

use std::fs::read_to_string;
use std::env;
use std::io::{stdout, stdin, Write};

fn repl() {
    println!("Welcome to the brainfuck REPL, type 'exit' to exit");
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let input = {
            let mut buffer = String::new();
            match stdin().read_line(&mut buffer) {
                Ok(_) => {},
                Err(_) => panic!("Error: failed to read input"),
            }
            buffer
        };
        let input = input.trim_end();

        match &*input {
            "exit" => return,
            _ => {
                let tokens = interpreter::lex(&input);
                interpreter::run(tokens);
            },
        }
        println!("");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
       repl(); 
    } else {
        let code = read_to_string(&args[1]).unwrap();
        let tokens = interpreter::lex(&code);
        interpreter::run(tokens);
    }
}
