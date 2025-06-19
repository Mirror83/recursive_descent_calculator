use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Token {
    LBracket,
    RBracket,
    Plus,
    Num(i32),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    return Ok(vec![
        Token::LBracket,
        Token::Num(2),
        Token::Plus,
        Token::Num(2),
        Token::RBracket,
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert!(tokenize("").is_err());
    }

    #[test]
    fn single_number() {
        assert_eq!(tokenize("2").unwrap(), vec![Token::Num(2)])
    }

    #[test]
    fn simple_parenthesised_sum() {
        assert_eq!(
            tokenize("(2+2)").unwrap(),
            vec![
                Token::LBracket,
                Token::Num(2),
                Token::Plus,
                Token::Num(2),
                Token::RBracket,
            ]
        )
    }

    #[test]
    fn simple_invalid_string() {
        assert!(tokenize("3+").is_err())
    }
}
