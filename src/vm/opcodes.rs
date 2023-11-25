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
}

impl OpCode {
    #[inline(always)]
    pub(in crate::vm) fn from_u8(value: u8) -> OpCode {
        unsafe { transmute(value) }
    }
}
