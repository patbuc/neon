use crate::common::NativeCallError;
use crate::number;
use crate::vm::{InterpretResult, VirtualMachine};

#[test]
fn call_value_invokes_a_global_function_and_returns_its_result() {
    let mut vm = VirtualMachine::new();
    let result = vm.interpret("fn double(x) { return x * 2 }".to_string());
    assert_eq!(InterpretResult::Ok, result);

    let callee = vm.stack[0].clone();
    let outcome = vm.call_value(callee, &[number!(21.0)]).unwrap();
    assert_eq!(number!(42.0), outcome);
}

#[test]
fn call_value_wrong_arity_is_an_error_without_running_the_callee() {
    let mut vm = VirtualMachine::new();
    let result = vm.interpret("fn needs_two(a, b) { return a + b }".to_string());
    assert_eq!(InterpretResult::Ok, result);

    let callee = vm.stack[0].clone();
    let outcome = vm.call_value(callee, &[number!(1.0)]);
    match outcome {
        Err(NativeCallError::Runtime(e)) => {
            assert!(e.message.contains("Expected 2 arguments"))
        }
        other => panic!("expected a runtime error, got {:?}", other),
    }
}
