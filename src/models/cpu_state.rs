pub struct CPUState {
    /// empty String has semantic meaning, treated like a None. We avoid Option to avoid frequent String allocation by functions referencing this field
    pub const_or_predefined_a_register: String,
    /// empty String has semantic meaning, treated like a None. We avoid Option to avoid frequent String allocation by functions referencing this field 
    pub const_or_predefined_d_register: String,
    // below don't really indicate CPU State but they're quite convenient to have here, might think about renaming this struct
    pub loop_label_name: String,
    pub loop_label_count: u32,
    pub static_variable_prefix: String,
}

pub enum RegisterType {
    A,
    D,
}

impl CPUState {
    pub fn get_prefix(
        &self,
        necessary_value: &str,
        register_type: &RegisterType,
    ) -> Option<String> {
        match register_type {
            RegisterType::A => {
                if self.const_or_predefined_a_register.is_empty() {
                    None
                } else {
                    Some(format!("@{}\n", { necessary_value }))
                }
            }
            RegisterType::D => {
                if self.const_or_predefined_d_register.is_empty() {
                    None
                } else {
                    Some(format!("@{}\n", { necessary_value }))
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.const_or_predefined_a_register.clear();
        self.const_or_predefined_d_register.clear();
    }
}
