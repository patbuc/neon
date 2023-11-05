type Value = f64;

pub struct Constants {
    pub values: Vec<Value>,
}

impl Constants {
    pub fn new() -> Self {
        Constants { values: Vec::new() }
    }

    pub fn push_value(&mut self, value: Value) -> u32 {
        self.values.push(value);
        (self.values.len() - 1) as u32
    }

    pub fn get_value(&self, index: u32) -> Value {
        self.values[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_can_be_pushed_to_constants() {
        let mut constants = Constants::new();
        constants.push_value(123.45);

        assert_eq!(1, constants.values.len());
        assert_eq!(123.45, constants.values[0]);
    }
}
