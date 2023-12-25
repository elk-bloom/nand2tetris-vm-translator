use crate::{
    errors::{translation_error::TranslationError, vm_translation_error::VMTranslationError},
    models::{
        command_arithmetic::CommandArithmetic,
        command_pop::CommandPop,
        command_push::CommandPush,
        command_type::CommandType,
        cpu_state::{CPUState, CPUStateBuilder},
        segment::Segment,
        to_assembly::ToAssembly,
    },
};

use super::translator_traits::Translate;

pub struct VMTranslator {
    current_vm_instruction: String,
    cpu_state: CPUState,
}

impl Translate for VMTranslator {
    fn convert(&mut self, vm_instruction: &str) -> Result<String, TranslationError> {
        let split_vm_instruction = self.split_vm_instruction(vm_instruction);

        // from vm_instruction, identify command type (map to a command enum)
        let command_type = self.get_command_type(&split_vm_instruction)?;
        // if the command type uses a segment, identify the segment (taking the first argument)
        match command_type {
            CommandType::Arithmetic(arithmetic_command_type) => {
                let command_arithmetic = CommandArithmetic {
                    arithmetic_command_type,
                };
                Ok(command_arithmetic.to_assembly(&mut self.cpu_state))
            }
            CommandType::Push | CommandType::Pop => {
                let arg_1 = self.get_arg_1(&split_vm_instruction)?;
                let segment = self.get_segment(&arg_1)?;
                let arg_2 = self.get_arg_2(&split_vm_instruction)?;
                let offset: u32 =
                    arg_2
                        .parse::<u32>()
                        .map_err(|_| VMTranslationError::InvalidArgument {
                            vm_instruction: self.current_vm_instruction.clone(),
                            command: split_vm_instruction[0].to_string(),
                            argument_number: 2,
                            argument: split_vm_instruction[2].to_string(),
                        })?;

                match command_type {
                    CommandType::Push => {
                        let command_push = CommandPush { segment, offset };
                        Ok(command_push.to_assembly(&mut self.cpu_state))
                    }
                    CommandType::Pop => {
                        let command_pop = CommandPop { segment, offset };
                        Ok(command_pop.to_assembly(&mut self.cpu_state))
                    }
                    _ => unreachable!(),
                }
            }
            _ => todo!(),
        }
    }

    fn termination_string(&mut self) -> Option<String> {
        let loop_label_name = self.cpu_state.loop_label_prefix.as_str();
        let index = self.cpu_state.loop_label_count;
        let jump_label = format!("{}.{}", loop_label_name, index);
        self.cpu_state.loop_label_count += 1;

        Some(format!(
            "\
            ({})\n\
            @{}\n\
            0;JMP\n",
            jump_label, jump_label
        ))
    }
}

impl VMTranslator {
    pub fn new(file_stem: String) -> Self {
        VMTranslator {
            current_vm_instruction: String::new(),
            cpu_state: CPUStateBuilder::new()
                .loop_label_name(format!("RSRVD_LOOP_{file_stem}"))
                .static_variable_prefix(file_stem)
                .build(),
        }
    }
    /// Splits the current vm_instruction string on whitespace so that the command type and arguments are easily determinable by other functions without them having to do their own splitting.
    fn split_vm_instruction<'a>(&self, vm_instruction: &'a str) -> Vec<&'a str> {
        vm_instruction.split_whitespace().collect()
    }

    /// Gets the command type enum from the split vm instruction
    fn get_command_type(
        &self,
        split_vm_instruction: &Vec<&str>,
    ) -> Result<CommandType, VMTranslationError> {
        if split_vm_instruction.is_empty() {
            return Err(VMTranslationError::EmptyInstruction);
        }

        let command = split_vm_instruction[0];
        command
            .parse()
            .map_err(|_| VMTranslationError::InvalidCommand {
                vm_instruction: self.current_vm_instruction.to_string(),
                command: command.to_string(),
            })
    }

    fn get_arg_1(&self, split_vm_instruction: &[&str]) -> Result<String, VMTranslationError> {
        split_vm_instruction
            .get(1)
            .map(|s| s.to_string())
            .ok_or_else(|| VMTranslationError::MissingArgument {
                vm_instruction: self.current_vm_instruction.to_string(),
                command: split_vm_instruction[0].to_string(),
                argument_number: 1,
            })
    }

    fn get_arg_2(&self, split_vm_instruction: &[&str]) -> Result<String, VMTranslationError> {
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
            .map_err(|_| VMTranslationError::InvalidSegment {
                vm_instruction: self.current_vm_instruction.to_string(),
                segment: arg_1.to_string(),
            })
    }
}
