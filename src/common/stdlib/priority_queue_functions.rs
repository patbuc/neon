use crate::common::{Numeric, Value};
use crate::extract_receiver;

/// Native implementation of `pq.new()`.
pub fn native_priority_queue_constructor(args: &[Value]) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(format!("pq.new() expects 0 arguments, got {}", args.len()));
    }

    Ok(Value::new_priority_queue())
}

/// Native implementation of PriorityQueue.push(priority, value). Returns nil.
pub fn native_priority_queue_push(args: &[Value]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "push() expects 2 arguments (priority, value), got {}",
            args.len() - 1
        ));
    }

    let pq = extract_receiver!(args, PriorityQueue, "push")?;
    let priority = match Numeric::from_value(&args[1]) {
        Some(Numeric::Float(n)) if n.is_nan() => {
            return Err("push() priority must not be NaN".to_string());
        }
        Some(numeric) => numeric,
        None => {
            return Err(format!(
                "push() priority must be a number, got {}",
                args[1].type_name()
            ));
        }
    };

    pq.borrow_mut().push(priority, args[2].clone());
    Ok(Value::Nil)
}

/// Native implementation of PriorityQueue.pop(). Removes and returns the
/// value with the smallest priority, or nil if the queue is empty.
pub fn native_priority_queue_pop(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "pop() expects 0 arguments (only receiver), got {}",
            args.len() - 1
        ));
    }

    let pq = extract_receiver!(args, PriorityQueue, "pop")?;
    Ok(pq.borrow_mut().pop().unwrap_or(Value::Nil))
}

/// Native implementation of PriorityQueue.peek(). Same as pop() but leaves
/// the queue unchanged.
pub fn native_priority_queue_peek(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "peek() expects 0 arguments (only receiver), got {}",
            args.len() - 1
        ));
    }

    let pq = extract_receiver!(args, PriorityQueue, "peek")?;
    Ok(pq.borrow().peek().cloned().unwrap_or(Value::Nil))
}

/// Native implementation of PriorityQueue.size().
pub fn native_priority_queue_size(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "size() expects 0 arguments (only receiver), got {}",
            args.len() - 1
        ));
    }

    let pq = extract_receiver!(args, PriorityQueue, "size")?;
    Ok(Value::Int(pq.borrow().len() as i64))
}

/// Native implementation of PriorityQueue.isEmpty().
pub fn native_priority_queue_is_empty(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "isEmpty() expects 0 arguments (only receiver), got {}",
            args.len() - 1
        ));
    }

    let pq = extract_receiver!(args, PriorityQueue, "isEmpty")?;
    Ok(Value::Boolean(pq.borrow().len() == 0))
}
