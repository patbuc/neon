/// A statically known Neon type, tracked by the semantic analyzer to
/// validate method calls and field access ahead of runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StaticType {
    Number,
    String,
    Boolean,
    Nil,
    Array,
    Map,
    Set,
    Struct(String),
}

impl StaticType {
    /// The runtime type name (`get_type_name` in `src/vm/functions.rs`), or
    /// the struct's own name for `Struct`.
    pub fn name(&self) -> &str {
        match self {
            StaticType::Number => "Number",
            StaticType::String => "String",
            StaticType::Boolean => "Boolean",
            StaticType::Nil => "Nil",
            StaticType::Array => "Array",
            StaticType::Map => "Map",
            StaticType::Set => "Set",
            StaticType::Struct(name) => name,
        }
    }

    /// The `StaticType` for a builtin type name, or `Struct(name)` for
    /// anything else - used for `impl` blocks, which can target either a
    /// struct or a builtin type.
    pub fn from_name(name: &str) -> StaticType {
        match name {
            "Number" => StaticType::Number,
            "String" => StaticType::String,
            "Boolean" => StaticType::Boolean,
            "Nil" => StaticType::Nil,
            "Array" => StaticType::Array,
            "Map" => StaticType::Map,
            "Set" => StaticType::Set,
            other => StaticType::Struct(other.to_string()),
        }
    }
}
