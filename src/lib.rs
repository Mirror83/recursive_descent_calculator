mod parser;
mod tokenizer;

use crate::parser::Parser;
use crate::tokenizer::tokenize;
use std::io::{self, Write};

pub fn run() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");
        let tokens = tokenize(input.trim()).unwrap();
        let mut parser = Parser::new(tokens).unwrap();
        let result = parser.parse().unwrap();
        println!("{result}");
    }
}
