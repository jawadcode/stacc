use logos::Logos;
use std::fmt;

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum LogosToken {
    #[token("pop")]
    Pop,

    #[token("print")]
    Print,

    #[token("push")]
    Push,

    #[token("set")]
    Set,

    #[token("call")]
    Call,

    #[regex(r#"([A-Za-z]|_)([A-Za-z]|_|\d)*"#)]
    Ident,

    #[regex("[0-9]+", priority = 2)]
    IntLit,

    #[regex(r#"((\d+(\.\d+)?)|(\.\d+))([Ee](\+|-)?\d+)?"#, priority = 1)]
    FloatLit,

    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    StringLit,

    #[token("begin")]
    Begin,

    #[token("end")]
    End,

    #[regex(r#"\{[^\}]*\}"#)]
    Comment,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("and")]
    And,

    #[token("not")]
    Not,

    #[token("or")]
    Or,

    #[regex("(\r\n|\n)+")]
    Newline,

    #[token(":")]
    Colon,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("<")]
    Less,

    #[token(">")]
    Greater,

    #[token("<=")]
    LessEq,

    #[token(">=")]
    GreaterEq,

    #[token("!=")]
    NotEq,

    #[token("==")]
    Equals,

    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenKind {
    Pop,
    Print,
    Push,
    Set,
    Call,
    Ident,
    IntLit,
    FloatLit,
    StringLit,
    Begin,
    End,
    Comment,
    True,
    False,
    And,
    Not,
    Or,
    Newline,
    Colon,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Multiply,
    Divide,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    NotEq,
    Equals,
    Error,
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pop => "pop",
                Self::Print => "print",
                Self::Push => "push",
                Self::Set => "set",
                Self::Call => "call",
                Self::Ident => "identifier",
                Self::IntLit => "integer literal",
                Self::FloatLit => "float literal",
                Self::StringLit => "string literal",
                Self::Begin => "begin",
                Self::End => "'end'",
                Self::Comment => "comment literal",
                Self::True => "true",
                Self::False => "false",
                Self::And => "and",
                Self::Not => "not",
                Self::Or => "or",
                Self::Newline => "newline",
                Self::Colon => "colon",
                Self::LeftBracket => "[",
                Self::RightBracket => "]",
                Self::LeftParen => "(",
                Self::RightParen => ")",
                Self::Plus => "+",
                Self::Minus => "-",
                Self::Multiply => "*",
                Self::Divide => "/",
                Self::Less => "<",
                Self::Greater => ">",
                Self::LessEq => "<=",
                Self::GreaterEq => ">=",
                Self::NotEq => "!=",
                Self::Equals => "==",
                Self::Error => "error",
                Self::Eof => "EOF",
            }
        )
    }
}

impl From<LogosToken> for TokenKind {
    fn from(logos_token: LogosToken) -> Self {
        match logos_token {
            LogosToken::Pop => Self::Pop,
            LogosToken::Print => Self::Print,
            LogosToken::Push => Self::Push,
            LogosToken::Set => Self::Set,
            LogosToken::Call => Self::Call,
            LogosToken::Ident => Self::Ident,
            LogosToken::IntLit => Self::IntLit,
            LogosToken::FloatLit => Self::FloatLit,
            LogosToken::StringLit => Self::StringLit,
            LogosToken::Begin => Self::Begin,
            LogosToken::End => Self::End,
            LogosToken::Comment => Self::Comment,
            LogosToken::True => Self::True,
            LogosToken::False => Self::False,
            LogosToken::And => Self::And,
            LogosToken::Not => Self::Not,
            LogosToken::Or => Self::Or,
            LogosToken::Newline => Self::Newline,
            LogosToken::Colon => Self::Colon,
            LogosToken::LeftBracket => Self::LeftBracket,
            LogosToken::RightBracket => Self::RightBracket,
            LogosToken::LeftParen => Self::LeftParen,
            LogosToken::RightParen => Self::RightParen,
            LogosToken::Plus => Self::Plus,
            LogosToken::Minus => Self::Minus,
            LogosToken::Multiply => Self::Multiply,
            LogosToken::Divide => Self::Divide,
            LogosToken::Less => Self::Less,
            LogosToken::Greater => Self::Greater,
            LogosToken::LessEq => Self::LessEq,
            LogosToken::GreaterEq => Self::GreaterEq,
            LogosToken::NotEq => Self::NotEq,
            LogosToken::Equals => Self::Equals,
            LogosToken::Error => Self::Error,
        }
    }
}
