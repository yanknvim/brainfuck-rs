use libc::getchar;

pub enum Token {
    PInc,
    PDec,
    Inc,
    Dec,
    Output,
    Input,
    Begin,
    End,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for token in input.chars() {
        match token {
            '>' => tokens.push(Token::PInc),
            '<' => tokens.push(Token::PDec),
            '+' => tokens.push(Token::Inc),
            '-' => tokens.push(Token::Dec),
            '.' => tokens.push(Token::Output),
            ',' => tokens.push(Token::Input),
            '[' => tokens.push(Token::Begin),
            ']' => tokens.push(Token::End),
            _ => {},
        }
    }

    tokens
}

pub fn run(tokens: Vec<Token>) {
    let mut memory = [0 as u8; 256];
    let mut pointer = 0;

    let mut index = 0;
    while index < tokens.len() {
        match tokens[index] {
            Token::PInc => pointer += 1,
            Token::PDec => pointer -= 1,
            Token::Inc => memory[pointer] += 1,
            Token::Dec => memory[pointer] -= 1,
            Token::Output => print!("{}", memory[pointer] as char),
            Token::Input => {
                memory[pointer] = unsafe {getchar() as u8};
            },
            Token::Begin => {
                if memory[pointer] == 0 {
                    let mut counter = 1;
                    while counter > 0 {
                        index += 1;
                        match tokens[index] {
                            Token::Begin => counter += 1,
                            Token::End => counter -= 1,
                            _ => {},
                        }
                    }
                }
            },
            Token::End => {
                if memory[pointer] != 0 {
                    let mut counter = 1;
                    while counter > 0 {
                        index -= 1;
                        match tokens[index] {
                            Token::Begin => counter -= 1,
                            Token::End => counter += 1,
                            _ => {},
                        }
                    }
                }
            }
            _ => {},
        }
        index += 1;
    }
}

