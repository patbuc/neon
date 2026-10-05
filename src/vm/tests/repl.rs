use crate::vm::{InterpretResult, VirtualMachine};

#[test]
fn var_then_print() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("var x = 1".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(x)".to_string())
    );
    assert_eq!("1", vm.get_output());
}

#[test]
fn for_loop_slots() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("var x = 1".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("for i in 0..3 {\n    print(i)\n}".to_string())
    );
    assert_eq!("0\n1\n2", vm.get_output());
}

#[test]
fn fn_then_call() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("fn double(n) { return n * 2 }".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(double(21))".to_string())
    );
    assert_eq!("42", vm.get_output());
}

#[test]
fn compile_error_rollback() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("var x = 1".to_string())
    );
    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret_line("var y = ".to_string())
    );
    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret_line("print(y)".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(x)".to_string())
    );
    assert_eq!("1", vm.get_output());
}

#[test]
fn runtime_error_rollback() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret_line("val y = 1\n1 + true".to_string())
    );
    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret_line("print(y)".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("val y = 1".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(y)".to_string())
    );
    assert_eq!("1", vm.get_output());
}

#[test]
fn earlier_assignment_kept() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("var x = 1".to_string())
    );
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret_line("x = 2\n1 + true".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(x)".to_string())
    );
    assert_eq!("2", vm.get_output());
}

#[test]
fn impl_blocks_accumulate_across_lines() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("struct Point {\n    x\n    y\n}".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("impl Point {\n    fn getX(self) { return self.x }\n}".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("impl Point {\n    fn getY(self) { return self.y }\n}".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("val p = Point(1, 2)".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(p.getX())\nprint(p.getY())".to_string())
    );
    assert_eq!("1\n2", vm.get_output());
}

#[test]
fn failed_impl_line_adds_no_methods() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("struct Point {\n    x\n    y\n}".to_string())
    );
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret_line(
            "impl Point {\n    fn describe(self) { return self.x }\n}\n1 + true".to_string()
        )
    );
    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret_line("val p = Point(1, 2)\nprint(p.describe())".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line(
            "impl Point {\n    fn describe(self) { return self.x + self.y }\n}".to_string()
        )
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("val p = Point(1, 2)".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(p.describe())".to_string())
    );
    assert_eq!("3", vm.get_output());
}

#[test]
fn struct_defined_then_used_later() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("struct Point {\n    x\n    y\n}".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("val p = Point(3, 4)".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(p.x + p.y)".to_string())
    );
    assert_eq!("7", vm.get_output());
}

#[test]
fn poisoned_slot() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("var g = nil".to_string())
    );
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret_line("val newvar = 5\ng = fn() { return newvar }\n1 + true".to_string())
    );
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret_line("print(g())".to_string())
    );
    assert_eq!(
        "variable 'newvar' used before initialization",
        vm.get_runtime_error().unwrap().message
    );
}

#[test]
fn interpret_resets_repl() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("var x = 1".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(x)".to_string())
    );

    let program = "print(42)".to_string();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.clone()));

    let mut fresh_vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, fresh_vm.interpret(program));
    assert_eq!(fresh_vm.stack.len(), vm.stack.len());

    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret_line("print(x)".to_string())
    );
}

#[test]
fn redeclared_val_keeps_earlier_fn_binding() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("val x = 1".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("fn f() { return x }".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("val x = 2".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(x)\nprint(f())".to_string())
    );
    assert_eq!("2\n1", vm.get_output());
}

#[test]
fn redeclared_fn_is_called() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("fn f() { return 1 }".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("fn f() { return 2 }".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(f())".to_string())
    );
    assert_eq!("2", vm.get_output());
}

#[test]
fn redeclared_val_rejects_earlier_var_assignment() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("var x = 1".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("val x = 2".to_string())
    );
    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret_line("x = 3".to_string())
    );
}

#[test]
fn same_line_redeclaration_still_errors() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("val x = 1".to_string())
    );
    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret_line("val x = 2\nval x = 3".to_string())
    );
}

#[test]
fn redeclaring_struct_name_errors() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("struct Point {\n    x\n    y\n}".to_string())
    );
    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret_line("val Point = 1".to_string())
    );
}

#[test]
fn redeclaring_val_as_struct_errors() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("val x = 1".to_string())
    );
    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret_line("struct x {\n    a\n}".to_string())
    );
}

#[test]
fn failed_redeclaration_keeps_old_value() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("val x = 1".to_string())
    );
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret_line("val x = 3\n1 + true".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(x)".to_string())
    );
    assert_eq!("1", vm.get_output());
}

#[test]
fn failed_line_forgets_an_assigned_global_type() {
    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("var x = 1".to_string())
    );
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret_line("print([x = \"ab\", [1][5]])".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(x)".to_string())
    );
    assert_eq!(
        InterpretResult::Ok,
        vm.interpret_line("print(x.size())".to_string())
    );
    assert_eq!("ab\n2", vm.get_output());
}
