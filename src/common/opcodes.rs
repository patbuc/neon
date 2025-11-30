use std::mem::transmute;

impl OpCode {
    #[inline(always)]
    pub(crate) fn from_u8(value: u8) -> OpCode {
        unsafe { transmute(value) }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub(crate) enum OpCode {
    Return = 0x00,
    Constant,
    Constant2,
    Constant4,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Nil,
    True,
    False,
    Equal,
    Greater,
    Less,
    Not,
    String,
    String2,
    String4,
    Print,
    Pop,
    SetLocal,
    SetLocal2,
    SetLocal4,
    GetLocal,
    GetLocal2,
    GetLocal4,
    JumpIfFalse,
    Jump,
    Loop,
    Call,
    GetGlobal,
    GetGlobal2,
    GetGlobal4,
    SetGlobal,
    SetGlobal2,
    SetGlobal4,
    GetField,
    GetField2,
    GetField4,
    SetField,
    SetField2,
    SetField4,

    // Map operations
    /// Create empty map
    /// Stack: [] -> [map]
    Map,

    /// Set key-value pair in map, operand: key string index
    /// Stack: [map, value] -> [map]
    MapSet,

    /// Get value by key from map, operand: key string index
    /// Stack: [map] -> [value]
    MapGet,

    /// Check if key exists in map, operand: key string index
    /// Stack: [map] -> [boolean]
    MapHas,

    /// Remove key from map, operand: key string index
    /// Stack: [map] -> [map]
    MapRemove,

    /// Get array of keys from map
    /// Stack: [map] -> [array]
    MapKeys,

    /// Get array of values from map
    /// Stack: [map] -> [array]
    MapValues,

    /// Get size of map
    /// Stack: [map] -> [number]
    MapSize,

    // Set operations
    /// Create empty set
    /// Stack: [] -> [set]
    Set,

    /// Add element to set
    /// Stack: [set, value] -> [set]
    SetAdd,

    /// Check if value exists in set
    /// Stack: [set, value] -> [boolean]
    SetHas,

    /// Remove element from set
    /// Stack: [set, value] -> [set]
    SetRemove,

    /// Get size of set
    /// Stack: [set] -> [number]
    SetSize,

    /// Get array of values from set
    /// Stack: [set] -> [array]
    SetValues,
}
