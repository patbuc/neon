use crate::common::{Value, Object};
use crate::vm::VirtualMachine;

/// Native implementation of Array.size()
/// Returns the number of elements in the array
pub fn native_array_size(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("array.size() requires an array receiver".to_string());
    }

    match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(array) => {
                let elements = array.borrow();
                Ok(Value::Number(elements.len() as f64))
            }
            _ => Err("size() can only be called on arrays".to_string()),
        },
        _ => Err("size() can only be called on arrays".to_string()),
    }
}

/// Native implementation of Array.contains(element)
/// Returns true if the array contains the specified element
pub fn native_array_contains(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "contains() expects 1 argument (element), got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => arr,
            _ => return Err("contains() can only be called on arrays".to_string()),
        },
        _ => return Err("contains() can only be called on arrays".to_string()),
    };

    // Get the element to search for
    let element = &args[1];

    // Check if the array contains the element
    let elements = array.borrow();
    let contains = elements.iter().any(|e| e == element);

    Ok(Value::Boolean(contains))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Object;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_array_size_empty() {
        let mut vm = VirtualMachine::new();
        let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![])))));
        let result = native_array_size(&mut vm, &[array]).unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_array_size_with_elements() {
        let mut vm = VirtualMachine::new();
        let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ])))));
        let result = native_array_size(&mut vm, &[array]).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn test_array_size_no_args() {
        let mut vm = VirtualMachine::new();
        let result = native_array_size(&mut vm, &[]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "array.size() requires an array receiver");
    }

    #[test]
    fn test_array_size_wrong_type() {
        let mut vm = VirtualMachine::new();
        let result = native_array_size(&mut vm, &[Value::Number(42.0)]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "size() can only be called on arrays");
    }

    #[test]
    fn test_array_contains_found() {
        let mut vm = VirtualMachine::new();
        let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ])))));
        let result = native_array_contains(&mut vm, &[array, Value::Number(2.0)]).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_array_contains_not_found() {
        let mut vm = VirtualMachine::new();
        let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ])))));
        let result = native_array_contains(&mut vm, &[array, Value::Number(5.0)]).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_array_contains_empty_array() {
        let mut vm = VirtualMachine::new();
        let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![])))));
        let result = native_array_contains(&mut vm, &[array, Value::Number(1.0)]).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_array_contains_string() {
        use crate::common::ObjString;
        let mut vm = VirtualMachine::new();
        let string_val = Value::Object(Rc::new(Object::String(ObjString {
            value: "hello".into(),
        })));
        let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![
            string_val.clone(),
            Value::Number(2.0),
        ])))));
        let result = native_array_contains(&mut vm, &[array, string_val]).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_array_contains_wrong_arg_count() {
        let mut vm = VirtualMachine::new();
        let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![])))));
        let result = native_array_contains(&mut vm, &[array]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 1 argument"));
    }

    #[test]
    fn test_array_contains_wrong_type() {
        let mut vm = VirtualMachine::new();
        let result = native_array_contains(&mut vm, &[Value::Number(42.0), Value::Number(1.0)]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "contains() can only be called on arrays");
    }
}
