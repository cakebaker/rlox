use crate::expr::Expr;
use crate::expr::Literal;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct ParseError {
    token_type: TokenType,
    message: String,
}

impl ParseError {
    pub fn new(token_type: TokenType, message: &str) -> Self {
        Self { token_type, message: message.to_string() }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.do_match(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.do_match(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.do_match(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.do_match(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        let token = self.peek();

        match token.token_type {
            TokenType::Bang | TokenType::Minus => {
                let operator = self.advance();
                let right = self.unary()?;
                Ok(Expr::Unary {
                    operator,
                    right: Box::new(right),
                })
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.advance();

        match token.token_type {
            TokenType::False => Ok(Expr::Literal(Literal::Bool(false))),
            TokenType::True => Ok(Expr::Literal(Literal::Bool(true))),
            TokenType::Nil => Ok(Expr::Literal(Literal::Nil)),
            TokenType::Number(number) => Ok(Expr::Literal(Literal::Number(number))),
            TokenType::String(string) => Ok(Expr::Literal(Literal::String(string))),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.");
                Ok(Expr::Grouping {
                    expression: Box::new(expr),
                })
            }
            _ => Err(ParseError::new(token.token_type, "Invalid token")),
        }
    }

    fn do_match(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(token_type.clone()) {
            Ok(self.advance())
        } else {
            Err(ParseError::new(token_type, message))
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::expr::Expr;
    use crate::expr::Literal;
    use crate::token::Token;
    use crate::token_type::TokenType;

    #[test]
    fn parse_unary_bang() {
        let mut parser = Parser::new(vec![
            token(TokenType::Bang),
            token(TokenType::False),
            token(TokenType::Eof),
        ]);
        if let Ok(result) = parser.parse() {
            assert_eq!(
                Expr::Unary {
                    operator: token(TokenType::Bang),
                    right: Box::new(Expr::Literal(Literal::Bool(false)))
                },
                result
            );
        } else {
            panic!("parser.parse() returned unexpected Err");
        }
    }

    #[test]
    fn parse_unary_minus() {
        let mut parser = Parser::new(vec![
            token(TokenType::Minus),
            token(TokenType::Number(1.0)),
            token(TokenType::Eof),
        ]);
        if let Ok(result) = parser.parse() {
            assert_eq!(
                Expr::Unary {
                    operator: token(TokenType::Minus),
                    right: Box::new(Expr::Literal(Literal::Number(1.0)))
                },
                result
            );
        } else {
            panic!("parser.parse() returned unexpected Err");
        }
    }

    fn token(token_type: TokenType) -> Token {
        Token::new(token_type, "".to_string(), 1)
    }
}
