use crate::common::opcodes::OpCode;
use crate::common::{Chunk, Value};
use crate::vm::{Result, TraceFrame, VirtualMachine};
use crate::{as_number, number};
use std::rc::Rc;

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
    chunk.write_indexed(OpCode::GetLocal, 0, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn set_local_out_of_range_slot_halts() {
    // One value on the stack, so slot 1 is exactly one past the last valid slot.
    let mut chunk = Chunk::new("set_local_oob");
    chunk.write_constant(number!(42.0), 0, 0);
    chunk.write_indexed(OpCode::SetLocal, 1, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn get_global_unknown_slot_halts() {
    // Stack is empty, so slot 0 is exactly one past the last valid slot.
    let mut chunk = Chunk::new("get_global_unknown");
    chunk.write_indexed(OpCode::GetGlobal, 0, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn set_global_out_of_range_slot_halts() {
    // One value on the stack, so slot 1 is exactly one past the last valid slot.
    let mut chunk = Chunk::new("set_global_oob");
    chunk.write_constant(number!(42.0), 0, 0);
    chunk.write_indexed(OpCode::SetGlobal, 1, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn get_builtin_unknown_index_halts() {
    let mut chunk = Chunk::new("get_builtin_unknown");
    chunk.write_indexed(OpCode::GetBuiltin, 9999, 0, 0);

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

#[test]
fn call_field_holding_number_is_not_callable() {
    let program = r#"
        struct S { f }
        fn call_f(s) { return s.f(2) }
        call_f(S(1))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("not callable"), "{}", errors);
}

#[test]
fn call_name_that_is_neither_method_nor_field_is_unknown_method() {
    let program = r#"
        struct S { f }
        fn call_g(s) { return s.g(2) }
        call_g(S(1))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("Unknown method 'g' for type S"),
        "{}",
        errors
    );
}

#[test]
fn top_level_fn_reads_later_val_before_init() {
    let program = r#"
fn f() {
    return y
}
print(f())
val y = 2
"#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("variable 'y' used before initialization"),
        "{}",
        errors
    );
    assert!(errors.contains("[3:"), "{}", errors);
}

#[test]
fn top_level_fn_assigns_later_var_before_init() {
    let program = r#"
fn f() {
    x = 5
}
f()
var x = 3
"#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("variable 'x' used before initialization"),
        "{}",
        errors
    );
    assert!(errors.contains("[3:"), "{}", errors);
}

#[test]
fn nested_fn_called_before_its_declaration_line() {
    let program = r#"
fn outer() {
    g()
    fn g() { return 1 }
}
outer()
"#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("variable 'g' used before initialization"),
        "{}",
        errors
    );
    assert!(errors.contains("[3:"), "{}", errors);
}

#[test]
fn nested_fn_stored_in_val_before_its_declaration_line() {
    let program = r#"
fn outer() {
    val g = f
    fn f() { return 1 }
    return g
}
outer()
"#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("variable 'f' used before initialization"),
        "{}",
        errors
    );
    assert!(errors.contains("[3:"), "{}", errors);
}

#[test]
fn sibling_fn_called_before_its_declaration_line() {
    let program = r#"
fn outer() {
    fn a() { return b() }
    print(a())
    fn b() { return 1 }
}
outer()
"#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("variable 'b' used before initialization"),
        "{}",
        errors
    );
    assert!(errors.contains("[3:"), "{}", errors);
}

#[test]
fn get_global_uninitialized_slot_halts() {
    let mut chunk = Chunk::new("get_global_uninitialized");
    chunk.write_constant(Value::Uninitialized(Rc::from("x")), 1, 1);
    chunk.write_indexed(OpCode::GetGlobal, 0, 2, 5);
    chunk.write_op_code(OpCode::Return, 3, 1);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("variable 'x' used before initialization"),
        "{}",
        errors
    );
    assert!(errors.contains("[2:5]"), "{}", errors);
}

#[test]
fn set_global_uninitialized_slot_halts() {
    let mut chunk = Chunk::new("set_global_uninitialized");
    chunk.write_constant(Value::Uninitialized(Rc::from("x")), 1, 1);
    chunk.write_constant(number!(1.0), 1, 1);
    chunk.write_indexed(OpCode::SetGlobal, 0, 2, 5);
    chunk.write_op_code(OpCode::Return, 3, 1);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("variable 'x' used before initialization"),
        "{}",
        errors
    );
    assert!(errors.contains("[2:5]"), "{}", errors);
}

#[test]
fn check_initialized_on_uninitialized_value_halts() {
    let mut chunk = Chunk::new("check_initialized_uninitialized");
    chunk.write_constant(Value::Uninitialized(Rc::from("x")), 1, 1);
    chunk.write_op_code(OpCode::CheckInitialized, 2, 5);
    chunk.write_op_code(OpCode::Return, 3, 1);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("variable 'x' used before initialization"),
        "{}",
        errors
    );
    assert!(errors.contains("[2:5]"), "{}", errors);
}

#[test]
fn check_initialized_on_normal_value_passes_through() {
    let mut chunk = Chunk::new("check_initialized_normal");
    chunk.write_constant(number!(42.0), 0, 0);
    chunk.write_op_code(OpCode::CheckInitialized, 0, 0);
    chunk.write_op_code(OpCode::Return, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::Ok, result);
    assert_eq!(42.0, as_number!(vm.pop()));
}

#[test]
fn nested_call_error_reports_frames_innermost_first() {
    let program = "fn outer() { return inner() }\nfn inner() { return 1 + true }\nouter()";

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let error = vm.get_runtime_error().unwrap();
    assert_eq!(
        vec![
            TraceFrame {
                function: "inner".to_string(),
                line: Some(2),
            },
            TraceFrame {
                function: "outer".to_string(),
                line: Some(1),
            },
            TraceFrame {
                function: "<script>".to_string(),
                line: Some(3),
            },
        ],
        error.frames
    );
    assert_eq!(
        "  at inner (line 2)\n  at outer (line 1)\n  at <script> (line 3)",
        error.trace()
    );
}

#[test]
fn native_callback_error_trace_has_no_native_frame() {
    let program = r#"
        fn boom(x) { return x + true }
        print([1, 2, 3].map(boom))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let error = vm.get_runtime_error().unwrap();
    assert_eq!(
        vec![
            TraceFrame {
                function: "boom".to_string(),
                line: Some(2),
            },
            TraceFrame {
                function: "<script>".to_string(),
                line: Some(3),
            },
        ],
        error.frames
    );
}

#[test]
fn unbounded_recursion_trace_is_capped() {
    let program = "fn f(n) { return f(n + 1) }\nf(0)";

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let error = vm.get_runtime_error().unwrap();

    assert_eq!(10_000, error.frames.len());

    let trace = error.trace();
    let lines: Vec<&str> = trace.lines().collect();
    assert_eq!(21, lines.len());
    assert_eq!("  ... 9980 frames omitted", lines[10]);
    for line in &lines[..10] {
        assert_eq!("  at f (line 1)", *line);
    }
    for line in &lines[11..20] {
        assert_eq!("  at f (line 1)", *line);
    }
    assert_eq!("  at <script> (line 2)", lines[20]);
}

#[test]
fn trace_at_exactly_twenty_one_frames_is_not_collapsed() {
    let program = r#"
        fn f(n) {
            if (n == 0) { return 1 + true }
            return f(n - 1)
        }
        f(19)
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let error = vm.get_runtime_error().unwrap();

    assert_eq!(21, error.frames.len());
    let trace = error.trace();
    let lines: Vec<&str> = trace.lines().collect();
    assert_eq!(21, lines.len());
    assert!(!trace.contains("omitted"));
    assert_eq!("  at f (line 3)", lines[0]);
    for line in &lines[1..20] {
        assert_eq!("  at f (line 4)", *line);
    }
    assert_eq!("  at <script> (line 6)", lines[20]);
}

#[test]
fn trace_at_twenty_two_frames_is_collapsed() {
    let program = r#"
        fn f(n) {
            if (n == 0) { return 1 + true }
            return f(n - 1)
        }
        f(20)
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let error = vm.get_runtime_error().unwrap();

    assert_eq!(22, error.frames.len());
    let trace = error.trace();
    let lines: Vec<&str> = trace.lines().collect();
    assert_eq!(21, lines.len());
    assert_eq!("  ... 2 frames omitted", lines[10]);
}

#[test]
fn caller_frame_line_is_the_call_site_not_the_next_statement() {
    let program = "fn boom() { return 1 + true }\nfn outer() {\n  boom()\n  print(1)\n}\nouter()";

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let error = vm.get_runtime_error().unwrap();

    assert_eq!(
        vec![
            TraceFrame {
                function: "boom".to_string(),
                line: Some(1),
            },
            TraceFrame {
                function: "outer".to_string(),
                line: Some(3),
            },
            TraceFrame {
                function: "<script>".to_string(),
                line: Some(6),
            },
        ],
        error.frames
    );
}

#[test]
fn arity_mismatch_through_a_stored_function_reports_the_call_site() {
    let program = "fn g(a) { return a }\nval k = g\nfn h() {\n  k()\n  print(1)\n}\nh()";

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let error = vm.get_runtime_error().unwrap();

    assert_eq!(Some((4, 4)), error.location);
    assert_eq!(
        vec![
            TraceFrame {
                function: "h".to_string(),
                line: Some(4),
            },
            TraceFrame {
                function: "<script>".to_string(),
                line: Some(7),
            },
        ],
        error.frames
    );
}

#[test]
fn native_message_error_through_a_nested_call_reports_the_call_site() {
    let program = "fn h() {\n  Math.sqrt(\"a\")\n  print(1)\n}\nh()";

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let error = vm.get_runtime_error().unwrap();

    assert_eq!(2, error.location.unwrap().0);
    assert_eq!(
        vec![
            TraceFrame {
                function: "h".to_string(),
                line: Some(2),
            },
            TraceFrame {
                function: "<script>".to_string(),
                line: Some(5),
            },
        ],
        error.frames
    );
}

#[test]
fn not_callable_error_through_a_nested_call_reports_the_call_site() {
    let program = "fn h() {\n  val x = 1\n  x()\n  print(1)\n}\nh()";

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let error = vm.get_runtime_error().unwrap();

    assert_eq!(3, error.location.unwrap().0);
    assert_eq!(
        vec![
            TraceFrame {
                function: "h".to_string(),
                line: Some(3),
            },
            TraceFrame {
                function: "<script>".to_string(),
                line: Some(6),
            },
        ],
        error.frames
    );
}

#[test]
fn struct_field_count_error_through_a_nested_call_reports_the_call_site() {
    // Calling through a stored value, since a literal `P(1)` is an arity
    // error the compiler catches before this runs.
    let program = "struct P { x y }\nval ctor = P\nfn h() {\n  ctor(1)\n  print(1)\n}\nh()";

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let error = vm.get_runtime_error().unwrap();

    assert_eq!(4, error.location.unwrap().0);
    assert_eq!(
        vec![
            TraceFrame {
                function: "h".to_string(),
                line: Some(4),
            },
            TraceFrame {
                function: "<script>".to_string(),
                line: Some(7),
            },
        ],
        error.frames
    );
}

#[test]
fn method_mismatch_error_through_a_nested_call_reports_the_call_site() {
    let program = "struct Point { x y }\nimpl Point {\n    fn origin() { return Point(0, 0) }\n}\nfn call_origin(p) {\n  p.origin()\n  print(1)\n}\ncall_origin(Point(1, 2))";

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let error = vm.get_runtime_error().unwrap();

    assert_eq!(6, error.location.unwrap().0);
    assert_eq!(
        vec![
            TraceFrame {
                function: "call_origin".to_string(),
                line: Some(6),
            },
            TraceFrame {
                function: "<script>".to_string(),
                line: Some(9),
            },
        ],
        error.frames
    );
}

#[test]
fn native_method_arity_error_reports_native_message() {
    let program = "fn f(a) { return a.push() }\nf([1])";

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("push() expects 1 argument (value), got 0"),
        "{}",
        errors
    );
}

#[test]
fn native_error_inside_builtin_type_user_method_reports_native_message() {
    let program = r#"
        impl Array {
            fn addNothing(self) { return self.push() }
        }
        val a = [1]
        a.addNothing()
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("push() expects 1 argument (value), got 0"),
        "{}",
        errors
    );
}
