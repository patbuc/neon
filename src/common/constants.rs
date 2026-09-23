pub const MAX_FUNCTION_PARAMS: usize = 255;
pub const MAX_CALL_ARGUMENTS: usize = 255;

/// Bounds how many call frames (nested function calls) the VM allows at once.
pub const MAX_FRAMES: usize = 10_000;

/// Arity marker for variadic functions (functions that accept any number of arguments)
/// Using u8::MAX (255) as a special marker to indicate variadic functions
pub const VARIADIC_ARITY: u8 = u8::MAX;
