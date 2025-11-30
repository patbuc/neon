use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub(crate) mod bloq;
pub(crate) mod opcodes;
pub mod errors;
pub mod constants;
pub mod error_renderer;

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
    Struct(Rc<ObjStruct>),
    Instance(Rc<RefCell<ObjInstance>>),
    Map(Rc<RefCell<ObjMap>>),
    Set(Rc<RefCell<ObjSet>>),
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

#[derive(Debug, Clone)]
pub(crate) struct ObjMap {
    pub entries: std::collections::HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub(crate) struct ObjSet {
    pub elements: std::collections::HashMap<String, Value>,
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

    pub(crate) fn new_map() -> Self {
        Value::Object(Rc::new(Object::Map(Rc::new(RefCell::new(ObjMap {
            entries: std::collections::HashMap::new(),
        })))))
    }

    pub(crate) fn new_set() -> Self {
        Value::Object(Rc::new(Object::Set(Rc::new(RefCell::new(ObjSet {
            elements: std::collections::HashMap::new(),
        })))))
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
            Object::Struct(obj_struct) => write!(f, "<struct {}>", obj_struct.name),
            Object::Instance(obj_instance) => {
                let instance = obj_instance;
                write!(f, "<{} instance>", instance.borrow().r#struct.name)
            }
            Object::Map(obj_map) => {
                let map = obj_map.borrow();
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in &map.entries {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                    first = false;
                }
                write!(f, "}}")
            }
            Object::Set(obj_set) => {
                let set = obj_set.borrow();
                write!(f, "#{{")?;
                let mut first = true;
                for value in set.elements.values() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                    first = false;
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

impl PartialEq for ObjMap {
    fn eq(&self, other: &Self) -> bool {
        // Maps are equal if they have the same entries
        self.entries == other.entries
    }
}

impl PartialEq for ObjSet {
    fn eq(&self, other: &Self) -> bool {
        // Sets are equal if they have the same elements
        self.elements == other.elements
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
