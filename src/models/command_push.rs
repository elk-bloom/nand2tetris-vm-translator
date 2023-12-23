use super::{to_assembly::ToAssembly, command_push_stack::PushStack, segment::Segment};


pub struct CommandPush {
    pub segment: Segment,
    pub offset: u32,
}

impl ToAssembly for CommandPush {
    fn to_assembly(&self, cpu_state: &mut super::cpu_state::CPUState) -> String {
        // the PushStack::to_assembly handles the update of cpu_state since it 
        // appears last in all of our assembly for CommandPush
        let push_stack_assembly = PushStack::to_assembly(cpu_state);
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
                        _ => unreachable!() // or it should be anyway :lenny_face:
                    },
                    _ => unreachable!(),
                };
                
                if self.offset == 0 {
                    format!(
                        "\
                        @{}
                        A=M
                        D=M
                        {}", segment_name, push_stack_assembly
                    )                    
                } else {
                   format!(
                    "\
                    @{}
                    D=A
                    @{}
                    A=D+M
                    D=M
                    {}", self.offset, segment_name, push_stack_assembly
                   ) 
                }
            }
            Segment::Constant => {
                format!(
                "\
                @{}
                D=M
                {}", self.offset, push_stack_assembly 
            )
            }
            Segment::Temp => {
                // TODO: return error if outside of bounds
                let index = 5 + self.offset;
                format!(
                    "\
                    @{}
                    D=M
                    {}", index, push_stack_assembly
                )
            }
            Segment::Static => {
                // TODO: ensure number of static variables is not greater than the reserved address space (16-255) can hold
                let symbol = format!("{}.{}", cpu_state.loop_label_name, self.offset);
                format!(
                    "\
                    @{}
                    D=M
                    {}", symbol, push_stack_assembly
                )
            }
        }
    }
}