use crate::common::Value;
#[cfg(not(target_arch = "wasm32"))]
use std::rc::Rc;

/// Native implementation of Stdin.read()
/// Reads all remaining standard input and returns it as a string. A call
/// after EOF has already been reached returns "".
#[cfg(not(target_arch = "wasm32"))]
pub fn native_stdin_read(args: &[Value]) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(format!("read() expects 0 arguments, got {}", args.len()));
    }

    let contents = read_stdin_to_string("read")?;
    Ok(Value::String(Rc::new(contents)))
}

/// Native implementation of Stdin.readLines()
/// Reads all remaining standard input and returns an array of lines, split
/// the same way as File.readLines().
#[cfg(not(target_arch = "wasm32"))]
pub fn native_stdin_read_lines(args: &[Value]) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(format!(
            "readLines() expects 0 arguments, got {}",
            args.len()
        ));
    }

    let contents = read_stdin_to_string("readLines")?;
    let lines: Vec<Value> = contents
        .lines()
        .map(|line| Value::String(Rc::new(line.to_string())))
        .collect();
    Ok(Value::new_array(lines))
}

#[cfg(not(target_arch = "wasm32"))]
fn read_stdin_to_string(method: &str) -> Result<String, String> {
    use std::io::Read;

    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .map_err(|e| format!("{}() failed to read stdin: {}", method, e))?;
    Ok(buf)
}

/// Native implementation of Stdin.read() on wasm32: there is no standard
/// input in the browser, so this always errors instead.
#[cfg(target_arch = "wasm32")]
pub fn native_stdin_read(_args: &[Value]) -> Result<Value, String> {
    Err("Stdin is not supported in the browser".to_string())
}

/// Native implementation of Stdin.readLines() on wasm32: see
/// `native_stdin_read`.
#[cfg(target_arch = "wasm32")]
pub fn native_stdin_read_lines(_args: &[Value]) -> Result<Value, String> {
    Err("Stdin is not supported in the browser".to_string())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn read_rejects_arguments() {
        let args = vec![Value::Nil];
        let result = native_stdin_read(&args);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "read() expects 0 arguments, got 1");
    }

    #[test]
    fn read_lines_rejects_arguments() {
        let args = vec![Value::Nil];
        let result = native_stdin_read_lines(&args);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "readLines() expects 0 arguments, got 1"
        );
    }
}
