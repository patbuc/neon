use crate::common::{GeneratorState, Value};
use crate::extract_receiver;

pub fn native_generator_is_done(args: &[Value]) -> Result<Value, String> {
    let generator = extract_receiver!(args, Generator, "isDone")?;
    Ok(Value::Boolean(
        generator.state.get() == GeneratorState::Done,
    ))
}
