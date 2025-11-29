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

    // Array operations
    /// Create an empty array.
    /// Stack: [] -> [array]
    Array,

    /// Create an array with a pre-allocated size.
    /// Operand: 1-byte size value (0-255)
    /// Stack: [] -> [array]
    ArrayWithSize,

    /// Create an array with a pre-allocated size.
    /// Operand: 2-byte size value (0-65535)
    /// Stack: [] -> [array]
    ArrayWithSize2,

    /// Create an array with a pre-allocated size.
    /// Operand: 4-byte size value
    /// Stack: [] -> [array]
    ArrayWithSize4,

    /// Append an element to an array.
    /// Stack: [array, value] -> [array]
    /// Note: The array is returned to support method chaining
    ArrayPush,

    /// Get the length of an array.
    /// Stack: [array] -> [length]
    ArrayLength,

    /// Get an element from an array at the given index.
    /// Operand: 1-byte index value (0-255)
    /// Stack: [array] -> [value]
    GetIndex,

    /// Get an element from an array at the given index.
    /// Operand: 2-byte index value (0-65535)
    /// Stack: [array] -> [value]
    GetIndex2,

    /// Get an element from an array at the given index.
    /// Operand: 4-byte index value
    /// Stack: [array] -> [value]
    GetIndex4,

    /// Set an element in an array at the given index.
    /// Operand: 1-byte index value (0-255)
    /// Stack: [array, value] -> [value]
    /// Note: The value is returned to support assignment expressions
    SetIndex,

    /// Set an element in an array at the given index.
    /// Operand: 2-byte index value (0-65535)
    /// Stack: [array, value] -> [value]
    /// Note: The value is returned to support assignment expressions
    SetIndex2,

    /// Set an element in an array at the given index.
    /// Operand: 4-byte index value
    /// Stack: [array, value] -> [value]
    /// Note: The value is returned to support assignment expressions
    SetIndex4,
}
