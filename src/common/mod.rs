use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use ordered_float::OrderedFloat;

pub(crate) mod bloq;
pub(crate) mod opcodes;
pub mod errors;
pub mod constants;
pub mod error_renderer;
pub mod string_similarity;
pub mod method_registry;

#[cfg(test)]
mod tests;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

/// HTTP Server object - stores port, routes, and server state
#[derive(Debug)]
pub(crate) struct ObjHttpServer {
    pub port: u16,
    pub routes: HashMap<String, Value>,
}

/// HTTP Request object - immutable wrapper around request data
#[derive(Debug)]
pub(crate) struct ObjHttpRequest {
    pub method: String,
    pub path: String,
    pub body: String,
    pub headers: HashMap<String, String>,
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
    Set(Rc<RefCell<BTreeSet<SetKey>>>),
    File(Rc<str>),
    HttpServer(Rc<RefCell<ObjHttpServer>>),
    HttpRequest(Rc<ObjHttpRequest>),
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

    pub(crate) fn new_set(elements: BTreeSet<SetKey>) -> Self {
        Value::Object(Rc::new(Object::Set(Rc::new(RefCell::new(elements)))))
    }

    pub(crate) fn new_file(path: String) -> Self {
        Value::Object(Rc::new(Object::File(Rc::from(path))))
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
            Object::HttpServer(srv) => {
                let server = srv.borrow();
                write!(f, "<HttpServer port:{}>", server.port)
            }
            Object::HttpRequest(req) => {
                write!(f, "<HttpRequest {} {}>", req.method, req.path)
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

impl PartialEq for ObjHttpServer {
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port
    }
}

impl PartialEq for ObjHttpRequest {
    fn eq(&self, other: &Self) -> bool {
        self.method == other.method && self.path == other.path
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
