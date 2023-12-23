use super::{command_pop_stack::PopStack, segment::Segment, to_assembly::ToAssembly};

pub struct CommandPop {
    pub segment: Segment,
    pub offset: u32,
}

impl ToAssembly for CommandPop {
    fn to_assembly(&self, cpu_state: &mut super::cpu_state::CPUState) -> String {
        let pop_stack_assembly = PopStack::to_assembly(cpu_state);
        match self.segment {
            Segment::Argument
            | Segment::Local
            | Segment::This
            | Segment::That
            | Segment::Pointer => {
                let segment_name: &str = match self.segment {
                    Segment::Argument => "ARG",
                    Segment::Local => "LCL",
                    Segment::This => "THIS",
                    Segment::That => "THAT",
                    Segment::Pointer => match self.offset {
                        0 => "THIS",
                        1 => "THAT",
                        _ => unreachable!(), // or it should be anyway ( ͡° ͜ʖ ͡°)
                    },
                    _ => unreachable!(),
                };

                cpu_state.clear();

                if self.offset == 0 {
                    let main_assembly = format!(
                        "\
                        @{}
                        A=M
                        M=D",
                        segment_name
                    );
                    format!(
                        "\
                        {}
                        {}",
                        pop_stack_assembly, main_assembly
                    )
                } else {
                    format!(
                        "\
                        @{}
                        D=A
                        @{}
                        D=D+M
                        @R13
                        M=D
                        {}
                        @R13
                        A=M
                        M=D",
                        self.offset, segment_name, pop_stack_assembly
                    )
                }
            }
            Segment::Temp => {
                // TODO: return error if outside of bounds
                let index = 5 + self.offset;

                cpu_state.const_or_predefined_a_register.clear();
                cpu_state
                    .const_or_predefined_a_register
                    .push_str(index.to_string().as_ref());
                cpu_state.const_or_predefined_d_register.clear();

                format!(
                    "\
                    {}
                    @{}
                    M=D",
                    pop_stack_assembly, index
                )
            }
            Segment::Static => {
                // TODO: ensure number of static variables is not greater than the reserved address space (16-255) can hold
                let symbol = format!("{}.{}", cpu_state.loop_label_name, self.offset);
                format!(
                    "\
                    {}
                    @{}
                    M=D", pop_stack_assembly, symbol
                )
            }
            Segment::Constant => {
                // TODO: make this function return a result I guess
                panic!("CONSTANT NOT VALID SEGMENT FOR POP")
            }
        }
    }
}
