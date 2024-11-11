use lachs::Span;

use crate::{Identifier, ParseError, Parseable, Token, TokenKind, Value};

use super::{AstNode, ParsedValue};

#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    KeyValue {
        key: String,
        value: String,
        position: Span,
    },
    Boolean {
        key: String,
        set: bool,
        position: Span,
    },
}

impl Parseable for Attribute {
    fn try_parse(tokens: &mut crate::ParseStream) -> Result<super::AstNode, crate::ParseError> {
        let Some(next) = tokens.next() else {
            return Err(ParseError::Eof);
        };

        let Token::Identifier(Identifier {
            position,
            value: key,
        }) = next
        else {
            return Err(ParseError::Mismatch(TokenKind::Identifier, next));
        };

        let Some(Token::Equals(_)) = tokens.peek() else {
            return Ok(AstNode::Attribute(Attribute::Boolean {
                key,
                set: true,
                position,
            }));
        };

        tokens.next();

        let AstNode::Value(ParsedValue {
            position: end,
            value,
        }) = ParsedValue::try_parse(tokens)?
        else {
            unreachable!()
        };

        let value = value.to_lowercase();

        let position = position.merge(&end);

        if let Ok(set) = value.parse::<bool>() {
            Ok(AstNode::Attribute(Attribute::Boolean {
                key: value,
                set,
                position,
            }))
        } else {
            Ok(AstNode::Attribute(Attribute::KeyValue {
                key,
                value,
                position,
            }))
        }
    }
}
