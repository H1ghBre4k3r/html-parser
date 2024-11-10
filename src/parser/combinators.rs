use std::ops::Shr;

use crate::lexer::*;

use super::ParseStream;

#[derive(Debug, Clone)]
pub enum Combinator {
    /// Combinator which consumes a token and does not yield anything.
    Consumer { token: TokenKind },
    /// Combinator which consumes and yields a token.
    Yielder { token: TokenKind },
    /// Combinator for chaining two other combinators together.
    Sequence {
        left: Box<Combinator>,
        right: Box<Combinator>,
    },
}

macro_rules! consumer {
    ($name:ident, $token:ident) => {
        pub const $name: Combinator = Combinator::Consumer {
            token: TokenKind::$token,
        };
    };
}

macro_rules! yielder {
    ($name:ident, $token:ident) => {
        pub const $name: Combinator = Combinator::Yielder {
            token: TokenKind::$token,
        };
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    Mismatch(TokenKind, Token),
    Eof,
}

impl Combinator {
    consumer!(LANGLE, LAngle);
    consumer!(RANGLE, RAngle);
    consumer!(EQUALS, Equals);
    consumer!(SLASH, Slash);
    yielder!(NUMBER, Number);
    yielder!(IDENTIFIER, Identifier);
    yielder!(VALUE, Value);

    pub fn try_parse(&self, tokens: &mut ParseStream) -> Result<Vec<Token>, ParseError> {
        use Combinator::*;

        match self {
            Consumer { token } => {
                if let Some(next) = tokens.next() {
                    if *token == next {
                        Ok(vec![])
                    } else {
                        Err(ParseError::Mismatch(*token, next))
                    }
                } else {
                    Err(ParseError::Eof)
                }
            }
            Yielder { token } => {
                if let Some(next) = tokens.next() {
                    if *token == next {
                        Ok(vec![next])
                    } else {
                        Err(ParseError::Mismatch(*token, next))
                    }
                } else {
                    Err(ParseError::Eof)
                }
            }
            Sequence { left, right } => left.try_parse(tokens).map(|mut first_result| {
                right.try_parse(tokens).map(|mut second_result| {
                    first_result.append(&mut second_result);
                    first_result
                })
            })?,
        }
    }
}

impl Shr for Combinator {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Combinator::Sequence {
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use lachs::Span;

    use crate::{Identifier, ParseStream};

    use super::{Combinator, Token};

    fn stream(input: &str) -> ParseStream {
        let lex = Token::lex(input).unwrap();

        ParseStream::new(lex)
    }

    #[test]
    fn test_consumer() {
        let mut stream = stream("<");
        let combs = Combinator::LANGLE;

        let result = combs.try_parse(&mut stream);

        assert_eq!(result, Ok(vec![]));
    }

    #[test]
    fn test_yields() {
        let mut stream = stream("foo");
        let combs = Combinator::IDENTIFIER;

        let result = combs.try_parse(&mut stream);

        assert_eq!(
            result,
            Ok(vec![Token::Identifier(Identifier {
                position: Span::default(),
                value: String::from("foo")
            })])
        );
    }

    #[test]
    fn test_sequence() {
        let mut stream = stream("<foo>");
        let combs = Combinator::LANGLE >> Combinator::IDENTIFIER >> Combinator::RANGLE;

        let result = combs.try_parse(&mut stream);

        assert_eq!(
            result,
            Ok(vec![Token::Identifier(Identifier {
                position: Span::default(),
                value: String::from("foo")
            })])
        );
    }
}
