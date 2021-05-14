use crate::tokenizer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq)]
enum Node {
    Program { body: Vec<Node> },
    CallExpression { name: String, params: Vec<Node> },
}

fn parser(tokens: Vec<Token>) -> Result<Node, String> {
    fn walk(token: Token, token_iter: &mut Peekable<IntoIter<Token>>) -> Result<Node, String> {
        match token {
            Token::ParenOpen => {
                if let Some(token) = token_iter.next() {
                    match token {
                        Token::Name(name) => {
                            let mut params: Vec<Node> = vec![];

                            while match token_iter.peek() {
                                Some(Token::ParenClose) | None => false,
                                _ => true,
                            } {
                                match walk(token_iter.next().unwrap(), token_iter) {
                                    Ok(nodes) => params.push(nodes),
                                    Err(value) => return Err(value),
                                }
                            }

                            token_iter.next().unwrap();

                            Ok(Node::CallExpression { name, params })
                        }
                        _ => {
                            return Err(format!(
                                "{:?} isn't followed by a {:?}.",
                                Token::ParenOpen,
                                Token::Name("example".to_string())
                            ))
                        }
                    }
                } else {
                    return Err(format!("{:?} isn't followed by a node.", Token::ParenOpen));
                }
            }
            _ => return Err(format!("Unknown token {:?}", token)),
        }
    }

    let mut body: Vec<Node> = vec![];
    let mut token_iter = tokens.into_iter().peekable();
    while let Some(token) = token_iter.next() {
        match walk(token, &mut token_iter) {
            Ok(nodes) => body.push(nodes),
            Err(value) => return Err(value),
        }
    }
    Ok(Node::Program { body })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() -> Result<(), String> {
        assert_eq!(
            parser(vec![
                Token::ParenOpen,
                Token::Name(String::from("add")),
                Token::ParenClose
            ])?,
            Node::Program {
                body: vec![Node::CallExpression {
                    name: String::from("add"),
                    params: vec![]
                }]
            }
        );
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        parser(vec![Token::ParenOpen]).unwrap();
    }
}
