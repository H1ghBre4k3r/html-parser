use std::ops::Shr;

use crate::lexer::*;

use super::ast::*;
use super::Parseable;
use super::{AstNode, ParseStream};

#[derive(Clone)]
pub enum Combinator<'a> {
    /// Combinator which consumes a token and does not yield anything.
    Consumer { token: TokenKind },
    /// Combinator which consumes and yields a token.
    Yielder {
        parser: &'a dyn Fn(&mut ParseStream) -> Result<AstNode, ParseError>,
    },
    /// Combinator for chaining two other combinators together.
    Sequence {
        left: Box<Combinator<'a>>,
        right: Box<Combinator<'a>>,
    },
}

macro_rules! consumer {
    ($name:ident, $token:ident) => {
        pub const $name: Combinator<'static> = Combinator::Consumer {
            token: TokenKind::$token,
        };
    };
}

macro_rules! yielder {
    ($name:ident, $struct:ident) => {
        pub const $name: Combinator<'static> = Combinator::Yielder {
            parser: &$struct::try_parse,
        };
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    Mismatch(TokenKind, Token),
    Eof,
}

impl<'a> Combinator<'a> {
    consumer!(LANGLE, LAngle);
    consumer!(RANGLE, RAngle);
    consumer!(EQUALS, Equals);
    consumer!(SLASH, Slash);
    yielder!(IDENTIFIER, ParsedIdentifier);
    yielder!(VALUE, ParsedValue);

    pub fn try_parse(&self, tokens: &mut ParseStream) -> Result<Vec<AstNode>, ParseError> {
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
            Yielder { parser } => {
                let result = parser(tokens)?;
                Ok(vec![result])
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

impl<'a> Shr for Combinator<'a> {
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

    use crate::{AstNode, ParseStream, ParsedIdentifier};

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
            Ok(vec![AstNode::Identifier(ParsedIdentifier {
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
            Ok(vec![AstNode::Identifier(ParsedIdentifier {
                position: Span::default(),
                value: String::from("foo")
            })])
        );
    }
}
