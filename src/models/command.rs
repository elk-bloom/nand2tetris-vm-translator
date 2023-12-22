use std::str::FromStr;

use crate::errors::parse_command_error::ParseCommandError;

pub enum Command {
    Arithmetic(ArithmeticCommand),
    Push,
    Pop,
    Label,
    Goto,
    If,
    Function,
    Return,
    Call,
}

pub enum ArithmeticCommand {
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

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
                Ok(Command::Arithmetic(s.parse().map_err(|_| {
                    ParseCommandError {
                        command: s.to_string(),
                    }
                })?))
            }
            "push" => Ok(Command::Push),
            "pop" => Ok(Command::Pop),
            _ => Err(ParseCommandError {
                command: s.to_string(),
            }),
        }
    }
}

impl FromStr for ArithmeticCommand {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(ArithmeticCommand::Add),
            "sub" => Ok(ArithmeticCommand::Sub),
            "neg" => Ok(ArithmeticCommand::Neg),
            "eq" => Ok(ArithmeticCommand::Eq),
            "gt" => Ok(ArithmeticCommand::Gt),
            "lt" => Ok(ArithmeticCommand::Lt),
            "and" => Ok(ArithmeticCommand::And),
            "or" => Ok(ArithmeticCommand::Or),
            "not" => Ok(ArithmeticCommand::Not),
            _ => Err(ParseCommandError {
                command: s.to_string(),
            }),
        }
    }
}
