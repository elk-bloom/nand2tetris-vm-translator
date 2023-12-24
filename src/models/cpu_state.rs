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
                    Some(format!("@{}\n", { necessary_value }))
                } else {
                    None
                }
            }
            RegisterType::D => {
                if self.const_or_predefined_d_register.is_empty() {
                    Some(format!("@{}\n", { necessary_value }))
                } else {
                    None
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.const_or_predefined_a_register.clear();
        self.const_or_predefined_d_register.clear();
    }
}

pub struct CPUStateBuilder {
    a_register: String,
    d_register: String,
    loop_label_name: String,
    loop_label_count: u32,
    static_variable_prefix: String,
}

impl CPUStateBuilder {
    pub fn new() -> Self {
        CPUStateBuilder {
            a_register: String::new(),
            d_register: String::new(),
            loop_label_name: String::new(),
            loop_label_count: 0,
            static_variable_prefix: String::new(),
        }
    }

    pub fn a_register(mut self, val: String) -> Self {
        self.a_register = val;
        self
    }

    pub fn d_register(mut self, val: String) -> Self {
        self.d_register = val;
        self
    }

    pub fn loop_label_name(mut self, val: String) -> Self {
        self.loop_label_name = val;
        self
    }

    pub fn loop_label_count(mut self, val: u32) -> Self {
        self.loop_label_count = val;
        self
    }

    pub fn static_variable_prefix(mut self, val: String) -> Self {
        self.static_variable_prefix = val;
        self
    }

    pub fn build(self) -> CPUState {
        CPUState {
            const_or_predefined_a_register: self.a_register,
            const_or_predefined_d_register: self.d_register,
            loop_label_name: self.loop_label_name,
            loop_label_count: self.loop_label_count,
            static_variable_prefix: self.static_variable_prefix,
        }
    }
}
