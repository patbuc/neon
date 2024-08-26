use std::mem::transmute;

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub(crate) enum OpCode {
    Return = 0x00,
    Constant = 0x01,
    Constant2 = 0x02,
    Constant4 = 0x03,
    Negate = 0x04,
    Add = 0x05,
    Subtract = 0x06,
    Multiply = 0x07,
    Divide = 0x08,
    Nil = 0x09,
    True = 0x0A,
    False = 0x0B,
    Equal = 0x0C,
    Greater = 0x0D,
    Less = 0x0E,
    Not = 0x0F,
    String = 0x10,
    String2 = 0x11,
    String4 = 0x12,
    Print = 0x13,
    Pop = 0x14,
    SetValue = 0x15,
    SetValue2 = 0x16,
    SetValue4 = 0x17,
    SetVariable = 0x18,
    SetVariable2 = 0x19,
    SetVariable4 = 0x1A,
    GetValue = 0x1B,
    GetValue2 = 0x1C,
    GetValue4 = 0x1D,
    GetVariable = 0x1E,
    GetVariable2 = 0x1F,
    GetVariable4 = 0x20,
}

impl OpCode {
    #[inline(always)]
    pub(in crate::vm) fn from_u8(value: u8) -> OpCode {
        unsafe { transmute(value) }
    }
}
