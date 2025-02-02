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
    DefineGlobal,
    DefineGlobal2,
    DefineGlobal4,
    GetGlobal,
    GetGlobal2,
    GetGlobal4,
    JumpIfFalse,
}

impl OpCode {
    #[inline(always)]
    pub(in crate::vm) fn from_u8(value: u8) -> OpCode {
        unsafe { transmute(value) }
    }
}
