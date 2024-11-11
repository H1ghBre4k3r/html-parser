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
