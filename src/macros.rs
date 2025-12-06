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
macro_rules! is_false_like {
    ($value: expr) => {
        match $value {
            $crate::common::Value::Nil => true,
            $crate::common::Value::Boolean(value) => !value,
            _ => false,
        }
    };
}
