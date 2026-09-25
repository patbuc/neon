use indexmap::IndexMap;
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
pub mod static_type;
pub mod stdlib;
pub mod string_similarity;
#[cfg(test)]
mod tests;

pub(crate) type NativeFn = fn(&[Value]) -> Result<Value, String>;
pub(crate) type NativeFnWithVm =
    fn(&mut crate::vm::VirtualMachine, &[Value]) -> Result<Value, NativeCallError>;

/// `Runtime` holds an error a `call_value` callback already built.
#[derive(Debug)]
pub(crate) enum NativeCallError {
    Message(String),
    Runtime(crate::vm::RuntimeError),
}

impl From<String> for NativeCallError {
    fn from(message: String) -> Self {
        NativeCallError::Message(message)
    }
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
    #[allow(dead_code)]
    pub name: String,
    pub constants: Constants,
    pub strings: Constants,
    pub instructions: Vec<u8>,
    pub line_infos: Vec<LineInfo>,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineInfo {
    pub ip: usize,
    pub line: u32,
    pub column: u32,
}

#[derive(Clone)]
pub enum Object {
    String(ObjString),
    Function(Rc<ObjFunction>),
    Closure(Rc<ObjClosure>),
    NativeFunction(Rc<ObjNativeFunction>),
    Struct(Rc<ObjStruct>),
    Instance(Rc<RefCell<ObjInstance>>),
    Array(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<IndexMap<MapKey, Value>>>),
    Set(Rc<RefCell<BTreeSet<SetKey>>>),
    File(Rc<str>),
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

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Object(Rc<Object>),
    Boolean(bool),
    Nil,
    /// Placeholder held by a hoisted global or block-level function slot
    /// until its declaration runs; GetGlobal, SetGlobal, and CheckInitialized
    /// turn reading it into a runtime error.
    Uninitialized(Rc<str>),
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

/// A function bundled with the values it closes over. This is the only
/// callable representation of a Neon function at runtime; `ObjFunction`
/// itself is just the compiled template stored in a constant pool, read by
/// the `Closure` opcode.
#[derive(Debug)]
pub struct ObjClosure {
    pub function: Rc<ObjFunction>,
    pub upvalues: Vec<Rc<RefCell<Upvalue>>>,
}

/// A variable captured by a closure. Open while the stack slot that holds
/// it is still live; closed (its value copied out) once that slot's frame
/// returns or the block that declared it exits.
#[derive(Debug)]
pub enum Upvalue {
    Open(usize),
    Closed(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjNativeFunction {
    pub name: String,
    pub arity: u8,
    pub method_index: u32,
    pub method_name: String,
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

    pub(crate) fn new_closure(
        function: Rc<ObjFunction>,
        upvalues: Vec<Rc<RefCell<Upvalue>>>,
    ) -> Self {
        Value::Object(Rc::new(Object::Closure(Rc::new(ObjClosure {
            function,
            upvalues,
        }))))
    }

    pub(crate) fn new_native_function(
        name: String,
        arity: u8,
        method_index: u32,
        method_name: String,
    ) -> Self {
        Value::Object(Rc::new(Object::NativeFunction(Rc::new(
            ObjNativeFunction {
                name,
                arity,
                method_index,
                method_name,
            },
        ))))
    }

    pub(crate) fn new_array(elements: Vec<Value>) -> Self {
        Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(elements)))))
    }

    pub(crate) fn new_map(entries: IndexMap<MapKey, Value>) -> Self {
        Value::Object(Rc::new(Object::Map(Rc::new(RefCell::new(entries)))))
    }

    pub(crate) fn new_set(elements: BTreeSet<SetKey>) -> Self {
        Value::Object(Rc::new(Object::Set(Rc::new(RefCell::new(elements)))))
    }

    pub(crate) fn new_file(path: String) -> Self {
        Value::Object(Rc::new(Object::File(Rc::from(path))))
    }

    /// Name of this value's type, for runtime error messages.
    pub(crate) fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Boolean(_) => "boolean",
            Value::Nil => "nil",
            Value::Uninitialized(_) => "uninitialized",
            Value::Object(obj) => match obj.as_ref() {
                Object::String(_) => "string",
                Object::Function(_) => "function",
                Object::Closure(_) => "function",
                Object::NativeFunction(_) => "function",
                Object::Struct(_) => "struct",
                Object::Instance(_) => "instance",
                Object::Array(_) => "array",
                Object::Map(_) => "map",
                Object::Set(_) => "set",
                Object::File(_) => "file",
            },
        }
    }
}

pub struct CallFrame {
    pub closure: Rc<ObjClosure>,
    pub ip: usize,
    pub slot_start: isize, // Can be -1 for script frame
    /// iterator_stack depth when this frame was pushed.
    pub iterator_depth: usize,
}

impl Object {
    fn fmt_with_seen(&self, f: &mut Formatter<'_>, seen: &mut Vec<*const ()>) -> std::fmt::Result {
        match self {
            Object::String(obj_string) => write!(f, "{}", obj_string.value),
            Object::Function(obj_function) => write!(f, "<fn {}>", obj_function.name),
            Object::Closure(obj_closure) => write!(f, "<fn {}>", obj_closure.function.name),
            Object::NativeFunction(obj_native_function) => {
                write!(f, "<native fn {}>", obj_native_function.name)
            }
            Object::Struct(obj_struct) => write!(f, "<struct {}>", obj_struct.name),
            Object::Instance(obj_instance) => {
                let instance = obj_instance;
                write!(f, "<{} instance>", instance.borrow().r#struct.name)
            }
            Object::Array(array) => {
                let ptr = Rc::as_ptr(array) as *const ();
                if seen.contains(&ptr) {
                    return write!(f, "[...]");
                }
                seen.push(ptr);
                write!(f, "[")?;
                for (i, value) in array.borrow().iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    value.fmt_with_seen(f, seen)?;
                }
                write!(f, "]")?;
                seen.pop();
                Ok(())
            }
            Object::Map(map) => {
                let ptr = Rc::as_ptr(map) as *const ();
                if seen.contains(&ptr) {
                    return write!(f, "{{...}}");
                }
                seen.push(ptr);
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in map.borrow().iter() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    first = false;
                    write!(f, "{}: ", key)?;
                    value.fmt_with_seen(f, seen)?;
                }
                write!(f, "}}")?;
                seen.pop();
                Ok(())
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
            Object::File(path) => write!(f, "<file: {}>", path),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_seen(f, &mut Vec::new())
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
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

/// Runs `compare` guarded against cycles through `a`/`b`: if this exact
/// pointer pair is already being compared further up the call stack, treats
/// them as equal instead of recursing again.
fn guarded_eq<T>(
    a: &Rc<T>,
    b: &Rc<T>,
    seen: &mut Vec<(*const (), *const ())>,
    compare: impl FnOnce(&mut Vec<(*const (), *const ())>) -> bool,
) -> bool {
    let key = (Rc::as_ptr(a) as *const (), Rc::as_ptr(b) as *const ());
    if seen.contains(&key) {
        return true;
    }
    seen.push(key);
    let equal = compare(seen);
    seen.pop();
    equal
}

impl Object {
    fn eq_with_seen(&self, other: &Self, seen: &mut Vec<(*const (), *const ())>) -> bool {
        match (self, other) {
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Function(a), Object::Function(b)) => a == b,
            (Object::Closure(a), Object::Closure(b)) => Rc::ptr_eq(a, b),
            (Object::NativeFunction(a), Object::NativeFunction(b)) => a == b,
            (Object::Struct(a), Object::Struct(b)) => a == b,
            (Object::Instance(a), Object::Instance(b)) => guarded_eq(a, b, seen, |seen| {
                let ia = a.borrow();
                let ib = b.borrow();
                ia.r#struct.name == ib.r#struct.name
                    && ia.fields.len() == ib.fields.len()
                    && ia
                        .fields
                        .iter()
                        .all(|(k, v)| ib.fields.get(k).is_some_and(|w| v.eq_with_seen(w, seen)))
            }),
            (Object::Array(a), Object::Array(b)) => guarded_eq(a, b, seen, |seen| {
                let va = a.borrow();
                let vb = b.borrow();
                va.len() == vb.len()
                    && va
                        .iter()
                        .zip(vb.iter())
                        .all(|(x, y)| x.eq_with_seen(y, seen))
            }),
            (Object::Map(a), Object::Map(b)) => guarded_eq(a, b, seen, |seen| {
                let ma = a.borrow();
                let mb = b.borrow();
                ma.len() == mb.len()
                    && ma
                        .iter()
                        .all(|(k, v)| mb.get(k).is_some_and(|w| v.eq_with_seen(w, seen)))
            }),
            (Object::Set(a), Object::Set(b)) => a == b,
            (Object::File(a), Object::File(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.eq_with_seen(other, &mut Vec::new())
    }
}

impl Value {
    fn fmt_with_seen(&self, f: &mut Formatter<'_>, seen: &mut Vec<*const ()>) -> std::fmt::Result {
        match self {
            Value::Number(val) => write!(f, "{}", val),
            Value::Boolean(val) => write!(f, "{}", val),
            Value::Nil => write!(f, "nil"),
            Value::Uninitialized(_) => write!(f, "<uninitialized>"),
            Value::Object(val) => val.fmt_with_seen(f, seen),
        }
    }

    fn eq_with_seen(&self, other: &Self, seen: &mut Vec<(*const (), *const ())>) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Object(a), Value::Object(b)) => a.eq_with_seen(b, seen),
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_seen(f, &mut Vec::new())
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.eq_with_seen(other, &mut Vec::new())
    }
}
