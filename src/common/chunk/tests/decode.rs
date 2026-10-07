use crate::common::chunk::Instr;
use crate::common::opcodes::OpCode;
use crate::common::Chunk;

#[test]
fn instr_fits_in_eight_bytes() {
    assert!(std::mem::size_of::<Instr>() <= 8);
}

#[test]
fn jumps_decode_to_instruction_indices() {
    let mut chunk = Chunk::new("jumps");
    chunk.write_op_code(OpCode::Nil, 1, 1);
    let jump = chunk.emit_jump(OpCode::JumpIfFalse, 2, 1);
    chunk.write_indexed(OpCode::GetLocal, 7, 3, 1);
    chunk.patch_jump(jump);
    chunk.emit_loop(0, 4, 1);
    chunk.write_op_code(OpCode::Return, 5, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::Nil,
            Instr::JumpIfFalse(3),
            Instr::GetLocal(7),
            Instr::Loop(0),
            Instr::Return,
        ],
        chunk.code
    );
    let lines: Vec<u32> = (0..5)
        .map(|i| chunk.instr_line_info(i).unwrap().line)
        .collect();
    assert_eq!(vec![1, 2, 3, 4, 5], lines);
    assert!(chunk.instr_line_info(5).is_none());
}
