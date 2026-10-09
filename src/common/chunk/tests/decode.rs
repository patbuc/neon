use crate::common::chunk::Instr;
use crate::common::opcodes::OpCode;
use crate::common::{Chunk, Value};
use std::rc::Rc;

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
fn get_local_a_number_constant_add_and_store_local_of_the_same_slot_decode_to_increment_local() {
    let pool = [Value::Int(1), Value::Number(2.5)];
    for value in pool {
        let mut chunk = Chunk::new("increment local");
        chunk.write_indexed(OpCode::GetLocal, 3, 1, 1);
        let constant = chunk.write_constant(value, 1, 1) as u16;
        chunk.write_op_code(OpCode::Add, 2, 1);
        chunk.write_indexed(OpCode::SetLocal, 3, 3, 1);
        chunk.write_op_code(OpCode::Pop, 3, 1);
        chunk.write_op_code(OpCode::Return, 4, 1);

        chunk.decode();

        assert_eq!(
            vec![Instr::IncrementLocal { slot: 3, constant }, Instr::Return],
            chunk.code
        );
        assert_eq!(2, chunk.instr_lines.len());
        assert_eq!(2, chunk.instr_line_info(0).unwrap().line);
        assert_eq!(4, chunk.instr_line_info(1).unwrap().line);
    }
}

#[test]
fn a_store_into_a_different_slot_decodes_without_increment_local() {
    let mut chunk = Chunk::new("store elsewhere");
    chunk.write_indexed(OpCode::GetLocal, 3, 1, 1);
    let constant = chunk.write_constant(Value::Int(1), 1, 1) as u16;
    chunk.write_op_code(OpCode::Add, 2, 1);
    chunk.write_indexed(OpCode::SetLocal, 4, 3, 1);
    chunk.write_op_code(OpCode::Pop, 3, 1);
    chunk.write_op_code(OpCode::Return, 4, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::GetLocalAddConstant { slot: 3, constant },
            Instr::StoreLocal(4),
            Instr::Return
        ],
        chunk.code
    );
}

#[test]
fn an_assignment_used_as_an_expression_decodes_without_increment_local() {
    let mut chunk = Chunk::new("assignment expression");
    chunk.write_indexed(OpCode::GetLocal, 3, 1, 1);
    let constant = chunk.write_constant(Value::Int(1), 1, 1) as u16;
    chunk.write_op_code(OpCode::Add, 2, 1);
    chunk.write_indexed(OpCode::SetLocal, 3, 3, 1);
    chunk.write_op_code(OpCode::Return, 4, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::GetLocalAddConstant { slot: 3, constant },
            Instr::SetLocal(3),
            Instr::Return
        ],
        chunk.code
    );
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

#[cfg(feature = "opcode-stats")]
fn variant_index(instr: Instr) -> usize {
    match instr {
        Instr::Return => 0,
        Instr::Constant(_) => 1,
        Instr::Negate => 2,
        Instr::Add => 3,
        Instr::Subtract => 4,
        Instr::Multiply => 5,
        Instr::Divide => 6,
        Instr::Modulo => 7,
        Instr::Exponent => 8,
        Instr::Nil => 9,
        Instr::True => 10,
        Instr::False => 11,
        Instr::Equal => 12,
        Instr::Greater => 13,
        Instr::GreaterEqual => 14,
        Instr::Less => 15,
        Instr::LessEqual => 16,
        Instr::Not => 17,
        Instr::Pop => 18,
        Instr::SetLocal(_) => 19,
        Instr::GetLocal(_) => 20,
        Instr::JumpIfFalse(_) => 21,
        Instr::PopJumpIfFalse(_) => 22,
        Instr::GreaterJumpIfFalse(_) => 23,
        Instr::GreaterEqualJumpIfFalse(_) => 24,
        Instr::LessJumpIfFalse(_) => 25,
        Instr::LessEqualJumpIfFalse(_) => 26,
        Instr::GreaterConstantJumpIfFalse { .. } => 27,
        Instr::GreaterEqualConstantJumpIfFalse { .. } => 28,
        Instr::LessConstantJumpIfFalse { .. } => 29,
        Instr::LessEqualConstantJumpIfFalse { .. } => 30,
        Instr::Jump(_) => 31,
        Instr::Loop(_) => 32,
        Instr::Call(_) => 33,
        Instr::Invoke { .. } => 34,
        Instr::GetBuiltin(_) => 35,
        Instr::GetGlobal(_) => 36,
        Instr::SetGlobal(_) => 37,
        Instr::GetField(_) => 38,
        Instr::SetField(_) => 39,
        Instr::GetLocalField { .. } => 40,
        Instr::CreateMap(_) => 41,
        Instr::CreateArray(_) => 42,
        Instr::CreateSet(_) => 43,
        Instr::GetIndex => 44,
        Instr::SetIndex => 45,
        Instr::GetIterator { .. } => 46,
        Instr::IteratorNext(_) => 47,
        Instr::IteratorDone(_) => 48,
        Instr::CreateRange { .. } => 49,
        Instr::ToString => 50,
        Instr::BitwiseAnd => 51,
        Instr::BitwiseOr => 52,
        Instr::BitwiseXor => 53,
        Instr::BitwiseNot => 54,
        Instr::LeftShift => 55,
        Instr::RightShift => 56,
        Instr::Closure { .. } => 57,
        Instr::GetUpvalue(_) => 58,
        Instr::SetUpvalue(_) => 59,
        Instr::CloseUpvalue => 60,
        Instr::DefineMethod { .. } => 61,
        Instr::DefineBuiltinMethod { .. } => 62,
        Instr::CheckInitialized => 63,
        Instr::CheckTuple(_) => 64,
        Instr::StoreLocal(_) => 65,
        Instr::StoreField(_) => 66,
        Instr::StoreLocalField { .. } => 67,
        Instr::AddConstant(_) => 68,
        Instr::SubtractConstant(_) => 69,
        Instr::GreaterConstant(_) => 70,
        Instr::GreaterEqualConstant(_) => 71,
        Instr::LessConstant(_) => 72,
        Instr::LessEqualConstant(_) => 73,
        Instr::ModuloConstant(_) => 74,
        Instr::MultiplyConstant(_) => 75,
        Instr::Dup => 76,
        Instr::Dup2 => 77,
        Instr::JumpIfNotNil(_) => 78,
        Instr::JumpIfNil(_) => 79,
        Instr::NoMatchArm => 80,
        Instr::EnumConstruct(_) => 81,
        Instr::IsArrayOfLen { .. } => 82,
        Instr::IsVariant(_) => 83,
        Instr::TailCall(_) => 84,
        Instr::TailInvoke { .. } => 85,
        Instr::IsNumber => 86,
        Instr::BeginTry(_) => 87,
        Instr::EndTry => 88,
        Instr::Throw => 89,
        Instr::Invalid(_) => 90,
        Instr::MultiplyLocal(_) => 91,
        Instr::AddLocal(_) => 92,
        Instr::SubtractLocal(_) => 93,
        Instr::DivideLocal(_) => 94,
        Instr::MultiplyLocalField { .. } => 95,
        Instr::AddLocalField { .. } => 96,
        Instr::SubtractLocalField { .. } => 97,
        Instr::DivideLocalField { .. } => 98,
        Instr::GetLocalAddConstant { .. } => 99,
        Instr::GetLocalSubtractConstant { .. } => 100,
        Instr::GetLocalMultiplyConstant { .. } => 101,
        Instr::GetLocalModuloConstant { .. } => 102,
        Instr::IncrementLocal { .. } => 103,
    }
}

#[cfg(feature = "opcode-stats")]
#[test]
fn name_matches_the_variant_name() {
    let samples = [
        Instr::Return,
        Instr::Constant(1),
        Instr::Negate,
        Instr::Add,
        Instr::Subtract,
        Instr::Multiply,
        Instr::Divide,
        Instr::Modulo,
        Instr::Exponent,
        Instr::Nil,
        Instr::True,
        Instr::False,
        Instr::Equal,
        Instr::Greater,
        Instr::GreaterEqual,
        Instr::Less,
        Instr::LessEqual,
        Instr::Not,
        Instr::Pop,
        Instr::SetLocal(1),
        Instr::GetLocal(1),
        Instr::JumpIfFalse(1),
        Instr::PopJumpIfFalse(1),
        Instr::GreaterJumpIfFalse(1),
        Instr::GreaterEqualJumpIfFalse(1),
        Instr::LessJumpIfFalse(1),
        Instr::LessEqualJumpIfFalse(1),
        Instr::GreaterConstantJumpIfFalse {
            constant: 1,
            target: 1,
        },
        Instr::GreaterEqualConstantJumpIfFalse {
            constant: 1,
            target: 1,
        },
        Instr::LessConstantJumpIfFalse {
            constant: 1,
            target: 1,
        },
        Instr::LessEqualConstantJumpIfFalse {
            constant: 1,
            target: 1,
        },
        Instr::Jump(1),
        Instr::Loop(1),
        Instr::Call(1),
        Instr::Invoke {
            method_symbol: 1,
            arg_count: 1,
        },
        Instr::GetBuiltin(1),
        Instr::GetGlobal(1),
        Instr::SetGlobal(1),
        Instr::GetField(1),
        Instr::SetField(1),
        Instr::GetLocalField { slot: 1, symbol: 1 },
        Instr::CreateMap(1),
        Instr::CreateArray(1),
        Instr::CreateSet(1),
        Instr::GetIndex,
        Instr::SetIndex,
        Instr::GetIterator { pairs: true },
        Instr::IteratorNext(1),
        Instr::IteratorDone(1),
        Instr::CreateRange { inclusive: true },
        Instr::ToString,
        Instr::BitwiseAnd,
        Instr::BitwiseOr,
        Instr::BitwiseXor,
        Instr::BitwiseNot,
        Instr::LeftShift,
        Instr::RightShift,
        Instr::Closure {
            const_index: 1,
            upvalue_count: 1,
            upvalues: 1,
        },
        Instr::GetUpvalue(1),
        Instr::SetUpvalue(1),
        Instr::CloseUpvalue,
        Instr::DefineMethod {
            type_symbol: 1,
            method_symbol: 1,
            takes_self: true,
        },
        Instr::DefineBuiltinMethod {
            type_symbol: 1,
            method_symbol: 1,
            takes_self: true,
        },
        Instr::CheckInitialized,
        Instr::CheckTuple(1),
        Instr::StoreLocal(1),
        Instr::StoreField(1),
        Instr::StoreLocalField { slot: 1, symbol: 1 },
        Instr::AddConstant(1),
        Instr::SubtractConstant(1),
        Instr::GreaterConstant(1),
        Instr::GreaterEqualConstant(1),
        Instr::LessConstant(1),
        Instr::LessEqualConstant(1),
        Instr::ModuloConstant(1),
        Instr::MultiplyConstant(1),
        Instr::Dup,
        Instr::Dup2,
        Instr::JumpIfNotNil(1),
        Instr::JumpIfNil(1),
        Instr::NoMatchArm,
        Instr::EnumConstruct(1),
        Instr::IsArrayOfLen {
            length: 1,
            at_least: true,
        },
        Instr::IsVariant(1),
        Instr::TailCall(1),
        Instr::TailInvoke {
            method_symbol: 1,
            arg_count: 1,
        },
        Instr::IsNumber,
        Instr::BeginTry(1),
        Instr::EndTry,
        Instr::Throw,
        Instr::Invalid(1),
        Instr::MultiplyLocal(1),
        Instr::AddLocal(1),
        Instr::SubtractLocal(1),
        Instr::DivideLocal(1),
        Instr::MultiplyLocalField { slot: 1, symbol: 1 },
        Instr::AddLocalField { slot: 1, symbol: 1 },
        Instr::SubtractLocalField { slot: 1, symbol: 1 },
        Instr::DivideLocalField { slot: 1, symbol: 1 },
        Instr::GetLocalAddConstant {
            slot: 1,
            constant: 1,
        },
        Instr::GetLocalSubtractConstant {
            slot: 1,
            constant: 1,
        },
        Instr::GetLocalMultiplyConstant {
            slot: 1,
            constant: 1,
        },
        Instr::GetLocalModuloConstant {
            slot: 1,
            constant: 1,
        },
        Instr::IncrementLocal {
            slot: 1,
            constant: 1,
        },
    ];
    let mut seen: Vec<usize> = samples.iter().map(|&instr| variant_index(instr)).collect();
    seen.sort_unstable();
    assert_eq!((0..104).collect::<Vec<_>>(), seen);

    for instr in samples {
        let debug = format!("{instr:?}");
        let variant = debug.split(['(', ' ']).next().unwrap();
        match instr {
            Instr::Invalid(_) => assert_eq!(None, instr.name()),
            _ => assert_eq!(Some(variant), instr.name()),
        }
    }
}

#[test]
fn get_local_followed_by_multiply_decodes_to_multiply_local() {
    let mut chunk = Chunk::new("fused multiply");
    chunk.write_indexed(OpCode::GetLocal, 3, 1, 1);
    chunk.write_op_code(OpCode::Multiply, 2, 1);
    chunk.write_op_code(OpCode::Return, 3, 1);

    chunk.decode();

    assert_eq!(vec![Instr::MultiplyLocal(3), Instr::Return], chunk.code);
    assert_eq!(2, chunk.instr_lines.len());
    assert_eq!(2, chunk.instr_line_info(0).unwrap().line);
    assert_eq!(3, chunk.instr_line_info(1).unwrap().line);
}

#[test]
fn get_local_followed_by_add_subtract_divide_decodes_to_local_variants() {
    let cases = [
        (OpCode::Add, Instr::AddLocal(3)),
        (OpCode::Subtract, Instr::SubtractLocal(3)),
        (OpCode::Divide, Instr::DivideLocal(3)),
    ];
    for (op, fused) in cases {
        let mut chunk = Chunk::new("fused arithmetic");
        chunk.write_indexed(OpCode::GetLocal, 3, 1, 1);
        chunk.write_op_code(op, 2, 1);
        chunk.write_op_code(OpCode::Return, 3, 1);

        chunk.decode();

        assert_eq!(vec![fused, Instr::Return], chunk.code);
        assert_eq!(2, chunk.instr_lines.len());
        assert_eq!(2, chunk.instr_line_info(0).unwrap().line);
        assert_eq!(3, chunk.instr_line_info(1).unwrap().line);
    }
}

#[test]
fn get_local_get_field_and_multiply_decode_to_multiply_local_field() {
    let mut chunk = Chunk::new("fused multiply field");
    chunk.write_indexed(OpCode::GetLocal, 3, 1, 1);
    chunk.write_indexed(OpCode::GetField, 9, 2, 1);
    chunk.write_op_code(OpCode::Multiply, 3, 1);
    chunk.write_op_code(OpCode::Return, 4, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::MultiplyLocalField { slot: 3, symbol: 9 },
            Instr::Return
        ],
        chunk.code
    );
    assert_eq!(2, chunk.instr_lines.len());
    assert_eq!(3, chunk.instr_line_info(0).unwrap().line);
    assert_eq!(4, chunk.instr_line_info(1).unwrap().line);
}

#[test]
fn get_local_get_field_and_add_subtract_divide_decode_to_local_field_variants() {
    let cases = [
        (OpCode::Add, Instr::AddLocalField { slot: 3, symbol: 9 }),
        (
            OpCode::Subtract,
            Instr::SubtractLocalField { slot: 3, symbol: 9 },
        ),
        (
            OpCode::Divide,
            Instr::DivideLocalField { slot: 3, symbol: 9 },
        ),
    ];
    for (op, fused) in cases {
        let mut chunk = Chunk::new("fused arithmetic field");
        chunk.write_indexed(OpCode::GetLocal, 3, 1, 1);
        chunk.write_indexed(OpCode::GetField, 9, 2, 1);
        chunk.write_op_code(op, 3, 1);
        chunk.write_op_code(OpCode::Return, 4, 1);

        chunk.decode();

        assert_eq!(vec![fused, Instr::Return], chunk.code);
        assert_eq!(2, chunk.instr_lines.len());
        assert_eq!(3, chunk.instr_line_info(0).unwrap().line);
        assert_eq!(4, chunk.instr_line_info(1).unwrap().line);
    }
}

#[test]
fn a_local_read_followed_by_check_initialized_decodes_unfused() {
    let mut chunk = Chunk::new("checked read");
    chunk.write_indexed(OpCode::GetLocal, 3, 1, 1);
    chunk.write_op_code(OpCode::CheckInitialized, 1, 1);
    chunk.write_op_code(OpCode::Multiply, 2, 1);
    chunk.write_op_code(OpCode::Return, 3, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::GetLocal(3),
            Instr::CheckInitialized,
            Instr::Multiply,
            Instr::Return
        ],
        chunk.code
    );
}

#[test]
fn a_number_constant_followed_by_add_decodes_to_add_constant() {
    let pool = [Value::Int(7), Value::Number(2.5)];
    for value in pool {
        let mut chunk = Chunk::new("fused add constant");
        chunk.write_op_code(OpCode::Nil, 1, 1);
        let index = chunk.write_constant(value, 2, 1) as u16;
        chunk.write_op_code(OpCode::Add, 3, 1);
        chunk.write_op_code(OpCode::Return, 4, 1);

        chunk.decode();

        assert_eq!(
            vec![Instr::Nil, Instr::AddConstant(index), Instr::Return],
            chunk.code
        );
        assert_eq!(3, chunk.instr_lines.len());
        assert_eq!(3, chunk.instr_line_info(1).unwrap().line);
        assert_eq!(4, chunk.instr_line_info(2).unwrap().line);
    }
}

#[test]
fn subtract_multiply_modulo_greater_greater_equal_less_and_less_equal_each_fuse_a_number_constant()
{
    let cases = [
        (OpCode::Subtract, Instr::SubtractConstant(0)),
        (OpCode::Multiply, Instr::MultiplyConstant(0)),
        (OpCode::Modulo, Instr::ModuloConstant(0)),
        (OpCode::Greater, Instr::GreaterConstant(0)),
        (OpCode::GreaterEqual, Instr::GreaterEqualConstant(0)),
        (OpCode::Less, Instr::LessConstant(0)),
        (OpCode::LessEqual, Instr::LessEqualConstant(0)),
    ];
    for (op, fused) in cases {
        let mut chunk = Chunk::new("fused constant operator");
        chunk.write_constant(Value::Int(7), 2, 1);
        chunk.write_op_code(op, 3, 1);
        chunk.write_op_code(OpCode::Return, 4, 1);

        chunk.decode();

        assert_eq!(vec![fused, Instr::Return], chunk.code);
        assert_eq!(2, chunk.instr_lines.len());
        assert_eq!(3, chunk.instr_line_info(0).unwrap().line);
        assert_eq!(4, chunk.instr_line_info(1).unwrap().line);
    }
}

#[test]
fn a_number_constant_left_of_subtract_decodes_without_a_constant_operand_instruction() {
    let mut chunk = Chunk::new("one minus x");
    let index = chunk.write_constant(Value::Int(1), 1, 1) as u16;
    chunk.write_indexed(OpCode::GetLocal, 0, 2, 1);
    chunk.write_op_code(OpCode::Subtract, 3, 1);
    chunk.write_op_code(OpCode::Return, 4, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::Constant(index),
            Instr::SubtractLocal(0),
            Instr::Return
        ],
        chunk.code
    );
}

#[test]
fn a_literal_left_operand_decodes_without_a_local_fusion() {
    let mut chunk = Chunk::new("five minus one");
    let five = chunk.write_constant(Value::Int(5), 1, 1) as u16;
    let one = chunk.write_constant(Value::Int(1), 2, 1) as u16;
    chunk.write_op_code(OpCode::Subtract, 3, 1);
    chunk.write_op_code(OpCode::Return, 4, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::Constant(five),
            Instr::SubtractConstant(one),
            Instr::Return
        ],
        chunk.code
    );
}

#[test]
fn a_string_constant_followed_by_add_decodes_unfused() {
    let mut chunk = Chunk::new("string add");
    let index = chunk.write_constant(Value::String(Rc::new("a".to_string())), 2, 1) as u16;
    chunk.write_op_code(OpCode::Add, 3, 1);
    chunk.write_op_code(OpCode::Return, 4, 1);

    chunk.decode();

    assert_eq!(
        vec![Instr::Constant(index), Instr::Add, Instr::Return],
        chunk.code
    );
}

#[test]
fn get_local_a_number_constant_and_subtract_decode_to_get_local_subtract_constant() {
    let pool = [Value::Int(7), Value::Number(2.5)];
    for value in pool {
        let mut chunk = Chunk::new("fused local subtract constant");
        chunk.write_indexed(OpCode::GetLocal, 3, 1, 1);
        let constant = chunk.write_constant(value, 2, 1) as u16;
        chunk.write_op_code(OpCode::Subtract, 3, 1);
        chunk.write_op_code(OpCode::Return, 4, 1);

        chunk.decode();

        assert_eq!(
            vec![
                Instr::GetLocalSubtractConstant { slot: 3, constant },
                Instr::Return
            ],
            chunk.code
        );
        assert_eq!(2, chunk.instr_lines.len());
        assert_eq!(3, chunk.instr_line_info(0).unwrap().line);
        assert_eq!(4, chunk.instr_line_info(1).unwrap().line);
    }
}

#[test]
fn add_subtract_multiply_and_modulo_each_fuse_a_local_left_operand_with_a_number_constant() {
    let cases = [
        (
            OpCode::Add,
            Instr::GetLocalAddConstant {
                slot: 2,
                constant: 0,
            },
        ),
        (
            OpCode::Subtract,
            Instr::GetLocalSubtractConstant {
                slot: 2,
                constant: 0,
            },
        ),
        (
            OpCode::Multiply,
            Instr::GetLocalMultiplyConstant {
                slot: 2,
                constant: 0,
            },
        ),
        (
            OpCode::Modulo,
            Instr::GetLocalModuloConstant {
                slot: 2,
                constant: 0,
            },
        ),
    ];
    for (op, fused) in cases {
        let mut chunk = Chunk::new("fused local constant operator");
        chunk.write_indexed(OpCode::GetLocal, 2, 1, 1);
        chunk.write_constant(Value::Int(7), 2, 1);
        chunk.write_op_code(op, 3, 1);
        chunk.write_op_code(OpCode::Return, 4, 1);

        chunk.decode();

        assert_eq!(vec![fused, Instr::Return], chunk.code);
        assert_eq!(2, chunk.instr_lines.len());
        assert_eq!(3, chunk.instr_line_info(0).unwrap().line);
    }
}

#[test]
fn a_local_constant_operator_run_with_a_jump_target_inside_it_decodes_unfused() {
    let mut chunk = Chunk::new("jump to constant");
    let jump = chunk.emit_jump(OpCode::Jump, 1, 1);
    chunk.write_indexed(OpCode::GetLocal, 3, 2, 1);
    chunk.patch_jump(jump);
    let constant = chunk.write_constant(Value::Int(7), 3, 1) as u16;
    chunk.write_op_code(OpCode::Subtract, 4, 1);
    chunk.write_op_code(OpCode::Return, 5, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::Jump(2),
            Instr::GetLocal(3),
            Instr::SubtractConstant(constant),
            Instr::Return
        ],
        chunk.code
    );

    let mut chunk = Chunk::new("jump to operator");
    let jump = chunk.emit_jump(OpCode::Jump, 1, 1);
    chunk.write_indexed(OpCode::GetLocal, 3, 2, 1);
    let constant = chunk.write_constant(Value::Int(7), 3, 1) as u16;
    chunk.patch_jump(jump);
    chunk.write_op_code(OpCode::Subtract, 4, 1);
    chunk.write_op_code(OpCode::Return, 5, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::Jump(3),
            Instr::GetLocal(3),
            Instr::Constant(constant),
            Instr::Subtract,
            Instr::Return
        ],
        chunk.code
    );
}

#[test]
fn less_followed_by_pop_jump_if_false_decodes_to_less_jump_if_false() {
    let mut chunk = Chunk::new("less jump");
    chunk.write_indexed(OpCode::GetLocal, 1, 1, 1);
    chunk.write_indexed(OpCode::GetLocal, 2, 1, 5);
    chunk.write_op_code(OpCode::Less, 2, 3);
    let jump = chunk.emit_jump(OpCode::PopJumpIfFalse, 3, 1);
    chunk.write_op_code(OpCode::Nil, 4, 1);
    chunk.write_indexed(OpCode::SetLocal, 3, 5, 1);
    chunk.write_op_code(OpCode::Pop, 5, 1);
    chunk.patch_jump(jump);
    chunk.write_op_code(OpCode::Return, 6, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::GetLocal(1),
            Instr::GetLocal(2),
            Instr::LessJumpIfFalse(5),
            Instr::Nil,
            Instr::StoreLocal(3),
            Instr::Return,
        ],
        chunk.code
    );
    assert_eq!(6, chunk.instr_lines.len());
    assert_eq!(2, chunk.instr_line_info(2).unwrap().line);
    assert_eq!(3, chunk.instr_line_info(2).unwrap().column);
}

#[test]
fn greater_greater_equal_and_less_equal_each_fuse_into_their_jump() {
    let table = [
        (OpCode::Greater, Instr::GreaterJumpIfFalse(3)),
        (OpCode::GreaterEqual, Instr::GreaterEqualJumpIfFalse(3)),
        (OpCode::LessEqual, Instr::LessEqualJumpIfFalse(3)),
    ];
    for (compare, fused) in table {
        let mut chunk = Chunk::new("compare jump");
        chunk.write_indexed(OpCode::GetLocal, 1, 1, 1);
        chunk.write_indexed(OpCode::GetLocal, 2, 1, 5);
        chunk.write_op_code(compare, 2, 3);
        let jump = chunk.emit_jump(OpCode::PopJumpIfFalse, 3, 1);
        chunk.patch_jump(jump);
        chunk.write_op_code(OpCode::Return, 4, 1);

        chunk.decode();

        assert_eq!(
            vec![Instr::GetLocal(1), Instr::GetLocal(2), fused, Instr::Return],
            chunk.code
        );
        assert_eq!(2, chunk.instr_line_info(2).unwrap().line);
    }
}

#[test]
fn a_number_constant_compare_and_pop_jump_if_false_decode_to_constant_jump_if_false() {
    type Fused = fn(u16, u32) -> Instr;
    let table: [(OpCode, Fused); 4] = [
        (OpCode::Greater, |constant, target| {
            Instr::GreaterConstantJumpIfFalse { constant, target }
        }),
        (OpCode::GreaterEqual, |constant, target| {
            Instr::GreaterEqualConstantJumpIfFalse { constant, target }
        }),
        (OpCode::Less, |constant, target| {
            Instr::LessConstantJumpIfFalse { constant, target }
        }),
        (OpCode::LessEqual, |constant, target| {
            Instr::LessEqualConstantJumpIfFalse { constant, target }
        }),
    ];
    for (compare, fused) in table {
        for value in [Value::Int(10), Value::Number(2.5)] {
            let mut chunk = Chunk::new("constant compare jump");
            chunk.write_indexed(OpCode::GetLocal, 1, 1, 1);
            let constant = chunk.write_constant(value, 1, 5) as u16;
            chunk.write_op_code(compare, 2, 3);
            let jump = chunk.emit_jump(OpCode::PopJumpIfFalse, 3, 1);
            chunk.write_op_code(OpCode::Nil, 4, 1);
            chunk.patch_jump(jump);
            chunk.write_op_code(OpCode::Return, 5, 1);

            chunk.decode();

            assert_eq!(
                vec![
                    Instr::GetLocal(1),
                    fused(constant, 3),
                    Instr::Nil,
                    Instr::Return,
                ],
                chunk.code,
                "{compare:?}"
            );
            assert_eq!(2, chunk.instr_line_info(1).unwrap().line);
            assert_eq!(3, chunk.instr_line_info(1).unwrap().column);
        }
    }
}

#[test]
fn a_pop_jump_if_false_that_is_a_jump_target_keeps_its_comparison_unfused() {
    let mut chunk = Chunk::new("shared jump target");
    chunk.write_indexed(OpCode::GetLocal, 1, 1, 1);
    let skip = chunk.emit_jump(OpCode::JumpIfFalse, 1, 3);
    chunk.write_op_code(OpCode::Pop, 1, 4);
    chunk.write_indexed(OpCode::GetLocal, 2, 1, 6);
    chunk.write_indexed(OpCode::GetLocal, 3, 1, 10);
    chunk.write_op_code(OpCode::Less, 1, 8);
    chunk.patch_jump(skip);
    let jump = chunk.emit_jump(OpCode::PopJumpIfFalse, 1, 12);
    chunk.write_op_code(OpCode::Nil, 2, 1);
    chunk.patch_jump(jump);
    chunk.write_op_code(OpCode::Return, 3, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::GetLocal(1),
            Instr::JumpIfFalse(6),
            Instr::Pop,
            Instr::GetLocal(2),
            Instr::GetLocal(3),
            Instr::Less,
            Instr::PopJumpIfFalse(8),
            Instr::Nil,
            Instr::Return,
        ],
        chunk.code
    );
}

#[test]
fn a_loop_back_to_the_start_of_a_fused_compare_jump_resolves() {
    let mut chunk = Chunk::new("loop to compare");
    chunk.write_indexed(OpCode::GetLocal, 1, 1, 1);
    let start = chunk.instruction_count() as u32;
    let constant = chunk.write_constant(Value::Int(10), 2, 1) as u16;
    chunk.write_op_code(OpCode::Less, 2, 3);
    let jump = chunk.emit_jump(OpCode::PopJumpIfFalse, 2, 5);
    chunk.emit_loop(start, 3, 1);
    chunk.patch_jump(jump);
    chunk.write_op_code(OpCode::Return, 4, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::GetLocal(1),
            Instr::LessConstantJumpIfFalse {
                constant,
                target: 3
            },
            Instr::Loop(1),
            Instr::Return,
        ],
        chunk.code
    );
}

#[test]
fn a_comparison_used_as_a_value_decodes_unfused() {
    let mut chunk = Chunk::new("compare value");
    chunk.write_indexed(OpCode::GetLocal, 1, 1, 1);
    chunk.write_indexed(OpCode::GetLocal, 2, 1, 5);
    chunk.write_op_code(OpCode::Less, 2, 3);
    chunk.write_indexed(OpCode::SetLocal, 3, 2, 8);
    chunk.write_op_code(OpCode::Return, 4, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::GetLocal(1),
            Instr::GetLocal(2),
            Instr::Less,
            Instr::SetLocal(3),
            Instr::Return,
        ],
        chunk.code
    );
}

#[test]
fn a_comparison_followed_by_jump_if_false_decodes_unfused() {
    let mut chunk = Chunk::new("compare jump if false");
    chunk.write_indexed(OpCode::GetLocal, 1, 1, 1);
    chunk.write_indexed(OpCode::GetLocal, 2, 1, 5);
    chunk.write_op_code(OpCode::Less, 2, 3);
    let jump = chunk.emit_jump(OpCode::JumpIfFalse, 3, 1);
    chunk.patch_jump(jump);
    chunk.write_op_code(OpCode::Return, 4, 1);

    chunk.decode();

    assert_eq!(
        vec![
            Instr::GetLocal(1),
            Instr::GetLocal(2),
            Instr::Less,
            Instr::JumpIfFalse(4),
            Instr::Return,
        ],
        chunk.code
    );
}
