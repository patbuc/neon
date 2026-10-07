use indexmap::IndexMap;
use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub mod chunk;
pub mod constants;
pub mod error_renderer;
pub mod errors;
pub mod method_registry;
pub(crate) mod opcodes;
pub mod runtime_error;
pub mod static_type;
pub mod stdlib;
pub mod string_similarity;
#[cfg(test)]
mod tests;

/// 2^63, one past `i64::MAX` and the negation of `i64::MIN`.
const TWO_POW_63: f64 = 9223372036854775808.0;

pub(crate) type NativeFn = fn(&[Value]) -> Result<Value, String>;
pub(crate) type NativeFnWithVm =
    fn(&mut dyn NativeContext, &[Value]) -> Result<Value, NativeCallError>;

/// Lets a native method call back into Neon code without depending on the
/// VM's concrete type, implemented by `VirtualMachine`.
pub(crate) trait NativeContext {
    fn call_value(&mut self, callee: Value, args: &[Value]) -> Result<Value, NativeCallError>;
}

/// `Runtime` holds an error a `call_value` callback already built.
#[derive(Debug)]
pub(crate) enum NativeCallError {
    Message(String),
    Runtime(crate::common::runtime_error::RuntimeError),
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
    pub instructions: Vec<u8>,
    pub line_infos: Vec<LineInfo>,
    /// `instructions` decoded once the chunk is final, with one source
    /// location per decoded instruction.
    pub(crate) code: Vec<chunk::Instr>,
    pub(crate) instr_lines: Vec<Option<LineInfo>>,
    pub(crate) closure_upvalues: Vec<(bool, u16)>,
    /// Field, method, and type names interned during semantic analysis,
    /// indexed by symbol id. Shared by every chunk of one compile.
    pub symbols: Rc<[Rc<str>]>,
    /// The source file this chunk was compiled from, when it came from one.
    /// Shared by every chunk of one compilation unit.
    pub file: Option<Rc<str>>,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapKey {
    String(Rc<String>),
    Int(i64),
    Number(OrderedFloat<f64>),
    Boolean(bool),
    /// A frozen copy of an array's elements, taken when the array is used
    /// as a key. Compared and hashed by content, so two arrays with equal
    /// elements share an entry even after the original array is mutated.
    Array(Rc<Vec<MapKey>>),
    /// An enum variant, unit or payload, frozen so no interior-mutable
    /// value is held.
    EnumVariant(Rc<EnumKey>),
}

/// A variant frozen as a key: its identity plus every field converted to
/// key form. A unit variant has no fields. Compared and hashed by
/// `enum_name`, `ordinal`, and `fields`, which determine the rest.
#[derive(Debug, Clone)]
pub struct EnumKey {
    enum_name: Rc<str>,
    variant_name: Rc<str>,
    ordinal: u16,
    field_symbols: Rc<[u16]>,
    fields: Vec<MapKey>,
}

impl PartialEq for EnumKey {
    fn eq(&self, other: &Self) -> bool {
        self.ordinal == other.ordinal
            && self.enum_name == other.enum_name
            && self.fields == other.fields
    }
}

impl Eq for EnumKey {}

impl std::hash::Hash for EnumKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.enum_name.hash(state);
        self.ordinal.hash(state);
        self.fields.hash(state);
    }
}

pub type SetKey = MapKey;

impl MapKey {
    /// Converts a hashable `Value` into a `MapKey`, or an error message if
    /// `value` can't be used as a `kind` (e.g. `"map key"` or `"set
    /// element"`). An integral float (including `-0.0`) that fits in `i64`
    /// normalizes to `MapKey::Int`, so `m[1]` and `m[1.0]` hit the same
    /// entry. An array is copied recursively, element by element, each of
    /// which must itself be a valid key; a self-referencing array is
    /// rejected rather than recursing forever.
    #[inline]
    pub fn from_value(value: &Value, kind: &str) -> Result<MapKey, String> {
        MapKey::from_value_with_seen(value, kind, &mut Vec::new())
    }

    fn from_value_with_seen(
        value: &Value,
        kind: &str,
        seen: &mut Vec<*const ()>,
    ) -> Result<MapKey, String> {
        match value {
            Value::String(s) => Ok(MapKey::String(Rc::clone(s))),
            Value::Int(i) => Ok(MapKey::Int(*i)),
            Value::Number(n) => {
                if n.fract() == 0.0 && f64_fits_i64(*n) {
                    Ok(MapKey::Int(*n as i64))
                } else {
                    Ok(MapKey::Number(OrderedFloat(*n)))
                }
            }
            Value::Boolean(b) => Ok(MapKey::Boolean(*b)),
            Value::EnumVariant(variant) => {
                if let Some(key) = &variant.unit_key {
                    return Ok(MapKey::EnumVariant(Rc::clone(key)));
                }
                let fields = variant
                    .fields
                    .iter()
                    .map(|field| MapKey::from_value_with_seen(field, kind, seen))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(MapKey::EnumVariant(Rc::new(EnumKey {
                    enum_name: Rc::clone(&variant.enum_name),
                    variant_name: Rc::clone(&variant.variant_name),
                    ordinal: variant.ordinal,
                    field_symbols: Rc::clone(&variant.field_symbols),
                    fields,
                })))
            }
            Value::Array(array) => from_value_array(array, kind, seen),
            other => Err(invalid_key_message(kind, other)),
        }
    }

    /// Converts a `MapKey` back into the `Value` it was built from. An
    /// array key is rebuilt into a fresh `Value::Array` every call, so
    /// mutating the returned array never changes the map or set it came
    /// from.
    #[inline]
    pub fn to_value(&self) -> Value {
        match self {
            MapKey::String(s) => Value::String(Rc::clone(s)),
            MapKey::Int(i) => Value::Int(*i),
            MapKey::Number(n) => Value::Number(n.into_inner()),
            MapKey::Boolean(b) => Value::Boolean(*b),
            MapKey::Array(items) => Value::new_array(items.iter().map(MapKey::to_value).collect()),
            MapKey::EnumVariant(key) => Value::EnumVariant(Rc::new(ObjEnumVariant {
                enum_name: Rc::clone(&key.enum_name),
                variant_name: Rc::clone(&key.variant_name),
                ordinal: key.ordinal,
                field_symbols: Rc::clone(&key.field_symbols),
                fields: key.fields.iter().map(MapKey::to_value).collect(),
                unit_key: (key.field_symbols.is_empty()).then(|| Rc::clone(key)),
            })),
        }
    }
}

fn invalid_key_message(kind: &str, value: &Value) -> String {
    format!(
        "Invalid {kind} type: {value}. Only strings, numbers, booleans, arrays, and enum variants \
         can be used as {kind}s."
    )
}

/// Freezes `array` into a `MapKey::Array`, recursing into nested arrays.
/// `seen` tracks the `Rc` pointers on the current path so a self-referencing
/// array is rejected instead of recursing forever.
fn from_value_array(
    array: &Rc<RefCell<Vec<Value>>>,
    kind: &str,
    seen: &mut Vec<*const ()>,
) -> Result<MapKey, String> {
    let ptr = Rc::as_ptr(array) as *const ();
    if seen.contains(&ptr) {
        return Err(format!(
            "Cannot use a self-referencing array as a {}.",
            kind
        ));
    }
    seen.push(ptr);
    let elements = array.borrow();
    let mut keys = Vec::with_capacity(elements.len());
    for element in elements.iter() {
        keys.push(MapKey::from_value_with_seen(element, kind, seen)?);
    }
    seen.pop();
    Ok(MapKey::Array(Rc::new(keys)))
}

impl Display for MapKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapKey::String(s) => write!(f, "{}", s),
            MapKey::Int(i) => write!(f, "{}", i),
            MapKey::Number(n) => write!(f, "{}", n),
            MapKey::Boolean(b) => write!(f, "{}", b),
            MapKey::Array(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            MapKey::EnumVariant(key) => {
                write!(f, "{}.{}", key.enum_name, key.variant_name)?;
                write_fields(f, &key.fields)
            }
        }
    }
}

fn write_fields<T: Display>(f: &mut Formatter<'_>, fields: &[T]) -> std::fmt::Result {
    if fields.is_empty() {
        return Ok(());
    }
    write!(f, "(")?;
    for (i, field) in fields.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        write!(f, "{}", field)?;
    }
    write!(f, ")")
}

/// `MapKey` groups for cross-variant ordering: strings, then numbers
/// (int and float together), then booleans, then enum variants, then
/// arrays.
fn map_key_rank(key: &MapKey) -> u8 {
    match key {
        MapKey::String(_) => 0,
        MapKey::Int(_) | MapKey::Number(_) => 1,
        MapKey::Boolean(_) => 2,
        MapKey::EnumVariant(_) => 3,
        MapKey::Array(_) => 4,
    }
}

impl PartialOrd for MapKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MapKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (self, other) {
            (MapKey::String(a), MapKey::String(b)) => a.cmp(b),
            (MapKey::Int(a), MapKey::Int(b)) => a.cmp(b),
            (MapKey::Number(a), MapKey::Number(b)) => a.cmp(b),
            (MapKey::Int(i), MapKey::Number(n)) => {
                compare_int_and_float(*i, n.into_inner()).unwrap_or(Ordering::Less)
            }
            (MapKey::Number(n), MapKey::Int(i)) => compare_int_and_float(*i, n.into_inner())
                .map(Ordering::reverse)
                .unwrap_or(Ordering::Greater),
            (MapKey::Boolean(a), MapKey::Boolean(b)) => a.cmp(b),
            (MapKey::EnumVariant(a), MapKey::EnumVariant(b)) => cmp_enum_keys(a, b),
            (MapKey::Array(a), MapKey::Array(b)) => cmp_arrays(a, b),
            _ => map_key_rank(self).cmp(&map_key_rank(other)),
        }
    }
}

/// Lexicographic by element `Ord`, then by length (`[T]`'s `Ord`).
#[inline(never)]
fn cmp_arrays(a: &[MapKey], b: &[MapKey]) -> std::cmp::Ordering {
    a.cmp(b)
}

/// Orders enum variants by `enum_name`, then `ordinal`, then fields.
#[inline(never)]
fn cmp_enum_keys(a: &EnumKey, b: &EnumKey) -> std::cmp::Ordering {
    a.enum_name
        .cmp(&b.enum_name)
        .then(a.ordinal.cmp(&b.ordinal))
        .then_with(|| a.fields.cmp(&b.fields))
}

/// Compares an `i64` and an `f64` by their exact numeric value (not by
/// casting the int to a possibly-imprecise float). Returns `None` for NaN.
pub(crate) fn compare_int_and_float(i: i64, f: f64) -> Option<std::cmp::Ordering> {
    use std::cmp::Ordering;
    if f.is_nan() {
        return None;
    }
    if f >= TWO_POW_63 {
        return Some(Ordering::Less);
    }
    if f < -TWO_POW_63 {
        return Some(Ordering::Greater);
    }
    let truncated = f.trunc();
    match i.cmp(&(truncated as i64)) {
        Ordering::Equal if f.fract() == 0.0 => Some(Ordering::Equal),
        Ordering::Equal if f.fract() > 0.0 => Some(Ordering::Less),
        Ordering::Equal => Some(Ordering::Greater),
        other => Some(other),
    }
}

/// Whether an `f64` holding an integral value fits in `i64`.
pub(crate) fn f64_fits_i64(f: f64) -> bool {
    (-TWO_POW_63..TWO_POW_63).contains(&f)
}

/// The numeric value of an `Int` or `Number`.
#[derive(Clone, Copy)]
pub(crate) enum Numeric {
    Int(i64),
    Float(f64),
}

impl Numeric {
    pub(crate) fn from_value(value: &Value) -> Option<Numeric> {
        match value {
            Value::Int(i) => Some(Numeric::Int(*i)),
            Value::Number(n) => Some(Numeric::Float(*n)),
            _ => None,
        }
    }

    pub(crate) fn as_f64(self) -> f64 {
        match self {
            Numeric::Int(i) => i as f64,
            Numeric::Float(f) => f,
        }
    }

    pub(crate) fn into_value(self) -> Value {
        match self {
            Numeric::Int(i) => Value::Int(i),
            Numeric::Float(f) => Value::Number(f),
        }
    }
}

/// Compares two `Numeric`s by exact value, `None` for an unordered float
/// pair (NaN involved).
pub(crate) fn compare_numeric(a: Numeric, b: Numeric) -> Option<std::cmp::Ordering> {
    match (a, b) {
        (Numeric::Float(a), Numeric::Float(b)) => a.partial_cmp(&b),
        (Numeric::Int(a), Numeric::Int(b)) => Some(a.cmp(&b)),
        (Numeric::Int(i), Numeric::Float(n)) => compare_int_and_float(i, n),
        (Numeric::Float(n), Numeric::Int(i)) => {
            compare_int_and_float(i, n).map(std::cmp::Ordering::reverse)
        }
    }
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Int(i64),
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
    EnumVariant(Rc<ObjEnumVariant>),
    Array(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<IndexMap<MapKey, Value>>>),
    Set(Rc<RefCell<BTreeSet<SetKey>>>),
    File(Rc<String>),
    Range(Rc<ObjRange>),
    PriorityQueue(Rc<RefCell<ObjPriorityQueue>>),
}

/// A min-heap of `(priority, value)` entries, ordered by `priority` then by
/// insertion order so equal priorities pop first-in-first-out.
pub struct ObjPriorityQueue {
    heap: BinaryHeap<Reverse<PriorityQueueEntry>>,
    next_seq: u64,
}

struct PriorityQueueEntry {
    priority: Numeric,
    seq: u64,
    value: Value,
}

impl PartialEq for PriorityQueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.seq == other.seq
    }
}

impl Eq for PriorityQueueEntry {}

impl PartialOrd for PriorityQueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityQueueEntry {
    #[allow(clippy::expect_used)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare_numeric(self.priority, other.priority)
            .expect("priority queue entries never hold a NaN priority")
            .then(self.seq.cmp(&other.seq))
    }
}

impl ObjPriorityQueue {
    pub(crate) fn new() -> Self {
        ObjPriorityQueue {
            heap: BinaryHeap::new(),
            next_seq: 0,
        }
    }

    pub(crate) fn push(&mut self, priority: Numeric, value: Value) {
        self.heap.push(Reverse(PriorityQueueEntry {
            priority,
            seq: self.next_seq,
            value,
        }));
        self.next_seq += 1;
    }

    pub(crate) fn pop(&mut self) -> Option<Value> {
        self.heap.pop().map(|Reverse(entry)| entry.value)
    }

    pub(crate) fn peek(&self) -> Option<&Value> {
        self.heap.peek().map(|Reverse(entry)| &entry.value)
    }

    pub(crate) fn len(&self) -> usize {
        self.heap.len()
    }
}

/// An immutable range of integers.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjRange {
    pub start: i64,
    pub end: i64,
    pub inclusive: bool,
}

impl ObjRange {
    /// Number of integers the range covers; empty (e.g. `5..1`) is 0, never
    /// negative. Saturates instead of overflowing for extreme bounds (e.g.
    /// `i64::MIN..i64::MAX`), where the true count doesn't fit in an `i64`.
    pub(crate) fn len(&self) -> i64 {
        let len = self.end.saturating_sub(self.start);
        if self.inclusive {
            len.saturating_add(1).max(0)
        } else {
            len.max(0)
        }
    }

    pub(crate) fn contains(&self, n: i64) -> bool {
        n >= self.start
            && if self.inclusive {
                n <= self.end
            } else {
                n < self.end
            }
    }

    /// The i-th element (0-based), assuming `0 <= i < self.len()`.
    pub(crate) fn get(&self, i: i64) -> i64 {
        self.start + i
    }

    /// The range's elements as `Int` values, in order, up to `limit` of
    /// them. Shared by callers that materialize a whole range (`limit` set
    /// to its length) and ones that only need as many elements as some
    /// other collection has, so a huge range isn't materialized needlessly.
    pub(crate) fn elements_upto(&self, limit: usize) -> Vec<Value> {
        let end = limit.min(usize::try_from(self.len()).unwrap_or(usize::MAX));
        (0..end).map(|i| Value::Int(self.get(i as i64))).collect()
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
    pub(crate) fn field(&self, symbol: u16) -> Option<&Value> {
        self.r#struct
            .field_index(symbol)
            .map(|index| &self.fields[index])
    }
}

/// A user method: (method symbol, closure, takes `self`).
pub(crate) type MethodEntry = (u16, Rc<ObjClosure>, bool);

#[inline]
pub(crate) fn find_method_entry(
    methods: &[MethodEntry],
    method_symbol: u16,
) -> Option<MethodEntry> {
    methods
        .iter()
        .find(|(symbol, _, _)| *symbol == method_symbol)
        .cloned()
}

#[derive(Clone)]
pub struct ObjStruct {
    pub name: String,
    pub fields: Vec<String>,
    field_table: Vec<Option<u16>>,
    pub name_symbol: u16,
    /// Methods the struct's `impl` blocks define, appended by `DefineMethod`.
    pub(crate) methods: RefCell<Vec<MethodEntry>>,
}

impl std::fmt::Debug for ObjStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjStruct")
            .field("name", &self.name)
            .field("fields", &self.fields)
            .field("name_symbol", &self.name_symbol)
            .field("method_count", &self.methods.borrow().len())
            .finish()
    }
}

impl ObjStruct {
    fn build_field_table(field_symbols: &[u16]) -> Vec<Option<u16>> {
        let max_symbol = field_symbols.iter().copied().max().unwrap_or(0);
        let mut table = vec![None; max_symbol as usize + 1];
        for (index, &symbol) in field_symbols.iter().enumerate() {
            table[symbol as usize] = Some(index as u16);
        }
        table
    }

    pub(crate) fn field_index(&self, symbol: u16) -> Option<usize> {
        self.field_table
            .get(symbol as usize)
            .copied()
            .flatten()
            .map(|index| index as usize)
    }

    pub(crate) fn find_method(&self, method_symbol: u16) -> Option<MethodEntry> {
        find_method_entry(&self.methods.borrow(), method_symbol)
    }
}

#[derive(Debug, Clone)]
pub struct ObjEnumVariant {
    pub enum_name: Rc<str>,
    pub variant_name: Rc<str>,
    pub ordinal: u16,
    /// Symbol ids of the payload field names, in declaration order; empty
    /// for a unit variant.
    pub field_symbols: Rc<[u16]>,
    /// The payload values, parallel to `field_symbols`. Empty on the
    /// constant-pool template a constructor call copies from.
    pub fields: Vec<Value>,
    /// The frozen key of a unit variant, built once at creation and shared
    /// by every map key conversion. `None` for payload variants.
    unit_key: Option<Rc<EnumKey>>,
}

impl ObjEnumVariant {
    pub(crate) fn field(&self, symbol: u16) -> Option<&Value> {
        let index = self.field_symbols.iter().position(|&s| s == symbol)?;
        self.fields.get(index)
    }

    /// True for the constant-pool template of a payload variant, which
    /// declares fields but holds no values.
    pub(crate) fn is_template(&self) -> bool {
        self.fields.is_empty() && !self.field_symbols.is_empty()
    }

    pub(crate) fn with_fields(&self, fields: Vec<Value>) -> ObjEnumVariant {
        ObjEnumVariant {
            enum_name: Rc::clone(&self.enum_name),
            variant_name: Rc::clone(&self.variant_name),
            ordinal: self.ordinal,
            field_symbols: Rc::clone(&self.field_symbols),
            fields,
            unit_key: None,
        }
    }
}

impl Value {
    /// Clones the value, copying scalars inline instead of calling `Clone`.
    #[inline(always)]
    pub(crate) fn copy_or_clone(&self) -> Value {
        match self {
            Value::Number(n) => Value::Number(*n),
            Value::Int(i) => Value::Int(*i),
            Value::Boolean(b) => Value::Boolean(*b),
            Value::Nil => Value::Nil,
            _ => self.clone(),
        }
    }

    /// Drops the value, skipping drop glue for scalars.
    #[inline(always)]
    pub(crate) fn discard(self) {
        if let Value::Number(_) | Value::Int(_) | Value::Boolean(_) | Value::Nil = self {
            std::mem::forget(self);
        }
    }

    pub(crate) fn new_instance(instance: ObjInstance) -> Value {
        Value::Instance(Rc::new(RefCell::new(instance)))
    }

    pub(crate) fn new_struct(
        name: String,
        fields: Vec<String>,
        field_symbols: Vec<u16>,
        name_symbol: u16,
    ) -> Self {
        let field_table = ObjStruct::build_field_table(&field_symbols);
        Value::Struct(Rc::new(ObjStruct {
            name,
            fields,
            field_table,
            name_symbol,
            methods: RefCell::new(Vec::new()),
        }))
    }

    pub(crate) fn new_enum_variant(enum_name: String, variant_name: String, ordinal: u16) -> Self {
        let enum_name: Rc<str> = Rc::from(enum_name);
        let variant_name: Rc<str> = Rc::from(variant_name);
        let field_symbols: Rc<[u16]> = Rc::from([]);
        let unit_key = Rc::new(EnumKey {
            enum_name: Rc::clone(&enum_name),
            variant_name: Rc::clone(&variant_name),
            ordinal,
            field_symbols: Rc::clone(&field_symbols),
            fields: Vec::new(),
        });
        Value::EnumVariant(Rc::new(ObjEnumVariant {
            enum_name,
            variant_name,
            ordinal,
            field_symbols,
            fields: Vec::new(),
            unit_key: Some(unit_key),
        }))
    }

    /// A payload variant's constant-pool template: it declares the fields
    /// but holds no values yet.
    pub(crate) fn new_enum_variant_template(
        enum_name: String,
        variant_name: String,
        ordinal: u16,
        field_symbols: Vec<u16>,
    ) -> Self {
        Value::EnumVariant(Rc::new(ObjEnumVariant {
            enum_name: Rc::from(enum_name),
            variant_name: Rc::from(variant_name),
            ordinal,
            field_symbols: Rc::from(field_symbols),
            fields: Vec::new(),
            unit_key: None,
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

    pub(crate) fn new_priority_queue() -> Self {
        Value::PriorityQueue(Rc::new(RefCell::new(ObjPriorityQueue::new())))
    }

    /// Name of this value's type, for runtime error messages.
    pub(crate) fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) | Value::Int(_) => "number",
            Value::Boolean(_) => "boolean",
            Value::Nil => "nil",
            Value::Uninitialized(_) => "uninitialized",
            Value::String(_) => "string",
            Value::Function(_) => "function",
            Value::Closure(_) => "function",
            Value::NativeFunction(_) => "function",
            Value::Struct(_) => "struct",
            Value::Instance(_) => "instance",
            Value::EnumVariant(_) => "enum",
            Value::Array(_) => "array",
            Value::Map(_) => "map",
            Value::Set(_) => "set",
            Value::File(_) => "file",
            Value::Range(_) => "range",
            Value::PriorityQueue(_) => "priority queue",
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
            Value::Int(val) => write!(f, "{}", val),
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
            Value::EnumVariant(variant) => {
                write!(f, "{}.{}", variant.enum_name, variant.variant_name)?;
                if variant.fields.is_empty() {
                    return Ok(());
                }
                write!(f, "(")?;
                for (i, field) in variant.fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    field.fmt_with_seen(f, seen)?;
                }
                write!(f, ")")
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
            Value::PriorityQueue(pq) => write!(f, "PriorityQueue(size={})", pq.borrow().len()),
        }
    }

    fn eq_with_seen(&self, other: &Self, seen: &mut Vec<(*const (), *const ())>) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Int(a), Value::Number(b)) | (Value::Number(b), Value::Int(a)) => {
                compare_int_and_float(*a, *b) == Some(std::cmp::Ordering::Equal)
            }
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Function(a), Value::Function(b)) => a == b,
            (Value::Closure(a), Value::Closure(b)) => Rc::ptr_eq(a, b),
            (Value::NativeFunction(a), Value::NativeFunction(b)) => a == b,
            (Value::Struct(a), Value::Struct(b)) => a == b,
            (Value::EnumVariant(a), Value::EnumVariant(b)) => {
                a.enum_name == b.enum_name
                    && a.ordinal == b.ordinal
                    && a.fields.len() == b.fields.len()
                    && a.fields
                        .iter()
                        .zip(b.fields.iter())
                        .all(|(x, y)| x.eq_with_seen(y, seen))
            }
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
            (Value::PriorityQueue(a), Value::PriorityQueue(b)) => Rc::ptr_eq(a, b),
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
            Value::Int(i) => f.debug_tuple("Int").field(i).finish(),
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
