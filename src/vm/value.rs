use crate::vm::{Value, ValueType, ValueUnion};

impl Value {
    pub fn from_number(value: f64) -> Self {
        Value {
            value_type: ValueType::Number,
            value: ValueUnion { number: value },
        }
    }

    pub fn from_bool(value: bool) -> Self {
        Value {
            value_type: ValueType::Bool,
            value: ValueUnion { boolean: value },
        }
    }

    pub fn from_string(value: *const String) -> Self {
        Value {
            value_type: ValueType::Bool,
            value: ValueUnion { string: value },
        }
    }

    pub fn nil() -> Value {
        Value {
            value_type: ValueType::Nil,
            value: ValueUnion { number: 0.0 },
        }
    }
}
