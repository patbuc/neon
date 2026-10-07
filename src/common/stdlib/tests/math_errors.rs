use crate::vm::InterpretResult;
use crate::vm::VirtualMachine;

#[test]
fn math_errors() {
    let cases: &[(&str, &str)] = &[
        (r#"print(math.abs("string"))"#, "abs() x must be a number"),
        ("print(math.abs(true))", "abs() x must be a number"),
        ("print(math.abs(nil))", "abs() x must be a number"),
        (
            "val f = math.abs\nprint(f())",
            "abs() expects 1 argument, got 0",
        ),
        (
            "val f = math.abs\nprint(f(1, 2))",
            "abs() expects 1 argument, got 2",
        ),
        (
            r#"print(math.floor("hello"))"#,
            "floor() x must be a number",
        ),
        ("print(math.floor(false))", "floor() x must be a number"),
        ("print(math.floor(nil))", "floor() x must be a number"),
        (
            "val f = math.floor\nprint(f())",
            "floor() expects 1 argument, got 0",
        ),
        (
            "val f = math.floor\nprint(f(1.5, 2.5))",
            "floor() expects 1 argument, got 2",
        ),
        (r#"print(math.ceil("world"))"#, "ceil() x must be a number"),
        ("print(math.ceil(true))", "ceil() x must be a number"),
        ("print(math.ceil(nil))", "ceil() x must be a number"),
        (
            "val f = math.ceil\nprint(f())",
            "ceil() expects 1 argument, got 0",
        ),
        (
            "val f = math.ceil\nprint(f(1.5, 2.5))",
            "ceil() expects 1 argument, got 2",
        ),
        (
            "print(math.sqrt(-1))",
            "sqrt() requires a non-negative number",
        ),
        (r#"print(math.sqrt("42"))"#, "sqrt() x must be a number"),
        ("print(math.sqrt(false))", "sqrt() x must be a number"),
        ("print(math.sqrt(nil))", "sqrt() x must be a number"),
        (
            "val f = math.sqrt\nprint(f())",
            "sqrt() expects 1 argument, got 0",
        ),
        (
            "val f = math.sqrt\nprint(f(4, 9))",
            "sqrt() expects 1 argument, got 2",
        ),
        (
            "print(math.sqrt(-100))",
            "sqrt() requires a non-negative number",
        ),
        ("print(math.min())", "min() requires at least 1 argument"),
        (
            r#"print(math.min("hello", 2, 3))"#,
            "min() first argument must be a number",
        ),
        (
            r#"print(math.min(1, "hello", 3))"#,
            "min() argument 1 must be a number",
        ),
        (
            r#"print(math.min(1, 2, "hello"))"#,
            "min() argument 2 must be a number",
        ),
        (
            "print(math.min(1, true, 3))",
            "min() argument 1 must be a number",
        ),
        (
            "print(math.min(1, nil, 3))",
            "min() argument 1 must be a number",
        ),
        (
            r#"print(math.min("a", "b", "c"))"#,
            "min() first argument must be a number",
        ),
        (
            r#"print(math.min(1, "hello", true, nil, 5))"#,
            "min() argument 1 must be a number",
        ),
        ("print(math.max())", "max() requires at least 1 argument"),
        (
            r#"print(math.max("hello", 2, 3))"#,
            "max() first argument must be a number",
        ),
        (
            r#"print(math.max(1, "hello", 3))"#,
            "max() argument 1 must be a number",
        ),
        (
            r#"print(math.max(1, 2, "hello"))"#,
            "max() argument 2 must be a number",
        ),
        (
            "print(math.max(1, false, 3))",
            "max() argument 1 must be a number",
        ),
        (
            "print(math.max(1, nil, 3))",
            "max() argument 1 must be a number",
        ),
        (
            "print(math.max(true, false, true))",
            "max() first argument must be a number",
        ),
        (
            r#"print(math.max(1, "world", false, nil, 5))"#,
            "max() argument 1 must be a number",
        ),
        (
            r#"val x = 5 + math.abs("invalid")"#,
            "abs() x must be a number",
        ),
        (
            r#"val min = -9223372036854775807 - 1
            print(math.abs(min))"#,
            "integer overflow in abs()",
        ),
        ("print(math.floor(1e300))", "floor() result is out of range"),
        ("print(math.ceil(-1e300))", "ceil() result is out of range"),
        ("print(math.div(1, 0))", "div() division by zero"),
        (
            "print(math.div(1.5, 1))",
            "div() expects two integers, got float",
        ),
        (
            "print(math.div(4.0, 1))",
            "div() expects two integers, got float",
        ),
        (
            r#"val min = -9223372036854775807 - 1
            print(math.div(min, -1))"#,
            "integer overflow in div()",
        ),
    ];

    for (program, expected_message) in cases {
        let mut vm = VirtualMachine::new();
        let result = vm.interpret(format!("use \"std/math\"\n{program}"));
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
