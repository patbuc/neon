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
    SetValue,
    SetValue2,
    SetValue4,
    GetValue,
    GetValue2,
    GetValue4,
    SetVariable,
    SetVariable2,
    SetVariable4,
    GetVariable,
    GetVariable2,
    GetVariable4,
    JumpIfFalse,
    Jump,
}

impl OpCode {
    #[inline(always)]
    pub(crate) fn from_u8(value: u8) -> OpCode {
        unsafe { transmute(value) }
    }
}
