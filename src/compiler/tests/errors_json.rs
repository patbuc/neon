use crate::compiler::errors_to_json;
use crate::vm::VirtualMachine;

#[test]
fn escapes_quotes_backslashes_and_newlines_in_the_message() {
    let mut vm = VirtualMachine::new();
    vm.interpret("use \"std/a\\\"b\\\\c\\n\"\n".to_string());

    let json = errors_to_json(vm.get_compile_errors());

    assert!(
        json.contains(r#""message":"unknown builtin module 'std/a\"b\\c\n'"#),
        "{json}"
    );
}
