use crate::lexer::*;

#[derive(Debug, Clone)]
pub struct ParseStream {
    tokens: Vec<Token>,
    position: usize,
}

impl ParseStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        ParseStream {
            tokens,
            position: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<Token> {
        let item = self.tokens.get(self.position).cloned();
        self.position += 1;
        item
    }
}
