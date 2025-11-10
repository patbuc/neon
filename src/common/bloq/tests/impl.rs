use crate::common::opcodes::OpCode;
use crate::common::Bloq;
use crate::{as_number, number};

#[test]
fn new_bloq_is_empty() {
    let bloq = Bloq::new("origin");

    assert_eq!("origin", bloq.name);
    assert_eq!(0, bloq.instructions.len());
    assert_eq!(0, bloq.constants.len());
}

#[test]
fn op_code_can_be_pushed_to_a_bloq() {
    let mut bloq = Bloq::new("jenny");
    bloq.write_op_code(OpCode::Return, 123, 42);

    assert_eq!(1, bloq.instructions.len());
    assert_eq!(OpCode::Return as u8, bloq.instructions[0]);
}

#[test]
fn can_write_more_then_256_constants_bloq() {
    let mut bloq = Bloq::new("maggie");
    for i in 0..258 {
        bloq.write_constant(number!(i as f64), i, 42);
    }

    assert_eq!(2 * 256 + 6, bloq.instructions.len());
    assert_eq!(
        OpCode::Constant2,
        OpCode::from_u8(bloq.instructions[2 * 256])
    );

    assert_eq!(256, bloq.read_u16(2 * 256 + 1));
    assert_eq!(
        OpCode::Constant2,
        OpCode::from_u8(bloq.instructions[2 * 256 + 3])
    );
    let constant_index = bloq.read_u16(2 * 256 + 4) as usize;
    assert_eq!(257, constant_index);
    assert_eq!(
        257f64,
        as_number!(bloq.constants.read_value(constant_index))
    );
}

#[test]
fn can_write_u8_bloq() {
    let mut bloq = Bloq::new("ruth");
    bloq.write_u8(123);
    assert_eq!(123, bloq.read_u8(0));
}

#[test]
fn can_write_u16_bloq() {
    let mut bloq = Bloq::new("ruth");
    bloq.write_u16(12345);
    assert_eq!(12345, bloq.read_u16(0));
}

#[test]
fn can_write_u32_bloq() {
    let mut bloq = Bloq::new("ruth");
    bloq.write_u32(12345678);
    assert_eq!(12345678, bloq.read_u32(0));
}

#[test]
fn can_write_bloq() {
    let mut bloq = Bloq::new("Zebloq");

    bloq.write_constant(number!(1234.56), 2, 0);
    bloq.write_op_code(OpCode::Negate, 3, 0);
    bloq.write_constant(number!(345.67), 4, 0);
    bloq.write_op_code(OpCode::Add, 4, 0);
    bloq.write_constant(number!(1.2), 5, 0);
    bloq.write_op_code(OpCode::Multiply, 6, 0);
    bloq.write_op_code(OpCode::Return, 8, 0);
}

#[test]
fn can_read_line_information_bloq() {
    let mut bloq = Bloq::new("Zebloq");

    bloq.write_constant(number!(1234.56), 2, 0);
    bloq.write_op_code(OpCode::Negate, 3, 0);
    bloq.write_constant(number!(345.67), 4, 0);
    bloq.write_op_code(OpCode::Add, 4, 0);
    bloq.write_constant(number!(1.2), 5, 0);
    bloq.write_op_code(OpCode::Multiply, 6, 0);
    bloq.write_op_code(OpCode::Return, 8, 0);

    assert_eq!(2, bloq.get_source_location(0).unwrap().line);
    assert_eq!(2, bloq.get_source_location(1).unwrap().line);
    assert_eq!(3, bloq.get_source_location(2).unwrap().line);
    assert_eq!(4, bloq.get_source_location(3).unwrap().line);
    assert_eq!(4, bloq.get_source_location(4).unwrap().line);
    assert_eq!(4, bloq.get_source_location(5).unwrap().line);
    assert_eq!(5, bloq.get_source_location(6).unwrap().line);
    assert_eq!(5, bloq.get_source_location(7).unwrap().line);
    assert_eq!(6, bloq.get_source_location(8).unwrap().line);
    assert_eq!(8, bloq.get_source_location(9).unwrap().line);

    assert!(bloq.get_source_location(10).is_none());
}
