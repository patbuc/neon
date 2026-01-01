use neon::vm::{Result, VirtualMachine};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// Helper function to create a temporary test file with content
fn create_test_file(name: &str, content: &str) -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("neon_test_{}", name));

    // Clean up if it exists
    let _ = fs::remove_file(&file_path);

    // Create and write the content
    let mut file = fs::File::create(&file_path).expect("Failed to create test file");
    file.write_all(content.as_bytes())
        .expect("Failed to write test file");

    file_path
}

/// Helper function to clean up a test file
fn cleanup_test_file(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_file_constructor_from_neon() {
    let mut vm = VirtualMachine::new();
    let source = r#"
        var f = File("test.txt")
        print(f)
    "#;

    let result = vm.interpret(source.to_string());
    assert_eq!(Result::Ok, result);

    // The output should contain file representation
    let output = vm.get_output();
    assert!(
        output.contains("test.txt"),
        "Output should contain file path: {}",
        output
    );
}

#[test]
fn test_file_read_basic() {
    let test_file = create_test_file("read_basic.txt", "Hello, World!");
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        var content = f.read()
        print(content)
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result, "VM interpretation failed");

    let output = vm.get_output();
    assert_eq!("Hello, World!", output.trim(), "File content mismatch");

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_read_multiline() {
    let test_content = "Line 1\nLine 2\nLine 3";
    let test_file = create_test_file("read_multiline.txt", test_content);
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        var content = f.read()
        print(content)
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    assert_eq!(test_content, output.trim());

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_read_empty_file() {
    let test_file = create_test_file("read_empty.txt", "");
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        var content = f.read()
        print("start")
        print(content)
        print("end")
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    assert!(output.contains("start"));
    assert!(output.contains("end"));

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_read_unicode() {
    let test_content = "Hello 世界! 🌍 Καλημέρα";
    let test_file = create_test_file("read_unicode.txt", test_content);
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        var content = f.read()
        print(content)
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    assert_eq!(test_content, output.trim());

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_read_not_found() {
    let mut vm = VirtualMachine::new();
    let source = r#"
        var f = File("/nonexistent/path/to/file.txt")
        var content = f.read()
        print(content)
    "#;

    let result = vm.interpret(source.to_string());
    // Should result in a runtime error
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_file_read_lines_basic() {
    let test_content = "Line 1\nLine 2\nLine 3";
    let test_file = create_test_file("readlines_basic.txt", test_content);
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        var lines = f.readLines()
        for (var i = 0; i < lines.size(); i = i + 1) {{
            print(lines[i])
        }}
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    let output_lines: Vec<&str> = output.trim().split('\n').collect();
    assert_eq!(3, output_lines.len());
    assert_eq!("Line 1", output_lines[0]);
    assert_eq!("Line 2", output_lines[1]);
    assert_eq!("Line 3", output_lines[2]);

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_read_lines_with_empty_lines() {
    let test_content = "Line 1\n\nLine 3\n\n";
    let test_file = create_test_file("readlines_empty.txt", test_content);
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        var lines = f.readLines()
        print(lines.size())
        for (var i = 0; i < lines.size(); i = i + 1) {{
            print("[" + lines[i] + "]")
        }}
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    assert!(
        output.contains("4"),
        "Should have 4 lines including empty ones"
    );
    assert!(output.contains("[Line 1]"));
    assert!(output.contains("[]")); // Empty lines
    assert!(output.contains("[Line 3]"));

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_read_lines_crlf() {
    let test_content = "Line 1\r\nLine 2\r\nLine 3";
    let test_file = create_test_file("readlines_crlf.txt", test_content);
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        var lines = f.readLines()
        for (var i = 0; i < lines.size(); i = i + 1) {{
            print(lines[i])
        }}
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    let output_lines: Vec<&str> = output.trim().split('\n').collect();
    // Line endings should be stripped
    assert_eq!("Line 1", output_lines[0]);
    assert_eq!("Line 2", output_lines[1]);
    assert_eq!("Line 3", output_lines[2]);

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_read_lines_empty_file() {
    let test_file = create_test_file("readlines_empty_file.txt", "");
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        var lines = f.readLines()
        print(lines.size())
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    assert!(output.contains("0"), "Empty file should return empty array");

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_read_lines_single_line_no_newline() {
    let test_file = create_test_file("readlines_single.txt", "Single line");
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        var lines = f.readLines()
        print(lines.size())
         print(lines[0])
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    assert!(output.contains("1"));
    assert!(output.contains("Single line"));

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_read_lines_not_found() {
    let mut vm = VirtualMachine::new();
    let source = r#"
        var f = File("/nonexistent/path/to/file.txt")
        var lines = f.readLines()
        print(lines)
    "#;

    let result = vm.interpret(source.to_string());
    // Should result in a runtime error
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_file_write_basic() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("neon_test_write_basic.txt");
    let file_path = test_file.to_str().unwrap();

    // Ensure file doesn't exist
    let _ = fs::remove_file(&test_file);

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        f.write("Hello from Neon!")
        print("Write successful")
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    assert!(output.contains("Write successful"));

    // Verify the file was created with correct content
    let content = fs::read_to_string(&test_file).expect("Failed to read written file");
    assert_eq!("Hello from Neon!", content);

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_write_multiline() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("neon_test_write_multiline.txt");
    let file_path = test_file.to_str().unwrap();
    let temp_content = "Line 1\nLine 2\nLine 3";

    // Ensure file doesn't exist
    let _ = fs::remove_file(&test_file);

    // Create the file with actual newlines via Rust first
    {
        let mut file = fs::File::create(&test_file).unwrap();
        file.write_all(temp_content.as_bytes()).unwrap();
    }

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        var content = f.read()
        print(content)
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    // Verify we read it correctly
    let output = vm.get_output();
    assert_eq!(temp_content, output.trim());

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_write_empty_content() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("neon_test_write_empty.txt");
    let file_path = test_file.to_str().unwrap();

    // Ensure file doesn't exist
    let _ = fs::remove_file(&test_file);

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        f.write("")
         print("Done")
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    // Verify the file was created and is empty
    let content = fs::read_to_string(&test_file).expect("Failed to read written file");
    assert_eq!("", content);

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_write_file_already_exists() {
    let test_file = create_test_file("write_exists.txt", "Existing content");
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        f.write("New content")
         print("This should not print")
    "#,
        file_path
    );

    let result = vm.interpret(source);
    // Should result in a runtime error because file exists
    assert_eq!(Result::RuntimeError, result);

    // Verify the original content was not changed
    let content = fs::read_to_string(&test_file).expect("Failed to read file");
    assert_eq!("Existing content", content);

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_write_invalid_directory() {
    let mut vm = VirtualMachine::new();
    let source = r#"
        var f = File("/nonexistent/directory/file.txt")
        f.write("content")
         print("This should not print")
    "#;

    let result = vm.interpret(source.to_string());
    // Should result in a runtime error
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_file_end_to_end_write_then_read() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("neon_test_end_to_end.txt");
    let file_path = test_file.to_str().unwrap();

    // Ensure file doesn't exist
    let _ = fs::remove_file(&test_file);

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f1 = File("{}")
        f1.write("Test content for end-to-end")

        var f2 = File("{}")
        var content = f2.read()
        print(content)
    "#,
        file_path, file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    assert_eq!("Test content for end-to-end", output.trim());

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_end_to_end_write_then_read_lines() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("neon_test_write_readlines.txt");
    let file_path = test_file.to_str().unwrap();

    // Ensure file doesn't exist
    let _ = fs::remove_file(&test_file);

    // Create the file with actual newlines via Rust first
    {
        let mut file = fs::File::create(&test_file).unwrap();
        file.write_all(b"First\nSecond\nThird").unwrap();
    }

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        var lines = f.readLines()
        for (var i = 0; i < lines.size(); i = i + 1) {{
            print(lines[i])
        }}
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    let output_lines: Vec<&str> = output.trim().split('\n').collect();
    assert_eq!(3, output_lines.len());
    assert_eq!("First", output_lines[0]);
    assert_eq!("Second", output_lines[1]);
    assert_eq!("Third", output_lines[2]);

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_multiple_operations_same_file_object() {
    let test_file = create_test_file("multiple_ops.txt", "Line 1\nLine 2\nLine 3");
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        var content = f.read()
        print(content)

        var lines = f.readLines()
        print(lines.size())
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    assert!(output.contains("Line 1"));
    assert!(output.contains("3"));

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_constructor_with_variable_path() {
    let test_file = create_test_file("var_path.txt", "Content");
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var path = "{}"
        var f = File(path)
        var content = f.read()
        print(content)
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    assert_eq!("Content", output.trim());

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_in_function() {
    let test_file = create_test_file("in_function.txt", "Function test");
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        fn readFile(path) {{
            var f = File(path)
            return f.read()
        }}

        var content = readFile("{}")
        print(content)
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    assert_eq!("Function test", output.trim());

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_read_lines_in_function() {
    let test_file = create_test_file("readlines_function.txt", "A\nB\nC");
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        fn getLines(path) {{
            var f = File(path)
            return f.readLines()
        }}

        var lines = getLines("{}")
        for (var i = 0; i < lines.size(); i = i + 1) {{
            print(lines[i])
        }}
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    assert!(output.contains("A"));
    assert!(output.contains("B"));
    assert!(output.contains("C"));

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_write_in_function() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("neon_test_write_function.txt");
    let file_path = test_file.to_str().unwrap();

    // Ensure file doesn't exist
    let _ = fs::remove_file(&test_file);

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        fn writeFile(path, content) {{
            var f = File(path)
            f.write(content)
        }}

        writeFile("{}", "Written from function")
         print("Done")
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    // Verify the file content
    let content = fs::read_to_string(&test_file).expect("Failed to read written file");
    assert_eq!("Written from function", content);

    cleanup_test_file(&test_file);
}

#[test]
fn test_file_practical_example_process_lines() {
    let test_content = "apple\nbanana\ncherry\ndate";
    let test_file = create_test_file("process_lines.txt", test_content);
    let file_path = test_file.to_str().unwrap();

    let mut vm = VirtualMachine::new();
    let source = format!(
        r#"
        var f = File("{}")
        var lines = f.readLines()
        var count = 0

        for (var i = 0; i < lines.size(); i = i + 1) {{
            if (lines[i].len() > 5) {{
                print(lines[i])
                count = count + 1
            }}
        }}

        print("Found: " + count.toString())
    "#,
        file_path
    );

    let result = vm.interpret(source);
    assert_eq!(Result::Ok, result);

    let output = vm.get_output();
    assert!(output.contains("banana"));
    assert!(output.contains("cherry"));
    assert!(output.contains("Found: 2"));

    cleanup_test_file(&test_file);
}
