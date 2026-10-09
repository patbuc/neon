use crate::common::opcodes::OpCode;
use crate::common::Chunk;
use std::rc::Rc;

#[test]
fn create_map_instruction_round_trips_counts_above_255() {
    let mut chunk = Chunk::new("origin");
    chunk.write_op_code(OpCode::CreateMap, 1, 1);
    chunk.write_u16(300);

    let mut out = String::new();
    let next_offset = chunk.disassemble_instruction(0, &mut out);

    assert_eq!(300, chunk.read_u16(1));
    assert_eq!(3, next_offset);
}

#[test]
fn create_set_instruction_round_trips_counts_above_255() {
    let mut chunk = Chunk::new("origin");
    chunk.write_op_code(OpCode::CreateSet, 1, 1);
    chunk.write_u16(300);

    let mut out = String::new();
    let next_offset = chunk.disassemble_instruction(0, &mut out);

    assert_eq!(300, chunk.read_u16(1));
    assert_eq!(3, next_offset);
}

#[test]
fn tail_call_instruction_prints_the_argument_count() {
    let mut chunk = Chunk::new("origin");
    chunk.write_op_code(OpCode::TailCall, 1, 1);
    chunk.write_u8(2);

    let mut out = String::new();
    let next_offset = chunk.disassemble_instruction(0, &mut out);

    assert_eq!(out, "0000      1 TailCall (args: 2)\n");
    assert_eq!(2, next_offset);
}

#[test]
fn get_field_instruction_prints_the_symbol_name() {
    let mut chunk = Chunk::new("origin");
    chunk.symbols = Rc::from(vec![Rc::from("x"), Rc::from("value")]);
    chunk.write_op_code(OpCode::GetField, 1, 1);
    chunk.write_u16(1);

    let mut out = String::new();
    let next_offset = chunk.disassemble_instruction(0, &mut out);

    assert_eq!(out, "0000      1 GetField 01 'value'\n");
    assert_eq!(3, next_offset);
}

#[test]
fn set_field_instruction_prints_the_symbol_name() {
    let mut chunk = Chunk::new("origin");
    chunk.symbols = Rc::from(vec![Rc::from("x"), Rc::from("value")]);
    chunk.write_op_code(OpCode::SetField, 1, 1);
    chunk.write_u16(0);

    let mut out = String::new();
    let next_offset = chunk.disassemble_instruction(0, &mut out);

    assert_eq!(out, "0000      1 SetField 00 'x'\n");
    assert_eq!(3, next_offset);
}

#[test]
fn store_local_field_instruction_prints_the_slot_and_symbol_name() {
    let mut chunk = Chunk::new("origin");
    chunk.symbols = Rc::from(vec![Rc::from("x"), Rc::from("value")]);
    chunk.write_op_code(OpCode::StoreLocalField, 1, 1);
    chunk.write_u16(0);
    chunk.write_u16(1);

    let mut out = String::new();
    let next_offset = chunk.disassemble_instruction(0, &mut out);

    assert_eq!(out, "0000      1 StoreLocalField 00 01 'value'\n");
    assert_eq!(5, next_offset);
}

#[test]
fn invoke_instruction_prints_the_symbol_name() {
    let mut chunk = Chunk::new("origin");
    chunk.symbols = Rc::from(vec![Rc::from("push")]);
    chunk.write_op_code(OpCode::Invoke, 1, 1);
    chunk.write_u16(0);
    chunk.write_u8(2);

    let mut out = String::new();
    let next_offset = chunk.disassemble_instruction(0, &mut out);

    assert_eq!(out, "0000      1 Invoke push (args: 2)\n");
    assert_eq!(4, next_offset);
}

#[test]
fn define_method_instruction_prints_the_symbol_names() {
    let mut chunk = Chunk::new("origin");
    chunk.symbols = Rc::from(vec![Rc::from("Point"), Rc::from("len")]);
    chunk.write_op_code(OpCode::DefineMethod, 1, 1);
    chunk.write_u16(0);
    chunk.write_u16(1);
    chunk.write_u8(1);

    let mut out = String::new();
    let next_offset = chunk.disassemble_instruction(0, &mut out);

    assert_eq!(out, "0000      1 DefineMethod Point.len (instance)\n");
    assert_eq!(6, next_offset);
}
