use super::{
    command_pop_stack::PopStack, command_type::ArithmeticCommandType, cpu_state::RegisterType,
    to_assembly::ToAssembly,
};

pub struct CommandArithmetic {
    pub arithmetic_command_type: ArithmeticCommandType,
}

impl ToAssembly for CommandArithmetic {
    fn to_assembly(&self, cpu_state: &mut super::cpu_state::CPUState) -> String {
        match self.arithmetic_command_type {
            ArithmeticCommandType::Neg | ArithmeticCommandType::Not => {
                let operation: &str = match self.arithmetic_command_type {
                    ArithmeticCommandType::Neg => "-M",
                    ArithmeticCommandType::Not => "!M",
                    _ => unreachable!(),
                };

                let prefix = cpu_state.get_prefix("SP", &RegisterType::A);
                let main_assembly = format!(
                    "\
                    A=M-1
                    M={}",
                    operation
                );

                cpu_state.clear();

                match prefix {
                    Some(p) => format!(
                        "\
                        {}
                        {}",
                        p, main_assembly
                    ),
                    None => main_assembly.to_string(),
                }
            }
            ArithmeticCommandType::Add
            | ArithmeticCommandType::Sub
            | ArithmeticCommandType::And
            | ArithmeticCommandType::Or => {
                let operation: &str = match self.arithmetic_command_type {
                    ArithmeticCommandType::Add => "M+D",
                    ArithmeticCommandType::Sub => "M-D",
                    ArithmeticCommandType::And => "D&M",
                    ArithmeticCommandType::Or => "D|M",
                    _ => unreachable!(),
                };

                let prefix = PopStack::to_assembly(cpu_state);
                let main_assembly = format!(
                    "\
                    @SP
                    A=M-1
                    M={}",
                    operation
                );

                cpu_state.clear();

                format!(
                    "\
                    {}
                    {}",
                    prefix, main_assembly
                )
            }
            ArithmeticCommandType::Eq | ArithmeticCommandType::Gt | ArithmeticCommandType::Lt => {
                let jump_condition: &str = match self.arithmetic_command_type {
                    ArithmeticCommandType::Eq => "JEQ",
                    ArithmeticCommandType::Gt => "JGT",
                    ArithmeticCommandType::Lt => "JLT",
                    _ => unreachable!(),
                };

                let prefix = PopStack::to_assembly(cpu_state);
                let loop_label_name = cpu_state.loop_label_name.as_str();
                let index = cpu_state.loop_label_count;

                let true_jump_label = format!("{}.{}", loop_label_name, index);
                let false_jump_label = format!("{}.{}", loop_label_name, index + 1);
                cpu_state.loop_label_count += 2;

                cpu_state.const_or_predefined_a_register.clear();
                cpu_state.const_or_predefined_a_register.push_str("SP");
                cpu_state.const_or_predefined_d_register.clear();

                format!(
                    "\
                    {prefix}
                    @SP
                    AM=M-1
                    D=M-D
                    @{true_jump_label}
                    D;{jump_condition}
                    D=0
                    @{false_jump_label}
                    0;JMP
                    ({true_jump_label})
                    D=-1
                    ({false_jump_label})
                    @SP
                    A=M
                    M=D
                    @SP
                    M=M+1"
                )
            }
        }
    }
}
