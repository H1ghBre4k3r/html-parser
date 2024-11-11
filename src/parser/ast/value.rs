use lachs::Span;

use crate::{ParseError, Parseable, Token, TokenKind, Value};

use super::AstNode;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedValue {
    pub position: Span,
    pub value: String,
}

impl Parseable for ParsedValue {
    fn try_parse(tokens: &mut crate::ParseStream) -> Result<AstNode, super::ParseError> {
        let next = tokens.next();

        match next {
            Some(Token::Value(Value { position, value })) => {
                let value = value.trim_matches('"').to_string();
                Ok(AstNode::Value(ParsedValue { position, value }))
            }
            Some(other) => Err(ParseError::Mismatch(TokenKind::Value, other)),
            _ => Err(ParseError::Eof),
        }
    }
}

#[cfg(test)]
mod tests {
    use lachs::Span;

    use crate::{
        AstNode, LAngle, ParseError, ParseStream, Parseable, ParsedValue, Token, TokenKind,
    };

    fn stream(input: &str) -> ParseStream {
        let tokens = Token::lex(input).unwrap();

        ParseStream::new(tokens)
    }

    #[test]
    fn test_ident() {
        let mut tokens = stream(r#""foo""#);
        let result = ParsedValue::try_parse(&mut tokens);

        assert_eq!(
            result,
            Ok(AstNode::Value(ParsedValue {
                value: "foo".into(),
                position: Span::default()
            }))
        );
    }

    #[test]
    fn test_non_ident() {
        let mut tokens = stream(r#"<"foo""#);
        let result = ParsedValue::try_parse(&mut tokens);

        assert_eq!(
            result,
            Err(ParseError::Mismatch(
                TokenKind::Value,
                Token::LAngle(LAngle {
                    position: Span::default()
                })
            ))
        );
    }

    #[test]
    fn test_eof() {
        let mut tokens = stream("");
        let result = ParsedValue::try_parse(&mut tokens);

        assert_eq!(result, Err(ParseError::Eof));
    }
}
