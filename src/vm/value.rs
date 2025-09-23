use crate::vm::Value;
use std::fmt::{Display, Formatter};

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Number(val) => val.to_string(),
                Value::Boolean(val) => val.to_string(),
                Value::Nil => "nil".to_string(),
                Value::Object(val) => format!("{}", val),
            }
        )
    }
}

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
        Value::Object(Rc::from(Object::String(ObjString {
            value: Rc::from($value),
        })))
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
macro_rules! as_object {
    ($value: expr) => {
        if let Value::Object(ref value) = $value {
            value
        } else {
            panic!("Expected object, got {:?}", $value);
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
macro_rules! as_string {
    ($value: expr) => {
        if let Value::Object(ref obj) = $value {
            if let Object::String(ref obj_string) = **obj {
                obj_string
            } else {
                panic!("Expected Object::String, got {:?}", $value);
            }
        } else {
            panic!("Expected Value::Object, got {:?}", $value);
        }
    };
}
#[macro_export]
macro_rules! is_false_like {
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
    use crate::vm::{ObjString, Object, Rc};

    #[test]
    fn value_can_be_created_from_number() {
        let value = number!(123.45);
        assert_eq!(123.45, as_number!(value));
    }

    #[test]
    fn value_can_be_created_from_bool() {
        let value = boolean!(true);
        assert!(as_bool!(value))
    }

    #[test]
    fn value_can_be_created_from_string() {
        let value = string!("Hello, World!");
        assert_eq!("Hello, World!", as_string!(value));
    }

    #[test]
    fn value_can_be_created_from_nil() {
        let value = nil!();
        assert_eq!(Value::Nil, as_nil!(value));
    }
}
