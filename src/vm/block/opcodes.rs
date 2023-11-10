use enum_primitive_derive::Primitive;

#[repr(u8)]
#[derive(Debug, PartialEq, Primitive)]
pub enum OpCode {
    Return = 0x00,
    Constant = 0x01,
    Constant2 = 0x02,
    Constant4 = 0x03,
}
