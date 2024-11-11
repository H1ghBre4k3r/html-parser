mod ast;
mod combinators;
mod parse_stream;

pub use ast::*;
pub use combinators::*;
pub use parse_stream::*;

pub trait Parseable {
    fn try_parse(tokens: &mut ParseStream) -> Result<AstNode, ParseError>;
}
