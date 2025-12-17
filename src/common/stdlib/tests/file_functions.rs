use crate::vm::{Result, VirtualMachine};
use std::fs;

// ============================================================================
// File Constructor - Success Cases
// ============================================================================

#[test]
fn test_file_constructor() {
    let program = r#"
        val f = File("test.txt")
        print "File created"
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("File created", vm.get_output());
}

#[test]
fn test_file_constructor_relative_path() {
    let program = r#"
        val f = File("../data/input.txt")
        print "File created"
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("File created", vm.get_output());
}

#[test]
fn test_file_constructor_absolute_path() {
    let program = r#"
        val f = File("/tmp/claude/test.txt")
        print "File created"
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("File created", vm.get_output());
}

// ============================================================================
// File.read() - Success Cases
// ============================================================================

#[test]
fn test_file_read() {
    // Setup: Create a test file
    let test_path = "/tmp/claude/file_read_test.txt";
    fs::create_dir_all("/tmp/claude").ok();
    fs::write(test_path, "Hello, World!").unwrap();

    let program = format!(r#"
        val f = File("{}")
        val content = f.read()
        print content
    "#, test_path);

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program));
    assert_eq!("Hello, World!", vm.get_output());

    // Cleanup
    fs::remove_file(test_path).ok();
}

#[test]
fn test_file_read_empty() {
    // Setup: Create an empty test file
    let test_path = "/tmp/claude/file_read_empty_test.txt";
    fs::create_dir_all("/tmp/claude").ok();
    fs::write(test_path, "").unwrap();

    let program = format!(r#"
        val f = File("{}")
        val content = f.read()
        print content
    "#, test_path);

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program));
    assert_eq!("", vm.get_output());

    // Cleanup
    fs::remove_file(test_path).ok();
}

#[test]
fn test_file_read_multiline() {
    // Setup: Create a multiline test file
    let test_path = "/tmp/claude/file_read_multiline_test.txt";
    fs::create_dir_all("/tmp/claude").ok();
    fs::write(test_path, "Line 1\nLine 2\nLine 3").unwrap();

    let program = format!(r#"
        val f = File("{}")
        val content = f.read()
        print content
    "#, test_path);

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program));
    assert_eq!("Line 1\nLine 2\nLine 3", vm.get_output());

    // Cleanup
    fs::remove_file(test_path).ok();
}

// ============================================================================
// File.readLines() - Success Cases
// ============================================================================

#[test]
fn test_file_read_lines() {
    // Setup: Create a test file with multiple lines
    let test_path = "/tmp/claude/file_read_lines_test.txt";
    fs::create_dir_all("/tmp/claude").ok();
    fs::write(test_path, "Line 1\nLine 2\nLine 3").unwrap();

    let program = format!(r#"
        val f = File("{}")
        val lines = f.readLines()
        print lines.length()
        print lines[0]
        print lines[1]
        print lines[2]
    "#, test_path);

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program));
    assert_eq!("3\nLine 1\nLine 2\nLine 3", vm.get_output());

    // Cleanup
    fs::remove_file(test_path).ok();
}

#[test]
fn test_file_read_lines_empty() {
    // Setup: Create an empty test file
    let test_path = "/tmp/claude/file_read_lines_empty_test.txt";
    fs::create_dir_all("/tmp/claude").ok();
    fs::write(test_path, "").unwrap();

    let program = format!(r#"
        val f = File("{}")
        val lines = f.readLines()
        print lines.length()
    "#, test_path);

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program));
    assert_eq!("0", vm.get_output());

    // Cleanup
    fs::remove_file(test_path).ok();
}

#[test]
fn test_file_read_lines_single() {
    // Setup: Create a single-line test file
    let test_path = "/tmp/claude/file_read_lines_single_test.txt";
    fs::create_dir_all("/tmp/claude").ok();
    fs::write(test_path, "Only one line").unwrap();

    let program = format!(r#"
        val f = File("{}")
        val lines = f.readLines()
        print lines.length()
        print lines[0]
    "#, test_path);

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program));
    assert_eq!("1\nOnly one line", vm.get_output());

    // Cleanup
    fs::remove_file(test_path).ok();
}

// ============================================================================
// File.write() - Success Cases
// ============================================================================

#[test]
fn test_file_write() {
    // Setup: Create test directory and ensure file doesn't exist
    let test_path = "/tmp/claude/file_write_test.txt";
    fs::create_dir_all("/tmp/claude").ok();
    fs::remove_file(test_path).ok();

    let program = format!(r#"
        val f = File("{}")
        f.write("Test content")
    "#, test_path);

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program));

    // Verify the file was written correctly
    let content = fs::read_to_string(test_path).unwrap();
    assert_eq!("Test content", content);

    // Cleanup
    fs::remove_file(test_path).ok();
}

#[test]
fn test_file_write_overwrite() {
    // Setup: Create test directory and ensure file doesn't exist
    let test_path = "/tmp/claude/file_write_overwrite_test.txt";
    fs::create_dir_all("/tmp/claude").ok();
    fs::remove_file(test_path).ok();

    // First write
    let program1 = format!(r#"
        val f = File("{}")
        f.write("Old content")
    "#, test_path);

    let mut vm1 = VirtualMachine::new();
    assert_eq!(Result::Ok, vm1.interpret(program1));

    // Verify first write
    let content1 = fs::read_to_string(test_path).unwrap();
    assert_eq!("Old content", content1);

    // Delete file and write again
    fs::remove_file(test_path).ok();

    // Second write
    let program2 = format!(r#"
        val f = File("{}")
        f.write("New content")
    "#, test_path);

    let mut vm2 = VirtualMachine::new();
    assert_eq!(Result::Ok, vm2.interpret(program2));

    // Verify the file was written with new content
    let content2 = fs::read_to_string(test_path).unwrap();
    assert_eq!("New content", content2);

    // Cleanup
    fs::remove_file(test_path).ok();
}

#[test]
fn test_file_write_empty() {
    // Setup: Create test directory and ensure file doesn't exist
    let test_path = "/tmp/claude/file_write_empty_test.txt";
    fs::create_dir_all("/tmp/claude").ok();
    fs::remove_file(test_path).ok();

    let program = format!(r#"
        val f = File("{}")
        f.write("")
    "#, test_path);

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program));

    // Verify the file was created and is empty
    let content = fs::read_to_string(test_path).unwrap();
    assert_eq!("", content);

    // Cleanup
    fs::remove_file(test_path).ok();
}

#[test]
fn test_file_write_multiline() {
    // Setup: Create test directory
    let test_path = "/tmp/claude/file_write_multiline_test.txt";
    fs::create_dir_all("/tmp/claude").ok();
    fs::remove_file(test_path).ok();

    let program = format!(r#"
        val f = File("{}")
        f.write("Line 1\nLine 2\nLine 3")
    "#, test_path);

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program));

    // Verify the file content - Neon escapes \n in string literals
    let content = fs::read_to_string(test_path).unwrap();
    assert_eq!("Line 1\\nLine 2\\nLine 3", content);

    // Cleanup
    fs::remove_file(test_path).ok();
}

// ============================================================================
// File Read/Write Integration
// ============================================================================

#[test]
fn test_file_write_then_read() {
    // Setup: Create test directory and ensure file doesn't exist
    let test_path = "/tmp/claude/file_write_read_test.txt";
    fs::create_dir_all("/tmp/claude").ok();
    fs::remove_file(test_path).ok();

    let program = format!(r#"
        val f = File("{}")
        f.write("Integration test")
        val content = f.read()
        print content
    "#, test_path);

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program));
    assert_eq!("Integration test", vm.get_output());

    // Cleanup
    fs::remove_file(test_path).ok();
}

// ============================================================================
// File Functions - Error Cases
// ============================================================================

#[test]
fn test_file_constructor_wrong_arg_count_zero() {
    let program = r#"
        val f = File()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::CompileError, vm.interpret(program.to_string()));
}

#[test]
fn test_file_constructor_wrong_arg_count_two() {
    let program = r#"
        val f = File("test.txt", "extra.txt")
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::CompileError, vm.interpret(program.to_string()));
}

#[test]
fn test_file_constructor_invalid_type_number() {
    let program = r#"
        val f = File(42)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_file_constructor_invalid_type_boolean() {
    let program = r#"
        val f = File(true)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_file_constructor_invalid_type_nil() {
    let program = r#"
        val f = File(nil)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_file_read_nonexistent() {
    let program = r#"
        val f = File("/tmp/claude/nonexistent_file_12345.txt")
        f.read()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_file_read_lines_nonexistent() {
    let program = r#"
        val f = File("/tmp/claude/nonexistent_file_67890.txt")
        f.readLines()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_file_write_wrong_arg_count() {
    let program = r#"
        val f = File("/tmp/claude/test.txt")
        f.write()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_file_read_wrong_arg_count() {
    // Setup: Create a test file
    let test_path = "/tmp/claude/file_read_wrong_args_test.txt";
    fs::create_dir_all("/tmp/claude").ok();
    fs::write(test_path, "test").unwrap();

    let program = format!(r#"
        val f = File("{}")
        f.read("unexpected")
    "#, test_path);

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program));

    // Cleanup
    fs::remove_file(test_path).ok();
}

#[test]
fn test_file_read_lines_wrong_arg_count() {
    // Setup: Create a test file
    let test_path = "/tmp/claude/file_read_lines_wrong_args_test.txt";
    fs::create_dir_all("/tmp/claude").ok();
    fs::write(test_path, "test").unwrap();

    let program = format!(r#"
        val f = File("{}")
        f.readLines("unexpected")
    "#, test_path);

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program));

    // Cleanup
    fs::remove_file(test_path).ok();
}
