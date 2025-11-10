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
    // Expect: Constant(opcode variant) + Return sequence at end (Nil, Return)
    // First opcode should be Constant variant
    use crate::common::opcodes::OpCode;
    let first = OpCode::from_u8(bloq.read_u8(0));
    assert!(matches!(first, OpCode::Constant | OpCode::Constant2 | OpCode::Constant4));
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
    use crate::common::opcodes::OpCode;
    // Expect two constants then Add somewhere before the final Return
    let mut saw_add = false;
    for i in 0..bloq.instruction_count() {
        let op = OpCode::from_u8(bloq.read_u8(i));
        if op == OpCode::Add { saw_add = true; break; }
    }
    assert!(saw_add, "Add opcode not found in binary expression");
}

#[test]
fn test_variable_reference() {
    let bloq = compile_program("val x = 5\nprint x\n").unwrap();
    assert!(bloq.instruction_count() > 0);
    use crate::common::opcodes::OpCode;
    // Should contain SetLocal then GetLocal then Print
    let mut saw_set_local = false;
    let mut saw_get_local = false;
    let mut saw_print = false;
    for i in 0..bloq.instruction_count() {
        match OpCode::from_u8(bloq.read_u8(i)) {
            OpCode::SetLocal | OpCode::SetLocal2 | OpCode::SetLocal4 => saw_set_local = true,
            OpCode::GetLocal | OpCode::GetLocal2 | OpCode::GetLocal4 => saw_get_local = true,
            OpCode::Print => saw_print = true,
            _ => {}
        }
    }
    assert!(saw_set_local && saw_get_local && saw_print, "Missing expected opcodes for variable reference");
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
    use crate::common::opcodes::OpCode;
    // Should contain Call opcode
    let mut saw_call = false;
    for i in 0..bloq.instruction_count() {
        if OpCode::from_u8(bloq.read_u8(i)) == OpCode::Call { saw_call = true; break; }
    }
    assert!(saw_call, "Call opcode not found for function invocation");
}

#[test]
fn test_field_get_set() {
    let program = r#"
    struct Point {
        x
        y
    }
    val p = Point(1, 2)
    print p.x
    p.y = 3
    "#;
    let bloq = compile_program(program).unwrap();
    use crate::common::opcodes::OpCode;
    let mut saw_get_field = false;
    let mut saw_set_field = false;
    for i in 0..bloq.instruction_count() {
        match OpCode::from_u8(bloq.read_u8(i)) {
            OpCode::GetField | OpCode::GetField2 | OpCode::GetField4 => saw_get_field = true,
            OpCode::SetField | OpCode::SetField2 | OpCode::SetField4 => saw_set_field = true,
            _ => {}
        }
    }
    assert!(saw_get_field && saw_set_field, "Missing field get/set opcodes");
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
    use crate::common::opcodes::OpCode;
    // Expect comparison (Greater or Less etc.), JumpIfFalse, optional Jump
    let mut saw_jump_if_false = false;
    for i in 0..bloq.instruction_count() {
        if OpCode::from_u8(bloq.read_u8(i)) == OpCode::JumpIfFalse { saw_jump_if_false = true; break; }
    }
    assert!(saw_jump_if_false, "JumpIfFalse not emitted for if statement");
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
    use crate::common::opcodes::OpCode;
    // Expect JumpIfFalse and Loop opcodes
    let mut saw_jump_if_false = false;
    let mut saw_loop = false;
    for i in 0..bloq.instruction_count() {
        match OpCode::from_u8(bloq.read_u8(i)) {
            OpCode::JumpIfFalse => saw_jump_if_false = true,
            OpCode::Loop => saw_loop = true,
            _ => {}
        }
    }
    assert!(saw_jump_if_false && saw_loop, "Missing loop control opcodes");
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
fn test_constant_operand_widths() {
    use crate::common::opcodes::OpCode;
    // Build a program with > 0xFF constants to force Constant2, and > 0xFFFF would be huge (skip due to test time)
    // We'll generate 300 simple constants.
    let mut source = String::new();
    for i in 0..300 { source.push_str(&format!("{}\n", i)); }
    let bloq = compile_program(&source).unwrap();
    // Scan instructions to find first constant opcode variant after index threshold
    let mut saw_constant = false;
    let mut saw_constant2 = false;
    for offset in 0..bloq.instruction_count() {
        match OpCode::from_u8(bloq.read_u8(offset)) {
            OpCode::Constant => saw_constant = true,
            OpCode::Constant2 => { saw_constant2 = true; break; },
            OpCode::Constant4 => { /* Not expected in this test */ },
            _ => {}
        }
    }
    assert!(saw_constant, "Did not see initial Constant opcode");
    assert!(saw_constant2, "Did not see Constant2 opcode after >0xFF constants threshold");
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
