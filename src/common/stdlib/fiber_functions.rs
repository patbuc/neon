use crate::common::fiber::{FiberKind, FiberState};
use crate::common::Value;

/// `Fiber(body)`: a suspended fiber that will run `body`, a function of zero
/// or one parameter, on its first `call`.
pub fn native_fiber_constructor(args: &[Value]) -> Result<Value, String> {
    let body = body_closure("Fiber", args)?;
    Ok(Value::new_fiber(FiberKind::Fiber, body))
}

/// `Task(body)`: an isolated task that will run `body` on `run`. The body
/// may not capture variables - a captured variable would be shared state -
/// so anything it needs is passed as its one argument.
pub fn native_task_constructor(args: &[Value]) -> Result<Value, String> {
    let body = body_closure("Task", args)?;
    if !body.upvalues.is_empty() {
        return Err(format!(
            "Task body '{}' cannot capture variables; pass them as its argument instead",
            body.function.name
        ));
    }
    Ok(Value::new_fiber(FiberKind::Task, body))
}

fn body_closure(
    type_name: &str,
    args: &[Value],
) -> Result<std::rc::Rc<crate::common::ObjClosure>, String> {
    if args.len() != 1 {
        return Err(format!(
            "{}() expects 1 argument, got {}",
            type_name,
            args.len()
        ));
    }
    let body = match &args[0] {
        Value::Closure(closure) => std::rc::Rc::clone(closure),
        other => {
            return Err(format!(
                "{}() expects a function, got {}",
                type_name,
                other.type_name()
            ))
        }
    };
    if body.function.arity > 1 {
        return Err(format!(
            "{} body '{}' must take zero or one parameter, but takes {}",
            type_name, body.function.name, body.function.arity
        ));
    }
    Ok(body)
}

/// `fiber.isDone()` / `task.isDone()`: whether the body has returned.
pub fn native_fiber_is_done(args: &[Value]) -> Result<Value, String> {
    match args.first() {
        Some(Value::Fiber(fiber)) => Ok(Value::Boolean(fiber.borrow().state == FiberState::Done)),
        _ => Err("isDone() expects a fiber or task receiver".to_string()),
    }
}
