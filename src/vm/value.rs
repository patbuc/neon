use crate::vm::Value;

#[macro_export]
macro_rules! number {
    ($value: expr) => {
        Value::Number($value)
    };
}

#[macro_export]
macro_rules! boolean {
    ($value: expr) => {
        Value::Boolean($value)
    };
}

#[macro_export]
macro_rules! string {
    ($value: expr) => {
        Value::String($value)
    };
}

#[macro_export]
macro_rules! nil {
    () => {
        Value::Nil
    };
}

#[macro_export]
macro_rules! as_number {
    ($value: expr) => {
        if let Value::Number(value) = $value {
            value
        } else {
            panic!("Expected number, got {:?}", $value);
        }
    };
}

#[macro_export]
macro_rules! as_bool {
    ($value: expr) => {
        if let Value::Boolean(value) = $value {
            value
        } else {
            panic!("Expected boolean, got {:?}", $value);
        }
    };
}

#[macro_export]
macro_rules! as_string {
    ($value: expr) => {
        if let Value::String(value) = $value {
            value
        } else {
            panic!("Expected string, got {:?}", $value);
        }
    };
}

#[macro_export]
macro_rules! as_nil {
    ($value: expr) => {
        if let Value::Nil = $value {
            Value::Nil
        } else {
            panic!("Expected nil, got {:?}", $value);
        }
    };
}

#[macro_export]
macro_rules! is_falsey {
    ($value: expr) => {
        match $value {
            Value::Nil => true,
            Value::Boolean(value) => !value,
            _ => false,
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_can_be_created_from_number() {
        let value = number!(123.45);
        assert_eq!(123.45, as_number!(value));
    }

    #[test]
    fn value_can_be_created_from_bool() {
        let value = boolean!(true);
        assert_eq!(true, as_bool!(value))
    }

    #[test]
    fn value_can_be_created_from_string() {
        let value = string!("Hello, World!".to_string());
        assert_eq!("Hello, World!", as_string!(value));
    }

    #[test]
    fn value_can_be_created_from_nil() {
        let value = nil!();
        assert_eq!(Value::Nil, as_nil!(value));
    }
}
