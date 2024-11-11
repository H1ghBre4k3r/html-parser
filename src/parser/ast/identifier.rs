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
