use super::cpu_state::RegisterType;

pub struct PushStack {}

impl PushStack {
    fn to_assembly(cpu_state: &mut super::cpu_state::CPUState) -> String {
        let prefix: Option<String> = cpu_state.get_prefix("SP", &RegisterType::A);
        let main_assembly = "\
        A=M
        M=D
        @SP
        M=M+1";

        cpu_state.const_or_predefined_a_register.clear();
        cpu_state.const_or_predefined_a_register.push_str("SP");
        cpu_state.const_or_predefined_d_register.clear();

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
}
