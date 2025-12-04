use crate::common::{Object, Value};
use crate::vm::VirtualMachine;

/// Native implementation of File(path) constructor
/// Creates a new File object with the given path
pub fn native_file_constructor(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("File() expects 1 argument, got {}", args.len()));
    }

    match &args[0] {
        Value::Object(obj) => {
            if let Object::String(s) = obj.as_ref() {
                Ok(Value::new_file(s.value.to_string()))
            } else {
                Err("File() requires a string argument".to_string())
            }
        }
        _ => Err("File() requires a string argument".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Object, ObjString};
    use std::rc::Rc;

    #[test]
    fn test_file_constructor_valid_path() {
        let mut vm = VirtualMachine::new();
        let path = "test.txt";
        let args = vec![Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from(path),
        })))];

        let result = native_file_constructor(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::File(file_path) => {
                    assert_eq!(file_path.as_ref(), path);
                }
                _ => panic!("Expected File object"),
            },
            _ => panic!("Expected Object value"),
        }
    }

    #[test]
    fn test_file_constructor_wrong_arg_count_zero() {
        let mut vm = VirtualMachine::new();
        let args = vec![];
        let result = native_file_constructor(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "File() expects 1 argument, got 0");
    }

    #[test]
    fn test_file_constructor_wrong_arg_count_two() {
        let mut vm = VirtualMachine::new();
        let args = vec![
            Value::Object(Rc::new(Object::String(ObjString {
                value: Rc::from("test.txt"),
            }))),
            Value::Object(Rc::new(Object::String(ObjString {
                value: Rc::from("extra.txt"),
            }))),
        ];
        let result = native_file_constructor(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "File() expects 1 argument, got 2");
    }

    #[test]
    fn test_file_constructor_invalid_type_number() {
        let mut vm = VirtualMachine::new();
        let args = vec![Value::Number(42.0)];
        let result = native_file_constructor(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "File() requires a string argument"
        );
    }

    #[test]
    fn test_file_constructor_invalid_type_boolean() {
        let mut vm = VirtualMachine::new();
        let args = vec![Value::Boolean(true)];
        let result = native_file_constructor(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "File() requires a string argument"
        );
    }

    #[test]
    fn test_file_constructor_invalid_type_nil() {
        let mut vm = VirtualMachine::new();
        let args = vec![Value::Nil];
        let result = native_file_constructor(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "File() requires a string argument"
        );
    }

    #[test]
    fn test_file_constructor_with_relative_path() {
        let mut vm = VirtualMachine::new();
        let path = "../data/input.txt";
        let args = vec![Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from(path),
        })))];

        let result = native_file_constructor(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::File(file_path) => {
                    assert_eq!(file_path.as_ref(), path);
                }
                _ => panic!("Expected File object"),
            },
            _ => panic!("Expected Object value"),
        }
    }

    #[test]
    fn test_file_constructor_with_absolute_path() {
        let mut vm = VirtualMachine::new();
        let path = "/home/user/data/input.txt";
        let args = vec![Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from(path),
        })))];

        let result = native_file_constructor(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::File(file_path) => {
                    assert_eq!(file_path.as_ref(), path);
                }
                _ => panic!("Expected File object"),
            },
            _ => panic!("Expected Object value"),
        }
    }
}
