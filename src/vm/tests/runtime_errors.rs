use crate::common::opcodes::OpCode;
use crate::common::Chunk;
use crate::vm::{Result, VirtualMachine};

#[test]
fn invalid_opcode_byte_halts() {
    let mut chunk = Chunk::new("invalid_opcode");
    chunk.write_op_code(OpCode::Nil, 1, 1);
    chunk.instructions[0] = 0xFF;

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}
