use crate::token_type::TokenType;

#[derive(Debug)]
pub struct ParseError {
    pub token_type: TokenType,
    pub message: String,
}

impl ParseError {
    pub fn new(token_type: TokenType, message: &str) -> Self {
        Self {
            token_type,
            message: message.to_string(),
        }
    }
}
