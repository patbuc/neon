use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub mod chunk;
pub mod constants;
pub mod error_renderer;
pub mod errors;
pub mod method_registry;
pub(crate) mod opcodes;
pub mod stdlib;
pub mod string_interner;
pub mod string_similarity;
#[cfg(test)]
mod tests;

// Forward declare VirtualMachine for NativeFn signature
// We need the actual VM type here to avoid circular dependencies
// The VM is needed for string interning and other runtime operations
use crate::vm::VirtualMachine;
pub(crate) type NativeFn = fn(&mut VirtualMachine, &[Value]) -> Result<Value, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chunk {
    #[allow(dead_code)]
    pub name: String,
    pub constants: Constants,
    pub strings: Constants,
    pub instructions: Vec<u8>,
    pub source_locations: Vec<SourceLocation>,
    pub locals: Vec<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Local {
    pub name: String,
    pub depth: u32,
    pub is_mutable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constants {
    values: Vec<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SourceLocation {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Number(f64),
    #[serde(with = "serde_rc")]
    Object(Rc<Object>),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MapKey {
    #[serde(with = "serde_rc_str")]
    String(Rc<str>),
    Number(OrderedFloat<f64>),
    Boolean(bool),
}

pub type SetKey = MapKey;

impl Display for MapKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapKey::String(s) => write!(f, "{}", s),
            MapKey::Number(n) => write!(f, "{}", n),
            MapKey::Boolean(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Object {
    String(ObjString),
    #[serde(with = "serde_rc")]
    Function(Rc<ObjFunction>),
    #[serde(with = "serde_rc")]
    Struct(Rc<ObjStruct>),
    #[serde(with = "serde_rc_refcell")]
    Instance(Rc<RefCell<ObjInstance>>),
    #[serde(with = "serde_rc_refcell")]
    Array(Rc<RefCell<Vec<Value>>>),
    #[serde(with = "serde_rc_refcell")]
    Map(Rc<RefCell<HashMap<MapKey, Value>>>),
    #[serde(with = "serde_rc_refcell")]
    Set(Rc<RefCell<BTreeSet<SetKey>>>),
    #[serde(with = "serde_rc_str")]
    File(Rc<str>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjString {
    #[serde(with = "serde_rc_str")]
    pub value: Rc<str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjFunction {
    pub name: String,
    pub arity: u8,
    #[serde(with = "serde_rc")]
    pub chunk: Rc<Chunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjStruct {
    pub name: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjInstance {
    #[serde(with = "serde_rc")]
    pub r#struct: Rc<ObjStruct>,
    pub fields: HashMap<String, Value>,
}

impl Value {
    pub(crate) fn new_object(instance: ObjInstance) -> Value {
        Value::Object(Rc::new(Object::Instance(Rc::new(RefCell::new(instance)))))
    }

    pub(crate) fn new_struct(name: String, fields: Vec<String>) -> Self {
        Value::Object(Rc::new(Object::Struct(Rc::new(ObjStruct { name, fields }))))
    }

    pub(crate) fn new_function(name: String, arity: u8, chunk: Chunk) -> Self {
        Value::Object(Rc::new(Object::Function(Rc::new(ObjFunction {
            name,
            arity,
            chunk: Rc::new(chunk),
        }))))
    }

    pub(crate) fn new_array(elements: Vec<Value>) -> Self {
        Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(elements)))))
    }

    pub(crate) fn new_map(entries: HashMap<MapKey, Value>) -> Self {
        Value::Object(Rc::new(Object::Map(Rc::new(RefCell::new(entries)))))
    }

    pub(crate) fn new_set(elements: BTreeSet<SetKey>) -> Self {
        Value::Object(Rc::new(Object::Set(Rc::new(RefCell::new(elements)))))
    }

    pub(crate) fn new_file(path: String) -> Self {
        Value::Object(Rc::new(Object::File(Rc::from(path))))
    }
}

pub struct CallFrame {
    pub function: Rc<ObjFunction>,
    pub ip: usize,
    pub slot_start: isize, // Can be -1 for script frame
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::String(obj_string) => write!(f, "{}", obj_string.value),
            Object::Function(obj_function) => write!(f, "<fn {}>", obj_function.name),
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
                write!(f, "{{")?;
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
            Object::File(path) => write!(f, "<file: {}>", path),
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
        // Fast path: pointer equality check for interned strings
        if Rc::ptr_eq(&self.value, &other.value) {
            return true;
        }
        // Slow path: content comparison
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
        // We don't compare chunks as they're complex and functions with same name/arity are considered equal
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

// Serde helper modules for Rc and RefCell serialization
mod serde_rc {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::rc::Rc;

    pub fn serialize<S, T>(value: &Rc<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        (**value).serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Rc<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        T::deserialize(deserializer).map(Rc::new)
    }
}

mod serde_rc_str {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::rc::Rc;

    pub fn serialize<S>(value: &Rc<str>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.as_ref().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Rc<str>, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(|s| Rc::from(s.as_str()))
    }
}

mod serde_rc_refcell {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn serialize<S, T>(value: &Rc<RefCell<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        value.borrow().serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Rc<RefCell<T>>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        T::deserialize(deserializer).map(|t| Rc::new(RefCell::new(t)))
    }
}
