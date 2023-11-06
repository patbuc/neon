type Value = f64;

pub(super) struct Constants {
    values: Vec<Value>,
}

impl Constants {
    pub fn new() -> Self {
        Constants { values: Vec::new() }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn write_value(&mut self, value: Value) -> u32 {
        self.values.push(value);
        (self.values.len() - 1) as u32
    }

    pub fn read_value(&self, index: u32) -> Value {
        self.values[index as usize]
    }
}

// pub fn print_value(value: Value) {
//     print!("{}", value);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_can_be_written_to_constants() {
        let mut constants = Constants::new();
        constants.write_value(123.45);

        assert_eq!(1, constants.len());
        assert_eq!(123.45, constants.read_value(0));
    }

    #[test]
    fn value_can_be_read_to_constants() {
        let mut constants = Constants::new();
        constants.write_value(123.45);

        assert_eq!(1, constants.len());
        assert_eq!(123.45, constants.read_value(0));
    }
}
