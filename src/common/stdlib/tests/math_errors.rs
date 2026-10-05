use crate::vm::InterpretResult;
use crate::vm::VirtualMachine;

#[test]
fn math_errors() {
    let cases: &[(&str, &str)] = &[
        (r#"print(Math.abs("string"))"#, "abs() x must be a number"),
        ("print(Math.abs(true))", "abs() x must be a number"),
        ("print(Math.abs(nil))", "abs() x must be a number"),
        ("print(Math.abs())", "abs() expects 1 argument, got 0"),
        ("print(Math.abs(1, 2))", "abs() expects 1 argument, got 2"),
        (
            r#"print(Math.floor("hello"))"#,
            "floor() x must be a number",
        ),
        ("print(Math.floor(false))", "floor() x must be a number"),
        ("print(Math.floor(nil))", "floor() x must be a number"),
        ("print(Math.floor())", "floor() expects 1 argument, got 0"),
        (
            "print(Math.floor(1.5, 2.5))",
            "floor() expects 1 argument, got 2",
        ),
        (r#"print(Math.ceil("world"))"#, "ceil() x must be a number"),
        ("print(Math.ceil(true))", "ceil() x must be a number"),
        ("print(Math.ceil(nil))", "ceil() x must be a number"),
        ("print(Math.ceil())", "ceil() expects 1 argument, got 0"),
        (
            "print(Math.ceil(1.5, 2.5))",
            "ceil() expects 1 argument, got 2",
        ),
        (
            "print(Math.sqrt(-1))",
            "sqrt() requires a non-negative number",
        ),
        (r#"print(Math.sqrt("42"))"#, "sqrt() x must be a number"),
        ("print(Math.sqrt(false))", "sqrt() x must be a number"),
        ("print(Math.sqrt(nil))", "sqrt() x must be a number"),
        ("print(Math.sqrt())", "sqrt() expects 1 argument, got 0"),
        ("print(Math.sqrt(4, 9))", "sqrt() expects 1 argument, got 2"),
        (
            "print(Math.sqrt(-100))",
            "sqrt() requires a non-negative number",
        ),
        ("print(Math.min())", "min() requires at least 1 argument"),
        (
            r#"print(Math.min("hello", 2, 3))"#,
            "min() first argument must be a number",
        ),
        (
            r#"print(Math.min(1, "hello", 3))"#,
            "min() argument 1 must be a number",
        ),
        (
            r#"print(Math.min(1, 2, "hello"))"#,
            "min() argument 2 must be a number",
        ),
        (
            "print(Math.min(1, true, 3))",
            "min() argument 1 must be a number",
        ),
        (
            "print(Math.min(1, nil, 3))",
            "min() argument 1 must be a number",
        ),
        (
            r#"print(Math.min("a", "b", "c"))"#,
            "min() first argument must be a number",
        ),
        (
            r#"print(Math.min(1, "hello", true, nil, 5))"#,
            "min() argument 1 must be a number",
        ),
        ("print(Math.max())", "max() requires at least 1 argument"),
        (
            r#"print(Math.max("hello", 2, 3))"#,
            "max() first argument must be a number",
        ),
        (
            r#"print(Math.max(1, "hello", 3))"#,
            "max() argument 1 must be a number",
        ),
        (
            r#"print(Math.max(1, 2, "hello"))"#,
            "max() argument 2 must be a number",
        ),
        (
            "print(Math.max(1, false, 3))",
            "max() argument 1 must be a number",
        ),
        (
            "print(Math.max(1, nil, 3))",
            "max() argument 1 must be a number",
        ),
        (
            "print(Math.max(true, false, true))",
            "max() first argument must be a number",
        ),
        (
            r#"print(Math.max(1, "world", false, nil, 5))"#,
            "max() argument 1 must be a number",
        ),
        (
            r#"val x = 5 + Math.abs("invalid")"#,
            "abs() x must be a number",
        ),
        (
            r#"val min = -9223372036854775807 - 1
            print(Math.abs(min))"#,
            "integer overflow in abs()",
        ),
        ("print(Math.floor(1e300))", "floor() result is out of range"),
        ("print(Math.ceil(-1e300))", "ceil() result is out of range"),
        ("print(Math.div(1, 0))", "div() division by zero"),
        (
            "print(Math.div(1.5, 1))",
            "div() expects two integers, got float",
        ),
        (
            "print(Math.div(4.0, 1))",
            "div() expects two integers, got float",
        ),
        (
            r#"val min = -9223372036854775807 - 1
            print(Math.div(min, -1))"#,
            "integer overflow in div()",
        ),
    ];

    for (program, expected_message) in cases {
        let mut vm = VirtualMachine::new();
        let result = vm.interpret(program.to_string());
        assert_eq!(
            InterpretResult::RuntimeError,
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
