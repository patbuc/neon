use crate::vm::{InterpretResult, VirtualMachine};

fn run(program: &str) -> (InterpretResult, VirtualMachine) {
    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    (result, vm)
}

#[test]
fn fiber_constructor_accepts_a_one_parameter_body() {
    let (result, vm) = run("val f = Fiber(fn(x) { return x * 2 })\nprint(f.call(21))");
    assert_eq!(InterpretResult::Ok, result);
    assert_eq!("42", vm.get_output());
}

#[test]
fn fiber_constructor_rejects_a_two_parameter_body() {
    let (result, vm) = run("Fiber(fn(a, b) { return a })");
    assert_eq!(InterpretResult::RuntimeError, result);
    assert_eq!(
        "Fiber body 'anonymous' must take zero or one parameter, but takes 2",
        vm.get_runtime_error().unwrap().message
    );
}

#[test]
fn fiber_constructor_arity_is_checked_at_compile_time() {
    let (result, _) = run("Fiber()");
    assert_eq!(InterpretResult::CompileError, result);
}

#[test]
fn fiber_call_takes_at_most_one_argument() {
    let (result, vm) = run("val f = Fiber(fn(x) { return x })\nf.call(1, 2)");
    assert_eq!(InterpretResult::RuntimeError, result);
    assert_eq!(
        "call() takes at most one argument, got 2",
        vm.get_runtime_error().unwrap().message
    );
}

#[test]
fn task_constructor_rejects_a_non_function() {
    let (result, vm) = run("Task(\"nope\")");
    assert_eq!(InterpretResult::RuntimeError, result);
    assert_eq!(
        "Task() expects a function, got string",
        vm.get_runtime_error().unwrap().message
    );
}

#[test]
fn task_without_argument_passes_nil_to_a_one_parameter_body() {
    let (result, vm) = run("val t = Task(fn(x) { return x })\nprint(t.run())");
    assert_eq!(InterpretResult::Ok, result);
    assert_eq!("nil", vm.get_output());
}

#[test]
fn task_reads_a_copy_of_the_args_builtin() {
    let mut vm = VirtualMachine::with_args(vec!["a".to_string()]);
    let program = "val t = Task(fn() {\n    args.push(\"b\")\n    return args\n})\nprint(t.run())\nprint(args)";
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[a, b]\n[a]", vm.get_output());
}

#[test]
fn fiber_and_task_names_are_reserved_for_structs() {
    let (result, _) = run("struct Fiber { x }");
    assert_eq!(InterpretResult::CompileError, result);
    let (result, _) = run("struct Task { x }");
    assert_eq!(InterpretResult::CompileError, result);
}
