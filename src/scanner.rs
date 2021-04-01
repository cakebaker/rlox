use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {}

impl Scanner {
    pub fn scan_tokens(source: &str) -> Vec<Token> {
        let tokens = vec![];
        let initial_line = 1;
        Self::scan_token(source, tokens, initial_line)
    }

    fn scan_token(source: &str, mut tokens: Vec<Token>, mut line: usize) -> Vec<Token> {
        if source.is_empty() {
            tokens.push(Token::new(TokenType::Eof, "".to_string(), line));
            tokens
        } else {
            let mut token_length = 1;
            let c = source.chars().next().unwrap();

            match c {
                '(' => tokens.push(Token::new(TokenType::LeftParen, c.to_string(), line)),
                ')' => tokens.push(Token::new(TokenType::RightParen, c.to_string(), line)),
                '{' => tokens.push(Token::new(TokenType::LeftBrace, c.to_string(), line)),
                '}' => tokens.push(Token::new(TokenType::RightBrace, c.to_string(), line)),
                ',' => tokens.push(Token::new(TokenType::Comma, c.to_string(), line)),
                '.' => tokens.push(Token::new(TokenType::Dot, c.to_string(), line)),
                '-' => tokens.push(Token::new(TokenType::Minus, c.to_string(), line)),
                '+' => tokens.push(Token::new(TokenType::Plus, c.to_string(), line)),
                ';' => tokens.push(Token::new(TokenType::Semicolon, c.to_string(), line)),
                '*' => tokens.push(Token::new(TokenType::Star, c.to_string(), line)),
                '!' => {
                    let look_ahead = source.chars().nth(1);
                    if look_ahead == Some('=') {
                        tokens.push(Token::new(TokenType::BangEqual, "!=".to_string(), line));
                        token_length = 2;
                    } else {
                        tokens.push(Token::new(TokenType::Bang, c.to_string(), line));
                    }
                }
                '=' => {
                    let look_ahead = source.chars().nth(1);
                    if look_ahead == Some('=') {
                        tokens.push(Token::new(TokenType::EqualEqual, "==".to_string(), line));
                        token_length = 2;
                    } else {
                        tokens.push(Token::new(TokenType::Equal, c.to_string(), line));
                    }
                }
                '<' => {
                    let look_ahead = source.chars().nth(1);
                    if look_ahead == Some('=') {
                        tokens.push(Token::new(TokenType::LessEqual, "<=".to_string(), line));
                        token_length = 2;
                    } else {
                        tokens.push(Token::new(TokenType::Less, c.to_string(), line));
                    }
                }
                '>' => {
                    let look_ahead = source.chars().nth(1);
                    if look_ahead == Some('=') {
                        tokens.push(Token::new(TokenType::GreaterEqual, ">=".to_string(), line));
                        token_length = 2;
                    } else {
                        tokens.push(Token::new(TokenType::Greater, c.to_string(), line));
                    }
                }
                ' ' | '\r' | '\t' => {} // ignore whitespace
                '\n' => line += 1,
                _ => {} // TODO handle error
            }
            Self::scan_token(&source[token_length..], tokens, line)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::token_type::TokenType;

    #[test]
    fn scan_empty_string() {
        let result = Scanner::scan_tokens("");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_type(), TokenType::Eof);
        assert_eq!(result[0].line(), 1);
    }

    #[test]
    fn scan_single_char_tokens() {
        let strings_and_token_types = vec![
            ("(", TokenType::LeftParen),
            (")", TokenType::RightParen),
            ("{", TokenType::LeftBrace),
            ("}", TokenType::RightBrace),
            (",", TokenType::Comma),
            (".", TokenType::Dot),
            ("-", TokenType::Minus),
            ("+", TokenType::Plus),
            (";", TokenType::Semicolon),
            ("*", TokenType::Star),
        ];

        for (string, expected_token_type) in strings_and_token_types {
            let result = Scanner::scan_tokens(string);
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].token_type(), expected_token_type);
            assert_eq!(result[1].token_type(), TokenType::Eof);
        }
    }

    #[test]
    fn scan_one_or_two_char_tokens() {
        let strings_and_token_types = vec![
            ("!", TokenType::Bang),
            ("=", TokenType::Equal),
            ("<", TokenType::Less),
            (">", TokenType::Greater),
            ("!=", TokenType::BangEqual),
            ("==", TokenType::EqualEqual),
            ("<=", TokenType::LessEqual),
            (">=", TokenType::GreaterEqual),
        ];

        for (string, expected_token_type) in strings_and_token_types {
            let result = Scanner::scan_tokens(string);
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].token_type(), expected_token_type);
            assert_eq!(result[1].token_type(), TokenType::Eof);
        }
    }

    #[test]
    fn ignore_whitespace() {
        let strings = vec![" ", "\r", "\t"];

        for string in strings {
            let result = Scanner::scan_tokens(string);
            assert_eq!(result.len(), 1);
            assert_eq!(result[0].token_type(), TokenType::Eof);
        }
    }

    #[test]
    fn increase_line_counter_after_linebreak() {
        let result = Scanner::scan_tokens("\n");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_type(), TokenType::Eof);
        assert_eq!(result[0].line(), 2);
    }
}
