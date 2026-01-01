//! Macros for extracting typed values from stdlib function arguments
//!
//! This module provides macros to eliminate boilerplate code when extracting
//! typed receivers and arguments from the `Value` enum in native stdlib functions.
//!
//! # Usage Examples
//!
//! ```ignore
//! // Extract a string receiver (args[0])
//! let obj_string = extract_receiver!(args, String, "len")?;
//!
//! // Extract an array receiver
//! let array_ref = extract_receiver!(args, Array, "push")?;
//!
//! // Extract a number argument at index 1
//! let index = extract_arg!(args, 1, Number, "index", "substring")?;
//!
//! // Extract a string value (unwraps ObjString.value)
//! let delimiter = extract_string_value!(args, 1, "delimiter", "split")?;
//! ```

/// Extract a typed receiver (args[0]) from the arguments slice.
///
/// Returns a `Result` with the extracted typed reference or an error message.
///
/// # Supported Types
///
/// - `String` → `&ObjString`
/// - `Array` → `&Rc<RefCell<Vec<Value>>>`
/// - `Map` → `&Rc<RefCell<HashMap<MapKey, Value>>>`
/// - `Set` → `&Rc<RefCell<BTreeSet<SetKey>>>`
/// - `File` → `&Rc<str>`
/// - `Number` → `f64`
/// - `Boolean` → `bool`
///
/// # Examples
///
/// ```ignore
/// let obj_string = extract_receiver!(args, String, "len")?;
/// let array_ref = extract_receiver!(args, Array, "push")?;
/// let num = extract_receiver!(args, Number, "abs")?;
/// ```
#[macro_export]
macro_rules! extract_receiver {
    // String extraction
    ($args:expr, String, $method:expr) => {
        match $args.get(0) {
            Some(Value::Object(obj)) => match obj.as_ref() {
                Object::String(s) => Ok(s),
                _ => Err(format!("{}() can only be called on strings", $method)),
            },
            Some(_) => Err(format!("{}() can only be called on strings", $method)),
            None => Err(format!("{}() can only be called on strings", $method)),
        }
    };

    // Array extraction
    ($args:expr, Array, $method:expr) => {
        match $args.get(0) {
            Some(Value::Object(obj)) => match obj.as_ref() {
                Object::Array(arr) => Ok(arr),
                _ => Err(format!("{}() can only be called on arrays", $method)),
            },
            Some(_) => Err(format!("{}() can only be called on arrays", $method)),
            None => Err(format!("{}() can only be called on arrays", $method)),
        }
    };

    // Map extraction
    ($args:expr, Map, $method:expr) => {
        match $args.get(0) {
            Some(Value::Object(obj)) => match obj.as_ref() {
                Object::Map(m) => Ok(m),
                _ => Err(format!("{}() can only be called on maps", $method)),
            },
            Some(_) => Err(format!("{}() can only be called on maps", $method)),
            None => Err(format!("{}() can only be called on maps", $method)),
        }
    };

    // Set extraction
    ($args:expr, Set, $method:expr) => {
        match $args.get(0) {
            Some(Value::Object(obj)) => match obj.as_ref() {
                Object::Set(s) => Ok(s),
                _ => Err(format!("{}() can only be called on sets", $method)),
            },
            Some(_) => Err(format!("{}() can only be called on sets", $method)),
            None => Err(format!("{}() can only be called on sets", $method)),
        }
    };

    // File extraction
    ($args:expr, File, $method:expr) => {
        match $args.get(0) {
            Some(Value::Object(obj)) => match obj.as_ref() {
                Object::File(f) => Ok(f),
                _ => Err(format!("{}() can only be called on File objects", $method)),
            },
            Some(_) => Err(format!("{}() can only be called on File objects", $method)),
            None => Err(format!("{}() can only be called on File objects", $method)),
        }
    };

    // Number extraction (direct from Value)
    ($args:expr, Number, $method:expr) => {
        match $args.get(0) {
            Some(Value::Number(n)) => Ok(*n),
            Some(_) => Err(format!("{}() can only be called on numbers", $method)),
            None => Err(format!("{}() can only be called on numbers", $method)),
        }
    };

    // Boolean extraction (direct from Value)
    ($args:expr, Boolean, $method:expr) => {
        match $args.get(0) {
            Some(Value::Boolean(b)) => Ok(*b),
            Some(_) => Err(format!("{}() can only be called on booleans", $method)),
            None => Err(format!("{}() can only be called on booleans", $method)),
        }
    };
}

/// Extract a typed argument at a specific index.
///
/// Returns a `Result` with the extracted typed reference or an error message.
///
/// # Supported Types
///
/// - `String` → `&ObjString`
/// - `Number` → `f64`
/// - `Set` → `&Rc<RefCell<BTreeSet<SetKey>>>`
/// - `Array` → `&Rc<RefCell<Vec<Value>>>`
///
/// # Examples
///
/// ```ignore
/// let delimiter = extract_arg!(args, 1, String, "delimiter", "split")?;
/// let index = extract_arg!(args, 1, Number, "index", "substring")?;
/// let other_set = extract_arg!(args, 1, Set, "other set", "union")?;
/// ```
///
/// # New calling convention
/// With unified calling [receiver, args...], indices are direct (no +1 offset)
#[macro_export]
macro_rules! extract_arg {
    // String argument
    ($args:expr, $idx:expr, String, $arg_name:expr, $method:expr) => {
        match $args.get($idx) {
            Some(Value::Object(obj)) => match obj.as_ref() {
                Object::String(s) => Ok(s),
                _ => Err(format!("{}() {} must be a string", $method, $arg_name)),
            },
            Some(_) => Err(format!("{}() {} must be a string", $method, $arg_name)),
            None => Err(format!(
                "{}() missing required argument: {}",
                $method, $arg_name
            )),
        }
    };

    // Number argument
    ($args:expr, $idx:expr, Number, $arg_name:expr, $method:expr) => {
        match $args.get($idx) {
            Some(Value::Number(n)) => Ok(*n),
            Some(_) => Err(format!("{}() {} must be a number", $method, $arg_name)),
            None => Err(format!(
                "{}() missing required argument: {}",
                $method, $arg_name
            )),
        }
    };

    // Set argument
    ($args:expr, $idx:expr, Set, $arg_name:expr, $method:expr) => {
        match $args.get($idx) {
            Some(Value::Object(obj)) => match obj.as_ref() {
                Object::Set(s) => Ok(s),
                _ => Err(format!("{}() {} must be a set", $method, $arg_name)),
            },
            Some(_) => Err(format!("{}() {} must be a set", $method, $arg_name)),
            None => Err(format!(
                "{}() missing required argument: {}",
                $method, $arg_name
            )),
        }
    };

    // Array argument
    ($args:expr, $idx:expr, Array, $arg_name:expr, $method:expr) => {
        match $args.get($idx) {
            Some(Value::Object(obj)) => match obj.as_ref() {
                Object::Array(arr) => Ok(arr),
                _ => Err(format!("{}() {} must be an array", $method, $arg_name)),
            },
            Some(_) => Err(format!("{}() {} must be an array", $method, $arg_name)),
            None => Err(format!(
                "{}() missing required argument: {}",
                $method, $arg_name
            )),
        }
    };
}

/// Extract the &str value from a String argument (unwraps ObjString.value).
///
/// This is a convenience macro that combines `extract_arg!` with unwrapping
/// the inner string value, which is a common pattern in string operations.
///
/// # Example
///
/// ```ignore
/// // Instead of:
/// let obj_string = extract_arg!(args, 1, String, "delimiter", "split")?;
/// let delimiter = obj_string.value.as_ref();
///
/// // Use:
/// let delimiter = extract_string_value!(args, 1, "delimiter", "split");
/// ```
#[macro_export]
macro_rules! extract_string_value {
    ($args:expr, $idx:expr, $arg_name:expr, $method:expr) => {{
        extract_arg!($args, $idx, String, $arg_name, $method)?
            .value
            .as_ref()
    }};
}
