use super::{command_push_stack::PushStack, segment::Segment, to_assembly::ToAssembly};

pub struct CommandPush {
    pub segment: Segment,
    pub offset: u32,
}

impl ToAssembly for CommandPush {
    fn to_assembly(&self, cpu_state: &mut super::cpu_state::CPUState) -> String {
        cpu_state.clear(); // just do not feel like implementing the perfect solution to removing redundant instructions
        let push_stack_assembly = PushStack::to_assembly(cpu_state);
        match self.segment {
            Segment::Argument | Segment::Local | Segment::This | Segment::That => {
                let segment_name = self.segment.to_assembly();

                if self.offset == 0 {
                    format!(
                        "\
                        @{}\n\
                        A=M\n\
                        D=M\n\
                        {}",
                        segment_name, push_stack_assembly
                    )
                } else {
                    format!(
                        "\
                    @{}\n\
                    D=A\n\
                    @{}\n\
                    A=D+M\n\
                    D=M\n\
                    {}",
                        self.offset, segment_name, push_stack_assembly
                    )
                }
            }
            Segment::Pointer => {
                let segment_name: String = match self.offset {
                    0 => Segment::This.to_assembly(),
                    1 => Segment::That.to_assembly(),
                    _ => unreachable!(), // or it should be anyway ( ͡° ͜ʖ ͡°) TODO: handle error case
                };

                format!(
                    "\
                    @{}\n\
                    D=M\n\
                    {}",
                    segment_name, push_stack_assembly
                )
            }
            Segment::Constant => {
                format!(
                    "\
                @{}\n\
                D=A\n\
                {}",
                    self.offset, push_stack_assembly
                )
            }
            Segment::Temp => {
                // TODO: return error if outside of bounds
                let index = 5 + self.offset;
                format!(
                    "\
                    @{}\n\
                    D=M\n\
                    {}",
                    index, push_stack_assembly
                )
            }
            Segment::Static => {
                // TODO: ensure number of static variables is not greater than the reserved address space (16-255) can hold
                let symbol = format!("{}.{}", cpu_state.loop_label_prefix, self.offset);
                format!(
                    "\
                    @{}\n\
                    D=M\n\
                    {}",
                    symbol, push_stack_assembly
                )
            }
        }
    }
}
