use std::fmt::Display;

use crate::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    Mismatch(TokenKind, Token),
    Eof,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Mismatch(token_kind, token) => {
                let pos = match token {
                    Token::LAngle(inner) => inner.position.clone(),
                    Token::RAngle(inner) => inner.position.clone(),
                    Token::Equals(inner) => inner.position.clone(),
                    Token::Slash(inner) => inner.position.clone(),
                    Token::Identifier(inner) => inner.position.clone(),
                    Token::Value(inner) => inner.position.clone(),
                };
                f.write_str(&pos.to_string(format!(
                    r#"Expected "{token_kind:?}" but got {:?}"#,
                    token.get_name()
                )))
            }
            ParseError::Eof => f.write_str("Unexpected EOF!"),
        }
    }
}
