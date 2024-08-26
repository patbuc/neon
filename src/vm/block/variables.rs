use crate::vm::Variables;

impl Variables {
    pub fn new() -> Self {
        Variables {
            values: Vec::new(),
            variables: Vec::new(),
        }
    }

    pub fn write_value(&mut self, name: String) -> u32 {
        self.values.push(name);
        (self.values.len() - 1) as u32
    }

    pub fn write_variable(&mut self, name: String) -> u32 {
        self.variables.push(name);
        (self.variables.len() - 1) as u32
    }
}
