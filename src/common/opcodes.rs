impl OpCode {
    /// Net operand-stack effect of this opcode on its own: how many values
    /// it pushes minus how many it pops. `Call`, `TailCall`, `Invoke`,
    /// `TailInvoke`, `CreateArray`, `CreateMap`, and `CreateSet` consume a
    /// count that is only known at the emit site (argument count or element
    /// count), so their entry here is 0 and codegen applies the rest of
    /// their effect itself right after emitting them.
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
            OpCode::Call | OpCode::TailCall | OpCode::Invoke | OpCode::TailInvoke => 0,
            OpCode::GetBuiltin | OpCode::GetGlobal => 1,
            OpCode::SetGlobal => 0,
            OpCode::GetField => 0,
            OpCode::SetField => -1,
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
            OpCode::DefineMethod => -2,
            OpCode::DefineBuiltinMethod => -1,
            OpCode::CheckInitialized => 0,
            OpCode::CheckTuple => 0,
            OpCode::StoreLocalField => -1,
            OpCode::AddConstant
            | OpCode::SubtractConstant
            | OpCode::GreaterConstant
            | OpCode::GreaterEqualConstant
            | OpCode::LessConstant
            | OpCode::LessEqualConstant
            | OpCode::ModuloConstant
            | OpCode::MultiplyConstant => 0,
            OpCode::Dup => 1,
            OpCode::Dup2 => 2,
            OpCode::JumpIfNotNil | OpCode::JumpIfNil => 0,
            OpCode::NoMatchArm => -1,
            OpCode::IsArrayOfLen => 0,
            OpCode::IsNumber => 0,
            OpCode::PopJumpIfFalse => -1,
            OpCode::GreaterJumpIfFalse
            | OpCode::GreaterEqualJumpIfFalse
            | OpCode::LessJumpIfFalse
            | OpCode::LessEqualJumpIfFalse => -2,
            OpCode::GreaterConstantJumpIfFalse
            | OpCode::GreaterEqualConstantJumpIfFalse
            | OpCode::LessConstantJumpIfFalse
            | OpCode::LessEqualConstantJumpIfFalse => -1,
            OpCode::IsVariant => 0,
            OpCode::EnumConstruct => 0,
            OpCode::BeginTry | OpCode::EndTry => 0,
            OpCode::Throw => -1,
        }
    }

    #[inline(always)]
    pub(crate) fn from_u8(value: u8) -> Option<OpCode> {
        const OPCODES: [OpCode; 87] = [
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
            OpCode::DefineBuiltinMethod,
            OpCode::CheckInitialized,
            OpCode::CheckTuple,
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
            OpCode::NoMatchArm,
            OpCode::EnumConstruct,
            OpCode::IsArrayOfLen,
            OpCode::IsVariant,
            OpCode::TailCall,
            OpCode::TailInvoke,
            OpCode::IsNumber,
            OpCode::PopJumpIfFalse,
            OpCode::GreaterJumpIfFalse,
            OpCode::GreaterEqualJumpIfFalse,
            OpCode::LessJumpIfFalse,
            OpCode::LessEqualJumpIfFalse,
            OpCode::GreaterConstantJumpIfFalse,
            OpCode::GreaterEqualConstantJumpIfFalse,
            OpCode::LessConstantJumpIfFalse,
            OpCode::LessEqualConstantJumpIfFalse,
            OpCode::ModuloConstant,
            OpCode::MultiplyConstant,
            OpCode::BeginTry,
            OpCode::EndTry,
            OpCode::Throw,
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
    CreateMap,
    CreateArray,
    CreateSet,
    GetIndex,
    SetIndex,
    /// Pops a collection, pushes it (or, for a Map/Set, an array built from
    /// it) and a starting index 0 as two hidden locals. An 8-bit operand: 1
    /// asks a Map for `[key, value]` entries instead of just its keys, for
    /// `for (k, v) in map`.
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
    /// Pops a closure and the struct value below it and appends the closure
    /// to the struct's methods. Operands: the struct's 16-bit name symbol
    /// (read only by the disassembler), the 16-bit method symbol, and an
    /// 8-bit takes-`self` flag.
    DefineMethod,
    /// Pops a closure and registers it as a method of a builtin type.
    /// Operands: the 16-bit builtin type symbol, the 16-bit method symbol,
    /// and an 8-bit takes-`self` flag.
    DefineBuiltinMethod,
    /// Peeks the top of the stack and errors if it holds a hoisted
    /// declaration's uninitialized sentinel; otherwise a no-op.
    CheckInitialized,
    /// Peeks the top of stack and errors unless it holds an Array of
    /// exactly the 16-bit operand's length; otherwise a no-op. Emitted for
    /// `val (a, b) = expr` (and a `for (a, b) in ...` pair) before
    /// extracting each name.
    CheckTuple,

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

    /// Pops the scrutinee value and raises a runtime error: "No match arm
    /// for <value>", formatted like `print`. Emitted once, after every
    /// `match` arm has been tested and none matched.
    NoMatchArm,

    /// Builds a payload enum variant from a template constant (a 16-bit
    /// constant-pool index) and one stack value per declared field.
    /// Stack: `[.., field values...]` -> `[.., variant]`.
    EnumConstruct,

    /// Replaces the top of stack with whether it is an array of the 16-bit
    /// operand's length: exactly, or at least when the 8-bit operand is 1.
    IsArrayOfLen,

    /// Replaces the top of stack with whether it is the enum variant whose
    /// template is the 16-bit constant-pool operand: same enum and ordinal.
    IsVariant,

    /// A call in tail position: `[callable, args...]` with an 8-bit argument
    /// count, like `Call` followed by `Return`. A closure callee reuses the
    /// running frame instead of pushing a new one.
    TailCall,

    /// A method call in tail position: `[receiver, args...]` with a 16-bit
    /// method symbol and an 8-bit argument count, like `Invoke` followed by
    /// `Return`. A user-defined method reuses the running frame instead of
    /// pushing a new one.
    TailInvoke,

    /// Replaces the top of stack with whether it is an Int or a Number.
    IsNumber,

    /// Pops the top of stack and jumps by the 32-bit operand if it was
    /// false-like. Unlike `JumpIfFalse`, it leaves nothing behind.
    PopJumpIfFalse,

    /// Pops `b` then `a` and jumps by the 32-bit operand unless `a` > `b`.
    GreaterJumpIfFalse,

    /// Pops `b` then `a` and jumps by the 32-bit operand unless `a` >= `b`.
    GreaterEqualJumpIfFalse,

    /// Pops `b` then `a` and jumps by the 32-bit operand unless `a` < `b`.
    LessJumpIfFalse,

    /// Pops `b` then `a` and jumps by the 32-bit operand unless `a` <= `b`.
    LessEqualJumpIfFalse,

    /// Pops `a` and jumps by the 32-bit operand unless `a` > the number
    /// constant at the 16-bit pool index that precedes the jump operand.
    GreaterConstantJumpIfFalse,

    /// Pops `a` and jumps by the 32-bit operand unless `a` >= the number
    /// constant at the 16-bit pool index that precedes the jump operand.
    GreaterEqualConstantJumpIfFalse,

    /// Pops `a` and jumps by the 32-bit operand unless `a` < the number
    /// constant at the 16-bit pool index that precedes the jump operand.
    LessConstantJumpIfFalse,

    /// Pops `a` and jumps by the 32-bit operand unless `a` <= the number
    /// constant at the 16-bit pool index that precedes the jump operand.
    LessEqualConstantJumpIfFalse,

    /// `Modulo` by a number literal.
    ModuloConstant,

    /// `Multiply` by a number literal.
    MultiplyConstant,

    /// Pushes an exception handler that records the 32-bit jump operand as
    /// the catch target, the running frame and the current stack height.
    BeginTry,

    /// Pops the innermost exception handler.
    EndTry,

    /// Pops a value and raises it as an error for the innermost handler to
    /// catch.
    Throw,
}
