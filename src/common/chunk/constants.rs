use crate::common::Constants;
use crate::common::Value;

impl Default for Constants {
    fn default() -> Self {
        Self::new()
    }
}

impl Constants {
    pub fn new() -> Self {
        Constants { values: Vec::new() }
    }

    pub fn write_value(&mut self, value: Value) -> u32 {
        self.values.push(value);
        (self.values.len() - 1) as u32
    }

    pub fn read_value(&self, index: usize) -> Value {
        self.values[index].clone()
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
