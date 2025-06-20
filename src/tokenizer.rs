use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LBracket,
    RBracket,
    Plus,
    Num(u32),
}

#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    NoInput,
    UnexpectedCharacter(char),
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoInput => write!(f, "Cannot retrieve token from empty string"),
            Self::UnexpectedCharacter(val) => write!(f, "Unexpected character {val}"),
        }
    }
}

impl Error for TokenizerError {}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizerError> {
    if input.len() == 0 {
        return Err(TokenizerError::NoInput);
    }

    let mut token_list: Vec<Token> = Vec::new();

    for character in input.chars() {
        if character.is_numeric() {
            // TODO: work on supporting multiple-digit numbers correctly
            let numeric_token = Token::Num(character.to_digit(10).unwrap());
            token_list.push(numeric_token);
            continue;
        }
        match character {
            '(' => token_list.push(Token::LBracket),
            ')' => token_list.push(Token::RBracket),
            '+' => token_list.push(Token::Plus),
            ' ' => continue,
            _ => return Err(TokenizerError::UnexpectedCharacter(character)),
        }
    }
    return Ok(token_list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(tokenize("").err(), Some(TokenizerError::NoInput));
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
    fn valid_string_with_spaces() {
        assert_eq!(
            tokenize("   (2 + 2)   ").unwrap(),
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
    fn simple_invalid_arithmetic_with_correct_tokens() {
        assert_eq!(tokenize("3+").unwrap(), vec![Token::Num(3), Token::Plus])
    }

    #[test]
    fn simple_invalid_string() {
        assert_eq!(
            tokenize("X").err(),
            Some(TokenizerError::UnexpectedCharacter('X'))
        )
    }

    #[test]
    fn word_sum() {
        assert_eq!(
            tokenize("a + b").err(),
            Some(TokenizerError::UnexpectedCharacter('a'))
        )
    }
}
