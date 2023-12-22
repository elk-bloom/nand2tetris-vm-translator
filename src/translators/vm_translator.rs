use std::str::FromStr;

use crate::{
    errors::{
        parse_offset_error::ParseOffsetError, translation_error::TranslationError,
        vm_translation_error::VMTranslationError,
    },
    models::{command::Command, segment::Segment},
};

use super::translator_traits::Translate;

pub struct VMTranslator {
    current_vm_instruction: String,
    last_a_register: u32,
    last_d_register: u32,
}

impl Translate for VMTranslator {
    fn convert(&self, vm_instruction: &str) -> Result<String, TranslationError> {
        let split_vm_instruction = self.split_vm_instruction(vm_instruction);

        // from vm_instruction, identify command type (map to a command enum)
        let command = self.get_command_type(&split_vm_instruction)?;
        // if the command type uses a segment, identify the segment (taking the first argument)
        match command {
            Command::Arithmetic(arithmetic_command) => {
                // TODO: write arithmetic command
                Ok("asdf".to_string())
            }
            Command::Push | Command::Pop => {
                let arg_1 = self.get_arg_1(&split_vm_instruction)?;
                let segment = self.get_segment(arg_1)?;
                let arg_2 = self.get_arg_2(&split_vm_instruction)?;
                let offset: u32 =
                    arg_2
                        .parse()
                        .unwrap_or_else(|_| Err(VMTranslationError::InvalidArgument {
                            vm_instruction: self.current_vm_instruction,
                            command: split_vm_instruction[0].to_string(),
                            argument_number: 2,
                            argument: split_vm_instruction[2].to_string(),
                        }))?;
                
            }
        }
        // if the command type expects a second argument, extract the second argument

        // it might make sense to just extract all arguments at once. If there are more than expected we can return an error. If there are less than expected or they are invalid we can also return an error.
    }
}

impl VMTranslator {
    /// Splits the current vm_instruction string on whitespace so that the command type and arguments are easily determinable by other functions without them having to do their own splitting.
    fn split_vm_instruction<'a>(&self, vm_instruction: &'a str) -> Vec<&'a str> {
        vm_instruction.split_whitespace().collect()
    }

    /// Gets the command type enum from the split vm instruction
    fn get_command_type(
        &self,
        split_vm_instruction: &Vec<&str>,
    ) -> Result<Command, VMTranslationError> {
        if split_vm_instruction.is_empty() {
            return Err(VMTranslationError::EmptyInstruction);
        }

        let command = split_vm_instruction[0];
        command
            .parse()
            .map_err(|e| VMTranslationError::InvalidCommand {
                vm_instruction: self.current_vm_instruction.to_string(),
                command: command.to_string(),
            })
    }

    fn get_arg_1(&self, split_vm_instruction: &Vec<&str>) -> Result<String, VMTranslationError> {
        split_vm_instruction
            .get(1)
            .map(|s| s.to_string())
            .ok_or_else(|| VMTranslationError::MissingArgument {
                vm_instruction: self.current_vm_instruction.to_string(),
                command: split_vm_instruction[0].to_string(),
                argument_number: 1,
            })
    }

    fn get_arg_2(&self, split_vm_instruction: &Vec<&str>) -> Result<String, VMTranslationError> {
        split_vm_instruction
            .get(2)
            .map(|s| s.to_string())
            .ok_or_else(|| VMTranslationError::MissingArgument {
                vm_instruction: self.current_vm_instruction.to_string(),
                command: split_vm_instruction[0].to_string(),
                argument_number: 2,
            })
    }

    fn get_segment(&self, arg_1: &str) -> Result<Segment, VMTranslationError> {
        arg_1
            .parse()
            .map_err(|e| VMTranslationError::InvalidSegment {
                vm_instruction: self.current_vm_instruction.to_string(),
                segment: arg_1.to_string(),
            })
    }
}
