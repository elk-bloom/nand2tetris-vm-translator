use super::cpu_state::CPUState;

pub trait ToAssembly {
    fn to_assembly(&self, cpu_state: &mut CPUState) -> String;
}
