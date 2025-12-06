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
