#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenKind {
    Plus,
    Minus,
    Times,
    Divide,
    LParen,
    RParen,
    Num,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Token {
    pub token_kind: TokenKind,
    pub val: Option<i32>,
}

impl Token {
    pub fn new(token_kind: TokenKind) -> Self {
        Self {
            token_kind: token_kind,
            val: None,
        }
    }

    pub fn new_num(num: i32) -> Self {
        Self {
            token_kind: TokenKind::Num,
            val: Some(num),
        }
    }
}

pub struct Tokenizer {
    tokens: Vec<Token>,
    cur: usize,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        let mut ret = Self {
            tokens: Vec::new(),
            cur: 0,
        };
        ret.tokenize(input.chars().collect());
        ret
    }

    pub fn comsume(&mut self, token_kind: TokenKind) -> Option<Token> {
        if self.cur >= self.tokens.len() {
            return None;
        }
        if self.tokens[self.cur].token_kind == token_kind {
            let ret = Some(self.tokens[self.cur]);
            self.cur += 1;
            ret
        } else {
            None
        }
    }
    
    pub fn expect(&mut self, token_kind: TokenKind) -> Token {
        if self.cur >= self.tokens.len() {
            panic!("syntax error");
        }
        if self.tokens[self.cur].token_kind == token_kind {
            let ret = self.tokens[self.cur];
            self.cur += 1;
            ret
        } else {
            panic!("syntax error");
        }
    }

    pub fn at_eof(&mut self) -> bool {
        self.cur >= self.tokens.len()
    }

    fn tokenize(&mut self, input: Vec<char>) {
        let mut cur = 0;
        while cur < input.len() {
            while cur < input.len() && input[cur].is_ascii_whitespace() {
                cur += 1;
            }
            let token = match input[cur] {
                '+' | '-' | '*' | '/' | '(' | ')' => {
                    let ret = match input[cur] {
                        '+' => Token::new(TokenKind::Plus),
                        '-' => Token::new(TokenKind::Minus),
                        '*' => Token::new(TokenKind::Times),
                        '/' => Token::new(TokenKind::Divide),
                        '(' => Token::new(TokenKind::LParen),
                        ')' => Token::new(TokenKind::RParen),
                        _ => unreachable!(),
                    };
                    cur += 1;
                    ret
                }
                '0'..='9' => {
                    let mut ret = 0;
                    while cur < input.len() && input[cur].is_ascii_digit() {
                        ret = ret * 10 + input[cur] as i32 - '0' as i32;
                        cur += 1;
                    }
                    Token::new_num(ret)
                },
                _ => panic!("error"),
            };
            self.tokens.push(token);
        }
    }
}
