use crate::common::opcodes::OpCode;

#[test]
fn from_u8_round_trips_every_valid_opcode() {
    let max = OpCode::Dup as u8;
    for byte in 0..=max {
        let op = OpCode::from_u8(byte)
            .unwrap_or_else(|| panic!("byte {} should decode to an opcode", byte));
        assert_eq!(
            byte, op as u8,
            "byte {} decoded to a different opcode",
            byte
        );
    }
}

#[test]
fn from_u8_rejects_byte_past_last_opcode() {
    assert_eq!(None, OpCode::from_u8(OpCode::Dup as u8 + 1));
}

#[test]
fn from_u8_rejects_unknown_byte() {
    assert_eq!(None, OpCode::from_u8(0xFF));
}

#[test]
fn from_u8_round_trips_dup() {
    assert_eq!(Some(OpCode::Dup), OpCode::from_u8(OpCode::Dup as u8));
}
