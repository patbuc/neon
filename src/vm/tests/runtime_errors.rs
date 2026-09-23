use crate::common::opcodes::OpCode;
use crate::common::Chunk;
use crate::number;
use crate::vm::{Result, VirtualMachine};

#[test]
fn array_index_read_out_of_bounds_halts() {
    let program = r#"
        print([1, 2][5])
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn array_index_read_out_of_bounds_reports_exactly_one_error() {
    let program = r#"
        print([1, 2][5])
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert_eq!(1, vm.get_runtime_errors().lines().count());
    assert_eq!("", vm.get_output());
}

#[test]
fn array_index_write_out_of_bounds_halts() {
    let program = r#"
        val a = [1, 2]
        a[5] = 3
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn get_field_missing_halts() {
    let program = r#"
        struct P { x }
        fn get_y(p) { return p.y }
        print(get_y(P(1)))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn set_field_missing_halts() {
    let program = r#"
        struct P { x }
        fn set_y(p) { p.y = 2 }
        set_y(P(1))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn create_map_invalid_key_reports_exactly_one_error() {
    let program = r#"
        print({[1]: 2})
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert_eq!(1, vm.get_runtime_errors().lines().count());
    assert_eq!("", vm.get_output());
}

#[test]
fn create_set_invalid_element_reports_exactly_one_error() {
    let program = r#"
        print(#{[1]})
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert_eq!(1, vm.get_runtime_errors().lines().count());
    assert_eq!("", vm.get_output());
}

#[test]
fn nothing_after_a_runtime_error_executes() {
    let program = r#"
        struct P { x }
        fn get_y(p) { return p.y }
        print(get_y(P(1)))
        print("after")
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert!(!vm.get_output().contains("after"));
}

#[test]
fn get_local_out_of_range_slot_halts() {
    // Stack is empty, so slot 0 is exactly one past the last valid slot.
    let mut chunk = Chunk::new("get_local_oob");
    chunk.write_op_code_variant(OpCode::GetLocal, 0, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn set_local_out_of_range_slot_halts() {
    // One value on the stack, so slot 1 is exactly one past the last valid slot.
    let mut chunk = Chunk::new("set_local_oob");
    chunk.write_constant(number!(42.0), 0, 0);
    chunk.write_op_code_variant(OpCode::SetLocal, 1, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn get_global_unknown_slot_halts() {
    // Stack is empty, so slot 0 is exactly one past the last valid slot.
    let mut chunk = Chunk::new("get_global_unknown");
    chunk.write_op_code_variant(OpCode::GetGlobal, 0, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn set_global_out_of_range_slot_halts() {
    // One value on the stack, so slot 1 is exactly one past the last valid slot.
    let mut chunk = Chunk::new("set_global_oob");
    chunk.write_constant(number!(42.0), 0, 0);
    chunk.write_op_code_variant(OpCode::SetGlobal, 1, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn get_builtin_unknown_index_halts() {
    let mut chunk = Chunk::new("get_builtin_unknown");
    chunk.write_op_code_variant(OpCode::GetBuiltin, 9999, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn iterator_done_without_iterator_halts() {
    let mut chunk = Chunk::new("iterator_done_without_iterator");
    chunk.write_op_code(OpCode::IteratorDone, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn invalid_opcode_byte_halts() {
    let mut chunk = Chunk::new("invalid_opcode");
    chunk.write_op_code(OpCode::Nil, 1, 1);
    chunk.instructions[0] = 0xFF;

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn subtract_with_non_number_operand_reports_operator_location() {
    let program = r#"
        val a = 1
        val b = a - "x"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert!(vm.get_runtime_errors().starts_with("[3:19]"));
}

#[test]
fn bad_operand_type_errors() {
    let cases: &[(&str, &str)] = &[
        (
            r#"print(1 - "a")"#,
            "Operands of '-' must be numbers, got number and string",
        ),
        (
            r#"print("a" - 1)"#,
            "Operands of '-' must be numbers, got string and number",
        ),
        (
            "print(true - 1)",
            "Operands of '-' must be numbers, got boolean and number",
        ),
        (
            r#"print(5 * "x")"#,
            "Operands of '*' must be numbers, got number and string",
        ),
        (
            "print(nil * 5)",
            "Operands of '*' must be numbers, got nil and number",
        ),
        (
            r#"print(5 / "x")"#,
            "Operands of '/' must be numbers, got number and string",
        ),
        (
            "print([1] / 5)",
            "Operands of '/' must be numbers, got array and number",
        ),
        (
            r#"print(5 % "x")"#,
            "Operands of '%' must be numbers, got number and string",
        ),
        (
            "print(5 % true)",
            "Operands of '%' must be numbers, got number and boolean",
        ),
        (
            r#"print(5 ** "x")"#,
            "Operands of '**' must be numbers, got number and string",
        ),
        (
            "print(5 ** nil)",
            "Operands of '**' must be numbers, got number and nil",
        ),
        (
            r#"print(5 & "x")"#,
            "Operands of '&' must be numbers, got number and string",
        ),
        (
            "print(5 & [1])",
            "Operands of '&' must be numbers, got number and array",
        ),
        (
            r#"print(5 | "x")"#,
            "Operands of '|' must be numbers, got number and string",
        ),
        (
            "print(5 | {1: 2})",
            "Operands of '|' must be numbers, got number and map",
        ),
        (
            r#"print(5 ^ "x")"#,
            "Operands of '^' must be numbers, got number and string",
        ),
        (
            "print(5 ^ true)",
            "Operands of '^' must be numbers, got number and boolean",
        ),
        (
            r#"print(5 << "x")"#,
            "Operands of '<<' must be numbers, got number and string",
        ),
        (
            "print(5 << nil)",
            "Operands of '<<' must be numbers, got number and nil",
        ),
        (
            r#"print(5 >> "x")"#,
            "Operands of '>>' must be numbers, got number and string",
        ),
        (
            "print(5 >> [1])",
            "Operands of '>>' must be numbers, got number and array",
        ),
        (
            r#"print(1 < "a")"#,
            "Operands of a comparison must be two numbers or two strings, got number and string",
        ),
        (
            r#"print("a" < 1)"#,
            "Operands of a comparison must be two numbers or two strings, got string and number",
        ),
        (
            "print(nil < 1)",
            "Operands of a comparison must be two numbers or two strings, got nil and number",
        ),
        (
            "print(true < false)",
            "Operands of a comparison must be two numbers or two strings, got boolean and boolean",
        ),
        (
            r#"print(1 > "a")"#,
            "Operands of a comparison must be two numbers or two strings, got number and string",
        ),
        (
            "print([1] > [2])",
            "Operands of a comparison must be two numbers or two strings, got array and array",
        ),
        (
            r#"print(1 <= "a")"#,
            "Operands of a comparison must be two numbers or two strings, got number and string",
        ),
        (
            r#"print(1 >= "a")"#,
            "Operands of a comparison must be two numbers or two strings, got number and string",
        ),
    ];

    for (program, expected_message) in cases {
        let mut vm = VirtualMachine::new();
        let result = vm.interpret(program.to_string());
        assert_eq!(
            Result::RuntimeError,
            result,
            "expected runtime error for: {}",
            program
        );
        let errors = vm.get_runtime_errors();
        assert!(
            errors.contains(expected_message),
            "program {:?}: expected {:?} in {:?}",
            program,
            expected_message,
            errors
        );
    }
}

#[test]
fn unbounded_recursion_reports_stack_overflow_at_the_call_site() {
    let program = "fn f(n) { return f(n + 1) }\nf(0)";

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("Stack overflow"), "{}", errors);
    assert!(errors.contains("[1:19]"), "{}", errors);
}

#[test]
fn native_callback_runtime_error_reports_exactly_one_error() {
    let program = r#"
        fn boom(x) { return x + true }
        print([1, 2, 3].map(boom))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert_eq!(1, vm.get_runtime_errors().lines().count());
    assert_eq!("", vm.get_output());
}

#[test]
fn native_callback_wrong_arity_reports_exactly_one_error() {
    let program = r#"
        fn needs_two(a, b) { return a + b }
        print([1, 2, 3].map(needs_two))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert_eq!(1, vm.get_runtime_errors().lines().count());
    assert!(
        vm.get_runtime_errors().contains("Expected 2 arguments"),
        "{}",
        vm.get_runtime_errors()
    );
}

#[test]
fn native_callback_stack_overflow_reports_exactly_one_error() {
    let program = r#"
        fn recurse(n) { return recurse(n + 1) }
        print([1].map(recurse))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert_eq!(1, vm.get_runtime_errors().lines().count());
    assert!(
        vm.get_runtime_errors().contains("Stack overflow"),
        "{}",
        vm.get_runtime_errors()
    );
}

#[test]
fn native_callback_error_reports_a_single_location_prefix() {
    let program = "fn boom(x) { return x + true }\nprint([1, 2].map(boom))";

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert_eq!(1, errors.lines().count());
    assert_eq!("[1:23] Operands must be two numbers or two strings", errors);
}

#[test]
fn nested_native_callback_error_reports_a_single_location_prefix() {
    let program =
        "fn boom(x) { return x + true }\nprint([[1]].map(fn(row) { return row.map(boom) }))";

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert_eq!(1, errors.lines().count());
    assert_eq!("[1:23] Operands must be two numbers or two strings", errors);
}

#[test]
fn recursive_callback_through_map_reports_stack_overflow_once() {
    let program = r#"
        fn r(n) {
            if (n == 0) { return 0 }
            return [n - 1].map(r)[0]
        }
        print(r(1000))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert_eq!(1, errors.lines().count());
    assert!(errors.contains("Stack overflow"), "{}", errors);
}

#[test]
fn unconditional_callback_recursion_through_map_reports_stack_overflow_once() {
    let program = r#"
        fn recurse(n) { return [n].map(recurse) }
        recurse(0)
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert_eq!(1, errors.lines().count());
    assert!(errors.contains("Stack overflow"), "{}", errors);
}

#[test]
fn map_over_a_large_array_produces_correct_output() {
    let program = r#"
        fn identity(x) { return x }
        val result = (0..100000).map(identity)
        print(result.size())
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("100000", vm.get_output());
}

#[test]
fn vm_is_usable_after_a_callback_error() {
    let failing = "fn boom(x) { return x + true }\n[1, 2].map(boom)";

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(failing.to_string()));

    let program = r#"
        fn add(a, b) { return a + b }
        var total = 0
        for (i in 0..5) {
            total = add(total, i)
        }
        print(total)
        "#;

    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("10", vm.get_output());

    let mut fresh_vm = VirtualMachine::new();
    assert_eq!(Result::Ok, fresh_vm.interpret(program.to_string()));
    assert_eq!("10", fresh_vm.get_output());

    assert_eq!(fresh_vm.stack.len(), vm.stack.len());
}

#[test]
fn method_arity_error_excludes_self() {
    let program = r#"
        struct Point { x y }
        impl Point {
            fn dist(self, o) { return 1 }
        }
        fn call_dist(p) { return p.dist() }
        call_dist(Point(1, 2))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("Expected 1 arguments but got 0"),
        "{}",
        errors
    );
    assert!(errors.contains("dist"), "{}", errors);
}

#[test]
fn undefined_method_on_unresolved_receiver_type_halts_at_runtime() {
    let program = r#"
        struct Point { x y }
        impl Point {
            fn len(self) { return self.x }
        }
        fn call_len(p) { return p.lne() }
        call_len(Point(1, 2))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert!(
        vm.get_runtime_errors().contains("lne"),
        "{}",
        vm.get_runtime_errors()
    );
}

#[test]
fn instance_call_on_static_method_halts_at_runtime() {
    let program = r#"
        struct Point { x y }
        impl Point {
            fn origin() { return Point(0, 0) }
        }
        fn call_origin(p) { return p.origin() }
        call_origin(Point(1, 2))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("origin"), "{}", errors);
    assert!(errors.contains("static"), "{}", errors);
}

#[test]
fn static_call_on_instance_method_halts_at_runtime() {
    let program = r#"
        struct Point { x y }
        impl Point {
            fn len(self) { return self.x }
        }
        fn call_len(t) { return t.len() }
        call_len(Point)
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("len"), "{}", errors);
    assert!(errors.contains("instance"), "{}", errors);
}

#[test]
fn method_named_this_is_static_on_untyped_receiver_halts_at_runtime() {
    let program = r#"
        struct Point { x y }
        impl Point {
            fn len(this) { return this.x }
        }
        fn call_len(p) { return p.len() }
        call_len(Point(1, 2))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("len"), "{}", errors);
    assert!(errors.contains("static"), "{}", errors);
}
