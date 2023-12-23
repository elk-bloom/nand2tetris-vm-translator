use std::str::FromStr;

use crate::errors::parse_command_error::ParseCommandError;

pub enum CommandType {
    Arithmetic(ArithmeticCommandType),
    Push,
    Pop,
    Label,
    Goto,
    If,
    Function,
    Return,
    Call,
}

pub enum ArithmeticCommandType {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl FromStr for CommandType {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
                Ok(CommandType::Arithmetic(s.parse().map_err(|_| {
                    ParseCommandError {
                        command: s.to_string(),
                    }
                })?))
            }
            "push" => Ok(CommandType::Push),
            "pop" => Ok(CommandType::Pop),
            _ => Err(ParseCommandError {
                command: s.to_string(),
            }),
        }
    }
}

impl FromStr for ArithmeticCommandType {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(ArithmeticCommandType::Add),
            "sub" => Ok(ArithmeticCommandType::Sub),
            "neg" => Ok(ArithmeticCommandType::Neg),
            "eq" => Ok(ArithmeticCommandType::Eq),
            "gt" => Ok(ArithmeticCommandType::Gt),
            "lt" => Ok(ArithmeticCommandType::Lt),
            "and" => Ok(ArithmeticCommandType::And),
            "or" => Ok(ArithmeticCommandType::Or),
            "not" => Ok(ArithmeticCommandType::Not),
            _ => Err(ParseCommandError {
                command: s.to_string(),
            }),
        }
    }
}
