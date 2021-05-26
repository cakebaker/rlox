use crate::token::Token;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    InvalidToken(Token),
    MissingBraceAfterBlock(Token),
    MissingBraceBeforeBody(Token, String),
    MissingName(Token, String),
    MissingParameterName(Token),
    MissingParenAfterArguments(Token),
    MissingParenAfterExpression(Token),
    MissingParenAfterFor(Token),
    MissingParenAfterForClauses(Token),
    MissingParenAfterIf(Token),
    MissingParenAfterIfCondition(Token),
    MissingParenAfterName(Token, String),
    MissingParenAfterParameters(Token),
    MissingParenAfterWhile(Token),
    MissingParenAfterWhileCondition(Token),
    MissingSemicolonAfterLoopCondition(Token),
    MissingSemicolonAfterReturnValue(Token),
    MissingSemicolonAfterValue(Token),
    MissingSemicolonAfterVariableDeclaration(Token),
    MissingVariableName(Token),
    UnexpectedError,
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidToken(token) => {
                write!(
                    f,
                    "Invalid token '{}' on line {}.",
                    token.lexeme, token.line
                )
            }
            Self::MissingBraceAfterBlock(token) => {
                write!(f, "Expect '}}' after block on line {}.", token.line)
            }
            Self::MissingBraceBeforeBody(token, kind) => {
                write!(
                    f,
                    "Expect '{{' before {} body on line {}.",
                    kind, token.line
                )
            }
            Self::MissingName(token, kind) => {
                write!(f, "Expect {} name on line {}.", kind, token.line)
            }
            Self::MissingParameterName(token) => {
                write!(f, "Expect parameter name on line {}.", token.line)
            }
            Self::MissingParenAfterArguments(token) => {
                write!(f, "Expect ')' after arguments on line {}.", token.line)
            }
            Self::MissingParenAfterExpression(token) => {
                write!(f, "Expect ')' after expression on line {}.", token.line)
            }
            Self::MissingParenAfterFor(token) => {
                write!(f, "Expect '(' after 'for' on line {}.", token.line)
            }
            Self::MissingParenAfterForClauses(token) => {
                write!(f, "Expect ')' after 'for' clauses on line {}.", token.line)
            }
            Self::MissingParenAfterIf(token) => {
                write!(f, "Expect '(' after 'if' on line {}.", token.line)
            }
            Self::MissingParenAfterIfCondition(token) => {
                write!(f, "Expect ')' after 'if' condition on line {}.", token.line)
            }
            Self::MissingParenAfterName(token, kind) => {
                write!(
                    f,
                    "Expect '(' after {} name '{}' on line {}.",
                    kind, token.lexeme, token.line
                )
            }
            Self::MissingParenAfterParameters(token) => {
                write!(f, "Expect ')' after parameters on line {}.", token.line)
            }
            Self::MissingParenAfterWhile(token) => {
                write!(f, "Expect '(' after 'while' on line {}.", token.line)
            }
            Self::MissingParenAfterWhileCondition(token) => {
                write!(
                    f,
                    "Expect ')' after 'while' condition on line {}.",
                    token.line
                )
            }
            Self::MissingSemicolonAfterLoopCondition(token) => {
                write!(f, "Expect ';' after loop condition on line {}.", token.line)
            }
            Self::MissingSemicolonAfterReturnValue(token) => {
                write!(f, "Expect ';' after return value on line {}.", token.line)
            }
            Self::MissingSemicolonAfterValue(token) => {
                write!(
                    f,
                    "Expect ';' after value '{}' on line {}.",
                    token.lexeme, token.line
                )
            }
            Self::MissingSemicolonAfterVariableDeclaration(token) => {
                write!(
                    f,
                    "Expect ';' after declaration of variable '{}' on line {}.",
                    token.lexeme, token.line
                )
            }
            Self::MissingVariableName(token) => {
                write!(f, "Expect variable name on line {}.", token.line)
            }
            Self::UnexpectedError => {
                write!(f, "Unexpected error.")
            }
        }
    }
}
