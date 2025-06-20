mod parser;
mod tokenizer;

use crate::parser::Parser;
use crate::tokenizer::tokenize;
use std::{
    error::Error,
    io::{self, Write},
};

pub fn run() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");

        match process_user_input(&input) {
            Ok(result) => {
                println!("{result}")
            }
            Err(err) => {
                eprintln!("{}", err.to_string())
            }
        }
    }
}

fn process_user_input(input: &str) -> Result<u32, Box<dyn Error>> {
    let tokens = tokenize(input.trim())?;
    let mut parser = Parser::new(tokens)?;
    Ok(parser.parse()?)
}
