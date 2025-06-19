use std::error::Error;

use crate::tokenizer::Token;

pub fn parse(tokens: &Vec<Token>) -> Result<i32, Box<dyn Error>> {
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_token_list() {
        assert!(parse(&vec![]).is_err());
    }

    #[test]
    fn token_list_with_single_num() {
        assert_eq!(parse(&vec![Token::Num(2)]).unwrap(), 2);
    }

    #[test]
    fn token_list_with_simple_sum() {
        assert_eq!(
            parse(&vec![Token::Num(2), Token::Plus, Token::Num(2)]).unwrap(),
            4
        )
    }

    #[test]
    fn token_list_with_simple_parenthesised_sum() {
        assert_eq!(
            parse(&vec![
                Token::LBracket,
                Token::Num(2),
                Token::Plus,
                Token::Num(2),
                Token::RBracket
            ])
            .unwrap(),
            4
        )
    }
}
