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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MapKey {
    String(Rc<String>),
    Number(OrderedFloat<f64>),
    Boolean(bool),
}

pub type SetKey = MapKey;

impl MapKey {
    /// Converts a hashable `Value` into a `MapKey`, or `None` if `value`
    /// can't be used as a map/set key.
    pub fn from_value(value: &Value) -> Option<MapKey> {
        match value {
            Value::String(s) => Some(MapKey::String(Rc::clone(s))),
            Value::Number(n) => Some(MapKey::Number(OrderedFloat(*n))),
            Value::Boolean(b) => Some(MapKey::Boolean(*b)),
            _ => None,
        }
    }

    /// Converts a `MapKey` back into the `Value` it was built from.
    pub fn to_value(&self) -> Value {
        match self {
            MapKey::String(s) => Value::String(Rc::clone(s)),
            MapKey::Number(n) => Value::Number(n.into_inner()),
            MapKey::Boolean(b) => Value::Boolean(*b),
        }
    }
}

impl Display for MapKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapKey::String(s) => write!(f, "{}", s),
            MapKey::Number(n) => write!(f, "{}", n),
            MapKey::Boolean(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
    /// Placeholder held by a hoisted global or block-level function slot
    /// until its declaration runs; GetGlobal, SetGlobal, and CheckInitialized
    /// turn reading it into a runtime error.
    Uninitialized(Rc<String>),
    String(Rc<String>),
    Function(Rc<ObjFunction>),
    Closure(Rc<ObjClosure>),
    NativeFunction(Rc<ObjNativeFunction>),
    Struct(Rc<ObjStruct>),
    Instance(Rc<RefCell<ObjInstance>>),
    Array(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<IndexMap<MapKey, Value>>>),
    Set(Rc<RefCell<BTreeSet<SetKey>>>),
    File(Rc<String>),
    Range(Rc<ObjRange>),
}

/// An immutable range of integers.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjRange {
    pub start: i64,
    pub end: i64,
    pub inclusive: bool,
}

impl ObjRange {
    /// Number of integers the range covers; empty (e.g. `5..1`) is 0, never negative.
    pub(crate) fn len(&self) -> i64 {
        if self.inclusive {
            (self.end - self.start + 1).max(0)
        } else {
            (self.end - self.start).max(0)
        }
    }

    /// The i-th element (0-based), assuming `0 <= i < self.len()`.
    pub(crate) fn get(&self, i: i64) -> i64 {
        self.start + i
    }
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
}

#[derive(Debug, Clone)]
pub struct ObjInstance {
    pub r#struct: Rc<ObjStruct>,
    pub fields: Vec<Value>,
}

impl ObjInstance {
    pub(crate) fn field(&self, name: &str) -> Option<&Value> {
        self.r#struct
            .field_index(name)
            .map(|index| &self.fields[index])
    }
}

#[derive(Debug, Clone)]
pub struct ObjStruct {
    pub name: String,
    pub fields: Vec<String>,
    pub field_indices: HashMap<String, usize>,
}

impl ObjStruct {
    pub(crate) fn field_index(&self, name: &str) -> Option<usize> {
        self.field_indices.get(name).copied()
    }
}

impl Value {
    pub(crate) fn new_instance(instance: ObjInstance) -> Value {
        Value::Instance(Rc::new(RefCell::new(instance)))
    }

    pub(crate) fn new_struct(name: String, fields: Vec<String>) -> Self {
        let field_indices = fields
            .iter()
            .enumerate()
            .map(|(index, name)| (name.clone(), index))
            .collect();
        Value::Struct(Rc::new(ObjStruct {
            name,
            fields,
            field_indices,
        }))
    }

    pub(crate) fn new_function(name: String, arity: u8, chunk: Chunk) -> Self {
        Value::Function(Rc::new(ObjFunction {
            name,
            arity,
            chunk: Rc::new(chunk),
        }))
    }

    pub(crate) fn new_closure(
        function: Rc<ObjFunction>,
        upvalues: Vec<Rc<RefCell<Upvalue>>>,
    ) -> Self {
        Value::Closure(Rc::new(ObjClosure { function, upvalues }))
    }

    pub(crate) fn new_native_function(name: String, arity: u8, method_index: u32) -> Self {
        Value::NativeFunction(Rc::new(ObjNativeFunction {
            name,
            arity,
            method_index,
        }))
    }

    pub(crate) fn new_array(elements: Vec<Value>) -> Self {
        Value::Array(Rc::new(RefCell::new(elements)))
    }

    pub(crate) fn new_map(entries: IndexMap<MapKey, Value>) -> Self {
        Value::Map(Rc::new(RefCell::new(entries)))
    }

    pub(crate) fn new_set(elements: BTreeSet<SetKey>) -> Self {
        Value::Set(Rc::new(RefCell::new(elements)))
    }

    pub(crate) fn new_file(path: String) -> Self {
        Value::File(Rc::new(path))
    }

    pub(crate) fn new_range(start: i64, end: i64, inclusive: bool) -> Self {
        Value::Range(Rc::new(ObjRange {
            start,
            end,
            inclusive,
        }))
    }

    /// Name of this value's type, for runtime error messages.
    pub(crate) fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Boolean(_) => "boolean",
            Value::Nil => "nil",
            Value::Uninitialized(_) => "uninitialized",
            Value::String(_) => "string",
            Value::Function(_) => "function",
            Value::Closure(_) => "function",
            Value::NativeFunction(_) => "function",
            Value::Struct(_) => "struct",
            Value::Instance(_) => "instance",
            Value::Array(_) => "array",
            Value::Map(_) => "map",
            Value::Set(_) => "set",
            Value::File(_) => "file",
            Value::Range(_) => "range",
        }
    }
}

pub struct CallFrame {
    pub closure: Rc<ObjClosure>,
    pub ip: usize,
    pub slot_start: isize, // Can be -1 for script frame
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

impl Value {
    fn fmt_with_seen(&self, f: &mut Formatter<'_>, seen: &mut Vec<*const ()>) -> std::fmt::Result {
        match self {
            Value::Number(val) => write!(f, "{}", val),
            Value::Boolean(val) => write!(f, "{}", val),
            Value::Nil => write!(f, "nil"),
            Value::Uninitialized(_) => write!(f, "<uninitialized>"),
            Value::String(s) => write!(f, "{}", s),
            Value::Function(function) => write!(f, "<fn {}>", function.name),
            Value::Closure(closure) => write!(f, "<fn {}>", closure.function.name),
            Value::NativeFunction(native_function) => {
                write!(f, "<native fn {}>", native_function.name)
            }
            Value::Struct(r#struct) => write!(f, "<struct {}>", r#struct.name),
            Value::Instance(instance) => {
                write!(f, "<{} instance>", instance.borrow().r#struct.name)
            }
            Value::Array(array) => {
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
            Value::Map(map) => {
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
            Value::Set(set) => {
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
            Value::File(path) => write!(f, "<file: {}>", path),
            Value::Range(range) => {
                if range.inclusive {
                    write!(f, "{}..={}", range.start, range.end)
                } else {
                    write!(f, "{}..{}", range.start, range.end)
                }
            }
        }
    }

    fn eq_with_seen(&self, other: &Self, seen: &mut Vec<(*const (), *const ())>) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Function(a), Value::Function(b)) => a == b,
            (Value::Closure(a), Value::Closure(b)) => Rc::ptr_eq(a, b),
            (Value::NativeFunction(a), Value::NativeFunction(b)) => a == b,
            (Value::Struct(a), Value::Struct(b)) => a == b,
            (Value::Instance(a), Value::Instance(b)) => guarded_eq(a, b, seen, |seen| {
                let ia = a.borrow();
                let ib = b.borrow();
                *ia.r#struct == *ib.r#struct
                    && ia
                        .fields
                        .iter()
                        .zip(ib.fields.iter())
                        .all(|(v, w)| v.eq_with_seen(w, seen))
            }),
            (Value::Array(a), Value::Array(b)) => guarded_eq(a, b, seen, |seen| {
                let va = a.borrow();
                let vb = b.borrow();
                va.len() == vb.len()
                    && va
                        .iter()
                        .zip(vb.iter())
                        .all(|(x, y)| x.eq_with_seen(y, seen))
            }),
            (Value::Map(a), Value::Map(b)) => guarded_eq(a, b, seen, |seen| {
                let ma = a.borrow();
                let mb = b.borrow();
                ma.len() == mb.len()
                    && ma
                        .iter()
                        .all(|(k, v)| mb.get(k).is_some_and(|w| v.eq_with_seen(w, seen)))
            }),
            (Value::Set(a), Value::Set(b)) => a == b,
            (Value::File(a), Value::File(b)) => a == b,
            (Value::Range(a), Value::Range(b)) => a == b,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_seen(f, &mut Vec::new())
    }
}

impl std::fmt::Debug for Value {
    /// Scalars get tagged output (`Number(1.0)`, `String("x")`) so panic
    /// messages stay unambiguous; other heap values fall back to the
    /// cycle-safe Display form.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => f.debug_tuple("Number").field(n).finish(),
            Value::Boolean(b) => f.debug_tuple("Boolean").field(b).finish(),
            Value::Nil => write!(f, "Nil"),
            Value::Uninitialized(name) => f.debug_tuple("Uninitialized").field(name).finish(),
            Value::String(s) => f.debug_tuple("String").field(s).finish(),
            _ => self.fmt_with_seen(f, &mut Vec::new()),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.eq_with_seen(other, &mut Vec::new())
    }
}
