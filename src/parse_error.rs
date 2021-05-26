use crate::token_type::TokenType;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    InvalidToken(TokenType),
    MissingBraceAfterBlock,
    MissingBraceBeforeBody(String),
    MissingName(String),
    MissingParameterName,
    MissingParenAfterArguments,
    MissingParenAfterExpression,
    MissingParenAfterFor,
    MissingParenAfterForClauses,
    MissingParenAfterIf,
    MissingParenAfterIfCondition,
    MissingParenAfterName(String),
    MissingParenAfterParameters,
    MissingParenAfterWhile,
    MissingParenAfterWhileCondition,
    MissingSemicolonAfterLoopCondition,
    MissingSemicolonAfterReturnValue,
    MissingSemicolonAfterValue,
    MissingSemicolonAfterVariableDeclaration,
    MissingVariableName,
    UnexpectedError,
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidToken(token_type) => {
                write!(f, "{}: Invalid token.", token_type)
            }
            Self::MissingBraceAfterBlock => {
                write!(f, "Expect '}}' after block.")
            }
            Self::MissingBraceBeforeBody(kind) => {
                write!(f, "Expect '{{' before {} body.", kind)
            }
            Self::MissingName(kind) => {
                write!(f, "Expect {} name.", kind)
            }
            Self::MissingParameterName => {
                write!(f, "Expect parameter name.")
            }
            Self::MissingParenAfterArguments => {
                write!(f, "Expect ')' after arguments.")
            }
            Self::MissingParenAfterExpression => {
                write!(f, "Expect ')' after expression.")
            }
            Self::MissingParenAfterFor => {
                write!(f, "Expect ')' after 'for'.")
            }
            Self::MissingParenAfterForClauses => {
                write!(f, "Expect ')' after 'for' clauses.")
            }
            Self::MissingParenAfterIf => {
                write!(f, "Expect '(' after 'if'.")
            }
            Self::MissingParenAfterIfCondition => {
                write!(f, "Expect ')' after 'if' condition.")
            }
            Self::MissingParenAfterName(kind) => {
                write!(f, "Expect '(' after {} name.", kind)
            }
            Self::MissingParenAfterParameters => {
                write!(f, "Expect ')' after parameters.")
            }
            Self::MissingParenAfterWhile => {
                write!(f, "Expect '(' after 'while'.")
            }
            Self::MissingParenAfterWhileCondition => {
                write!(f, "Expect ')' after 'while' condition.")
            }
            Self::MissingSemicolonAfterLoopCondition => {
                write!(f, "Expect ';' after loop condition.")
            }
            Self::MissingSemicolonAfterReturnValue => {
                write!(f, "Expect ';' after return value.")
            }
            Self::MissingSemicolonAfterValue => {
                write!(f, "Expect ';' after value.")
            }
            Self::MissingSemicolonAfterVariableDeclaration => {
                write!(f, "Expect ';' after variable declaration.")
            }
            Self::MissingVariableName => {
                write!(f, "Expect variable name.")
            }
            Self::UnexpectedError => {
                write!(f, "Unexpected error.")
            }
        }
    }
}
