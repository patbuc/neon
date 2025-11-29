#[macro_export]
macro_rules! number {
    ($value: expr) => {
        $crate::common::Value::Number($value)
    };
}

#[macro_export]
macro_rules! boolean {
    ($value: expr) => {
        $crate::common::Value::Boolean($value)
    };
}

#[macro_export]
macro_rules! string {
    ($value: expr) => {
        $crate::common::Value::Object(std::rc::Rc::from($crate::common::Object::String(
            $crate::common::ObjString {
                value: std::rc::Rc::from($value),
            },
        )))
    };
}

#[macro_export]
macro_rules! nil {
    () => {
        $crate::common::Value::Nil
    };
}

#[macro_export]
macro_rules! as_number {
    ($value: expr) => {
        if let $crate::common::Value::Number(value) = $value {
            value
        } else {
            panic!("Expected number, got {:?}", $value);
        }
    };
}

#[macro_export]
macro_rules! as_bool {
    ($value: expr) => {
        if let $crate::common::Value::Boolean(value) = $value {
            value
        } else {
            panic!("Expected boolean, got {:?}", $value);
        }
    };
}

#[macro_export]
macro_rules! as_object {
    ($value: expr) => {
        if let $crate::common::Value::Object(ref value) = $value {
            value
        } else {
            panic!("Expected object, got {:?}", $value);
        }
    };
}

#[macro_export]
macro_rules! as_nil {
    ($value: expr) => {
        if let $crate::common::Value::Nil = $value {
            $crate::common::Value::Nil
        } else {
            panic!("Expected nil, got {:?}", $value);
        }
    };
}

#[macro_export]
macro_rules! as_string {
    ($value: expr) => {
        if let $crate::common::Value::Object(ref obj) = $value {
            if let $crate::common::Object::String(ref obj_string) = **obj {
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
macro_rules! array {
    ($value: expr) => {
        $crate::common::Value::new_array($value)
    };
}

#[macro_export]
macro_rules! as_array {
    ($value: expr) => {
        if let $crate::common::Value::Object(ref obj) = $value {
            if let $crate::common::Object::Array(ref obj_array) = **obj {
                obj_array
            } else {
                panic!("Expected Object::Array, got {:?}", $value);
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
            $crate::common::Value::Nil => true,
            $crate::common::Value::Boolean(value) => !value,
            _ => false,
        }
    };
}

#[cfg(test)]
mod tests {
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
        assert_eq!(crate::common::Value::Nil, as_nil!(value));
    }

    #[test]
    fn value_can_be_created_from_array() {
        let value = array!(vec![number!(1.0), number!(2.0), number!(3.0)]);
        let arr = as_array!(value);
        let arr_ref = arr.borrow();
        let elements = arr_ref.elements.borrow();
        assert_eq!(3, elements.len());
        assert_eq!(number!(1.0), elements[0]);
        assert_eq!(number!(2.0), elements[1]);
        assert_eq!(number!(3.0), elements[2]);
    }

    #[test]
    fn arrays_with_same_elements_are_equal() {
        let arr1 = array!(vec![number!(1.0), string!("hello"), boolean!(true)]);
        let arr2 = array!(vec![number!(1.0), string!("hello"), boolean!(true)]);
        assert_eq!(arr1, arr2);
    }

    #[test]
    fn arrays_with_different_elements_are_not_equal() {
        let arr1 = array!(vec![number!(1.0), number!(2.0)]);
        let arr2 = array!(vec![number!(1.0), number!(3.0)]);
        assert_ne!(arr1, arr2);
    }

    #[test]
    fn empty_arrays_are_equal() {
        let arr1 = array!(vec![]);
        let arr2 = array!(vec![]);
        assert_eq!(arr1, arr2);
    }

    #[test]
    fn array_display_shows_correct_format() {
        let arr = array!(vec![number!(1.0), string!("test"), boolean!(true), nil!()]);
        let display_string = format!("{}", arr);
        assert_eq!("[1, test, true, nil]", display_string);
    }
}
