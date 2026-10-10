/// Native placeholder for Generator.next(); the VM resumes the generator
/// itself because a native cannot push a frame.
pub(crate) fn native_generator_next(
    _args: &[crate::common::Value],
) -> Result<crate::common::Value, String> {
    Err("Generator.next() is resumed by the VM.".to_string())
}
