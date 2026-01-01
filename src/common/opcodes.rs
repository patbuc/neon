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
    FloorDivide,
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
    GetBuiltin,
    GetBuiltin2,
    GetBuiltin4,
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

    CreateMap,
    CreateArray,
    CreateSet,
    GetIndex,
    SetIndex,
    GetIterator,
    IteratorNext,
    IteratorDone,
    PopIterator,
    CreateRange,
    ToString,
}
