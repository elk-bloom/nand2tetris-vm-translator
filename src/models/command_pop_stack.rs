use super::cpu_state::RegisterType;

pub struct PopStack {}

impl PopStack {
    pub fn to_assembly(cpu_state: &mut super::cpu_state::CPUState) -> String {
        let prefix: Option<String> = cpu_state.get_prefix("SP", &RegisterType::A);
        let main_assembly = "\
        A=M
        M=D
        @SP
        M=M+1";

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
}
