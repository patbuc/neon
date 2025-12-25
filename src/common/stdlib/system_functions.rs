use crate::common::Value;

/// Native print(function - this is a placeholder that should never be called)
/// The VM handles print(function calls directly in handle_print_function())
/// This function exists only to satisfy the method registry requirements
pub fn native_system_print(args: &[Value]) -> Result<Value, String> {
    // This should never be called - the VM intercepts print(calls)
    // But if it is called somehow, provide basic functionality
    if args.is_empty() {
        return Err("print() expects at least 1 argument".to_string());
    }

    // Join all arguments with spaces
    let output = args
        .iter()
        .map(|v: &Value| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    // Print to stdout as fallback
    println!("{}", output);

    Ok(Value::Nil)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{boolean, number, string};

    #[test]
    fn test_print_single_argument() {
        let args = vec![number!(42.0)];
        let result = native_system_print(&args);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn test_print_multiple_arguments() {
        let args = vec![number!(1.0), number!(2.0), number!(3.0)];
        let result = native_system_print(&args);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn test_print_mixed_types() {
        let args = vec![string!("Hello"), number!(42.0), boolean!(true)];
        let result = native_system_print(&args);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn test_print_no_arguments() {
        let args = vec![];
        let result = native_system_print(&args);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "print() expects at least 1 argument");
    }

    #[test]
    fn test_print_string_argument() {
        let args = vec![string!("Hello World")];
        let result = native_system_print(&args);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
    }
}
