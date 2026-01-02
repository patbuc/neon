use ordered_float::OrderedFloat;
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
pub mod string_similarity;
#[cfg(test)]
mod tests;

// Forward declare VirtualMachine for NativeFn signature
// We can't import VirtualMachine directly as it would create a circular dependency
// The actual implementation will be in vm/mod.rs
pub(crate) type NativeFn = fn(&[Value]) -> Result<Value, String>;

#[derive(Debug, PartialEq)]
pub struct Chunk {
    #[allow(dead_code)]
    pub name: String,
    pub constants: Constants,
    pub strings: Constants,
    pub instructions: Vec<u8>,
    pub source_locations: Vec<SourceLocation>,
    pub locals: Vec<Local>,
}

#[derive(Debug, PartialEq)]
pub struct Local {
    pub name: String,
    pub depth: i32,
    pub is_captured: bool,
}

impl Local {
    pub(crate) fn new(name: String, depth: u32, readonly: bool) -> Self {
        Local {
            name,
            depth: depth as i32,
            is_captured: readonly,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Constants {
    pub values: Vec<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(ObjString),
    Function(Rc<ObjFunction>),
    Struct(Rc<ObjStruct>),
    Instance(Rc<RefCell<ObjInstance>>),
    Array(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<HashMap<MapKey, Value>>>),
    Set(Rc<RefCell<BTreeSet<SetKey>>>),
    File(Rc<str>),
    Callable(Rc<ObjCallable>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MapKey {
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

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Object(Rc<Object>),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub struct ObjString {
    pub value: Rc<str>,
}

#[derive(Debug, Clone)]
pub struct ObjFunction {
    pub name: String,
    pub arity: u8,
    pub chunk: Rc<Chunk>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjCallable {
    pub kind: CallableKind,
    pub name: String,
    pub arity: usize, // Number of arguments the callable expects
}

#[derive(Debug, Clone)]
pub struct ObjInstance {
    pub r#struct: Rc<ObjStruct>,
    pub fields: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ObjStruct {
    pub name: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CallableKind {
    /// User-defined Neon function
    NeonFunction { chunk: Rc<Chunk> },
    /// Native method with known index at compile time
    /// Used for: static methods, constructors, global functions
    /// Examples: Math.abs(), print(), File.new()
    NativeByIndex { index: u16 },
    /// Native method resolved at runtime based on receiver type
    /// Used for: instance methods where type is unknown at compile time
    /// Examples: arr.push(), str.len(), map.get()
    NativeByName { method_name: String },
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

    pub(crate) fn new_callable(name: String, arity: u8, kind: CallableKind) -> Self {
        Value::Object(Rc::new(Object::Callable(Rc::new(ObjCallable {
            name,
            arity: arity.into(),
            kind,
        }))))
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
            Object::Callable(callable) => write!(f, "<callable {}>", callable.name),
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
