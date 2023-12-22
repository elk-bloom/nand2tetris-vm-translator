use std::fmt;

#[derive(Debug)]
pub enum VMTranslationError {
    EmptyInstruction,
    InvalidCommand {
        vm_instruction: String,
        command: String,
    },
    InvalidSegment {
        vm_instruction: String,
        segment: String,
    },
    MissingArgument {
        vm_instruction: String,
        command: String,
        argument_number: u8,
    },
    InvalidArgument {
        vm_instruction: String,
        command: String,
        argument_number: u8,
        argument: String,
    },
}

impl fmt::Display for VMTranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInstruction => write!(f, "VM Instruction is empty."),
            Self::InvalidCommand {
                vm_instruction,
                command,
            } => write!(
                f,
                "Command '{}' for VM Instruction '{}' is invalid.",
                command, vm_instruction
            ),
            Self::InvalidSegment {
                vm_instruction,
                segment,
            } => write!(
                f,
                "Segment '{}' for VM Instruction '{}' is invalid.",
                segment, vm_instruction
            ),
            Self::MissingArgument {
                vm_instruction,
                command,
                argument_number,
            } => write!(
                f,
                "Command '{}' for VM Instruction '{}' is missing argument {}.",
                command, vm_instruction, argument_number
            ),
            Self::InvalidArgument {
                vm_instruction,
                command,
                argument_number,
                argument,
            } => write!(
                f,
                "Command '{}' for VM Instruction '{}' has invalid argument {} of '{}'.",
                command, vm_instruction, argument_number, argument
            ),
        }
    }
}

impl std::error::Error for VMTranslationError {}
