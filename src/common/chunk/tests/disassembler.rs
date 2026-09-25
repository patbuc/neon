use crate::common::opcodes::OpCode;
use crate::common::Chunk;
use crate::string;

#[test]
fn create_map_instruction_round_trips_counts_above_255() {
    let mut chunk = Chunk::new("origin");
    chunk.write_op_code(OpCode::CreateMap, 1, 1);
    chunk.write_u16(300);

    let next_offset = chunk.disassemble_instruction(0);

    assert_eq!(300, chunk.read_u16(1));
    assert_eq!(3, next_offset);
}

#[test]
fn create_set_instruction_round_trips_counts_above_255() {
    let mut chunk = Chunk::new("origin");
    chunk.write_op_code(OpCode::CreateSet, 1, 1);
    chunk.write_u16(300);

    let next_offset = chunk.disassemble_instruction(0);

    assert_eq!(300, chunk.read_u16(1));
    assert_eq!(3, next_offset);
}

#[test]
fn invoke_instruction_next_offset_is_four() {
    let mut chunk = Chunk::new("origin");
    let name_index = chunk.add_string(string!("push")) as u16;
    chunk.write_op_code(OpCode::Invoke, 1, 1);
    chunk.write_u16(name_index);
    chunk.write_u8(2);

    let next_offset = chunk.disassemble_instruction(0);

    assert_eq!(4, next_offset);
}
