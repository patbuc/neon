use crate::common::Bloq;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;

fn compile_program(source: &str) -> Result<Bloq, String> {
    // Parse
    let mut parser = Parser::new(source);
    let ast = parser
        .parse()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    // Semantic analysis
    let mut analyzer = SemanticAnalyzer::new();
    let _ = analyzer
        .analyze(&ast)
        .map_err(|e| format!("Semantic error: {:?}", e))?;

    // Code generation
    let mut codegen = CodeGenerator::new();
    codegen
        .generate(&ast)
        .map_err(|e| format!("Codegen error: {:?}", e))
}

#[test]
fn test_simple_number() {
    let bloq = compile_program("42\n").unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_val_declaration() {
    let bloq = compile_program("val x = 5\n").unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_binary_expression() {
    let bloq = compile_program("1 + 2\n").unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_variable_reference() {
    let bloq = compile_program("val x = 5\nprint x\n").unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_function() {
    let program = r#"
    fn add(a, b) {
        return a + b
    }
    val result = add(1, 2)
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_if_statement() {
    let program = r#"
    val x = 10
    if (x > 5) {
        print x
    }
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_while_loop() {
    let program = r#"
    var i = 0
    while (i < 10) {
        i = i + 1
    }
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_end_to_end_execution() {
    use crate::vm::VirtualMachine;

    let program = r#"
    val x = 10
    val y = 20
    val sum = x + y
    print sum
    "#;
    let bloq = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_bloq(bloq);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "30");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}

#[test]
fn test_end_to_end_function() {
    use crate::vm::VirtualMachine;

    let program = r#"
    fn add(a, b) {
        return a + b
    }
    val result = add(15, 27)
    print result
    "#;
    let bloq = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_bloq(bloq);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "42");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}

#[test]
fn test_end_to_end_forward_reference() {
    use crate::vm::VirtualMachine;

    // This tests that forward function references work!
    let program = r#"
    fn foo() {
        return bar()
    }

    fn bar() {
        return 99
    }

    print foo()
    "#;
    let bloq = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_bloq(bloq);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "99");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}

#[test]
fn test_else_if_bytecode_simple() {
    use crate::common::opcodes::OpCode;

    // Test simple else-if chain bytecode generation
    let program = r#"
    val x = 5
    if (x == 1) {
        print 1
    } else if (x == 2) {
        print 2
    } else {
        print 3
    }
    "#;
    let bloq = compile_program(program).unwrap();

    // Verify that bytecode contains the expected jump instructions
    // Pattern should be:
    // 1. First condition (x == 1)
    // 2. JumpIfFalse (skip first then-branch)
    // 3. First then-branch code
    // 4. Jump (skip else-if and else)
    // 5. Second condition (x == 2) - this is the else-if
    // 6. JumpIfFalse (skip second then-branch)
    // 7. Second then-branch code
    // 8. Jump (skip else)
    // 9. Else-branch code

    let mut jump_if_false_count = 0;
    let mut jump_count = 0;

    let mut offset = 0;
    while offset < bloq.instruction_count() {
        let op = OpCode::from_u8(bloq.read_u8(offset));
        match op {
            OpCode::JumpIfFalse => {
                jump_if_false_count += 1;
                offset += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            OpCode::Jump => {
                jump_count += 1;
                offset += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            OpCode::Constant | OpCode::SetLocal | OpCode::GetLocal |
            OpCode::GetGlobal | OpCode::SetGlobal | OpCode::GetField | OpCode::SetField => {
                offset += 2; // OpCode (1 byte) + 1-byte operand
            }
            OpCode::Constant2 | OpCode::SetLocal2 | OpCode::GetLocal2 |
            OpCode::GetGlobal2 | OpCode::SetGlobal2 | OpCode::GetField2 | OpCode::SetField2 |
            OpCode::String | OpCode::Call => {
                offset += 2; // OpCode (1 byte) + 1-byte operand (Call uses 1 byte for arg count)
            }
            OpCode::Constant4 | OpCode::SetLocal4 | OpCode::GetLocal4 |
            OpCode::GetGlobal4 | OpCode::SetGlobal4 | OpCode::GetField4 | OpCode::SetField4 => {
                offset += 5; // OpCode (1 byte) + 4-byte operand
            }
            _ => {
                offset += 1; // Simple instructions
            }
        }
    }

    // We should have 2 JumpIfFalse (one for each condition)
    assert_eq!(jump_if_false_count, 2, "Expected 2 JumpIfFalse instructions for if and else-if conditions");

    // We should have 2 Jump instructions (one after each then-branch)
    assert_eq!(jump_count, 2, "Expected 2 Jump instructions to skip remaining branches");
}

#[test]
fn test_else_if_bytecode_multiple_branches() {
    use crate::common::opcodes::OpCode;

    // Test multiple else-if branches
    let program = r#"
    val x = 10
    if (x < 5) {
        print 1
    } else if (x < 10) {
        print 2
    } else if (x < 15) {
        print 3
    } else if (x < 20) {
        print 4
    } else {
        print 5
    }
    "#;
    let bloq = compile_program(program).unwrap();

    let mut jump_if_false_count = 0;
    let mut jump_count = 0;

    let mut offset = 0;
    while offset < bloq.instruction_count() {
        let op = OpCode::from_u8(bloq.read_u8(offset));
        match op {
            OpCode::JumpIfFalse => {
                jump_if_false_count += 1;
                offset += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            OpCode::Jump => {
                jump_count += 1;
                offset += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            OpCode::Constant | OpCode::SetLocal | OpCode::GetLocal |
            OpCode::GetGlobal | OpCode::SetGlobal | OpCode::GetField | OpCode::SetField => {
                offset += 2; // OpCode (1 byte) + 1-byte operand
            }
            OpCode::Constant2 | OpCode::SetLocal2 | OpCode::GetLocal2 |
            OpCode::GetGlobal2 | OpCode::SetGlobal2 | OpCode::GetField2 | OpCode::SetField2 |
            OpCode::String | OpCode::Call => {
                offset += 2; // OpCode (1 byte) + 1-byte operand (Call uses 1 byte for arg count)
            }
            OpCode::Constant4 | OpCode::SetLocal4 | OpCode::GetLocal4 |
            OpCode::GetGlobal4 | OpCode::SetGlobal4 | OpCode::GetField4 | OpCode::SetField4 => {
                offset += 5; // OpCode (1 byte) + 4-byte operand
            }
            _ => {
                offset += 1; // Simple instructions
            }
        }
    }

    // We should have 4 JumpIfFalse (one for each condition)
    assert_eq!(jump_if_false_count, 4, "Expected 4 JumpIfFalse instructions for all conditions");

    // We should have 4 Jump instructions (one after each then-branch)
    assert_eq!(jump_count, 4, "Expected 4 Jump instructions to skip remaining branches");
}

#[test]
fn test_else_if_bytecode_without_final_else() {
    use crate::common::opcodes::OpCode;

    // Test else-if chain without final else
    let program = r#"
    val x = 7
    if (x == 5) {
        print 5
    } else if (x == 7) {
        print 7
    }
    "#;
    let bloq = compile_program(program).unwrap();

    let mut jump_if_false_count = 0;
    let mut jump_count = 0;

    let mut offset = 0;
    while offset < bloq.instruction_count() {
        let op = OpCode::from_u8(bloq.read_u8(offset));
        match op {
            OpCode::JumpIfFalse => {
                jump_if_false_count += 1;
                offset += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            OpCode::Jump => {
                jump_count += 1;
                offset += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            OpCode::Constant | OpCode::SetLocal | OpCode::GetLocal |
            OpCode::GetGlobal | OpCode::SetGlobal | OpCode::GetField | OpCode::SetField => {
                offset += 2; // OpCode (1 byte) + 1-byte operand
            }
            OpCode::Constant2 | OpCode::SetLocal2 | OpCode::GetLocal2 |
            OpCode::GetGlobal2 | OpCode::SetGlobal2 | OpCode::GetField2 | OpCode::SetField2 |
            OpCode::String | OpCode::Call => {
                offset += 2; // OpCode (1 byte) + 1-byte operand (Call uses 1 byte for arg count)
            }
            OpCode::Constant4 | OpCode::SetLocal4 | OpCode::GetLocal4 |
            OpCode::GetGlobal4 | OpCode::SetGlobal4 | OpCode::GetField4 | OpCode::SetField4 => {
                offset += 5; // OpCode (1 byte) + 4-byte operand
            }
            _ => {
                offset += 1; // Simple instructions
            }
        }
    }

    // We should have 2 JumpIfFalse (one for each condition)
    assert_eq!(jump_if_false_count, 2, "Expected 2 JumpIfFalse instructions");

    // We should have 2 Jump instructions (one after each then-branch)
    assert_eq!(jump_count, 2, "Expected 2 Jump instructions");
}

#[test]
fn test_else_if_bytecode_jump_offsets() {
    // Test that jump offsets are correctly calculated
    let program = r#"
    val x = 5
    if (x == 1) {
        print 1
    } else if (x == 2) {
        print 2
    } else {
        print 3
    }
    "#;
    let bloq = compile_program(program).unwrap();

    // Verify the bytecode compiles and has instructions
    assert!(bloq.instruction_count() > 0, "Bytecode should not be empty");

    // Walk through bytecode to find and verify jump instructions
    let mut i = 0;
    let mut jumps = Vec::new();

    while i < bloq.instruction_count() {
        let op = crate::common::opcodes::OpCode::from_u8(bloq.read_u8(i));
        match op {
            crate::common::opcodes::OpCode::JumpIfFalse | crate::common::opcodes::OpCode::Jump => {
                // Read the 4-byte offset
                let offset = bloq.read_u32(i + 1);
                let target = i + 5 + offset as usize;
                jumps.push((i, op, target));
                i += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            _ => i += 1,
        }
    }

    // Verify jumps are pointing to valid locations within bytecode
    for (pos, _op, target) in &jumps {
        assert!(
            *target <= bloq.instruction_count(),
            "Jump at position {} targets invalid offset {}",
            pos,
            target
        );
    }
}

#[test]
fn test_else_if_end_to_end_execution() {
    use crate::vm::VirtualMachine;

    // Test that else-if chains execute correctly
    let program = r#"
    val x = 15
    if (x < 10) {
        print 10
    } else if (x < 20) {
        print 20
    } else {
        print 30
    }
    "#;
    let bloq = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_bloq(bloq);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "20");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}
