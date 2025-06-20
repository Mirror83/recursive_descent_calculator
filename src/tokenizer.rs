use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LBracket,
    RBracket,
    Plus,
    Num(u32),
    // Represents end of input
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    UnexpectedCharacter(char),
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCharacter(val) => write!(f, "Unexpected character {val}"),
        }
    }
}

impl Error for TokenizerError {}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizerError> {
    let mut token_list: Vec<Token> = Vec::new();
    let mut numeric_chars: Vec<char> = vec![];

    for character in input.chars() {
        if character.is_numeric() {
            numeric_chars.push(character);
            continue;
        } else if numeric_chars.len() > 0 {
            add_num_to_token_list(&mut numeric_chars, &mut token_list);
        }

        match character {
            '(' => token_list.push(Token::LBracket),
            ')' => token_list.push(Token::RBracket),
            '+' => token_list.push(Token::Plus),
            ' ' => continue,
            _ => return Err(TokenizerError::UnexpectedCharacter(character)),
        }
    }

    if numeric_chars.len() > 0 {
        add_num_to_token_list(&mut numeric_chars, &mut token_list);
    }

    token_list.push(Token::EOF);

    return Ok(token_list);
}

/// This is a helper function for the `tokenize` function
fn add_num_to_token_list(numeric_chars: &mut Vec<char>, token_list: &mut Vec<Token>) {
    let num = numeric_char_vec_to_u32(&numeric_chars);
    token_list.push(Token::Num(num));
    // Reset the numeric character vector for the next number
    numeric_chars.clear();
}

fn numeric_char_vec_to_u32(numeric_chars: &Vec<char>) -> u32 {
    let mut place_value = 1;

    if numeric_chars.len() == 0 {
        panic!("numeric chars should have at least one element");
    }

    let mut num = 0;
    for digit_letter in numeric_chars.iter().rev() {
        num += digit_letter.to_digit(10).unwrap() * place_value;
        place_value *= 10;
    }
    num
}

#[cfg(test)]
mod numeric_char_vec_to_u32_tests {
    use super::numeric_char_vec_to_u32;

    #[test]
    #[should_panic]
    fn empty_vec() {
        let numeric_chars: Vec<char> = vec![];
        numeric_char_vec_to_u32(&numeric_chars);
    }

    #[test]
    fn single_digit_numbers() {
        assert_eq!(numeric_char_vec_to_u32(&vec!['1']), 1);
        assert_eq!(numeric_char_vec_to_u32(&vec!['7']), 7);
        assert_eq!(numeric_char_vec_to_u32(&vec!['0']), 0);
    }

    #[test]
    fn multiple_digit_numbers() {
        assert_eq!(numeric_char_vec_to_u32(&vec!['2', '0']), 20);
        assert_eq!(numeric_char_vec_to_u32(&vec!['7', '0', '0']), 700);
        assert_eq!(
            numeric_char_vec_to_u32(&vec!['1', '0', '4', '3', '5']),
            10435
        );
    }

    #[test]
    #[should_panic]
    fn test_single_invalid_char() {
        numeric_char_vec_to_u32(&vec!['e']);
    }

    #[test]
    #[should_panic]
    fn test_vec_starting_with_invalid_char() {
        numeric_char_vec_to_u32(&vec!['1', '2', 'e']);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(tokenize("").unwrap(), vec![Token::EOF]);
    }

    #[test]
    fn single_digit_number() {
        assert_eq!(tokenize("2").unwrap(), vec![Token::Num(2), Token::EOF])
    }

    #[test]
    fn multiple_digit_number() {
        assert_eq!(
            tokenize("1012").unwrap(),
            vec![Token::Num(1012), Token::EOF]
        )
    }

    // I am yet to check on how to handle
    // overflow, so the test is ignored
    #[test]
    #[ignore]
    fn multiple_digit_number_large_enough_to_cause_overflow() {
        assert_eq!(
            tokenize("10122490909809809234").unwrap(),
            vec![Token::Num(1012), Token::EOF]
        )
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
                Token::EOF
            ]
        )
    }

    #[test]
    fn simple_multiple_digit_parenthesized_sum() {
        assert_eq!(
            tokenize("(10+ 122)").unwrap(),
            vec![
                Token::LBracket,
                Token::Num(10),
                Token::Plus,
                Token::Num(122),
                Token::RBracket,
                Token::EOF
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
                Token::EOF
            ]
        )
    }

    #[test]
    fn simple_invalid_arithmetic_with_correct_tokens() {
        assert_eq!(
            tokenize("3+").unwrap(),
            vec![Token::Num(3), Token::Plus, Token::EOF]
        )
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
            tokenize("animal + bourgeoisie").err(),
            Some(TokenizerError::UnexpectedCharacter('a'))
        )
    }
}
