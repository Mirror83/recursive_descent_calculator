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
    current_token_index: usize,
}

impl Parser {
    /// Creates a new parser given a list of tokens
    pub fn new(tokens: Vec<Token>) -> Result<Parser, ParseError> {
        if tokens.len() == 0 {
            Err(ParseError::NoInput)
        } else {
            Ok(Parser {
                tokens,
                current_token_index: 0,
            })
        }
    }

    /// Parses the arithmetic expression represented by the token list
    /// and returns the result of that expression, or an error
    /// if the original expression was malformed.
    /// Corresponds to the `P -> E` production in the grammar.
    pub fn parse(&self) -> Result<u32, ParseError> {
        return self.parse_e();
    }

    /// Corresponds to the `E -> TE'` production in the grammar.
    fn parse_e(&self) -> Result<u32, ParseError> {
        let t_result = self.parse_t()?;
        match self.parse_e_prime()? {
            Some(e_prime_result) => Ok(t_result + e_prime_result),
            None => Ok(t_result), // i.e e_prime produced an ϵ
        }
    }

    /// Corresponds to the `E' -> +TE' | ϵ` production in the grammar.
    /// Since this rule can produce `ϵ`, the method returns a `Result`
    /// with an `Option `instead of just a value like the other parse_* methods.
    /// The Option values will be `None` if `ϵ` is produced and returns
    /// a `Some` that contains the value of evaluating `+TE` otherwise
    fn parse_e_prime(&self) -> Result<Option<u32>, ParseError> {
        let token = self.next_token();
        match token {
            Token::Plus => {
                let t_result = self.parse_t()?;
                let next_e_prime_result = self.parse_e_prime()?;
                match next_e_prime_result {
                    Some(val) => Ok(Some(val + t_result)),
                    None => Ok(Some(t_result)),
                }
            }
            _ => {
                // The token will be considered by some other rule.
                // Essentially we have produced an ϵ
                self.put_back_token(token);
                return Ok(None);
            }
        }
    }

    /// Corresponds to the `T -> (E) | num` production in the grammar.
    fn parse_t(&self) -> Result<u32, ParseError> {
        let token = self.next_token();
        match token {
            Token::LBracket => {
                let inner_expression_result = self.parse_e()?;
                if !self.expect_token(Token::RBracket) {
                    Err(ParseError::GenericError)
                } else {
                    Ok(inner_expression_result)
                }
            }
            Token::Num(val) => Ok(val),
            _ => Err(ParseError::GenericError),
        }
    }

    /// Returns the next token to be considered
    fn next_token(&self) -> Token {
        Token::Num(1)
    }

    /// Returns `true` if [t] is the same as the next token
    /// and false otherwise
    fn expect_token(&self, t: Token) -> bool {
        return false;
    }

    /// Puts an unexpected token [t] back for consideration. That token
    /// will be returned again by the next call to `next_token`
    fn put_back_token(&self, t: Token) {}
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
