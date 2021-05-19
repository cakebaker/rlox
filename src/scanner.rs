use crate::scan_error::ScanError;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {}

impl Scanner {
    pub fn scan_tokens(source: &str) -> Result<Vec<Token>, ScanError> {
        let tokens = vec![];
        let initial_line = 1;
        Self::scan_token(source, tokens, initial_line)
    }

    fn scan_token(
        source: &str,
        mut tokens: Vec<Token>,
        mut line: usize,
    ) -> Result<Vec<Token>, ScanError> {
        if source.is_empty() {
            tokens.push(Token::new(TokenType::Eof, line));
            Ok(tokens)
        } else {
            let mut munched_chars = 1;
            let c = source.chars().next().unwrap();

            let maybe_token = match c {
                '(' => Some(Token::new(TokenType::LeftParen, line)),
                ')' => Some(Token::new(TokenType::RightParen, line)),
                '{' => Some(Token::new(TokenType::LeftBrace, line)),
                '}' => Some(Token::new(TokenType::RightBrace, line)),
                ',' => Some(Token::new(TokenType::Comma, line)),
                '.' => Some(Token::new(TokenType::Dot, line)),
                '-' => Some(Token::new(TokenType::Minus, line)),
                '+' => Some(Token::new(TokenType::Plus, line)),
                ';' => Some(Token::new(TokenType::Semicolon, line)),
                '*' => Some(Token::new(TokenType::Star, line)),
                '/' if matches!(source.chars().nth(1), Some('/')) => {
                    let linebreak_position = source.find('\n');
                    if linebreak_position == None {
                        munched_chars = source.len();
                    } else {
                        munched_chars = linebreak_position.unwrap();
                    }
                    None
                }
                '/' => Some(Token::new(TokenType::Slash, line)),
                '!' if matches!(source.chars().nth(1), Some('=')) => {
                    munched_chars = 2;
                    Some(Token::new(TokenType::BangEqual, line))
                }
                '!' => Some(Token::new(TokenType::Bang, line)),
                '=' if matches!(source.chars().nth(1), Some('=')) => {
                    munched_chars = 2;
                    Some(Token::new(TokenType::EqualEqual, line))
                }
                '=' => Some(Token::new(TokenType::Equal, line)),
                '<' if matches!(source.chars().nth(1), Some('=')) => {
                    munched_chars = 2;
                    Some(Token::new(TokenType::LessEqual, line))
                }
                '<' => Some(Token::new(TokenType::Less, line)),
                '>' if matches!(source.chars().nth(1), Some('=')) => {
                    munched_chars = 2;
                    Some(Token::new(TokenType::GreaterEqual, line))
                }
                '>' => Some(Token::new(TokenType::Greater, line)),
                ' ' | '\r' | '\t' => None, // ignore whitespace
                '\n' => {
                    line += 1;
                    None
                }
                '"' => {
                    if let Some(position) = source[1..].find('"') {
                        // correct position because the find() doesn't start from the beginning
                        let close_position = position + 1;
                        munched_chars = close_position + 1;
                        line += source[..close_position].matches('\n').count();

                        Some(Token::new(
                            TokenType::String(source[1..close_position].to_string()),
                            line,
                        ))
                    } else {
                        return Err(ScanError::UnterminatedString(line));
                    }
                }
                '0'..='9' => {
                    let token = Self::scan_number(source, line);

                    munched_chars = token.lexeme.len();
                    Some(token)
                }
                '_' | 'a'..='z' | 'A'..='Z' => {
                    let token = Self::scan_identifier(source, line);

                    munched_chars = token.lexeme.len();
                    Some(token)
                }
                _ => return Err(ScanError::UnexpectedChar(c, line)),
            };

            if let Some(token) = maybe_token {
                tokens.push(token);
            }

            Self::scan_token(&source[munched_chars..], tokens, line)
        }
    }

    fn scan_identifier(source: &str, line: usize) -> Token {
        let identifier: String = source
            .chars()
            .take_while(|c| c.is_ascii_alphabetic() || *c == '_')
            .collect();

        let token_type = match Self::get_type_if_keyword(&identifier) {
            Some(keyword_type) => keyword_type,
            None => TokenType::Identifier,
        };

        Token::new_with_lexeme(token_type, identifier, line)
    }

    fn scan_number(source: &str, line: usize) -> Token {
        let mut munched_chars = source.chars().take_while(char::is_ascii_digit).count();

        if source[munched_chars..].chars().take(1).collect::<String>() == "." {
            let n = source[(munched_chars + 1)..]
                .chars()
                .take_while(char::is_ascii_digit)
                .count();

            if n > 0 {
                munched_chars = munched_chars + 1 + n;
            }
        }

        let number = &source[..munched_chars];
        Token::new(TokenType::Number(number.parse().unwrap()), line)
    }

    fn get_type_if_keyword(keyword: &str) -> Option<TokenType> {
        match keyword {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "fun" => Some(TokenType::Fun),
            "for" => Some(TokenType::For),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::scan_error::ScanError;
    use crate::token_type::TokenType;

    #[test]
    fn scan_empty_string() {
        let result = Scanner::scan_tokens("").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_type, TokenType::Eof);
        assert_eq!(result[0].line, 1);
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
            ("/", TokenType::Slash),
        ];

        for (string, expected_token_type) in strings_and_token_types {
            let result = Scanner::scan_tokens(string).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].token_type, expected_token_type);
            assert_eq!(result[1].token_type, TokenType::Eof);
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
            let result = Scanner::scan_tokens(string).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].token_type, expected_token_type);
            assert_eq!(result[1].token_type, TokenType::Eof);
        }
    }

    #[test]
    fn ignore_comments() {
        let mut result = Scanner::scan_tokens("// a comment").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_type, TokenType::Eof);

        result = Scanner::scan_tokens("// a comment\n;").unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].token_type, TokenType::Semicolon);
        assert_eq!(result[1].token_type, TokenType::Eof);
    }

    #[test]
    fn ignore_whitespace() {
        let strings = vec![" ", "\r", "\t"];

        for string in strings {
            let result = Scanner::scan_tokens(string).unwrap();
            assert_eq!(result.len(), 1);
            assert_eq!(result[0].token_type, TokenType::Eof);
        }
    }

    #[test]
    fn increase_line_counter_after_linebreak() {
        let result = Scanner::scan_tokens("\n").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_type, TokenType::Eof);
        assert_eq!(result[0].line, 2);
    }

    #[test]
    fn scan_string_literals() {
        let result = Scanner::scan_tokens("\"A string\"").unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0].token_type,
            TokenType::String("A string".to_string())
        );
        assert_eq!(result[0].lexeme, "\"A string\"");
        assert_eq!(result[1].token_type, TokenType::Eof);
    }

    #[test]
    fn scan_unterminated_string() {
        match Scanner::scan_tokens("\"A string") {
            Err(ScanError::UnterminatedString(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn scan_multiline_strings() {
        let result = Scanner::scan_tokens("\"Line A\nLine B\"").unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0].token_type,
            TokenType::String("Line A\nLine B".to_string())
        );
        assert_eq!(result[1].token_type, TokenType::Eof);
        assert_eq!(result[1].line, 2);
    }

    #[test]
    fn scan_number_literals() {
        let numbers_and_literals = vec![("123", 123 as f64), ("123.45", 123.45)];

        for (number, literal) in numbers_and_literals {
            let result = Scanner::scan_tokens(number).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].token_type, TokenType::Number(literal));
            assert_eq!(result[0].lexeme, number);
            assert_eq!(result[1].token_type, TokenType::Eof);
        }
    }

    #[test]
    fn scan_identifiers() {
        let identifiers = vec!["_id", "id", "ID", "i_d"];

        for identifier in identifiers {
            let result = Scanner::scan_tokens(identifier).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].token_type, TokenType::Identifier);
            assert_eq!(result[1].token_type, TokenType::Eof);
        }
    }

    #[test]
    fn scan_keywords() {
        let keywords_and_token_types = vec![
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("fun", TokenType::Fun),
            ("for", TokenType::For),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ];

        for (keyword, token_type) in keywords_and_token_types {
            let result = Scanner::scan_tokens(keyword).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].token_type, token_type);
            assert_eq!(result[1].token_type, TokenType::Eof);
        }
    }

    #[test]
    fn scan_invalid_character() {
        let invalid_chars = vec!["@", "ä"];

        for invalid_char in invalid_chars {
            match Scanner::scan_tokens(invalid_char) {
                Err(ScanError::UnexpectedChar(_, _)) => assert!(true),
                _ => assert!(false),
            }
        }
    }
}
