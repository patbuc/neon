use std::mem::transmute;

#[repr(u8)]
#[derive(Debug, PartialEq)]
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
}

impl OpCode {
    #[inline(always)]
    pub(crate) fn from_u8(value: u8) -> OpCode {
        unsafe { transmute(value) }
    }
}
