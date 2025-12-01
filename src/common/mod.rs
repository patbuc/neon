use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use ordered_float::OrderedFloat;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum MapKey {
    String(Rc<str>),
    Number(OrderedFloat<f64>),
    Boolean(bool),
}

pub(crate) type SetKey = MapKey;

impl Display for MapKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapKey::String(s) => write!(f, "{}", s),
            MapKey::Number(n) => write!(f, "{}", n),
            MapKey::Boolean(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Object {
    String(ObjString),
    Function(Rc<ObjFunction>),
    NativeFunction(ObjNativeFunction),
    Struct(Rc<ObjStruct>),
    Instance(Rc<RefCell<ObjInstance>>),
    Array(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<HashMap<MapKey, Value>>>),
    Set(Rc<RefCell<HashSet<SetKey>>>),
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
        // Native functions are equal if they have the same name, arity, and function pointer
        self.name == other.name
            && self.arity == other.arity
            && (self.function as usize) == (other.function as usize)
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

    pub(crate) fn new_map(entries: HashMap<MapKey, Value>) -> Self {
        Value::Object(Rc::new(Object::Map(Rc::new(RefCell::new(entries)))))
    }

    pub(crate) fn new_set(elements: HashSet<SetKey>) -> Self {
        Value::Object(Rc::new(Object::Set(Rc::new(RefCell::new(elements)))))
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
            Object::Map(map) => {
                let entries = map.borrow();
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in entries.iter() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    first = false;
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "}}")
            }
            Object::Set(set) => {
                let elements = set.borrow();
                write!(f, "#{{")?;
                let mut first = true;
                for element in elements.iter() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    first = false;
                    write!(f, "{}", element)?;
                }
                write!(f, "}}")
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

#[cfg(test)]
mod test_map {
    use super::*;

    #[test]
    fn test_map_creation() {
        let mut entries = HashMap::new();
        entries.insert(MapKey::String(Rc::from("name")), Value::Object(Rc::new(Object::String(ObjString { value: Rc::from("Alice") }))));
        entries.insert(MapKey::Number(OrderedFloat(42.0)), Value::Number(100.0));

        let map = Value::new_map(entries);

        // Test that the map was created
        match map {
            Value::Object(obj) => {
                match obj.as_ref() {
                    Object::Map(_) => {
                        // Success - map was created
                    },
                    _ => panic!("Expected Map object"),
                }
            },
            _ => panic!("Expected Object value"),
        }
    }

    #[test]
    fn test_map_display() {
        let mut entries = HashMap::new();
        entries.insert(MapKey::String(Rc::from("a")), Value::Number(1.0));
        entries.insert(MapKey::String(Rc::from("b")), Value::Number(2.0));

        let map = Value::new_map(entries);
        let display = format!("{}", map);

        // HashMap order is not guaranteed, so we test both possible orders
        assert!(display == "{a: 1, b: 2}" || display == "{b: 2, a: 1}");
    }

    #[test]
    fn test_empty_map_display() {
        let map = Value::new_map(HashMap::new());
        let display = format!("{}", map);
        assert_eq!(display, "{}");
    }

    #[test]
    fn test_map_equality() {
        let mut entries1 = HashMap::new();
        entries1.insert(MapKey::String(Rc::from("x")), Value::Number(1.0));
        entries1.insert(MapKey::String(Rc::from("y")), Value::Number(2.0));
        let map1 = Value::new_map(entries1);

        let mut entries2 = HashMap::new();
        entries2.insert(MapKey::String(Rc::from("x")), Value::Number(1.0));
        entries2.insert(MapKey::String(Rc::from("y")), Value::Number(2.0));
        let map2 = Value::new_map(entries2);

        let mut entries3 = HashMap::new();
        entries3.insert(MapKey::String(Rc::from("x")), Value::Number(1.0));
        entries3.insert(MapKey::String(Rc::from("y")), Value::Number(3.0));
        let map3 = Value::new_map(entries3);

        // These should be equal
        assert_eq!(map1, map2);

        // These should not be equal
        assert_ne!(map1, map3);
    }

    #[test]
    fn test_map_with_different_key_types() {
        let mut entries = HashMap::new();
        entries.insert(MapKey::String(Rc::from("name")), Value::Object(Rc::new(Object::String(ObjString { value: Rc::from("Alice") }))));
        entries.insert(MapKey::Number(OrderedFloat(42.0)), Value::Number(100.0));
        entries.insert(MapKey::Boolean(true), Value::Boolean(false));

        let map = Value::new_map(entries);
        let display = format!("{}", map);

        // Check that all key types are represented
        assert!(display.contains("name:"));
        assert!(display.contains("42:"));
        assert!(display.contains("true:"));
    }

    #[test]
    fn test_map_with_mixed_value_types() {
        let mut entries = HashMap::new();
        entries.insert(MapKey::String(Rc::from("num")), Value::Number(42.0));
        entries.insert(MapKey::String(Rc::from("bool")), Value::Boolean(true));
        entries.insert(MapKey::String(Rc::from("nil")), Value::Nil);

        let map = Value::new_map(entries);
        let display = format!("{}", map);

        // Check that all value types are represented
        assert!(display.contains("num:"));
        assert!(display.contains("bool:"));
        assert!(display.contains("nil:"));
    }
}

#[cfg(test)]
mod test_set {
    use super::*;

    #[test]
    fn test_set_creation() {
        let mut elements = HashSet::new();
        elements.insert(SetKey::Number(OrderedFloat(1.0)));
        elements.insert(SetKey::Number(OrderedFloat(2.0)));
        elements.insert(SetKey::Number(OrderedFloat(3.0)));

        let set = Value::new_set(elements);

        // Test that the set was created
        match set {
            Value::Object(obj) => {
                match obj.as_ref() {
                    Object::Set(_) => {
                        // Success - set was created
                    },
                    _ => panic!("Expected Set object"),
                }
            },
            _ => panic!("Expected Object value"),
        }
    }

    #[test]
    fn test_set_display() {
        let mut elements = HashSet::new();
        elements.insert(SetKey::Number(OrderedFloat(1.0)));
        elements.insert(SetKey::Number(OrderedFloat(2.0)));
        elements.insert(SetKey::Number(OrderedFloat(3.0)));

        let set = Value::new_set(elements);
        let display = format!("{}", set);

        // HashSet order is not guaranteed, so we check for the format and presence of elements
        assert!(display.starts_with("#{"));
        assert!(display.ends_with("}"));
        assert!(display.contains("1"));
        assert!(display.contains("2"));
        assert!(display.contains("3"));
    }

    #[test]
    fn test_empty_set_display() {
        let set = Value::new_set(HashSet::new());
        let display = format!("{}", set);
        assert_eq!(display, "#{}");
    }

    #[test]
    fn test_set_equality() {
        let mut elements1 = HashSet::new();
        elements1.insert(SetKey::String(Rc::from("a")));
        elements1.insert(SetKey::String(Rc::from("b")));
        let set1 = Value::new_set(elements1);

        let mut elements2 = HashSet::new();
        elements2.insert(SetKey::String(Rc::from("a")));
        elements2.insert(SetKey::String(Rc::from("b")));
        let set2 = Value::new_set(elements2);

        let mut elements3 = HashSet::new();
        elements3.insert(SetKey::String(Rc::from("a")));
        elements3.insert(SetKey::String(Rc::from("c")));
        let set3 = Value::new_set(elements3);

        // These should be equal
        assert_eq!(set1, set2);

        // These should not be equal
        assert_ne!(set1, set3);
    }

    #[test]
    fn test_set_with_different_key_types() {
        let mut elements = HashSet::new();
        elements.insert(SetKey::String(Rc::from("hello")));
        elements.insert(SetKey::Number(OrderedFloat(42.0)));
        elements.insert(SetKey::Boolean(true));

        let set = Value::new_set(elements);
        let display = format!("{}", set);

        // Check that all key types are represented
        assert!(display.contains("hello"));
        assert!(display.contains("42"));
        assert!(display.contains("true"));
    }

    #[test]
    fn test_set_uniqueness() {
        let mut elements = HashSet::new();
        elements.insert(SetKey::Number(OrderedFloat(1.0)));
        elements.insert(SetKey::Number(OrderedFloat(1.0))); // Duplicate
        elements.insert(SetKey::Number(OrderedFloat(2.0)));

        let set = Value::new_set(elements);

        // Verify that the set contains only unique elements
        if let Value::Object(obj) = &set {
            if let Object::Set(set_ref) = obj.as_ref() {
                assert_eq!(set_ref.borrow().len(), 2); // Should only contain 2 unique elements
            } else {
                panic!("Expected Set object");
            }
        } else {
            panic!("Expected Object value");
        }
    }
}
