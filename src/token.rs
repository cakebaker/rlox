use crate::token_type::TokenType;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    None,
    String(String),
    Number(f64),
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub const fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn lexeme(&self) -> String {
        self.lexeme.clone()
    }

    pub const fn line(&self) -> usize {
        self.line
    }

    pub fn literal(&self) -> Option<Literal> {
        self.literal.clone()
    }

    pub const fn token_type(&self) -> TokenType {
        self.token_type
    }
}
