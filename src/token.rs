use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    pub const fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
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

    pub fn literal(&self) -> Option<String> {
        self.literal.clone()
    }

    pub const fn token_type(&self) -> TokenType {
        self.token_type
    }
}
