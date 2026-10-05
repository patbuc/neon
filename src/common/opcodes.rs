impl OpCode {
    /// Net operand-stack effect of this opcode on its own: how many values
    /// it pushes minus how many it pops. `Call`, `Invoke`, `CreateArray`,
    /// `CreateMap`, and `CreateSet` consume a count that is only known at
    /// the emit site (argument count or element count), so their entry here
    /// is 0 and codegen applies the rest of their effect itself right after
    /// emitting them.
    pub(crate) fn stack_effect(self) -> i32 {
        match self {
            OpCode::Return => -1,
            OpCode::Constant => 1,
            OpCode::Negate => 0,
            OpCode::Add
            | OpCode::Subtract
            | OpCode::Multiply
            | OpCode::Divide
            | OpCode::Modulo
            | OpCode::Exponent => -1,
            OpCode::Nil | OpCode::True | OpCode::False => 1,
            OpCode::Equal
            | OpCode::Greater
            | OpCode::GreaterEqual
            | OpCode::Less
            | OpCode::LessEqual => -1,
            OpCode::Not => 0,
            OpCode::Pop => -1,
            OpCode::SetLocal => 0,
            OpCode::GetLocal => 1,
            OpCode::JumpIfFalse | OpCode::Jump | OpCode::Loop => 0,
            OpCode::Call | OpCode::Invoke => 0,
            OpCode::GetBuiltin | OpCode::GetGlobal => 1,
            OpCode::SetGlobal => 0,
            OpCode::GetField => 0,
            OpCode::SetField => -1,
            OpCode::GetLocalField => 1,
            OpCode::CreateMap | OpCode::CreateArray | OpCode::CreateSet => 0,
            OpCode::GetIndex => -1,
            OpCode::SetIndex => -2,
            OpCode::GetIterator => 1,
            OpCode::IteratorNext | OpCode::IteratorDone => 1,
            OpCode::CreateRange => -1,
            OpCode::ToString => 0,
            OpCode::BitwiseAnd
            | OpCode::BitwiseOr
            | OpCode::BitwiseXor
            | OpCode::LeftShift
            | OpCode::RightShift => -1,
            OpCode::BitwiseNot => 0,
            OpCode::Closure => 1,
            OpCode::GetUpvalue => 1,
            OpCode::SetUpvalue => 0,
            OpCode::CloseUpvalue => -1,
            OpCode::DefineMethod => -1,
            OpCode::CheckInitialized => 0,
            OpCode::StoreLocal => -1,
            OpCode::StoreField => -2,
            OpCode::StoreLocalField => -1,
            OpCode::AddConstant
            | OpCode::SubtractConstant
            | OpCode::GreaterConstant
            | OpCode::GreaterEqualConstant
            | OpCode::LessConstant
            | OpCode::LessEqualConstant => 0,
            OpCode::Dup => 1,
            OpCode::Dup2 => 2,
            OpCode::JumpIfNotNil | OpCode::JumpIfNil => 0,
        }
    }

    #[inline(always)]
    pub(crate) fn from_u8(value: u8) -> Option<OpCode> {
        const OPCODES: [OpCode; 67] = [
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
            OpCode::Dup2,
            OpCode::JumpIfNotNil,
            OpCode::JumpIfNil,
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
    /// Copies the top two values of the stack, pushing the copies in the
    /// same order.
    Dup2,

    /// Peeks the top of stack: if it is not nil, jumps by the 32-bit operand
    /// without popping. Otherwise pops it and falls through to evaluate the
    /// right operand of `??`.
    JumpIfNotNil,

    /// Peeks the top of stack: if it is nil, jumps by the 32-bit operand
    /// without popping, leaving that nil as the result of `a?.b`. Otherwise
    /// falls through without popping, leaving the non-nil object on top for
    /// the field access or method call that follows.
    JumpIfNil,
}
