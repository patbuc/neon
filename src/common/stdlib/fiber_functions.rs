use crate::common::fiber::{FiberKind, FiberState};
use crate::common::Value;
use std::rc::Rc;

/// `Fiber(body)`: a suspended fiber that will run `body`, a function of zero
/// or one parameter, on its first `call`.
pub fn native_fiber_constructor(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("Fiber() expects 1 argument, got {}", args.len()));
    }
    let body = match &args[0] {
        Value::Closure(closure) => Rc::clone(closure),
        other => {
            return Err(format!(
                "Fiber() expects a function, got {}",
                other.type_name()
            ))
        }
    };
    if body.function.arity > 1 {
        return Err(format!(
            "Fiber body '{}' must take zero or one parameter, but takes {}",
            body.function.name, body.function.arity
        ));
    }
    Ok(Value::new_fiber(FiberKind::Fiber, body))
}

/// `fiber.isDone()` / `task.isDone()`: whether the body has returned.
pub fn native_fiber_is_done(args: &[Value]) -> Result<Value, String> {
    match args.first() {
        Some(Value::Fiber(fiber)) => Ok(Value::Boolean(fiber.borrow().state == FiberState::Done)),
        _ => Err("isDone() expects a fiber or task receiver".to_string()),
    }
}

/// `task.join()`: the task's deep-copied return value, the same on every
/// call. `Task.spawn` runs its task to completion before returning the
/// handle, so the result is always ready by the time `join` can be called.
pub fn native_task_join(args: &[Value]) -> Result<Value, String> {
    match args.first() {
        Some(Value::Fiber(fiber)) => fiber
            .borrow()
            .result
            .clone()
            .ok_or_else(|| "join() called before the task finished running".to_string()),
        _ => Err("join() expects a task receiver".to_string()),
    }
}
