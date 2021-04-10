#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl TokenType {
    pub fn get_type_for_keyword(keyword: &str) -> Option<Self> {
        match keyword {
            "and" => Some(Self::And),
            "class" => Some(Self::Class),
            "else" => Some(Self::Else),
            "false" => Some(Self::False),
            "fun" => Some(Self::Fun),
            "for" => Some(Self::For),
            "if" => Some(Self::If),
            "nil" => Some(Self::Nil),
            "or" => Some(Self::Or),
            "print" => Some(Self::Print),
            "return" => Some(Self::Return),
            "super" => Some(Self::Super),
            "this" => Some(Self::This),
            "true" => Some(Self::True),
            "var" => Some(Self::Var),
            "while" => Some(Self::While),
            _ => None,
        }
    }
}
