use crate::common::{Object, Value, ObjString};
use crate::vm::VirtualMachine;
use std::rc::Rc;

/// Native implementation of File(path) constructor
/// Creates a new File object with the given path
pub fn native_file_constructor(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("File() expects 1 argument, got {}", args.len()));
    }

    match &args[0] {
        Value::Object(obj) => {
            if let Object::String(s) = obj.as_ref() {
                Ok(Value::new_file(s.value.to_string()))
            } else {
                Err("File() requires a string argument".to_string())
            }
        }
        _ => Err("File() requires a string argument".to_string()),
    }
}

/// Native implementation of File.read()
/// Reads the entire contents of the file and returns it as a string
pub fn native_file_read(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("read() expects 0 arguments (only receiver), got {}", args.len() - 1));
    }

    // Extract the File object from args[0] (the receiver)
    let file_path = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::File(path) => path,
            _ => return Err("read() can only be called on File objects".to_string()),
        },
        _ => return Err("read() can only be called on File objects".to_string()),
    };

    // Read the file contents using std::fs::read_to_string
    match std::fs::read_to_string(file_path.as_ref()) {
        Ok(contents) => {
            // Return the contents as a String value
            Ok(Value::Object(Rc::new(Object::String(ObjString {
                value: Rc::from(contents),
            }))))
        }
        Err(e) => {
            // Return descriptive error message based on error kind
            let error_msg = match e.kind() {
                std::io::ErrorKind::NotFound => {
                    format!("File not found: {}", file_path.as_ref())
                }
                std::io::ErrorKind::PermissionDenied => {
                    format!("Permission denied: {}", file_path.as_ref())
                }
                _ => {
                    format!("Failed to read file '{}': {}", file_path.as_ref(), e)
                }
            };
            Err(error_msg)
        }
    }
}

/// Native implementation of File.readLines()
/// Reads the file and returns an array of strings, one per line
/// Line endings (\n, \r\n) are stripped from each line
pub fn native_file_read_lines(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("readLines() expects 0 arguments (only receiver), got {}", args.len() - 1));
    }

    // Extract the File object from args[0] (the receiver)
    let file_path = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::File(path) => path,
            _ => return Err("readLines() can only be called on File objects".to_string()),
        },
        _ => return Err("readLines() can only be called on File objects".to_string()),
    };

    // Read the file contents using std::fs::read_to_string
    match std::fs::read_to_string(file_path.as_ref()) {
        Ok(contents) => {
            // Split by lines - this automatically strips \n and \r\n line endings
            let lines: Vec<Value> = contents
                .lines()
                .map(|line| {
                    Value::Object(Rc::new(Object::String(ObjString {
                        value: Rc::from(line),
                    })))
                })
                .collect();

            // Return an array of string values
            Ok(Value::new_array(lines))
        }
        Err(e) => {
            // Return descriptive error message based on error kind
            let error_msg = match e.kind() {
                std::io::ErrorKind::NotFound => {
                    format!("File not found: {}", file_path.as_ref())
                }
                std::io::ErrorKind::PermissionDenied => {
                    format!("Permission denied: {}", file_path.as_ref())
                }
                _ => {
                    format!("Failed to read file '{}': {}", file_path.as_ref(), e)
                }
            };
            Err(error_msg)
        }
    }
}

/// Native implementation of File.write()
/// Writes content to the file, fails if file already exists for safety
pub fn native_file_write(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("write() expects 1 argument, got {}", args.len() - 1));
    }

    // Extract the File object from args[0] (the receiver)
    let file_path = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::File(path) => path,
            _ => return Err("write() can only be called on File objects".to_string()),
        },
        _ => return Err("write() can only be called on File objects".to_string()),
    };

    // Extract the content string from args[1]
    let content = match &args[1] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => s.value.as_ref(),
            _ => return Err("write() requires a string argument".to_string()),
        },
        _ => return Err("write() requires a string argument".to_string()),
    };

    // Check if file already exists
    if std::path::Path::new(file_path.as_ref()).exists() {
        return Err(format!("File already exists: {}", file_path.as_ref()));
    }

    // Write the content to the file
    match std::fs::write(file_path.as_ref(), content) {
        Ok(()) => Ok(Value::Nil),
        Err(e) => {
            // Return descriptive error message based on error kind
            let error_msg = match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    format!("Permission denied: {}", file_path.as_ref())
                }
                std::io::ErrorKind::NotFound => {
                    format!("Directory not found for file: {}", file_path.as_ref())
                }
                _ => {
                    format!("Failed to write file '{}': {}", file_path.as_ref(), e)
                }
            };
            Err(error_msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Object, ObjString};
    use std::rc::Rc;

    #[test]
    fn test_file_constructor_valid_path() {
        let mut vm = VirtualMachine::new(Vec::new());
        let path = "test.txt";
        let args = vec![Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from(path),
        })))];

        let result = native_file_constructor(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
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
        let mut vm = VirtualMachine::new(Vec::new());
        let args = vec![];
        let result = native_file_constructor(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "File() expects 1 argument, got 0");
    }

    #[test]
    fn test_file_constructor_wrong_arg_count_two() {
        let mut vm = VirtualMachine::new(Vec::new());
        let args = vec![
            Value::Object(Rc::new(Object::String(ObjString {
                value: Rc::from("test.txt"),
            }))),
            Value::Object(Rc::new(Object::String(ObjString {
                value: Rc::from("extra.txt"),
            }))),
        ];
        let result = native_file_constructor(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "File() expects 1 argument, got 2");
    }

    #[test]
    fn test_file_constructor_invalid_type_number() {
        let mut vm = VirtualMachine::new(Vec::new());
        let args = vec![Value::Number(42.0)];
        let result = native_file_constructor(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "File() requires a string argument"
        );
    }

    #[test]
    fn test_file_constructor_invalid_type_boolean() {
        let mut vm = VirtualMachine::new(Vec::new());
        let args = vec![Value::Boolean(true)];
        let result = native_file_constructor(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "File() requires a string argument"
        );
    }

    #[test]
    fn test_file_constructor_invalid_type_nil() {
        let mut vm = VirtualMachine::new(Vec::new());
        let args = vec![Value::Nil];
        let result = native_file_constructor(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "File() requires a string argument"
        );
    }

    #[test]
    fn test_file_constructor_with_relative_path() {
        let mut vm = VirtualMachine::new(Vec::new());
        let path = "../data/input.txt";
        let args = vec![Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from(path),
        })))];

        let result = native_file_constructor(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
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
        let mut vm = VirtualMachine::new(Vec::new());
        let path = "/home/user/data/input.txt";
        let args = vec![Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from(path),
        })))];

        let result = native_file_constructor(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
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
        use std::io::Write;
        use std::fs::File;

        let mut vm = VirtualMachine::new(Vec::new());

        // Create a temporary file with some content
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_read.txt");
        let test_content = "Hello, World!\nThis is a test file.";

        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(test_content.as_bytes()).unwrap();
        }

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let args = vec![file_obj];

        // Call read()
        let result = native_file_read(&mut vm, &args).unwrap();

        // Verify the result
        match result {
            Value::Object(obj) => match obj.as_ref() {
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

        let mut vm = VirtualMachine::new(Vec::new());

        // Create an empty temporary file
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_read_empty.txt");

        {
            File::create(&file_path).unwrap();
        }

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let args = vec![file_obj];

        // Call read()
        let result = native_file_read(&mut vm, &args).unwrap();

        // Verify the result is an empty string
        match result {
            Value::Object(obj) => match obj.as_ref() {
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
        use std::io::Write;
        use std::fs::File;

        let mut vm = VirtualMachine::new(Vec::new());

        // Create a temporary file with Unicode content
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_read_unicode.txt");
        let test_content = "Hello 世界! 🌍 Καλημέρα";

        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(test_content.as_bytes()).unwrap();
        }

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let args = vec![file_obj];

        // Call read()
        let result = native_file_read(&mut vm, &args).unwrap();

        // Verify the result
        match result {
            Value::Object(obj) => match obj.as_ref() {
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
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a File object with a non-existent path
        let file_obj = Value::new_file("/nonexistent/path/to/file.txt".to_string());
        let args = vec![file_obj];

        // Call read()
        let result = native_file_read(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("File not found"));
        assert!(error_msg.contains("/nonexistent/path/to/file.txt"));
    }

    #[test]
    fn test_file_read_wrong_arg_count() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a File object
        let file_obj = Value::new_file("test.txt".to_string());

        // Call read() with extra arguments
        let args = vec![file_obj, Value::Number(42.0)];
        let result = native_file_read(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("expects 0 arguments"));
    }

    #[test]
    fn test_file_read_invalid_receiver_type() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Try to call read() on a non-File object
        let args = vec![Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from("not a file"),
        })))];

        let result = native_file_read(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("can only be called on File objects"));
    }

    #[test]
    fn test_file_read_invalid_receiver_primitive() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Try to call read() on a number
        let args = vec![Value::Number(42.0)];

        let result = native_file_read(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("can only be called on File objects"));
    }

    #[test]
    fn test_file_read_multiline_content() {
        use std::io::Write;
        use std::fs::File;

        let mut vm = VirtualMachine::new(Vec::new());

        // Create a temporary file with multiline content
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_read_multiline.txt");
        let test_content = "Line 1\nLine 2\nLine 3\nLine 4";

        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(test_content.as_bytes()).unwrap();
        }

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let args = vec![file_obj];

        // Call read()
        let result = native_file_read(&mut vm, &args).unwrap();

        // Verify the result
        match result {
            Value::Object(obj) => match obj.as_ref() {
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
        use std::io::Write;
        use std::fs::File;

        let mut vm = VirtualMachine::new(Vec::new());

        // Create a temporary file with multiline content
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_read_lines.txt");
        let test_content = "Line 1\nLine 2\nLine 3";

        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(test_content.as_bytes()).unwrap();
        }

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let args = vec![file_obj];

        // Call readLines()
        let result = native_file_read_lines(&mut vm, &args).unwrap();

        // Verify the result is an array
        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(arr) => {
                    let elements = arr.borrow();
                    assert_eq!(elements.len(), 3);

                    // Check each line
                    if let Value::Object(line_obj) = &elements[0] {
                        if let Object::String(s) = line_obj.as_ref() {
                            assert_eq!(s.value.as_ref(), "Line 1");
                        } else {
                            panic!("Expected String object");
                        }
                    }

                    if let Value::Object(line_obj) = &elements[1] {
                        if let Object::String(s) = line_obj.as_ref() {
                            assert_eq!(s.value.as_ref(), "Line 2");
                        } else {
                            panic!("Expected String object");
                        }
                    }

                    if let Value::Object(line_obj) = &elements[2] {
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

        let mut vm = VirtualMachine::new(Vec::new());

        // Create an empty temporary file
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_read_lines_empty.txt");

        {
            File::create(&file_path).unwrap();
        }

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let args = vec![file_obj];

        // Call readLines()
        let result = native_file_read_lines(&mut vm, &args).unwrap();

        // Verify the result is an empty array
        match result {
            Value::Object(obj) => match obj.as_ref() {
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
        use std::io::Write;
        use std::fs::File;

        let mut vm = VirtualMachine::new(Vec::new());

        // Create a temporary file with a single line (no newline at end)
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_read_lines_single.txt");
        let test_content = "Single line";

        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(test_content.as_bytes()).unwrap();
        }

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let args = vec![file_obj];

        // Call readLines()
        let result = native_file_read_lines(&mut vm, &args).unwrap();

        // Verify the result
        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(arr) => {
                    let elements = arr.borrow();
                    assert_eq!(elements.len(), 1);

                    if let Value::Object(line_obj) = &elements[0] {
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
        use std::io::Write;
        use std::fs::File;

        let mut vm = VirtualMachine::new(Vec::new());

        // Create a temporary file with Windows-style line endings
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_read_lines_crlf.txt");
        let test_content = "Line 1\r\nLine 2\r\nLine 3";

        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(test_content.as_bytes()).unwrap();
        }

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let args = vec![file_obj];

        // Call readLines()
        let result = native_file_read_lines(&mut vm, &args).unwrap();

        // Verify the result - line endings should be stripped
        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(arr) => {
                    let elements = arr.borrow();
                    assert_eq!(elements.len(), 3);

                    // Verify each line has no line endings
                    if let Value::Object(line_obj) = &elements[0] {
                        if let Object::String(s) = line_obj.as_ref() {
                            assert_eq!(s.value.as_ref(), "Line 1");
                        }
                    }

                    if let Value::Object(line_obj) = &elements[1] {
                        if let Object::String(s) = line_obj.as_ref() {
                            assert_eq!(s.value.as_ref(), "Line 2");
                        }
                    }

                    if let Value::Object(line_obj) = &elements[2] {
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
        use std::io::Write;
        use std::fs::File;

        let mut vm = VirtualMachine::new(Vec::new());

        // Create a temporary file with Unicode content
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_read_lines_unicode.txt");
        let test_content = "Hello 世界\nΚαλημέρα κόσμε\n🌍🌎🌏";

        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(test_content.as_bytes()).unwrap();
        }

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let args = vec![file_obj];

        // Call readLines()
        let result = native_file_read_lines(&mut vm, &args).unwrap();

        // Verify the result
        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(arr) => {
                    let elements = arr.borrow();
                    assert_eq!(elements.len(), 3);

                    if let Value::Object(line_obj) = &elements[0] {
                        if let Object::String(s) = line_obj.as_ref() {
                            assert_eq!(s.value.as_ref(), "Hello 世界");
                        }
                    }

                    if let Value::Object(line_obj) = &elements[1] {
                        if let Object::String(s) = line_obj.as_ref() {
                            assert_eq!(s.value.as_ref(), "Καλημέρα κόσμε");
                        }
                    }

                    if let Value::Object(line_obj) = &elements[2] {
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
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a File object with a non-existent path
        let file_obj = Value::new_file("/nonexistent/path/to/file.txt".to_string());
        let args = vec![file_obj];

        // Call readLines()
        let result = native_file_read_lines(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("File not found"));
        assert!(error_msg.contains("/nonexistent/path/to/file.txt"));
    }

    #[test]
    fn test_file_read_lines_wrong_arg_count() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a File object
        let file_obj = Value::new_file("test.txt".to_string());

        // Call readLines() with extra arguments
        let args = vec![file_obj, Value::Number(42.0)];
        let result = native_file_read_lines(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("expects 0 arguments"));
    }

    #[test]
    fn test_file_read_lines_invalid_receiver_type() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Try to call readLines() on a non-File object
        let args = vec![Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from("not a file"),
        })))];

        let result = native_file_read_lines(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("can only be called on File objects"));
    }

    #[test]
    fn test_file_read_lines_invalid_receiver_primitive() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Try to call readLines() on a number
        let args = vec![Value::Number(42.0)];

        let result = native_file_read_lines(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("can only be called on File objects"));
    }

    #[test]
    fn test_file_read_lines_empty_lines() {
        use std::io::Write;
        use std::fs::File;

        let mut vm = VirtualMachine::new(Vec::new());

        // Create a temporary file with empty lines
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_read_lines_empty_lines.txt");
        let test_content = "Line 1\n\nLine 3\n\n";

        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(test_content.as_bytes()).unwrap();
        }

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let args = vec![file_obj];

        // Call readLines()
        let result = native_file_read_lines(&mut vm, &args).unwrap();

        // Verify the result includes empty lines
        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(arr) => {
                    let elements = arr.borrow();
                    assert_eq!(elements.len(), 4);

                    if let Value::Object(line_obj) = &elements[0] {
                        if let Object::String(s) = line_obj.as_ref() {
                            assert_eq!(s.value.as_ref(), "Line 1");
                        }
                    }

                    if let Value::Object(line_obj) = &elements[1] {
                        if let Object::String(s) = line_obj.as_ref() {
                            assert_eq!(s.value.as_ref(), "");
                        }
                    }

                    if let Value::Object(line_obj) = &elements[2] {
                        if let Object::String(s) = line_obj.as_ref() {
                            assert_eq!(s.value.as_ref(), "Line 3");
                        }
                    }

                    if let Value::Object(line_obj) = &elements[3] {
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
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a temporary file path that doesn't exist yet
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_write_success.txt");

        // Clean up any existing file first
        std::fs::remove_file(&file_path).ok();

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let content = Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from("Hello, World!"),
        })));
        let args = vec![file_obj, content];

        // Call write()
        let result = native_file_write(&mut vm, &args).unwrap();

        // Verify the result is Nil
        assert_eq!(result, Value::Nil);

        // Verify the file was actually created with the correct content
        let file_content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, "Hello, World!");

        // Clean up
        std::fs::remove_file(file_path).ok();
    }

    #[test]
    fn test_file_write_empty_content() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a temporary file path that doesn't exist yet
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_write_empty.txt");

        // Clean up any existing file first
        std::fs::remove_file(&file_path).ok();

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let content = Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from(""),
        })));
        let args = vec![file_obj, content];

        // Call write()
        let result = native_file_write(&mut vm, &args).unwrap();

        // Verify the result is Nil
        assert_eq!(result, Value::Nil);

        // Verify the file was created and is empty
        let file_content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, "");

        // Clean up
        std::fs::remove_file(file_path).ok();
    }

    #[test]
    fn test_file_write_multiline_content() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a temporary file path that doesn't exist yet
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_write_multiline.txt");

        // Clean up any existing file first
        std::fs::remove_file(&file_path).ok();

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let test_content = "Line 1\nLine 2\nLine 3";
        let content = Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from(test_content),
        })));
        let args = vec![file_obj, content];

        // Call write()
        let result = native_file_write(&mut vm, &args).unwrap();

        // Verify the result is Nil
        assert_eq!(result, Value::Nil);

        // Verify the file was created with the correct content
        let file_content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, test_content);

        // Clean up
        std::fs::remove_file(file_path).ok();
    }

    #[test]
    fn test_file_write_unicode_content() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a temporary file path that doesn't exist yet
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_write_unicode.txt");

        // Clean up any existing file first
        std::fs::remove_file(&file_path).ok();

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let test_content = "Hello 世界! 🌍 Καλημέρα";
        let content = Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from(test_content),
        })));
        let args = vec![file_obj, content];

        // Call write()
        let result = native_file_write(&mut vm, &args).unwrap();

        // Verify the result is Nil
        assert_eq!(result, Value::Nil);

        // Verify the file was created with the correct content
        let file_content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, test_content);

        // Clean up
        std::fs::remove_file(file_path).ok();
    }

    #[test]
    fn test_file_write_file_already_exists() {
        use std::io::Write;
        use std::fs::File;

        let mut vm = VirtualMachine::new(Vec::new());

        // Create a temporary file that already exists
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_file_write_exists.txt");

        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(b"Existing content").unwrap();
        }

        // Create a File object
        let file_obj = Value::new_file(file_path.to_str().unwrap().to_string());
        let content = Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from("New content"),
        })));
        let args = vec![file_obj, content];

        // Call write()
        let result = native_file_write(&mut vm, &args);

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
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a File object
        let file_obj = Value::new_file("test.txt".to_string());

        // Call write() with no content argument
        let args = vec![file_obj];
        let result = native_file_write(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("expects 1 argument"));
    }

    #[test]
    fn test_file_write_wrong_arg_count_too_many() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a File object
        let file_obj = Value::new_file("test.txt".to_string());
        let content = Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from("content"),
        })));
        let extra = Value::Number(42.0);

        // Call write() with too many arguments
        let args = vec![file_obj, content, extra];
        let result = native_file_write(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("expects 1 argument"));
    }

    #[test]
    fn test_file_write_invalid_content_type_number() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a File object
        let file_obj = Value::new_file("test.txt".to_string());
        let content = Value::Number(42.0);

        // Call write() with a number instead of string
        let args = vec![file_obj, content];
        let result = native_file_write(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("requires a string argument"));
    }

    #[test]
    fn test_file_write_invalid_content_type_boolean() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a File object
        let file_obj = Value::new_file("test.txt".to_string());
        let content = Value::Boolean(true);

        // Call write() with a boolean instead of string
        let args = vec![file_obj, content];
        let result = native_file_write(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("requires a string argument"));
    }

    #[test]
    fn test_file_write_invalid_content_type_nil() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a File object
        let file_obj = Value::new_file("test.txt".to_string());
        let content = Value::Nil;

        // Call write() with nil instead of string
        let args = vec![file_obj, content];
        let result = native_file_write(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("requires a string argument"));
    }

    #[test]
    fn test_file_write_invalid_receiver_type() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Try to call write() on a non-File object
        let receiver = Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from("not a file"),
        })));
        let content = Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from("content"),
        })));
        let args = vec![receiver, content];

        let result = native_file_write(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("can only be called on File objects"));
    }

    #[test]
    fn test_file_write_invalid_receiver_primitive() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Try to call write() on a number
        let receiver = Value::Number(42.0);
        let content = Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from("content"),
        })));
        let args = vec![receiver, content];

        let result = native_file_write(&mut vm, &args);

        // Verify it returns an error
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("can only be called on File objects"));
    }

    #[test]
    fn test_file_write_invalid_directory() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create a File object with a path in a non-existent directory
        let file_obj = Value::new_file("/nonexistent/directory/file.txt".to_string());
        let content = Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from("content"),
        })));
        let args = vec![file_obj, content];

        // Call write()
        let result = native_file_write(&mut vm, &args);

        // Verify it returns an error (either directory not found or failed to write)
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(
            error_msg.contains("Directory not found") || error_msg.contains("Failed to write file")
        );
    }
}
