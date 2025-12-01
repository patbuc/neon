use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub(crate) mod bloq;
pub(crate) mod opcodes;
pub mod errors;
pub mod constants;
pub mod error_renderer;

// Forward declare VirtualMachine for NativeFn signature
// We can't import VirtualMachine directly as it would create a circular dependency
// The actual implementation will be in vm/mod.rs
pub type NativeFn = fn(&mut crate::vm::VirtualMachine, &[Value]) -> Result<Value, String>;

#[derive(Debug)]
pub(crate) struct Bloq {
    #[allow(dead_code)]
    name: String,
    constants: Constants,
    strings: Constants,
    instructions: Vec<u8>,
    source_locations: Vec<SourceLocation>,
    locals: Vec<Local>,
}

#[derive(Debug)]
pub(crate) struct Local {
    pub name: String,
    pub depth: u32,
    pub is_mutable: bool,
}

#[derive(Debug)]
struct Constants {
    values: Vec<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SourceLocation {
    pub offset: usize,
    pub line: u32,
    pub column: u32,
}

impl Display for SourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[repr(u8)]
pub enum BitsSize {
    Eight,
    Sixteen,
    ThirtyTwo,
}

impl BitsSize {
    pub fn as_bytes(&self) -> usize {
        match self {
            BitsSize::Eight => 1,
            BitsSize::Sixteen => 2,
            BitsSize::ThirtyTwo => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Value {
    Number(f64),
    Object(Rc<Object>),
    Boolean(bool),
    Nil,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Object {
    String(ObjString),
    Function(Rc<ObjFunction>),
    NativeFunction(ObjNativeFunction),
    Struct(Rc<ObjStruct>),
    Instance(Rc<RefCell<ObjInstance>>),
    Array(Rc<RefCell<Vec<Value>>>),
}

#[derive(Debug, Clone)]
pub(crate) struct ObjString {
    pub value: Rc<str>,
}

#[derive(Debug, Clone)]
pub(crate) struct ObjFunction {
    pub name: String,
    pub arity: u8,
    pub bloq: Rc<Bloq>,
}

#[derive(Clone)]
pub(crate) struct ObjNativeFunction {
    pub name: String,
    pub arity: u8,
    pub function: NativeFn,
}

impl std::fmt::Debug for ObjNativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjNativeFunction")
            .field("name", &self.name)
            .field("arity", &self.arity)
            .field("function", &"<native fn>")
            .finish()
    }
}

impl PartialEq for ObjNativeFunction {
    fn eq(&self, other: &Self) -> bool {
        // Native functions are equal if they have the same name and arity
        // We can't compare function pointers directly in a meaningful way
        self.name == other.name && self.arity == other.arity
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ObjStruct {
    pub name: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct ObjInstance {
    pub r#struct: Rc<ObjStruct>,
    pub fields: std::collections::HashMap<String, Value>,
}

impl Value {
    pub(crate) fn new_object(instance: ObjInstance) -> Value {
        Value::Object(Rc::new(Object::Instance(Rc::new(RefCell::new(instance)))))
    }

    pub(crate) fn new_struct(name: String, fields: Vec<String>) -> Self {
        Value::Object(Rc::new(Object::Struct(Rc::new(ObjStruct { name, fields }))))
    }

    pub(crate) fn new_function(name: String, arity: u8, bloq: Bloq) -> Self {
        Value::Object(Rc::new(Object::Function(Rc::new(ObjFunction {
            name,
            arity,
            bloq: Rc::new(bloq),
        }))))
    }

    pub(crate) fn new_native_function(name: String, arity: u8, function: NativeFn) -> Self {
        Value::Object(Rc::new(Object::NativeFunction(ObjNativeFunction {
            name,
            arity,
            function,
        })))
    }

    pub(crate) fn new_array(elements: Vec<Value>) -> Self {
        Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(elements)))))
    }
}

pub(crate) struct CallFrame {
    pub function: Rc<ObjFunction>,
    pub ip: usize,
    pub slot_start: isize, // Can be -1 for script frame
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::String(obj_string) => write!(f, "{}", obj_string.value),
            Object::Function(obj_function) => write!(f, "<fn {}>", obj_function.name),
            Object::NativeFunction(native_fn) => write!(f, "<native fn {}>", native_fn.name),
            Object::Struct(obj_struct) => write!(f, "<struct {}>", obj_struct.name),
            Object::Instance(obj_instance) => {
                let instance = obj_instance;
                write!(f, "<{} instance>", instance.borrow().r#struct.name)
            }
            Object::Array(array) => {
                let elements = array.borrow();
                write!(f, "[")?;
                for (i, value) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl PartialEq<Rc<str>> for ObjString {
    fn eq(&self, other: &Rc<str>) -> bool {
        self.value == *other
    }
}

impl PartialEq for ObjString {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<&ObjString> for &str {
    fn eq(&self, other: &&ObjString) -> bool {
        *self == other.value.as_ref()
    }
}

impl PartialEq for ObjFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == other.arity
        // We don't compare bloqs as they're complex and functions with same name/arity are considered equal
    }
}

impl PartialEq for ObjStruct {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.fields == other.fields
    }
}

impl PartialEq for ObjInstance {
    fn eq(&self, other: &Self) -> bool {
        // Instances are equal if they point to the same struct and have same field values
        self.r#struct.name == other.r#struct.name && self.fields == other.fields
    }
}

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

#[cfg(test)]
mod test_array {
    use super::*;

    #[test]
    fn test_array_creation() {
        let arr = Value::new_array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);

        // Test that the array was created
        match arr {
            Value::Object(obj) => {
                match obj.as_ref() {
                    Object::Array(_) => {
                        // Success - array was created
                    },
                    _ => panic!("Expected Array object"),
                }
            },
            _ => panic!("Expected Object value"),
        }
    }

    #[test]
    fn test_array_display() {
        let arr = Value::new_array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);

        let display = format!("{}", arr);
        assert_eq!(display, "[1, 2, 3]");
    }

    #[test]
    fn test_empty_array_display() {
        let arr = Value::new_array(vec![]);
        let display = format!("{}", arr);
        assert_eq!(display, "[]");
    }

    #[test]
    fn test_array_equality() {
        let arr1 = Value::new_array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
        ]);

        let arr2 = Value::new_array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
        ]);

        let arr3 = Value::new_array(vec![
            Value::Number(1.0),
            Value::Number(3.0),
        ]);

        // These should be equal
        assert_eq!(arr1, arr2);

        // These should not be equal
        assert_ne!(arr1, arr3);
    }

    #[test]
    fn test_mixed_type_array() {
        let arr = Value::new_array(vec![
            Value::Number(42.0),
            Value::Boolean(true),
            Value::Nil,
        ]);

        let display = format!("{}", arr);
        assert_eq!(display, "[42, true, nil]");
    }
}
