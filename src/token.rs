use crate::token_type::TokenType;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Self {
        let lexeme = match token_type {
            TokenType::String(_) => format!("\"{}\"", token_type.to_string()),
            _ => token_type.to_string(),
        };

        Self::new_with_lexeme(token_type, lexeme, line)
    }

    pub const fn new_with_lexeme(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_string_token_type() {
        let token = Token::new(TokenType::String("test".to_string()), 1);
        assert_eq!("\"test\"", token.lexeme);
    }
}
