use super::{command_pop_stack::PopStack, segment::Segment, to_assembly::ToAssembly};

pub struct CommandPop {
    pub segment: Segment,
    pub offset: u32,
}

impl ToAssembly for CommandPop {
    fn to_assembly(&self, cpu_state: &mut super::cpu_state::CPUState) -> String {
        let pop_stack_assembly = PopStack::to_assembly(cpu_state);
        match self.segment {
            Segment::Argument | Segment::Local | Segment::This | Segment::That => {
                let segment_name = self.segment.to_assembly();

                cpu_state.clear();

                if self.offset == 0 {
                    let main_assembly = format!(
                        "\
                        @{}\n\
                        A=M\n\
                        M=D\n",
                        segment_name
                    );
                    format!(
                        "\
                        {}\
                        {}",
                        pop_stack_assembly, main_assembly
                    )
                } else {
                    format!(
                        "\
                        @{}\n\
                        D=A\n\
                        @{}\n\
                        D=D+M\n\
                        @R13\n\
                        M=D\n\
                        {}\
                        @R13\n\
                        A=M\n\
                        M=D\n",
                        self.offset, segment_name, pop_stack_assembly
                    )
                }
            }
            Segment::Pointer => {
                let segment_name = match self.offset {
                    0 => Segment::This.to_assembly(),
                    1 => Segment::That.to_assembly(),
                    _ => unreachable!(), // or it should be anyway ( ͡° ͜ʖ ͡°) TODO: handle error case
                };

                cpu_state.clear();
                cpu_state
                    .const_or_predefined_a_register
                    .push_str(segment_name.as_str());

                format!(
                    "\
                    {}\
                    @{}\n\
                    M=D\n",
                    pop_stack_assembly, segment_name
                )
            }
            Segment::Temp => {
                // TODO: return error if outside of bounds
                let index = 5 + self.offset;

                cpu_state.clear();
                cpu_state
                    .const_or_predefined_a_register
                    .push_str(index.to_string().as_ref());

                format!(
                    "\
                    {}\
                    @{}\n\
                    M=D\n",
                    pop_stack_assembly, index
                )
            }
            Segment::Static => {
                // TODO: ensure number of static variables is not greater than the reserved address space (16-255) can hold
                let symbol = format!("{}.{}", cpu_state.loop_label_name, self.offset);

                cpu_state.clear();
                cpu_state
                    .const_or_predefined_a_register
                    .push_str(symbol.as_str());

                format!(
                    "\
                    {}\
                    @{}\n\
                    M=D\n",
                    pop_stack_assembly, symbol
                )
            }
            Segment::Constant => {
                // TODO: make this function return a result I guess
                panic!("CONSTANT NOT VALID SEGMENT FOR POP")
            }
        }
    }
}
