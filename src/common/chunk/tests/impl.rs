use crate::common::opcodes::OpCode;
use crate::common::Chunk;
use crate::{as_number, number};

#[test]
fn new_chunk_is_empty() {
    let chunk = Chunk::new("origin");

    assert_eq!("origin", chunk.name);
    assert_eq!(0, chunk.instructions.len());
    assert_eq!(0, chunk.constants.len());
}

#[test]
fn op_code_can_be_pushed_to_a_chunk() {
    let mut chunk = Chunk::new("jenny");
    chunk.write_op_code(OpCode::Return, 123, 42);

    assert_eq!(1, chunk.instructions.len());
    assert_eq!(OpCode::Return as u8, chunk.instructions[0]);
}

#[test]
fn can_write_more_then_256_constants_chunk() {
    let mut chunk = Chunk::new("maggie");
    for i in 0..258 {
        chunk.write_constant(number!(i as f64), i, 42);
    }

    assert_eq!(2 * 256 + 6, chunk.instructions.len());
    assert_eq!(
        OpCode::Constant2,
        OpCode::from_u8(chunk.instructions[2 * 256])
    );

    assert_eq!(256, chunk.read_u16(2 * 256 + 1));
    assert_eq!(
        OpCode::Constant2,
        OpCode::from_u8(chunk.instructions[2 * 256 + 3])
    );
    let constant_index = chunk.read_u16(2 * 256 + 4) as usize;
    assert_eq!(257, constant_index);
    assert_eq!(
        257f64,
        as_number!(chunk.constants.read_value(constant_index))
    );
}

#[test]
fn can_write_u8_chunk() {
    let mut chunk = Chunk::new("ruth");
    chunk.write_u8(123);
    assert_eq!(123, chunk.read_u8(0));
}

#[test]
fn can_write_u16_chunk() {
    let mut chunk = Chunk::new("ruth");
    chunk.write_u16(12345);
    assert_eq!(12345, chunk.read_u16(0));
}

#[test]
fn can_write_u32_chunk() {
    let mut chunk = Chunk::new("ruth");
    chunk.write_u32(12345678);
    assert_eq!(12345678, chunk.read_u32(0));
}

#[test]
fn can_write_chunk() {
    let mut chunk = Chunk::new("Zechunk");

    chunk.write_constant(number!(1234.56), 2, 0);
    chunk.write_op_code(OpCode::Negate, 3, 0);
    chunk.write_constant(number!(345.67), 4, 0);
    chunk.write_op_code(OpCode::Add, 4, 0);
    chunk.write_constant(number!(1.2), 5, 0);
    chunk.write_op_code(OpCode::Multiply, 6, 0);
    chunk.write_op_code(OpCode::Return, 8, 0);
}

#[test]
fn can_read_line_information_chunk() {
    let mut chunk = Chunk::new("Zechunk");

    chunk.write_constant(number!(1234.56), 2, 0);
    chunk.write_op_code(OpCode::Negate, 3, 0);
    chunk.write_constant(number!(345.67), 4, 0);
    chunk.write_op_code(OpCode::Add, 4, 0);
    chunk.write_constant(number!(1.2), 5, 0);
    chunk.write_op_code(OpCode::Multiply, 6, 0);
    chunk.write_op_code(OpCode::Return, 8, 0);

    assert_eq!(2, chunk.get_source_location(0).unwrap().line);
    assert_eq!(2, chunk.get_source_location(1).unwrap().line);
    assert_eq!(3, chunk.get_source_location(2).unwrap().line);
    assert_eq!(4, chunk.get_source_location(3).unwrap().line);
    assert_eq!(4, chunk.get_source_location(4).unwrap().line);
    assert_eq!(4, chunk.get_source_location(5).unwrap().line);
    assert_eq!(5, chunk.get_source_location(6).unwrap().line);
    assert_eq!(5, chunk.get_source_location(7).unwrap().line);
    assert_eq!(6, chunk.get_source_location(8).unwrap().line);
    assert_eq!(8, chunk.get_source_location(9).unwrap().line);

    assert!(chunk.get_source_location(10).is_none());
}
