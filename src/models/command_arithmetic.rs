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
                    A=M-1\n\
                    M={}\n",
                    operation
                );

                cpu_state.clear();

                match prefix {
                    Some(p) => format!(
                        "\
                        {}\
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
                    @SP\n\
                    A=M-1\n\
                    M={}\n",
                    operation
                );

                cpu_state.clear();

                format!(
                    "\
                    {}\
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
                let loop_label_name = cpu_state.loop_label_prefix.as_str();
                let index = cpu_state.loop_label_count;

                let true_jump_label = format!("{}.{}", loop_label_name, index);
                let false_jump_label = format!("{}.{}", loop_label_name, index + 1);
                cpu_state.loop_label_count += 2;

                cpu_state.clear();
                cpu_state.const_or_predefined_a_register.push_str("SP");

                format!(
                    "\
                    {prefix}\
                    @SP\n\
                    AM=M-1\n\
                    D=M-D\n\
                    @{true_jump_label}\n\
                    D;{jump_condition}\n\
                    D=0\n\
                    @{false_jump_label}\n\
                    0;JMP\n\
                    ({true_jump_label})\n\
                    D=-1\n\
                    ({false_jump_label})\n\
                    @SP\n\
                    A=M\n\
                    M=D\n\
                    @SP\n\
                    M=M+1\n"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cpu_state::CPUState;

    mod setup {
        use crate::models::cpu_state::CPUStateBuilder;

        use super::*;

        const DEFAULT_FILENAME: &str = "default";

        pub fn cpu_state_with_sp() -> CPUState {
            CPUStateBuilder::new()
                .a_register("SP".to_string())
                .loop_label_name(DEFAULT_FILENAME.to_string())
                .build()
        }

        pub fn empty_cpu_state() -> CPUState {
            CPUStateBuilder::new()
                .loop_label_name(DEFAULT_FILENAME.to_string())
                .build()
        }
    }

    #[test]
    fn neg() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Neg,
        };
        let result = ca.to_assembly(&mut setup::empty_cpu_state());
        let expected_result = "\
        @SP\n\
        A=M-1\n\
        M=-M\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn not() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Not,
        };
        let result = ca.to_assembly(&mut setup::empty_cpu_state());
        let expected_result = "\
        @SP\n\
        A=M-1\n\
        M=!M\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn add() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Add,
        };
        let result = ca.to_assembly(&mut setup::empty_cpu_state());
        let expected_result = "\
        @SP\n\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        A=M-1\n\
        M=M+D\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn sub() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Sub,
        };
        let result = ca.to_assembly(&mut setup::empty_cpu_state());
        let expected_result = "\
        @SP\n\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        A=M-1\n\
        M=M-D\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn and() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::And,
        };
        let result = ca.to_assembly(&mut setup::empty_cpu_state());
        let expected_result = "\
        @SP\n\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        A=M-1\n\
        M=D&M\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn or() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Or,
        };
        let result = ca.to_assembly(&mut setup::empty_cpu_state());
        let expected_result = "\
        @SP\n\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        A=M-1\n\
        M=D|M\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn eq() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Eq,
        };
        let result = ca.to_assembly(&mut setup::empty_cpu_state());

        let expected_result = "\
        @SP\n\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @default.0\n\
        D;JEQ\n\
        D=0\n\
        @default.1\n\
        0;JMP\n\
        (default.0)\n\
        D=-1\n\
        (default.1)\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn gt() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Gt,
        };
        let result = ca.to_assembly(&mut setup::empty_cpu_state());
        let expected_result = "\
        @SP\n\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @default.0\n\
        D;JGT\n\
        D=0\n\
        @default.1\n\
        0;JMP\n\
        (default.0)\n\
        D=-1\n\
        (default.1)\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn lt() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Lt,
        };
        let result = ca.to_assembly(&mut setup::empty_cpu_state());

        let expected_result = "\
        @SP\n\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @default.0\n\
        D;JLT\n\
        D=0\n\
        @default.1\n\
        0;JMP\n\
        (default.0)\n\
        D=-1\n\
        (default.1)\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn neg_sp() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Neg,
        };
        let result = ca.to_assembly(&mut setup::cpu_state_with_sp());
        let expected_result = "\
        A=M-1\n\
        M=-M\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn not_sp() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Not,
        };
        let result = ca.to_assembly(&mut setup::cpu_state_with_sp());
        let expected_result = "\
        A=M-1\n\
        M=!M\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn add_sp() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Add,
        };
        let result = ca.to_assembly(&mut setup::cpu_state_with_sp());
        let expected_result = "\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        A=M-1\n\
        M=M+D\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn sub_sp() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Sub,
        };
        let result = ca.to_assembly(&mut setup::cpu_state_with_sp());
        let expected_result = "\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        A=M-1\n\
        M=M-D\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn and_sp() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::And,
        };
        let result = ca.to_assembly(&mut setup::cpu_state_with_sp());
        let expected_result = "\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        A=M-1\n\
        M=D&M\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn or_sp() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Or,
        };
        let result = ca.to_assembly(&mut setup::cpu_state_with_sp());
        let expected_result = "\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        A=M-1\n\
        M=D|M\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn eq_sp() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Eq,
        };
        let result = ca.to_assembly(&mut setup::cpu_state_with_sp());

        let expected_result = "\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @default.0\n\
        D;JEQ\n\
        D=0\n\
        @default.1\n\
        0;JMP\n\
        (default.0)\n\
        D=-1\n\
        (default.1)\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn gt_sp() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Gt,
        };
        let result = ca.to_assembly(&mut setup::cpu_state_with_sp());
        let expected_result = "\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @default.0\n\
        D;JGT\n\
        D=0\n\
        @default.1\n\
        0;JMP\n\
        (default.0)\n\
        D=-1\n\
        (default.1)\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n";

        assert_eq!(result, expected_result)
    }

    #[test]
    fn lt_sp() {
        let ca = CommandArithmetic {
            arithmetic_command_type: ArithmeticCommandType::Lt,
        };
        let result = ca.to_assembly(&mut setup::cpu_state_with_sp());

        let expected_result = "\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @default.0\n\
        D;JLT\n\
        D=0\n\
        @default.1\n\
        0;JMP\n\
        (default.0)\n\
        D=-1\n\
        (default.1)\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n";

        assert_eq!(result, expected_result)
    }
}
