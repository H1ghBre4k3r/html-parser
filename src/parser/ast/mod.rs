mod identifier;
mod value;

pub use self::identifier::*;
pub use self::value::*;

use crate::ParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Identifier(ParsedIdentifier),
    Value(ParsedValue),
}