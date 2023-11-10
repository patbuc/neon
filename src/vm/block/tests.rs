use crate::vm::block::opcodes::OpCode;
use crate::vm::block::Block;
use num_traits::FromPrimitive;

#[test]
fn new_block_is_empty() {
    let block = Block::new("origin");

    assert_eq!("origin", block.name);
    assert_eq!(0, block.instructions.len());
    assert_eq!(0, block.constants.len());
}

#[test]
fn op_code_can_be_pushed_to_an_block() {
    let mut block = Block::new("jenny");
    block.write_op_code(OpCode::Return, 123);

    assert_eq!(1, block.instructions.len());
    assert_eq!(OpCode::Return as u8, block.instructions[0]);
}

#[test]
fn can_write_more_then_256_constants() {
    let mut block = Block::new("maggie");
    for i in 0..258 {
        block.write_constant(i as f64, i);
    }

    assert_eq!(2 * 256 + 6, block.instructions.len());
    assert_eq!(
        OpCode::Constant2,
        OpCode::from_u8(block.instructions[2 * 256]).unwrap()
    );

    assert_eq!(256, block.read_u16(2 * 256 + 1));

    assert_eq!(
        OpCode::Constant2,
        OpCode::from_u8(block.instructions[2 * 256 + 3]).unwrap()
    );
    let constant_index = block.read_u16(2 * 256 + 4) as usize;
    assert_eq!(257, constant_index);
    assert_eq!(257f64, block.constants.read_value(constant_index));
}

#[test]
fn can_write_u8() {
    let mut block = Block::new("ruth");
    block.write_u8(123);
    assert_eq!(123, block.read_u8(0));
}

#[test]
fn can_write_u16() {
    let mut block = Block::new("ruth");
    block.write_u16(12345);
    assert_eq!(12345, block.read_u16(0));
}

#[test]
fn can_write_u32() {
    let mut block = Block::new("ruth");
    block.write_u32(12345678);
    assert_eq!(12345678, block.read_u32(0));
}
