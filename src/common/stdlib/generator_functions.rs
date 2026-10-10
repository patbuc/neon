/// Native placeholder for Generator.next(); the VM resumes the generator
/// itself because a native cannot push a frame.
pub(crate) fn native_generator_next(
    _args: &[crate::common::Value],
) -> Result<crate::common::Value, String> {
    Err("Generator.next() is resumed by the VM.".to_string())
}

pub(crate) fn native_generator_is_done(
    args: &[crate::common::Value],
) -> Result<crate::common::Value, String> {
    match args.first() {
        Some(crate::common::Value::Generator(generator)) => Ok(crate::common::Value::Boolean(
            generator.state.get() == crate::common::GeneratorState::Done,
        )),
        _ => Err("isDone() can only be called on generators".to_string()),
    }
}
