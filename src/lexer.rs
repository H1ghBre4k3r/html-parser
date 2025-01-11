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
    #[literal("[^\\s\\\"\\'<>=/]+")]
    Identifier,
    #[literal("\\\"[^\\\"]*\\\"")]
    Value,
}
