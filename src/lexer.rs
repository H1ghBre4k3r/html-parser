use lachs::token;
use tokenkind::TokenKind;

#[token(TokenKind)]
pub enum Token {
    #[terminal("<")]
    LAngle,
    #[terminal(">")]
    RAngle,
    #[terminal("=")]
    Equals,
    #[terminal("/")]
    Slash,
    #[literal("[1-9][0-9]*")]
    Number,
    #[literal("[^\\s\\\"\\'<>=/]+")]
    Identifier,
    #[literal("\\\"[^\\\"]*\\\"")]
    Value,
}
