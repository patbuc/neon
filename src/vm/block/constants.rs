use crate::vm::{Constants, Value};

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_number, number};

    #[test]
    fn value_can_be_written_to_constants() {
        let mut constants = Constants::new();
        constants.write_value(number!(123.45));

        assert_eq!(1, constants.len());
        assert_eq!(123.45, as_number!(constants.read_value(0)));
    }

    #[test]
    fn value_can_be_read_to_constants() {
        let mut constants = Constants::new();
        constants.write_value(number!(123.45));

        assert_eq!(1, constants.len());
        assert_eq!(123.45, as_number!(constants.read_value(0)));
    }
}
