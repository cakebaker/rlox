use crate::expr::Expr;
use crate::literal::Literal;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct ParseError {
    token_type: TokenType,
    message: String,
}

impl ParseError {
    pub fn new(token_type: TokenType, message: &str) -> Self {
        Self {
            token_type,
            message: message.to_string(),
        }
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

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(e) => println!("{:?}: {}", e.token_type, e.message), // TODO use reporter
            }
        }

        statements
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.do_match(vec![TokenType::Fun]) {
            self.function("function")
        } else if self.do_match(vec![TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, ParseError> {
        let name = self.consume(
            TokenType::Identifier,
            format!("Expect {} name.", kind).as_str(),
        )?;

        self.consume(
            TokenType::LeftParen,
            format!("Expect '(' after {} name.", kind).as_str(),
        )?;
        let mut parameters = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);

                if self.check(&TokenType::RightParen) {
                    break;
                }

                self.do_match(vec![TokenType::Comma]);
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        self.consume(
            TokenType::LeftBrace,
            format!("Expect '{{' before {} body.", kind).as_str(),
        )?;

        if let Stmt::Block(body) = self.block_statement()? {
            Ok(Stmt::Function(name, parameters, body))
        } else {
            // unreachable code, needed to make the compiler happy
            Err(ParseError::new(TokenType::LeftBrace, ""))
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.do_match(vec![TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.do_match(vec![TokenType::For]) {
            self.for_statement()
        } else if self.do_match(vec![TokenType::If]) {
            self.if_statement()
        } else if self.do_match(vec![TokenType::Print]) {
            self.print_statement()
        } else if self.do_match(vec![TokenType::Return]) {
            self.return_statement()
        } else if self.do_match(vec![TokenType::While]) {
            self.while_statement()
        } else if self.do_match(vec![TokenType::LeftBrace]) {
            self.block_statement()
        } else {
            self.expression_statement()
        }
    }

    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous();

        let value = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;

        Ok(Stmt::Return(keyword, value))
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.do_match(vec![TokenType::Semicolon]) {
            None
        } else if self.do_match(vec![TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let mut condition = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if self.check(&TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if increment != None {
            body = Stmt::Block(vec![body, Stmt::Expr(increment.unwrap())]);
        }

        if condition == None {
            condition = Some(Expr::Literal(Literal::Bool(true)));
        }
        body = Stmt::While(condition.unwrap(), Box::new(body));

        if initializer != None {
            body = Stmt::Block(vec![initializer.unwrap(), body]);
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;

        let body = Box::new(self.statement()?);

        Ok(Stmt::While(condition, body))
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);

        let else_branch = if self.do_match(vec![TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(condition, then_branch, else_branch))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(expr))
    }

    fn block_statement(&mut self) -> Result<Stmt, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block")?;

        Ok(Stmt::Block(statements))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.do_match(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.do_match(vec![TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.do_match(vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
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
            _ => self.call(),
        }
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.do_match(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);

                if !self.do_match(vec![TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.advance();

        match token.token_type {
            TokenType::False => Ok(Expr::Literal(Literal::Bool(false))),
            TokenType::True => Ok(Expr::Literal(Literal::Bool(true))),
            TokenType::Nil => Ok(Expr::Literal(Literal::Nil)),
            TokenType::Number(number) => Ok(Expr::Literal(Literal::Number(number))),
            TokenType::String(string) => Ok(Expr::Literal(Literal::String(string))),
            TokenType::Identifier => Ok(Expr::Variable(self.previous())),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::Grouping {
                    expression: Box::new(expr),
                })
            }
            _ => Err(ParseError::new(token.token_type, "Invalid token")),
        }
    }

    fn do_match(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(&token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError::new(token_type, message))
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == *token_type
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
    use crate::literal::Literal;
    use crate::token::Token;
    use crate::token_type::TokenType;

    #[test]
    fn parse_unary_bang() {
        let mut parser = Parser::new(vec![
            token(TokenType::Bang),
            token(TokenType::False),
            token(TokenType::Eof),
        ]);
        if let Ok(result) = parser.expression() {
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
        if let Ok(result) = parser.expression() {
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
