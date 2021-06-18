use crate::expr::Expr;
use crate::literal::Literal;
use crate::parse_error::ParseError;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;

type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub const fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Vec<Stmt>, Vec<ParseError>> {
        self.tokens = tokens;
        self.current = 0;
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(e) => errors.push(e),
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn declaration(&mut self) -> ParseResult<Stmt> {
        if self.do_match(vec![TokenType::Class]) {
            self.class_declaration()
        } else if self.do_match(vec![TokenType::Fun]) {
            self.function("function")
        } else if self.do_match(vec![TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn class_declaration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume_identifier(ParseError::MissingClassName(self.previous()))?;
        self.consume(
            TokenType::LeftBrace,
            ParseError::MissingBraceBeforeClassBody(self.previous()),
        )?;

        let mut methods = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }

        self.consume(
            TokenType::RightBrace,
            ParseError::MissingBraceAfterClassBody(self.previous()),
        )?;

        Ok(Stmt::Class(name, methods))
    }

    fn function(&mut self, kind: &str) -> ParseResult<Stmt> {
        let name =
            self.consume_identifier(ParseError::MissingName(self.previous(), kind.to_string()))?;

        self.consume(
            TokenType::LeftParen,
            ParseError::MissingParenAfterName(name.clone(), kind.to_string()),
        )?;
        let mut parameters = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                parameters.push(
                    self.consume_identifier(ParseError::MissingParameterName(self.previous()))?,
                );

                if self.check(&TokenType::RightParen) {
                    break;
                }

                self.do_match(vec![TokenType::Comma]);
            }
        }

        self.consume(
            TokenType::RightParen,
            ParseError::MissingParenAfterParameters(self.previous()),
        )?;
        self.consume(
            TokenType::LeftBrace,
            ParseError::MissingBraceBeforeBody(self.previous(), kind.to_string()),
        )?;

        if let Stmt::Block(body) = self.block_statement()? {
            Ok(Stmt::Function(name, parameters, body))
        } else {
            // unreachable code, needed to make the compiler happy
            Err(ParseError::UnexpectedError)
        }
    }

    fn var_declaration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume_identifier(ParseError::MissingVariableName(self.previous()))?;

        let initializer = if self.do_match(vec![TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            ParseError::MissingSemicolonAfterVariableDeclaration(name.clone()),
        )?;
        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> ParseResult<Stmt> {
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

    fn return_statement(&mut self) -> ParseResult<Stmt> {
        let keyword = self.previous();

        let value = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(
            TokenType::Semicolon,
            ParseError::MissingSemicolonAfterReturnValue(keyword.clone()),
        )?;

        Ok(Stmt::Return(keyword, value))
    }

    fn for_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(
            TokenType::LeftParen,
            ParseError::MissingParenAfterFor(self.previous()),
        )?;

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
        self.consume(
            TokenType::Semicolon,
            ParseError::MissingSemicolonAfterLoopCondition(self.previous()),
        )?;

        let increment = if self.check(&TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(
            TokenType::RightParen,
            ParseError::MissingParenAfterForClauses(self.previous()),
        )?;

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

    fn while_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(
            TokenType::LeftParen,
            ParseError::MissingParenAfterWhile(self.previous()),
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            ParseError::MissingParenAfterWhileCondition(self.previous()),
        )?;

        let body = Box::new(self.statement()?);

        Ok(Stmt::While(condition, body))
    }

    fn if_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(
            TokenType::LeftParen,
            ParseError::MissingParenAfterIf(self.previous()),
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            ParseError::MissingParenAfterIfCondition(self.previous()),
        )?;

        let then_branch = Box::new(self.statement()?);

        let else_branch = if self.do_match(vec![TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(condition, then_branch, else_branch))
    }

    fn print_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            ParseError::MissingSemicolonAfterValue(self.previous()),
        )?;
        Ok(Stmt::Print(expr))
    }

    fn block_statement(&mut self) -> ParseResult<Stmt> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(
            TokenType::RightBrace,
            ParseError::MissingBraceAfterBlock(self.previous()),
        )?;

        Ok(Stmt::Block(statements))
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            ParseError::MissingSemicolonAfterValue(self.previous()),
        )?;
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> ParseResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.or()?;

        if self.do_match(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            } else if let Expr::Get { object, name } = expr {
                return Ok(Expr::Set {
                    object,
                    name,
                    value: Box::new(value),
                });
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> ParseResult<Expr> {
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

    fn and(&mut self) -> ParseResult<Expr> {
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

    fn equality(&mut self) -> ParseResult<Expr> {
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

    fn comparison(&mut self) -> ParseResult<Expr> {
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

    fn term(&mut self) -> ParseResult<Expr> {
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

    fn factor(&mut self) -> ParseResult<Expr> {
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

    fn unary(&mut self) -> ParseResult<Expr> {
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

    fn call(&mut self) -> ParseResult<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.do_match(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.do_match(vec![TokenType::Dot]) {
                let name =
                    self.consume_identifier(ParseError::MissingPropertyName(self.previous()))?;
                expr = Expr::Get {
                    object: Box::new(expr),
                    name,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> ParseResult<Expr> {
        let mut arguments = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);

                if !self.do_match(vec![TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(
            TokenType::RightParen,
            ParseError::MissingParenAfterArguments(self.previous()),
        )?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        let token = self.advance();

        match token.token_type {
            TokenType::False => Ok(Expr::Literal(Literal::Bool(false))),
            TokenType::True => Ok(Expr::Literal(Literal::Bool(true))),
            TokenType::Nil => Ok(Expr::Literal(Literal::Nil)),
            TokenType::Number(number) => Ok(Expr::Literal(Literal::Number(number))),
            TokenType::String(string) => Ok(Expr::Literal(Literal::String(string))),
            TokenType::This => Ok(Expr::This(self.previous())),
            TokenType::Identifier(_) => Ok(Expr::Variable(self.previous())),
            // XXX a '(' at the end causes a stack overflow
            TokenType::LeftParen if !self.is_at_end() => {
                let expr = self.expression()?;
                self.consume(
                    TokenType::RightParen,
                    ParseError::MissingParenAfterExpression(token),
                )?;
                Ok(Expr::Grouping {
                    expression: Box::new(expr),
                })
            }
            _ => Err(ParseError::InvalidToken(token)),
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

    fn consume(&mut self, token_type: TokenType, error: ParseError) -> ParseResult<Token> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(error)
        }
    }

    fn consume_identifier(&mut self, error: ParseError) -> ParseResult<Token> {
        match self.peek().token_type {
            TokenType::Identifier(_) => Ok(self.advance()),
            _ => Err(error),
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
    use crate::parse_error::ParseError;
    use crate::scanner::Scanner;
    use crate::stmt::Stmt;
    use crate::token::Token;
    use crate::token_type::TokenType;

    #[test]
    fn parse_unary_bang() {
        let code = "!false;";
        let expected = Stmt::Expr(Expr::Unary {
            operator: token(TokenType::Bang),
            right: Box::new(Expr::Literal(Literal::Bool(false))),
        });

        let result = parse(code).unwrap();
        assert_eq!(expected, result[0]);
    }

    #[test]
    fn parse_unary_minus() {
        let code = "-1;";
        let expected = Stmt::Expr(Expr::Unary {
            operator: token(TokenType::Minus),
            right: Box::new(Expr::Literal(Literal::Number(1.0))),
        });

        let result = parse(code).unwrap();
        assert_eq!(expected, result[0]);
    }

    #[test]
    fn parse_class() {
        let result = parse(
            r#"class Test {
                   test() {
                       return "test";
                   }
               }"#,
        )
        .unwrap();
        let expected = Stmt::Class(
            Token::new(TokenType::Identifier("Test".to_string()), 1),
            vec![Stmt::Function(
                Token::new(TokenType::Identifier("test".to_string()), 2),
                vec![],
                vec![Stmt::Return(
                    Token::new(TokenType::Return, 3),
                    Some(Expr::Literal(Literal::String("test".to_string()))),
                )],
            )],
        );
        assert_eq!(expected, result[0]);
    }

    #[test]
    fn parse_class_without_name() {
        let errors = parse("class").unwrap_err();
        let expected = ParseError::MissingClassName(token(TokenType::Class));
        assert_eq!(expected, errors[0]);
    }

    #[test]
    fn parse_class_with_missing_left_brace() {
        let errors = parse("class Test").unwrap_err();
        let expected = ParseError::MissingBraceBeforeClassBody(token(TokenType::Identifier(
            "Test".to_string(),
        )));
        assert_eq!(expected, errors[0]);
    }

    #[test]
    fn parse_class_with_missing_right_brace() {
        let errors = parse("class Test {").unwrap_err();
        let expected = ParseError::MissingBraceAfterClassBody(token(TokenType::LeftBrace));
        assert_eq!(expected, errors[0]);
    }

    #[test]
    fn parse_getter() {
        let result = parse("someObject.someProperty;").unwrap();
        let expected = Stmt::Expr(Expr::Get {
            object: Box::new(Expr::Variable(token(TokenType::Identifier(
                "someObject".to_string(),
            )))),
            name: token(TokenType::Identifier("someProperty".to_string())),
        });
        assert_eq!(expected, result[0]);
    }

    #[test]
    fn parse_getter_with_missing_name() {
        let errors = parse("someObject.").unwrap_err();
        let expected = ParseError::MissingPropertyName(token(TokenType::Dot));
        assert_eq!(expected, errors[0]);
    }

    #[test]
    fn parse_setter() {
        let result = parse("someObject.someProperty = value;").unwrap();
        let expected = Stmt::Expr(Expr::Set {
            object: Box::new(Expr::Variable(token(TokenType::Identifier(
                "someObject".to_string(),
            )))),
            name: token(TokenType::Identifier("someProperty".to_string())),
            value: Box::new(Expr::Variable(token(TokenType::Identifier(
                "value".to_string(),
            )))),
        });
        assert_eq!(expected, result[0]);
    }

    #[test]
    fn parse_this() {
        let result = parse("this;").unwrap();
        let expected = Stmt::Expr(Expr::This(token(TokenType::This)));
        assert_eq!(expected, result[0]);
    }

    #[test]
    fn parse_invalid_statements() {
        let codes_and_expected_errors = vec![
            ("(", ParseError::InvalidToken(token(TokenType::LeftParen))),
            (")", ParseError::InvalidToken(token(TokenType::RightParen))),
            (
                "(1 + 2",
                ParseError::MissingParenAfterExpression(token(TokenType::LeftParen)),
            ),
            (
                "{ x = 0;",
                ParseError::MissingBraceAfterBlock(token(TokenType::Semicolon)),
            ),
            (
                "1",
                ParseError::MissingSemicolonAfterValue(token(TokenType::Number(1.0))),
            ),
            (
                "for",
                ParseError::MissingParenAfterFor(token(TokenType::For)),
            ),
            (
                "for (x = 0; x < 10",
                ParseError::MissingSemicolonAfterLoopCondition(token(TokenType::Number(10.0))),
            ),
            (
                "for (x = 0; x < 10; x + 1",
                ParseError::MissingParenAfterForClauses(token(TokenType::Number(1.0))),
            ),
            (
                "fun",
                ParseError::MissingName(token(TokenType::Fun), "function".to_string()),
            ),
            (
                "fun xyz",
                ParseError::MissingParenAfterName(
                    token(TokenType::Identifier("xyz".to_string())),
                    "function".to_string(),
                ),
            ),
            (
                "fun xyz(",
                ParseError::MissingParameterName(token(TokenType::LeftParen)),
            ),
            (
                "fun xyz()",
                ParseError::MissingBraceBeforeBody(
                    token(TokenType::RightParen),
                    "function".to_string(),
                ),
            ),
            (
                "fun xyz() { return 0 }",
                ParseError::MissingSemicolonAfterReturnValue(token(TokenType::Return)),
            ),
            ("if", ParseError::MissingParenAfterIf(token(TokenType::If))),
            (
                "if (x < y",
                ParseError::MissingParenAfterIfCondition(token(TokenType::Identifier(
                    "y".to_string(),
                ))),
            ),
            (
                "var",
                ParseError::MissingVariableName(token(TokenType::Var)),
            ),
            (
                "var x = 123",
                ParseError::MissingSemicolonAfterVariableDeclaration(token(TokenType::Identifier(
                    "x".to_string(),
                ))),
            ),
            (
                "while",
                ParseError::MissingParenAfterWhile(token(TokenType::While)),
            ),
            (
                "while (x < y",
                ParseError::MissingParenAfterWhileCondition(token(TokenType::Identifier(
                    "y".to_string(),
                ))),
            ),
            (
                "xyz(true",
                ParseError::MissingParenAfterArguments(token(TokenType::True)),
            ),
        ];

        for (code, expected_error) in codes_and_expected_errors {
            let parse_errors = parse(code).unwrap_err();
            assert_eq!(expected_error, parse_errors[0]);
        }
    }

    fn parse(code: &str) -> Result<Vec<Stmt>, Vec<ParseError>> {
        Parser::new().parse(Scanner::scan(code).unwrap())
    }

    fn token(token_type: TokenType) -> Token {
        Token::new(token_type, 1)
    }
}
