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
fn fiber_name_is_reserved_for_structs() {
    let (result, _) = run("struct Fiber { x }");
    assert_eq!(InterpretResult::CompileError, result);
}

// `Task` was briefly available for a struct name (#246); it's reserved again
// now that it names the builtin task type (#259).
#[test]
fn task_name_is_reserved_for_structs() {
    let (result, _) = run("struct Task { x }");
    assert_eq!(InterpretResult::CompileError, result);
}

#[test]
fn task_spawn_rejects_a_non_function_body() {
    let (result, vm) = run("Task.spawn(42)");
    assert_eq!(InterpretResult::RuntimeError, result);
    assert_eq!(
        "Task.spawn() expects a function, got number",
        vm.get_runtime_error().unwrap().message
    );
}

#[test]
fn task_spawn_rejects_more_than_two_arguments() {
    let (result, vm) = run("Task.spawn(fn(x) { return x }, 1, 2)");
    assert_eq!(InterpretResult::RuntimeError, result);
    assert_eq!(
        "Task.spawn() takes at most 2 arguments, got 3",
        vm.get_runtime_error().unwrap().message
    );
}

#[test]
fn task_spawn_is_rejected_inside_a_native_callback() {
    let (result, vm) = run("[1].map(fn(x) {\n    Task.spawn(fn() { return 1 })\n    return x\n})");
    assert_eq!(InterpretResult::RuntimeError, result);
    assert_eq!(
        "Cannot spawn a task from inside a native callback",
        vm.get_runtime_error().unwrap().message
    );
}
