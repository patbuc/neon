use crate::common::{ObjString, Object};
use crate::vm::file_functions::*;
use crate::vm::VirtualMachine;
use std::rc::Rc;

#[test]
fn test_file_constructor_valid_path() {
    let vm = VirtualMachine::new();
    let path = "test.txt";
    let args = vec![crate::common::Value::Object(Rc::new(Object::String(
        ObjString {
            value: Rc::from(path),
        },
    )))];

    let result = native_file_constructor(&args).unwrap();

    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::File(file_path) => {
                assert_eq!(file_path.as_ref(), path);
            }
            _ => panic!("Expected File object"),
        },
        _ => panic!("Expected Object value"),
    }
}

#[test]
fn test_file_constructor_wrong_arg_count_zero() {
    let vm = VirtualMachine::new();
    let args = vec![];
    let result = native_file_constructor(&args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "File() expects 1 argument, got 0");
}

#[test]
fn test_file_constructor_wrong_arg_count_two() {
    let vm = VirtualMachine::new();
    let args = vec![
        crate::common::Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from("test.txt"),
        }))),
        crate::common::Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from("extra.txt"),
        }))),
    ];
    let result = native_file_constructor(&args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "File() expects 1 argument, got 2");
}

#[test]
fn test_file_constructor_invalid_type_number() {
    let vm = VirtualMachine::new();
    let args = vec![crate::common::Value::Number(42.0)];
    let result = native_file_constructor(&args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "File() requires a string argument");
}

#[test]
fn test_file_constructor_invalid_type_boolean() {
    let vm = VirtualMachine::new();
    let args = vec![crate::common::Value::Boolean(true)];
    let result = native_file_constructor(&args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "File() requires a string argument");
}

#[test]
fn test_file_constructor_invalid_type_nil() {
    let vm = VirtualMachine::new();
    let args = vec![crate::common::Value::Nil];
    let result = native_file_constructor(&args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "File() requires a string argument");
}

#[test]
fn test_file_constructor_with_relative_path() {
    let vm = VirtualMachine::new();
    let path = "../data/input.txt";
    let args = vec![crate::common::Value::Object(Rc::new(Object::String(
        ObjString {
            value: Rc::from(path),
        },
    )))];

    let result = native_file_constructor(&args).unwrap();

    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::File(file_path) => {
                assert_eq!(file_path.as_ref(), path);
            }
            _ => panic!("Expected File object"),
        },
        _ => panic!("Expected Object value"),
    }
}

#[test]
fn test_file_constructor_with_absolute_path() {
    let vm = VirtualMachine::new();
    let path = "/home/user/data/input.txt";
    let args = vec![crate::common::Value::Object(Rc::new(Object::String(
        ObjString {
            value: Rc::from(path),
        },
    )))];

    let result = native_file_constructor(&args).unwrap();

    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::File(file_path) => {
                assert_eq!(file_path.as_ref(), path);
            }
            _ => panic!("Expected File object"),
        },
        _ => panic!("Expected Object value"),
    }
}

// Tests for File.read()
#[test]
fn test_file_read_success() {
    use std::fs::File;
    use std::io::Write;

    let vm = VirtualMachine::new();

    // Create a temporary file with some content
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_read.txt");
    let test_content = "Hello, World!\nThis is a test file.";

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_content.as_bytes()).unwrap();
    }

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let args = vec![file_obj];

    // Call read()
    let result = native_file_read(&args).unwrap();

    // Verify the result
    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => {
                assert_eq!(s.value.as_ref(), test_content);
            }
            _ => panic!("Expected String object"),
        },
        _ => panic!("Expected Object value"),
    }

    // Clean up
    std::fs::remove_file(file_path).ok();
}

#[test]
fn test_file_read_empty_file() {
    use std::fs::File;

    let vm = VirtualMachine::new();

    // Create an empty temporary file
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_read_empty.txt");

    {
        File::create(&file_path).unwrap();
    }

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let args = vec![file_obj];

    // Call read()
    let result = native_file_read(&args).unwrap();

    // Verify the result is an empty string
    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => {
                assert_eq!(s.value.as_ref(), "");
            }
            _ => panic!("Expected String object"),
        },
        _ => panic!("Expected Object value"),
    }

    // Clean up
    std::fs::remove_file(file_path).ok();
}

#[test]
fn test_file_read_unicode_content() {
    use std::fs::File;
    use std::io::Write;

    let vm = VirtualMachine::new();

    // Create a temporary file with Unicode content
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_read_unicode.txt");
    let test_content = "Hello 世界! 🌍 Καλημέρα";

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_content.as_bytes()).unwrap();
    }

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let args = vec![file_obj];

    // Call read()
    let result = native_file_read(&args).unwrap();

    // Verify the result
    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => {
                assert_eq!(s.value.as_ref(), test_content);
            }
            _ => panic!("Expected String object"),
        },
        _ => panic!("Expected Object value"),
    }

    // Clean up
    std::fs::remove_file(file_path).ok();
}

#[test]
fn test_file_read_file_not_found() {
    let vm = VirtualMachine::new();

    // Create a File object with a non-existent path
    let file_obj = crate::common::Value::new_file("/nonexistent/path/to/file.txt".to_string());
    let args = vec![file_obj];

    // Call read()
    let result = native_file_read(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("File not found"));
    assert!(error_msg.contains("/nonexistent/path/to/file.txt"));
}

#[test]
fn test_file_read_wrong_arg_count() {
    let vm = VirtualMachine::new();

    // Create a File object
    let file_obj = crate::common::Value::new_file("test.txt".to_string());

    // Call read() with extra arguments
    let args = vec![file_obj, crate::common::Value::Number(42.0)];
    let result = native_file_read(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("expects 0 arguments"));
}

#[test]
fn test_file_read_invalid_receiver_type() {
    let vm = VirtualMachine::new();

    // Try to call read() on a non-File object
    let args = vec![crate::common::Value::Object(Rc::new(Object::String(
        ObjString {
            value: Rc::from("not a file"),
        },
    )))];

    let result = native_file_read(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("can only be called on File objects"));
}

#[test]
fn test_file_read_invalid_receiver_primitive() {
    let vm = VirtualMachine::new();

    // Try to call read() on a number
    let args = vec![crate::common::Value::Number(42.0)];

    let result = native_file_read(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("can only be called on File objects"));
}

#[test]
fn test_file_read_multiline_content() {
    use std::fs::File;
    use std::io::Write;

    let vm = VirtualMachine::new();

    // Create a temporary file with multiline content
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_read_multiline.txt");
    let test_content = "Line 1\nLine 2\nLine 3\nLine 4";

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_content.as_bytes()).unwrap();
    }

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let args = vec![file_obj];

    // Call read()
    let result = native_file_read(&args).unwrap();

    // Verify the result
    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => {
                assert_eq!(s.value.as_ref(), test_content);
            }
            _ => panic!("Expected String object"),
        },
        _ => panic!("Expected Object value"),
    }

    // Clean up
    std::fs::remove_file(file_path).ok();
}

// Tests for File.readLines()
#[test]
fn test_file_read_lines_success() {
    use std::fs::File;
    use std::io::Write;

    let vm = VirtualMachine::new();

    // Create a temporary file with multiline content
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_read_lines.txt");
    let test_content = "Line 1\nLine 2\nLine 3";

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_content.as_bytes()).unwrap();
    }

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let args = vec![file_obj];

    // Call readLines()
    let result = native_file_read_lines(&args).unwrap();

    // Verify the result is an array
    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let elements = arr.borrow();
                assert_eq!(elements.len(), 3);

                // Check each line
                if let crate::common::Value::Object(line_obj) = &elements[0] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "Line 1");
                    } else {
                        panic!("Expected String object");
                    }
                }

                if let crate::common::Value::Object(line_obj) = &elements[1] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "Line 2");
                    } else {
                        panic!("Expected String object");
                    }
                }

                if let crate::common::Value::Object(line_obj) = &elements[2] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "Line 3");
                    } else {
                        panic!("Expected String object");
                    }
                }
            }
            _ => panic!("Expected Array object"),
        },
        _ => panic!("Expected Object value"),
    }

    // Clean up
    std::fs::remove_file(file_path).ok();
}

#[test]
fn test_file_read_lines_empty_file() {
    use std::fs::File;

    let vm = VirtualMachine::new();

    // Create an empty temporary file
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_read_lines_empty.txt");

    {
        File::create(&file_path).unwrap();
    }

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let args = vec![file_obj];

    // Call readLines()
    let result = native_file_read_lines(&args).unwrap();

    // Verify the result is an empty array
    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let elements = arr.borrow();
                assert_eq!(elements.len(), 0);
            }
            _ => panic!("Expected Array object"),
        },
        _ => panic!("Expected Object value"),
    }

    // Clean up
    std::fs::remove_file(file_path).ok();
}

#[test]
fn test_file_read_lines_single_line() {
    use std::fs::File;
    use std::io::Write;

    let vm = VirtualMachine::new();

    // Create a temporary file with a single line (no newline at end)
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_read_lines_single.txt");
    let test_content = "Single line";

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_content.as_bytes()).unwrap();
    }

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let args = vec![file_obj];

    // Call readLines()
    let result = native_file_read_lines(&args).unwrap();

    // Verify the result
    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let elements = arr.borrow();
                assert_eq!(elements.len(), 1);

                if let crate::common::Value::Object(line_obj) = &elements[0] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "Single line");
                    } else {
                        panic!("Expected String object");
                    }
                }
            }
            _ => panic!("Expected Array object"),
        },
        _ => panic!("Expected Object value"),
    }

    // Clean up
    std::fs::remove_file(file_path).ok();
}

#[test]
fn test_file_read_lines_crlf() {
    use std::fs::File;
    use std::io::Write;

    let vm = VirtualMachine::new();

    // Create a temporary file with Windows-style line endings
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_read_lines_crlf.txt");
    let test_content = "Line 1\r\nLine 2\r\nLine 3";

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_content.as_bytes()).unwrap();
    }

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let args = vec![file_obj];

    // Call readLines()
    let result = native_file_read_lines(&args).unwrap();

    // Verify the result - line endings should be stripped
    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let elements = arr.borrow();
                assert_eq!(elements.len(), 3);

                // Verify each line has no line endings
                if let crate::common::Value::Object(line_obj) = &elements[0] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "Line 1");
                    }
                }

                if let crate::common::Value::Object(line_obj) = &elements[1] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "Line 2");
                    }
                }

                if let crate::common::Value::Object(line_obj) = &elements[2] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "Line 3");
                    }
                }
            }
            _ => panic!("Expected Array object"),
        },
        _ => panic!("Expected Object value"),
    }

    // Clean up
    std::fs::remove_file(file_path).ok();
}

#[test]
fn test_file_read_lines_unicode() {
    use std::fs::File;
    use std::io::Write;

    let vm = VirtualMachine::new();

    // Create a temporary file with Unicode content
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_read_lines_unicode.txt");
    let test_content = "Hello 世界\nΚαλημέρα κόσμε\n🌍🌎🌏";

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_content.as_bytes()).unwrap();
    }

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let args = vec![file_obj];

    // Call readLines()
    let result = native_file_read_lines(&args).unwrap();

    // Verify the result
    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let elements = arr.borrow();
                assert_eq!(elements.len(), 3);

                if let crate::common::Value::Object(line_obj) = &elements[0] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "Hello 世界");
                    }
                }

                if let crate::common::Value::Object(line_obj) = &elements[1] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "Καλημέρα κόσμε");
                    }
                }

                if let crate::common::Value::Object(line_obj) = &elements[2] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "🌍🌎🌏");
                    }
                }
            }
            _ => panic!("Expected Array object"),
        },
        _ => panic!("Expected Object value"),
    }

    // Clean up
    std::fs::remove_file(file_path).ok();
}

#[test]
fn test_file_read_lines_file_not_found() {
    let vm = VirtualMachine::new();

    // Create a File object with a non-existent path
    let file_obj = crate::common::Value::new_file("/nonexistent/path/to/file.txt".to_string());
    let args = vec![file_obj];

    // Call readLines()
    let result = native_file_read_lines(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("File not found"));
    assert!(error_msg.contains("/nonexistent/path/to/file.txt"));
}

#[test]
fn test_file_read_lines_wrong_arg_count() {
    let vm = VirtualMachine::new();

    // Create a File object
    let file_obj = crate::common::Value::new_file("test.txt".to_string());

    // Call readLines() with extra arguments
    let args = vec![file_obj, crate::common::Value::Number(42.0)];
    let result = native_file_read_lines(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("expects 0 arguments"));
}

#[test]
fn test_file_read_lines_invalid_receiver_type() {
    let vm = VirtualMachine::new();

    // Try to call readLines() on a non-File object
    let args = vec![crate::common::Value::Object(Rc::new(Object::String(
        ObjString {
            value: Rc::from("not a file"),
        },
    )))];

    let result = native_file_read_lines(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("can only be called on File objects"));
}

#[test]
fn test_file_read_lines_invalid_receiver_primitive() {
    let vm = VirtualMachine::new();

    // Try to call readLines() on a number
    let args = vec![crate::common::Value::Number(42.0)];

    let result = native_file_read_lines(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("can only be called on File objects"));
}

#[test]
fn test_file_read_lines_empty_lines() {
    use std::fs::File;
    use std::io::Write;

    let vm = VirtualMachine::new();

    // Create a temporary file with empty lines
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_read_lines_empty_lines.txt");
    let test_content = "Line 1\n\nLine 3\n\n";

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_content.as_bytes()).unwrap();
    }

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let args = vec![file_obj];

    // Call readLines()
    let result = native_file_read_lines(&args).unwrap();

    // Verify the result includes empty lines
    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let elements = arr.borrow();
                assert_eq!(elements.len(), 4);

                if let crate::common::Value::Object(line_obj) = &elements[0] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "Line 1");
                    }
                }

                if let crate::common::Value::Object(line_obj) = &elements[1] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "");
                    }
                }

                if let crate::common::Value::Object(line_obj) = &elements[2] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "Line 3");
                    }
                }

                if let crate::common::Value::Object(line_obj) = &elements[3] {
                    if let Object::String(s) = line_obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "");
                    }
                }
            }
            _ => panic!("Expected Array object"),
        },
        _ => panic!("Expected Object value"),
    }

    // Clean up
    std::fs::remove_file(file_path).ok();
}

// Tests for File.write()
#[test]
fn test_file_write_success() {
    let vm = VirtualMachine::new();

    // Create a temporary file path that doesn't exist yet
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_write_success.txt");

    // Clean up any existing file first
    std::fs::remove_file(&file_path).ok();

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let content = crate::common::Value::Object(Rc::new(Object::String(ObjString {
        value: Rc::from("Hello, World!"),
    })));
    let args = vec![file_obj, content];

    // Call write()
    let result = native_file_write(&args).unwrap();

    // Verify the result is Nil
    assert_eq!(result, crate::common::Value::Nil);

    // Verify the file was actually created with the correct content
    let file_content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(file_content, "Hello, World!");

    // Clean up
    std::fs::remove_file(file_path).ok();
}

#[test]
fn test_file_write_empty_content() {
    let vm = VirtualMachine::new();

    // Create a temporary file path that doesn't exist yet
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_write_empty.txt");

    // Clean up any existing file first
    std::fs::remove_file(&file_path).ok();

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let content = crate::common::Value::Object(Rc::new(Object::String(ObjString {
        value: Rc::from(""),
    })));
    let args = vec![file_obj, content];

    // Call write()
    let result = native_file_write(&args).unwrap();

    // Verify the result is Nil
    assert_eq!(result, crate::common::Value::Nil);

    // Verify the file was created and is empty
    let file_content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(file_content, "");

    // Clean up
    std::fs::remove_file(file_path).ok();
}

#[test]
fn test_file_write_multiline_content() {
    let vm = VirtualMachine::new();

    // Create a temporary file path that doesn't exist yet
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_write_multiline.txt");

    // Clean up any existing file first
    std::fs::remove_file(&file_path).ok();

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let test_content = "Line 1\nLine 2\nLine 3";
    let content = crate::common::Value::Object(Rc::new(Object::String(ObjString {
        value: Rc::from(test_content),
    })));
    let args = vec![file_obj, content];

    // Call write()
    let result = native_file_write(&args).unwrap();

    // Verify the result is Nil
    assert_eq!(result, crate::common::Value::Nil);

    // Verify the file was created with the correct content
    let file_content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(file_content, test_content);

    // Clean up
    std::fs::remove_file(file_path).ok();
}

#[test]
fn test_file_write_unicode_content() {
    let vm = VirtualMachine::new();

    // Create a temporary file path that doesn't exist yet
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_write_unicode.txt");

    // Clean up any existing file first
    std::fs::remove_file(&file_path).ok();

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let test_content = "Hello 世界! 🌍 Καλημέρα";
    let content = crate::common::Value::Object(Rc::new(Object::String(ObjString {
        value: Rc::from(test_content),
    })));
    let args = vec![file_obj, content];

    // Call write()
    let result = native_file_write(&args).unwrap();

    // Verify the result is Nil
    assert_eq!(result, crate::common::Value::Nil);

    // Verify the file was created with the correct content
    let file_content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(file_content, test_content);

    // Clean up
    std::fs::remove_file(file_path).ok();
}

#[test]
fn test_file_write_file_already_exists() {
    use std::fs::File;
    use std::io::Write;

    let vm = VirtualMachine::new();

    // Create a temporary file that already exists
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_file_write_exists.txt");

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Existing content").unwrap();
    }

    // Create a File object
    let file_obj = crate::common::Value::new_file(file_path.to_str().unwrap().to_string());
    let content = crate::common::Value::Object(Rc::new(Object::String(ObjString {
        value: Rc::from("New content"),
    })));
    let args = vec![file_obj, content];

    // Call write()
    let result = native_file_write(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("File already exists"));
    assert!(error_msg.contains(file_path.to_str().unwrap()));

    // Verify the file content was not changed
    let file_content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(file_content, "Existing content");

    // Clean up
    std::fs::remove_file(file_path).ok();
}

#[test]
fn test_file_write_wrong_arg_count_zero() {
    let vm = VirtualMachine::new();

    // Create a File object
    let file_obj = crate::common::Value::new_file("test.txt".to_string());

    // Call write() with no content argument
    let args = vec![file_obj];
    let result = native_file_write(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("expects 1 argument"));
}

#[test]
fn test_file_write_wrong_arg_count_too_many() {
    let vm = VirtualMachine::new();

    // Create a File object
    let file_obj = crate::common::Value::new_file("test.txt".to_string());
    let content = crate::common::Value::Object(Rc::new(Object::String(ObjString {
        value: Rc::from("content"),
    })));
    let extra = crate::common::Value::Number(42.0);

    // Call write() with too many arguments
    let args = vec![file_obj, content, extra];
    let result = native_file_write(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("expects 1 argument"));
}

#[test]
fn test_file_write_invalid_content_type_number() {
    let vm = VirtualMachine::new();

    // Create a File object
    let file_obj = crate::common::Value::new_file("test.txt".to_string());
    let content = crate::common::Value::Number(42.0);

    // Call write() with a number instead of string
    let args = vec![file_obj, content];
    let result = native_file_write(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("requires a string argument"));
}

#[test]
fn test_file_write_invalid_content_type_boolean() {
    let vm = VirtualMachine::new();

    // Create a File object
    let file_obj = crate::common::Value::new_file("test.txt".to_string());
    let content = crate::common::Value::Boolean(true);

    // Call write() with a boolean instead of string
    let args = vec![file_obj, content];
    let result = native_file_write(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("requires a string argument"));
}

#[test]
fn test_file_write_invalid_content_type_nil() {
    let vm = VirtualMachine::new();

    // Create a File object
    let file_obj = crate::common::Value::new_file("test.txt".to_string());
    let content = crate::common::Value::Nil;

    // Call write() with nil instead of string
    let args = vec![file_obj, content];
    let result = native_file_write(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("requires a string argument"));
}

#[test]
fn test_file_write_invalid_receiver_type() {
    let vm = VirtualMachine::new();

    // Try to call write() on a non-File object
    let receiver = crate::common::Value::Object(Rc::new(Object::String(ObjString {
        value: Rc::from("not a file"),
    })));
    let content = crate::common::Value::Object(Rc::new(Object::String(ObjString {
        value: Rc::from("content"),
    })));
    let args = vec![receiver, content];

    let result = native_file_write(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("can only be called on File objects"));
}

#[test]
fn test_file_write_invalid_receiver_primitive() {
    let vm = VirtualMachine::new();

    // Try to call write() on a number
    let receiver = crate::common::Value::Number(42.0);
    let content = crate::common::Value::Object(Rc::new(Object::String(ObjString {
        value: Rc::from("content"),
    })));
    let args = vec![receiver, content];

    let result = native_file_write(&args);

    // Verify it returns an error
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("can only be called on File objects"));
}

#[test]
fn test_file_write_invalid_directory() {
    let vm = VirtualMachine::new();

    // Create a File object with a path in a non-existent directory
    let file_obj = crate::common::Value::new_file("/nonexistent/directory/file.txt".to_string());
    let content = crate::common::Value::Object(Rc::new(Object::String(ObjString {
        value: Rc::from("content"),
    })));
    let args = vec![file_obj, content];

    // Call write()
    let result = native_file_write(&args);

    // Verify it returns an error (either directory not found or failed to write)
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Directory not found") || error_msg.contains("Failed to write file")
    );
}
