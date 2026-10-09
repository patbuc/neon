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

#[test]
fn pop_jump_if_false_decodes_to_instruction_index() {
    let mut chunk = Chunk::new("pop jump");
    chunk.write_op_code(OpCode::Nil, 1, 1);
    let jump = chunk.emit_jump(OpCode::PopJumpIfFalse, 2, 1);
    chunk.write_op_code(OpCode::Nil, 3, 1);
    chunk.patch_jump(jump);
    chunk.write_op_code(OpCode::Return, 4, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::Nil,
            Instr::PopJumpIfFalse(3),
            Instr::Nil,
            Instr::Return,
        ],
        chunk.code
    );
}

#[test]
fn constant_compare_jump_decodes_constant_and_target() {
    let mut chunk = Chunk::new("compare jump");
    chunk.write_op_code(OpCode::Nil, 1, 1);
    let jump = chunk.emit_constant_jump(OpCode::LessConstantJumpIfFalse, 9, 2, 1);
    chunk.write_op_code(OpCode::Nil, 3, 1);
    chunk.patch_jump(jump);
    chunk.write_op_code(OpCode::Return, 4, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::Nil,
            Instr::LessConstantJumpIfFalse {
                constant: 9,
                target: 3,
            },
            Instr::Nil,
            Instr::Return,
        ],
        chunk.code
    );
}

#[test]
fn closure_upvalues_decode_into_side_table() {
    let mut chunk = Chunk::new("closure");
    chunk.write_indexed(OpCode::Closure, 3, 1, 1);
    chunk.write_u8(2);
    chunk.write_u8(1);
    chunk.write_u16(4);
    chunk.write_u8(0);
    chunk.write_u16(9);
    chunk.write_op_code(OpCode::Return, 2, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::Closure {
                const_index: 3,
                upvalue_count: 2,
                upvalues: 0,
            },
            Instr::Return,
        ],
        chunk.code
    );
    assert_eq!(vec![(true, 4), (false, 9)], chunk.closure_upvalues);
}

#[test]
fn unknown_opcode_decodes_to_invalid() {
    let mut chunk = Chunk::new("invalid");
    chunk.write_op_code(OpCode::Nil, 1, 1);
    chunk.instructions[0] = 0xFF;
    chunk.write_op_code(OpCode::Return, 2, 1);

    chunk.decode();

    assert_eq!(vec![Instr::Invalid(0xFF)], chunk.code);
}

#[test]
fn truncated_operand_decodes_to_invalid() {
    let mut chunk = Chunk::new("truncated");
    chunk.write_op_code(OpCode::Nil, 1, 1);
    chunk.write_indexed(OpCode::Closure, 3, 2, 1);
    chunk.write_u8(2);
    chunk.write_u8(1);
    chunk.write_u16(4);

    chunk.decode();

    assert_eq!(
        vec![Instr::Nil, Instr::Invalid(OpCode::Closure as u8)],
        chunk.code
    );
    assert!(chunk.closure_upvalues.is_empty());
}

#[test]
fn jump_to_invalid_byte_targets_invalid() {
    let mut chunk = Chunk::new("jump_to_invalid");
    let jump = chunk.emit_jump(OpCode::Jump, 1, 1);
    chunk.write_op_code(OpCode::Nil, 2, 1);
    chunk.patch_jump(jump);
    chunk.write_u8(0xFF);

    chunk.decode();

    assert_eq!(
        vec![Instr::Jump(2), Instr::Nil, Instr::Invalid(0xFF)],
        chunk.code
    );
}

#[test]
fn jump_into_an_operand_decodes_to_invalid() {
    let mut chunk = Chunk::new("jump_into_operand");
    chunk.write_op_code(OpCode::Jump, 1, 1);
    chunk.write_u32(1);
    chunk.write_indexed(OpCode::GetLocal, 7, 2, 1);
    chunk.write_op_code(OpCode::Return, 3, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::Invalid(OpCode::Jump as u8),
            Instr::GetLocal(7),
            Instr::Return,
        ],
        chunk.code
    );
}

#[test]
fn get_local_followed_by_get_field_decodes_to_one_get_local_field() {
    let mut chunk = Chunk::new("fused field");
    chunk.write_indexed(OpCode::GetLocal, 3, 1, 1);
    chunk.write_indexed(OpCode::GetField, 9, 2, 1);
    chunk.write_op_code(OpCode::Return, 3, 1);

    chunk.decode();

    assert_eq!(
        vec![Instr::GetLocalField { slot: 3, symbol: 9 }, Instr::Return],
        chunk.code
    );
    assert_eq!(2, chunk.instr_lines.len());
    assert_eq!(2, chunk.instr_line_info(0).unwrap().line);
    assert_eq!(3, chunk.instr_line_info(1).unwrap().line);
}

#[test]
fn set_local_followed_by_pop_decodes_to_one_store_local() {
    let mut chunk = Chunk::new("fused store");
    chunk.write_indexed(OpCode::SetLocal, 3, 1, 1);
    chunk.write_op_code(OpCode::Pop, 2, 1);
    chunk.write_op_code(OpCode::Return, 3, 1);

    chunk.decode();

    assert_eq!(vec![Instr::StoreLocal(3), Instr::Return], chunk.code);
    assert_eq!(2, chunk.instr_lines.len());
    assert_eq!(1, chunk.instr_line_info(0).unwrap().line);
    assert_eq!(3, chunk.instr_line_info(1).unwrap().line);
}

#[test]
fn set_field_followed_by_pop_decodes_to_one_store_field() {
    let mut chunk = Chunk::new("fused field store");
    chunk.write_indexed(OpCode::SetField, 4, 1, 1);
    chunk.write_op_code(OpCode::Pop, 2, 1);
    chunk.write_op_code(OpCode::Return, 3, 1);

    chunk.decode();

    assert_eq!(vec![Instr::StoreField(4), Instr::Return], chunk.code);
    assert_eq!(2, chunk.instr_lines.len());
    assert_eq!(1, chunk.instr_line_info(0).unwrap().line);
    assert_eq!(3, chunk.instr_line_info(1).unwrap().line);
}

#[test]
fn a_pair_whose_second_instruction_is_a_jump_target_decodes_unfused() {
    let mut chunk = Chunk::new("jump into pair");
    let jump = chunk.emit_jump(OpCode::Jump, 1, 1);
    chunk.write_indexed(OpCode::SetLocal, 3, 2, 1);
    chunk.patch_jump(jump);
    chunk.write_op_code(OpCode::Pop, 3, 1);
    chunk.write_op_code(OpCode::Return, 4, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::Jump(2),
            Instr::SetLocal(3),
            Instr::Pop,
            Instr::Return
        ],
        chunk.code
    );
}
