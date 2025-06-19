mod parser;
mod tokenizer;

use crate::parser::parse;
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
        let result = parse(&tokens).unwrap();
        println!("{result}");
    }
}
