use std::{error::Error, fmt};

use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    GenericError,
    NoInput,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse input.")
    }
}

impl Error for ParseError {}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    /// Creates a new parser given a list of tokens
    pub fn new(tokens: Vec<Token>) -> Result<Parser, ParseError> {
        if tokens.len() == 0 {
            Err(ParseError::NoInput)
        } else {
            Ok(Parser { tokens })
        }
    }

    /// Parses the arithmetic expression represented by the token list
    /// and returns the result of that expression, or an error
    /// if the original expression was malformed
    pub fn parse(&self) -> Result<i32, ParseError> {
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_token_list() {
        assert_eq!(Parser::new(vec![]).err(), Some(ParseError::NoInput));
    }

    #[test]
    fn token_list_with_single_num() {
        let parser = Parser::new(vec![Token::Num(2)]).unwrap();
        assert_eq!(parser.parse().unwrap(), 2);
    }

    #[test]
    fn token_list_with_simple_sum() {
        let parser = Parser::new(vec![Token::Num(2), Token::Plus, Token::Num(2)]).unwrap();
        assert_eq!(parser.parse().unwrap(), 4)
    }

    #[test]
    fn token_list_with_simple_parenthesised_sum() {
        let parser = Parser::new(vec![
            Token::LBracket,
            Token::Num(2),
            Token::Plus,
            Token::Num(2),
            Token::RBracket,
        ])
        .unwrap();
        assert_eq!(parser.parse().unwrap(), 4)
    }
}
