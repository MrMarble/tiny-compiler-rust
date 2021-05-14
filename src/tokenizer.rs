#[derive(Debug, PartialEq)]
pub enum Token {
    ParenOpen,
    ParenClose,
    Number(String),
    Name(String),
    String(String),
}

/**
 * Lexical analyzer
 *
 * Turns raw code into an array of Tokens
 */
fn tokenizer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];

    let mut char_iter = input.chars().peekable();

    while let Some(c) = char_iter.next() {
        match c {
            '(' => tokens.push(Token::ParenOpen),
            ')' => tokens.push(Token::ParenClose),
            '0'..='9' => {
                let mut value = String::new();
                value.push(c);
                while let Some('0'..='9') = char_iter.peek() {
                    value.push(char_iter.next().unwrap())
                }
                tokens.push(Token::Number(value))
            }
            'a'..='z' => {
                let mut value = String::new();
                value.push(c);
                while let Some('a'..='z') = char_iter.peek() {
                    value.push(char_iter.next().unwrap())
                }
                tokens.push(Token::Name(value))
            }
            '"' => {
                let mut value = String::new();

                while match char_iter.peek() {
                    Some('"') | None => false,
                    _ => true,
                } {
                    value.push(char_iter.next().unwrap());
                }
                tokens.push(Token::String(value));
                char_iter.next();
            }
            c if c.is_whitespace() => continue,
            _ => return Err(format!("Unknown token {}", c)),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() -> Result<(), String> {
        assert_eq!(tokenizer("(")?, vec![Token::ParenOpen]);
        assert_eq!(tokenizer("( ")?, vec![Token::ParenOpen]);
        assert_eq!(tokenizer(")")?, vec![Token::ParenClose]);
        assert_eq!(
            tokenizer("1337")?,
            vec![Token::Number(String::from("1337"))]
        );
        assert_eq!(tokenizer("add")?, vec![Token::Name(String::from("add"))]);
        assert_eq!(
            tokenizer("\"mrmarble\"")?,
            vec![Token::String(String::from("mrmarble"))]
        );

        assert_eq!(
            tokenizer("(add 2 (subtract 1 4))")?,
            vec![
                Token::ParenOpen,
                Token::Name(String::from("add")),
                Token::Number(String::from("2")),
                Token::ParenOpen,
                Token::Name(String::from("subtract")),
                Token::Number(String::from("1")),
                Token::Number(String::from("4")),
                Token::ParenClose,
                Token::ParenClose
            ]
        );
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_invalid() {
        tokenizer("!").unwrap();
    }
}
