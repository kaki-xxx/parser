pub mod tokenizer;

use tokenizer::*;

pub struct Parser {
    tokenizer: tokenizer::Tokenizer,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self {
            tokenizer: tokenizer::Tokenizer::new(input)
        }
    }

    pub fn parse(&mut self) -> Option<i32> {
        let ret = self.expr();
        if self.tokenizer.at_eof() {
            ret
        } else {
            None
        }
    }

    fn expr(&mut self) -> Option<i32> {
        let mut ret = self.term()?;
        loop {
            if let Some(_) = self.tokenizer.comsume(TokenKind::Plus) {
                ret += self.term()?;
            } else if let Some(_) = self.tokenizer.comsume(TokenKind::Minus) {
                ret -= self.term()?;
            } else {
                break;
            }
        }
        Some(ret)
    }

    fn term(&mut self) -> Option<i32> {
        let mut ret = self.factor()?;
        loop {
            if let Some(_) = self.tokenizer.comsume(TokenKind::Times) {
                ret *= self.factor()?;
            } else if let Some(_) = self.tokenizer.comsume(TokenKind::Divide) {
                ret /= self.factor()?;
            } else {
                break;
            }
        }
        Some(ret)
    }

    fn factor(&mut self) -> Option<i32> {
        if let Some(token) = self.tokenizer.comsume(TokenKind::Num) {
            token.val
        } else if let Some(_) = self.tokenizer.comsume(TokenKind::LParen) {
            let ret = self.expr();
            self.tokenizer.expect(TokenKind::RParen);
            ret
        } else {
            None
        }
    }
}
