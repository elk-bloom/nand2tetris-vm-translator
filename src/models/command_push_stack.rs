use super::cpu_state::RegisterType;

pub struct PushStack {}

impl PushStack {
    pub fn to_assembly(cpu_state: &mut super::cpu_state::CPUState) -> String {
        let prefix: Option<String> = cpu_state.get_prefix("SP", &RegisterType::A);
        let main_assembly = "\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n";

        cpu_state.clear();
        cpu_state.const_or_predefined_a_register.push_str("SP");

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
}
