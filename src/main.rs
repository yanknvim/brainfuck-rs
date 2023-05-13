mod interpreter;

use std::fs::read_to_string;
use std::env;

fn main() {
    let code = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+."; // Hello world

    let tokens = interpreter::lex(code);
    interpreter::run(tokens);
}
