impl OpCode {
    #[inline(always)]
    pub(crate) fn from_u8(value: u8) -> Option<OpCode> {
        const OPCODES: [OpCode; 64] = [
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
            OpCode::GetLocalField,
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
            OpCode::DefineMethod,
            OpCode::CheckInitialized,
            OpCode::StoreLocal,
            OpCode::StoreField,
            OpCode::StoreLocalField,
            OpCode::AddConstant,
            OpCode::SubtractConstant,
            OpCode::GreaterConstant,
            OpCode::GreaterEqualConstant,
            OpCode::LessConstant,
            OpCode::LessEqualConstant,
            OpCode::Dup,
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
    Pop,
    SetLocal,
    GetLocal,
    JumpIfFalse,
    Jump,
    Loop,
    Call,
    /// Method call dispatched by name at runtime: a 16-bit constant-pool
    /// index for the method name, then an 8-bit argument count excluding
    /// the receiver. Stack: `[receiver, args...]`.
    Invoke,
    GetBuiltin,
    GetGlobal,
    SetGlobal,
    GetField,
    SetField,
    /// Fused `GetLocal` + `GetField`: a 16-bit local slot, then a 16-bit
    /// symbol id.
    GetLocalField,

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
    /// Pops a closure and registers it as a method under a type name and
    /// method name (both read as fixed 16-bit constant-pool indices).
    DefineMethod,
    /// Peeks the top of the stack and errors if it holds a hoisted
    /// declaration's uninitialized sentinel; otherwise a no-op.
    CheckInitialized,

    /// Statement-position `SetLocal`: moves the top of stack into a 16-bit
    /// local slot without pushing it back.
    StoreLocal,
    /// Statement-position `SetField`: a 16-bit symbol id. Stack
    /// `[.., instance, value]` -> `[..]`, moving `value` into the field
    /// instead of pushing it back.
    StoreField,
    /// Statement-position fused `GetLocal` + `SetField`: a 16-bit local
    /// slot, then a 16-bit symbol id. Stack `[.., value]` -> `[..]`.
    StoreLocalField,

    /// Binary operators whose right operand is a number literal: a 16-bit
    /// constant-pool index replaces pushing it.
    AddConstant,
    SubtractConstant,
    GreaterConstant,
    GreaterEqualConstant,
    LessConstant,
    LessEqualConstant,

    /// Copies the top of the stack, pushing the copy.
    Dup,
}
