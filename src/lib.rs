use lex_gen::token;

#[token]
pub enum Token {
    #[terminal("<")]
    LAngle,
    #[terminal(">")]
    RAngle,
    #[terminal("=")]
    Equals,
    #[literal("[1-9][0-9]*")]
    Number,
    #[literal("[^\\s\\\"\\'<>=]+")]
    AttributeKey,
    #[literal("\\\"[^\\\"]*\\\"")]
    AttributeValue,
}
