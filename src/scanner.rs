use crate::reporter::Reporter;
use crate::token::Literal;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {}

impl Scanner {
    pub fn scan_tokens(source: &str, reporter: &mut Reporter) -> Vec<Token> {
        let tokens = vec![];
        let initial_line = 1;
        Self::scan_token(source, tokens, initial_line, reporter)
    }

    fn scan_token(
        source: &str,
        mut tokens: Vec<Token>,
        mut line: usize,
        reporter: &mut Reporter,
    ) -> Vec<Token> {
        if source.is_empty() {
            tokens.push(Token::new(TokenType::Eof, "".to_string(), None, line));
            tokens
        } else {
            let mut munched_chars = 1;
            let c = source.chars().next().unwrap();

            match c {
                '(' => tokens.push(Token::new(TokenType::LeftParen, c.to_string(), None, line)),
                ')' => tokens.push(Token::new(TokenType::RightParen, c.to_string(), None, line)),
                '{' => tokens.push(Token::new(TokenType::LeftBrace, c.to_string(), None, line)),
                '}' => tokens.push(Token::new(TokenType::RightBrace, c.to_string(), None, line)),
                ',' => tokens.push(Token::new(TokenType::Comma, c.to_string(), None, line)),
                '.' => tokens.push(Token::new(TokenType::Dot, c.to_string(), None, line)),
                '-' => tokens.push(Token::new(TokenType::Minus, c.to_string(), None, line)),
                '+' => tokens.push(Token::new(TokenType::Plus, c.to_string(), None, line)),
                ';' => tokens.push(Token::new(TokenType::Semicolon, c.to_string(), None, line)),
                '*' => tokens.push(Token::new(TokenType::Star, c.to_string(), None, line)),
                '/' => {
                    let look_ahead = source.chars().nth(1);
                    if look_ahead == Some('/') {
                        let linebreak_position = source.find('\n');
                        if linebreak_position == None {
                            munched_chars = source.len();
                        } else {
                            munched_chars = linebreak_position.unwrap();
                        }
                    } else {
                        tokens.push(Token::new(TokenType::Slash, c.to_string(), None, line));
                    }
                }
                '!' => {
                    let look_ahead = source.chars().nth(1);
                    if look_ahead == Some('=') {
                        tokens.push(Token::new(
                            TokenType::BangEqual,
                            "!=".to_string(),
                            None,
                            line,
                        ));
                        munched_chars = 2;
                    } else {
                        tokens.push(Token::new(TokenType::Bang, c.to_string(), None, line));
                    }
                }
                '=' => {
                    let look_ahead = source.chars().nth(1);
                    if look_ahead == Some('=') {
                        tokens.push(Token::new(
                            TokenType::EqualEqual,
                            "==".to_string(),
                            None,
                            line,
                        ));
                        munched_chars = 2;
                    } else {
                        tokens.push(Token::new(TokenType::Equal, c.to_string(), None, line));
                    }
                }
                '<' => {
                    let look_ahead = source.chars().nth(1);
                    if look_ahead == Some('=') {
                        tokens.push(Token::new(
                            TokenType::LessEqual,
                            "<=".to_string(),
                            None,
                            line,
                        ));
                        munched_chars = 2;
                    } else {
                        tokens.push(Token::new(TokenType::Less, c.to_string(), None, line));
                    }
                }
                '>' => {
                    let look_ahead = source.chars().nth(1);
                    if look_ahead == Some('=') {
                        tokens.push(Token::new(
                            TokenType::GreaterEqual,
                            ">=".to_string(),
                            None,
                            line,
                        ));
                        munched_chars = 2;
                    } else {
                        tokens.push(Token::new(TokenType::Greater, c.to_string(), None, line));
                    }
                }
                ' ' | '\r' | '\t' => {} // ignore whitespace
                '\n' => line += 1,
                '"' => {
                    if let Some(position) = source[1..].find('"') {
                        // correct position because the find() doesn't start from the beginning
                        let close_position = position + 1;
                        tokens.push(Token::new(
                            TokenType::String,
                            source[..=close_position].to_string(),
                            Some(Literal::String(source[1..close_position].to_string())),
                            line,
                        ));
                        munched_chars = close_position + 1;
                        line += source[..close_position].matches('\n').count();
                    } else {
                        reporter
                            .report_error(format!("Unterminated string starting on line {}", line));
                        munched_chars = source.len();
                    }
                }
                '0'..='9' => {
                    let token = Self::scan_number(source, line);

                    munched_chars = token.len();
                    tokens.push(token);
                }
                '_' | 'a'..='z' | 'A'..='Z' => {
                    let token = Self::scan_identifier(source, line);

                    munched_chars = token.len();
                    tokens.push(token);
                }
                _ => reporter.report_error(format!("Unexpected character {} on line {}", c, line)),
            }
            Self::scan_token(&source[munched_chars..], tokens, line, reporter)
        }
    }

    fn scan_identifier(source: &str, line: usize) -> Token {
        let identifier: String = source
            .chars()
            .take_while(|c| c.is_ascii_alphabetic() || *c == '_')
            .collect();

        let token_type = match TokenType::get_type_for_keyword(&identifier) {
            Some(keyword_type) => keyword_type,
            None => TokenType::Identifier,
        };

        Token::new(token_type, identifier, None, line)
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
        Token::new(
            TokenType::Number,
            number.to_string(),
            Some(Literal::Number(number.parse::<f64>().unwrap())),
            line,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::reporter::Reporter;
    use crate::token::Literal;
    use crate::token_type::TokenType;

    #[test]
    fn scan_empty_string() {
        let result = Scanner::scan_tokens("", &mut Reporter::new());
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
            let result = Scanner::scan_tokens(string, &mut Reporter::new());
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
            let result = Scanner::scan_tokens(string, &mut Reporter::new());
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].token_type, expected_token_type);
            assert_eq!(result[1].token_type, TokenType::Eof);
        }
    }

    #[test]
    fn ignore_comments() {
        let mut result = Scanner::scan_tokens("// a comment", &mut Reporter::new());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_type, TokenType::Eof);

        result = Scanner::scan_tokens("// a comment\n;", &mut Reporter::new());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].token_type, TokenType::Semicolon);
        assert_eq!(result[1].token_type, TokenType::Eof);
    }

    #[test]
    fn ignore_whitespace() {
        let strings = vec![" ", "\r", "\t"];

        for string in strings {
            let result = Scanner::scan_tokens(string, &mut Reporter::new());
            assert_eq!(result.len(), 1);
            assert_eq!(result[0].token_type, TokenType::Eof);
        }
    }

    #[test]
    fn increase_line_counter_after_linebreak() {
        let result = Scanner::scan_tokens("\n", &mut Reporter::new());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_type, TokenType::Eof);
        assert_eq!(result[0].line, 2);
    }

    #[test]
    fn scan_string_literals() {
        let result = Scanner::scan_tokens("\"A string\"", &mut Reporter::new());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].token_type, TokenType::String);
        assert_eq!(result[0].lexeme, "\"A string\"");
        assert_eq!(
            result[0].literal,
            Some(Literal::String("A string".to_string()))
        );
        assert_eq!(result[1].token_type, TokenType::Eof);
    }

    #[test]
    fn scan_unterminated_string() {
        let mut reporter = Reporter::new();
        assert_eq!(reporter.get_errors().len(), 0);
        Scanner::scan_tokens("\"A string", &mut reporter);
        assert_eq!(reporter.get_errors().len(), 1);
    }

    #[test]
    fn scan_multiline_strings() {
        let result = Scanner::scan_tokens("\"Line A\nLine B\"", &mut Reporter::new());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].token_type, TokenType::String);
        assert_eq!(result[1].token_type, TokenType::Eof);
        assert_eq!(result[1].line, 2);
    }

    #[test]
    fn scan_number_literals() {
        let numbers_and_literals = vec![("123", 123 as f64), ("123.45", 123.45)];

        for (number, literal) in numbers_and_literals {
            let result = Scanner::scan_tokens(number, &mut Reporter::new());
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].token_type, TokenType::Number);
            assert_eq!(result[0].lexeme, number);
            assert_eq!(result[0].literal, Some(Literal::Number(literal)));
            assert_eq!(result[1].token_type, TokenType::Eof);
        }
    }

    #[test]
    fn scan_identifiers() {
        let identifiers = vec!["_id", "id", "ID", "i_d"];

        for identifier in identifiers {
            let result = Scanner::scan_tokens(identifier, &mut Reporter::new());
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
            let result = Scanner::scan_tokens(keyword, &mut Reporter::new());
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].token_type, token_type);
            assert_eq!(result[1].token_type, TokenType::Eof);
        }
    }

    #[test]
    fn scan_invalid_character() {
        let mut reporter = Reporter::new();
        assert_eq!(reporter.get_errors().len(), 0);
        Scanner::scan_tokens("@", &mut reporter);
        assert_eq!(reporter.get_errors().len(), 1);
    }
}
