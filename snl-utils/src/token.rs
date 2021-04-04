use std::str::FromStr;
use std::fmt::{Display, Formatter};

#[repr(C)]
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub image: String,
    pub line: u32,
    pub column: u32,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t", self.line)?;
        write!(f, "{:?}", self.token_type)?;
        match self.token_type {
            TokenType::Identifer | TokenType::Int => {
                write!(f, "\t{}", self.image)?;
            }
            _ => {}
        }
        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Identifer,
    Int,

    Program,
    Type,
    Array,
    Integer,
    Char,
    Record,
    Of,
    Var,
    Procedure,
    Begin,
    End,
    If,
    Then,
    Else,
    Fi,
    While,
    Do,
    EndWhile,
    Read,
    Write,
    Return,

    Add,
    Minus,
    Multiply,
    Divide,
    Equal,
    Assign,
    Comma,
    Semicolon,
    LessThan,
    BracketOpen,
    BracketClose,
    SquareBracketOpen,
    SquareBracketClose,
    Dot,
    DotDot,

    EOF,
}

impl FromStr for TokenType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "program" => TokenType::Program,
            "type" => TokenType::Type,
            "array" => TokenType::Array,
            "integer" => TokenType::Integer,
            "char" => TokenType::Char,
            "record" => TokenType::Record,
            "of" => TokenType::Of,
            "var" => TokenType::Var,
            "procedure" => TokenType::Procedure,
            "begin" => TokenType::Begin,
            "end" => TokenType::End,
            "if" => TokenType::If,
            "then" => TokenType::Then,
            "else" => TokenType::Else,
            "fi" => TokenType::Fi,
            "while" => TokenType::While,
            "do" => TokenType::Do,
            "endwh" => TokenType::EndWhile,
            "read" => TokenType::Read,
            "write" => TokenType::Write,
            "return" => TokenType::Return,
            "+" => TokenType::Add,
            "-" => TokenType::Minus,
            "*" => TokenType::Multiply,
            "/" => TokenType::Divide,
            "=" => TokenType::Equal,
            ":=" => TokenType::Assign,
            "," => TokenType::Comma,
            ";" => TokenType::Semicolon,
            "<" => TokenType::LessThan,
            "[" => TokenType::SquareBracketOpen,
            "]" => TokenType::SquareBracketClose,
            "(" => TokenType::BracketOpen,
            ")" => TokenType::BracketClose,
            "." => TokenType::Dot,
            ".." => TokenType::DotDot,
            &_ => {
                return Err(format!("invalid token: {}", s));
            }
        })
    }
}