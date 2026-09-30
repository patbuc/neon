use crate::common::constants::MAX_FRAMES;
use crate::common::Value;
use crate::vm::{InterpretResult, VirtualMachine};
use std::rc::Rc;

#[test]
fn vm_keeps_no_reference_to_a_finished_fiber() {
    let program = "val f = Fiber(fn() {\n    val x = 1\n    val g = fn() { return x }\n    Fiber.yield(g())\n    return 2\n})\nf.call()\nf.call()";

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    let Value::Fiber(fiber) = vm.stack[0].clone() else {
        panic!("expected global 'f' to hold the fiber, got {:?}", vm.stack);
    };
    // The global slot and this test's clone.
    assert_eq!(2, Rc::strong_count(&fiber));
    assert_eq!(0, Rc::weak_count(&fiber));
}

/// Runs `program`, which must overflow the stack, and returns how many
/// frames the error's trace holds.
fn frames_at_overflow(program: &str) -> usize {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let error = vm.get_runtime_error().unwrap();
    assert_eq!("Stack overflow", error.message);
    error.frames.len()
}

#[test]
fn nested_fibers_overflow_at_the_plain_recursion_limit() {
    let with_fibers = frames_at_overflow(
        "fn nest() {\n    return Fiber(fn() { return nest() }).call()\n}\nnest()",
    );
    let plain = frames_at_overflow("fn nest() {\n    return nest()\n}\nnest()");
    assert_eq!(MAX_FRAMES, plain);
    assert_eq!(MAX_FRAMES, with_fibers);
}
