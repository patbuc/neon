type Value = f64;

pub struct Constants {
    pub values: Vec<Value>,
}

impl Constants {
    pub fn new() -> Self {
        Constants { values: Vec::new() }
    }

    pub fn push_value(&mut self, value: Value) -> i8 {
        self.values.push(value);
        self.values.len() as i8 - 1
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
