use lachs::Span;

use crate::{Identifier, ParseError, Parseable, Token, TokenKind};

use super::AstNode;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedIdentifier {
    pub position: Span,
    pub value: String,
}

impl Parseable for ParsedIdentifier {
    fn try_parse(tokens: &mut crate::ParseStream) -> Result<super::AstNode, crate::ParseError> {
        let next = tokens.next();

        match next {
            Some(Token::Identifier(Identifier { position, value })) => {
                Ok(AstNode::Identifier(ParsedIdentifier { position, value }))
            }
            Some(other) => Err(ParseError::Mismatch(TokenKind::Identifier, other)),
            _ => Err(ParseError::Eof),
        }
    }
}

#[cfg(test)]
mod tests {
    use lachs::Span;

    use crate::{AstNode, LAngle, ParseError, ParseStream, Parseable, Token, TokenKind};

    use super::ParsedIdentifier;

    fn stream(input: &str) -> ParseStream {
        let tokens = Token::lex(input).unwrap();

        ParseStream::new(tokens)
    }

    #[test]
    fn test_ident() {
        let mut tokens = stream("foo");
        let result = ParsedIdentifier::try_parse(&mut tokens);

        assert_eq!(
            result,
            Ok(AstNode::Identifier(ParsedIdentifier {
                value: "foo".into(),
                position: Span::default()
            }))
        );
    }

    #[test]
    fn test_non_ident() {
        let mut tokens = stream("<foo");
        let result = ParsedIdentifier::try_parse(&mut tokens);

        assert_eq!(
            result,
            Err(ParseError::Mismatch(
                TokenKind::Identifier,
                Token::LAngle(LAngle {
                    position: Span::default()
                })
            ))
        );
    }

    #[test]
    fn test_eof() {
        let mut tokens = stream("");
        let result = ParsedIdentifier::try_parse(&mut tokens);

        assert_eq!(result, Err(ParseError::Eof));
    }
}
