use std::cell::Cell;
use crate::token::{Token, TokenType};

pub mod token;

pub struct Tokens {
    inner: Vec<Token>,
    pos: Cell<usize>,
}

impl Tokens {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            inner: tokens,
            pos: Cell::new(0),
        }
    }

    pub fn current_token(&self) -> &Token {
        self.inner.get(self.pos.get()).unwrap()
    }

    pub fn current(&self) -> TokenType {
        self.current_token().token_type
    }

    pub fn look_after_token(&self) -> Option<&Token> {
        let next = self.pos.get() + 1;
        let token = self.inner.get(next);
        match token {
            Some(token) => {
                // self.pos.set(next);
                Some(token)
            }
            None => None,
        }
    }

    pub fn look_after(&self) -> Option<TokenType> {
        self.look_after_token().map(|t| t.token_type)
    }

    pub fn take(&self, t: TokenType) -> Result<&Token, String> {
        if t == self.current() {
            let token = self.current_token();
            self.pos.set(self.pos.get() + 1);
            Ok(token)
        } else {
            let now = self.current_token();
            Err(format!("line: {}, column: {}, expected {:?}, got {:?}", now.line, now.column, t, now.token_type))
        }
    }

    pub fn move_next(&self) {
        self.pos.set(self.pos.get() + 1);
    }
}
