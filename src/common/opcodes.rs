impl OpCode {
    #[inline(always)]
    pub(crate) fn from_u8(value: u8) -> Option<OpCode> {
        const OPCODES: [OpCode; 55] = [
            OpCode::Return,
            OpCode::Constant,
            OpCode::Negate,
            OpCode::Add,
            OpCode::Subtract,
            OpCode::Multiply,
            OpCode::Divide,
            OpCode::Modulo,
            OpCode::Exponent,
            OpCode::Nil,
            OpCode::True,
            OpCode::False,
            OpCode::Equal,
            OpCode::Greater,
            OpCode::GreaterEqual,
            OpCode::Less,
            OpCode::LessEqual,
            OpCode::Not,
            OpCode::String,
            OpCode::Pop,
            OpCode::SetLocal,
            OpCode::GetLocal,
            OpCode::JumpIfFalse,
            OpCode::Jump,
            OpCode::Loop,
            OpCode::Call,
            OpCode::Invoke,
            OpCode::GetBuiltin,
            OpCode::GetGlobal,
            OpCode::SetGlobal,
            OpCode::GetField,
            OpCode::SetField,
            OpCode::CreateMap,
            OpCode::CreateArray,
            OpCode::CreateSet,
            OpCode::GetIndex,
            OpCode::SetIndex,
            OpCode::GetIterator,
            OpCode::IteratorNext,
            OpCode::IteratorDone,
            OpCode::CreateRange,
            OpCode::ToString,
            OpCode::BitwiseAnd,
            OpCode::BitwiseOr,
            OpCode::BitwiseXor,
            OpCode::BitwiseNot,
            OpCode::LeftShift,
            OpCode::RightShift,
            OpCode::Closure,
            OpCode::GetUpvalue,
            OpCode::SetUpvalue,
            OpCode::CloseUpvalue,
            OpCode::CloseUpvalueInPlace,
            OpCode::DefineMethod,
            OpCode::CheckInitialized,
        ];
        OPCODES.get(value as usize).copied()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub(crate) enum OpCode {
    Return = 0x00,
    Constant,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    Nil,
    True,
    False,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Not,
    String,
    Pop,
    SetLocal,
    GetLocal,
    JumpIfFalse,
    Jump,
    Loop,
    Call,
    /// Method call dispatched by name at runtime: a 16-bit string-pool
    /// index for the method name, then an 8-bit argument count excluding
    /// the receiver. Stack: `[receiver, args...]`.
    Invoke,
    GetBuiltin,
    GetGlobal,
    SetGlobal,
    GetField,
    SetField,

    CreateMap,
    CreateArray,
    CreateSet,
    GetIndex,
    SetIndex,
    GetIterator,
    IteratorNext,
    IteratorDone,
    CreateRange,
    ToString,

    // Bitwise operations
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,

    /// Wraps a function constant in a closure: reads a function constant
    /// index, then an upvalue count and that many (is_local, index) pairs
    /// describing how to fill the closure's upvalue array.
    Closure,
    GetUpvalue,
    SetUpvalue,
    /// Closes the upvalue (if any) pointing at the top-of-stack slot, then
    /// pops it, so a captured local's value survives its scope exiting.
    CloseUpvalue,
    /// Closes the upvalue on the top-of-stack slot without popping it.
    CloseUpvalueInPlace,
    /// Pops a closure and registers it as a method under a type name and
    /// method name (both read as fixed 16-bit string indices).
    DefineMethod,
    /// Peeks the top of the stack and errors if it holds a hoisted
    /// declaration's uninitialized sentinel; otherwise a no-op.
    CheckInitialized,
}
