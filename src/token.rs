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

        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Token;
    use crate::token_type::TokenType;

    #[test]
    fn new_with_string_token_type() {
        let token = Token::new(TokenType::String("test".to_string()), 1);
        assert_eq!("\"test\"", token.lexeme);
    }
}
