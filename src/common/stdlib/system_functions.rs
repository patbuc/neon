use crate::common::Value;
use crate::extract_arg;

/// Joins print() arguments with spaces, the shared formatting used for both
/// stdout output and the in-VM output buffer (used in tests and on wasm).
pub fn format_print_args(args: &[Value]) -> String {
    args.iter()
        .map(|v: &Value| v.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Native implementation of print(): writes its arguments, space-joined, to stdout.
pub fn native_system_print(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("print() expects at least 1 argument".to_string());
    }

    let output = format_print_args(args);

    // Print to stdout
    #[cfg(not(target_arch = "wasm32"))]
    println!("{}", output);

    Ok(Value::Nil)
}

/// Native implementation of sleep(ms): blocks the current thread for `ms`
/// milliseconds (fractional values honoured).
pub fn native_system_sleep(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("sleep() expects 1 argument, got {}", args.len()));
    }

    let ms = extract_arg!(args, 0, Number, "ms", "sleep")?;

    #[cfg(target_arch = "wasm32")]
    {
        let _ = ms;
        return Err("sleep() is not supported in the browser".to_string());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Duration::try_from_secs_f64(ms / 1000.0)
            .map_err(|_| "sleep() requires a non-negative number of milliseconds".to_string())?;
        std::thread::sleep(duration);
        Ok(Value::Nil)
    }
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

    #[test]
    fn test_sleep_blocks_for_at_least_the_given_duration() {
        let args = vec![number!(50.0)];
        let start = std::time::Instant::now();
        let result = native_system_sleep(&args);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
        assert!(start.elapsed() >= std::time::Duration::from_millis(50));
    }

    #[test]
    fn test_sleep_zero_returns_immediately() {
        let args = vec![number!(0.0)];
        let result = native_system_sleep(&args);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn test_sleep_wrong_argument_count() {
        let result = native_system_sleep(&[]);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "sleep() expects 1 argument, got 0");
    }

    #[test]
    fn test_sleep_non_number_argument() {
        let args = vec![string!("x")];
        let result = native_system_sleep(&args);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "sleep() ms must be a number");
    }

    #[test]
    fn test_sleep_negative_argument() {
        let args = vec![number!(-1.0)];
        let result = native_system_sleep(&args);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "sleep() requires a non-negative number of milliseconds"
        );
    }

    #[test]
    fn test_sleep_nan_argument() {
        let args = vec![number!(f64::NAN)];
        let result = native_system_sleep(&args);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "sleep() requires a non-negative number of milliseconds"
        );
    }

    #[test]
    fn test_sleep_infinite_argument() {
        let args = vec![number!(f64::INFINITY)];
        let result = native_system_sleep(&args);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "sleep() requires a non-negative number of milliseconds"
        );
    }
}
