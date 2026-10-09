use crate::common::opcodes::OpCode;
use crate::common::{Chunk, LineInfo, Value};

/// One decoded instruction. Jump targets are absolute instruction indices;
/// `Closure.upvalues` is the start of its `(is_local, index)` run in
/// `Chunk::closure_upvalues`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Instr {
    Return,
    Constant(u16),
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
    SetLocal(u16),
    GetLocal(u16),
    JumpIfFalse(u32),
    PopJumpIfFalse(u32),
    GreaterJumpIfFalse(u32),
    GreaterEqualJumpIfFalse(u32),
    LessJumpIfFalse(u32),
    LessEqualJumpIfFalse(u32),
    GreaterConstantJumpIfFalse {
        constant: u16,
        target: u32,
    },
    GreaterEqualConstantJumpIfFalse {
        constant: u16,
        target: u32,
    },
    LessConstantJumpIfFalse {
        constant: u16,
        target: u32,
    },
    LessEqualConstantJumpIfFalse {
        constant: u16,
        target: u32,
    },
    LocalGreaterConstantJumpIfFalse {
        slot: u8,
        constant: u16,
        target: u32,
    },
    LocalGreaterEqualConstantJumpIfFalse {
        slot: u8,
        constant: u16,
        target: u32,
    },
    LocalLessConstantJumpIfFalse {
        slot: u8,
        constant: u16,
        target: u32,
    },
    LocalLessEqualConstantJumpIfFalse {
        slot: u8,
        constant: u16,
        target: u32,
    },
    Jump(u32),
    Loop(u32),
    Call(u8),
    Invoke {
        method_symbol: u16,
        arg_count: u8,
    },
    GetBuiltin(u16),
    GetGlobal(u16),
    SetGlobal(u16),
    GetField(u16),
    SetField(u16),
    GetLocalField {
        slot: u16,
        symbol: u16,
    },
    CreateMap(u16),
    CreateArray(u16),
    CreateSet(u16),
    GetIndex,
    SetIndex,
    GetIterator {
        pairs: bool,
    },
    IteratorNext(u16),
    IteratorDone(u16),
    CreateRange {
        inclusive: bool,
    },
    ToString,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    Closure {
        const_index: u16,
        upvalue_count: u8,
        upvalues: u32,
    },
    GetUpvalue(u16),
    SetUpvalue(u16),
    CloseUpvalue,
    DefineMethod {
        type_symbol: u16,
        method_symbol: u16,
        takes_self: bool,
    },
    DefineBuiltinMethod {
        type_symbol: u16,
        method_symbol: u16,
        takes_self: bool,
    },
    CheckInitialized,
    CheckTuple(u16),
    StoreLocal(u16),
    StoreField(u16),
    StoreLocalField {
        slot: u16,
        symbol: u16,
    },
    AddConstant(u16),
    SubtractConstant(u16),
    GreaterConstant(u16),
    GreaterEqualConstant(u16),
    LessConstant(u16),
    LessEqualConstant(u16),
    ModuloConstant(u16),
    MultiplyConstant(u16),
    GetLocalAddConstant {
        slot: u16,
        constant: u16,
    },
    GetLocalSubtractConstant {
        slot: u16,
        constant: u16,
    },
    GetLocalMultiplyConstant {
        slot: u16,
        constant: u16,
    },
    GetLocalModuloConstant {
        slot: u16,
        constant: u16,
    },
    IncrementLocal {
        slot: u16,
        constant: u16,
    },
    AddLocal(u16),
    SubtractLocal(u16),
    MultiplyLocal(u16),
    DivideLocal(u16),
    AddLocalField {
        slot: u16,
        symbol: u16,
    },
    SubtractLocalField {
        slot: u16,
        symbol: u16,
    },
    MultiplyLocalField {
        slot: u16,
        symbol: u16,
    },
    DivideLocalField {
        slot: u16,
        symbol: u16,
    },
    Dup,
    Dup2,
    JumpIfNotNil(u32),
    JumpIfNil(u32),
    NoMatchArm,
    EnumConstruct(u16),
    IsArrayOfLen {
        length: u16,
        at_least: bool,
    },
    IsVariant(u16),
    TailCall(u8),
    TailInvoke {
        method_symbol: u16,
        arg_count: u8,
    },
    IsNumber,
    BeginTry(u32),
    EndTry,
    Throw,
    /// A byte that is no opcode; executing it is a runtime error. Decoding
    /// stops here, since the width of what follows is unknown.
    Invalid(u8),
}

impl Instr {
    #[cfg(feature = "opcode-stats")]
    pub(crate) fn name(self) -> Option<&'static str> {
        Some(match self {
            Instr::Return => "Return",
            Instr::Constant(_) => "Constant",
            Instr::Negate => "Negate",
            Instr::Add => "Add",
            Instr::Subtract => "Subtract",
            Instr::Multiply => "Multiply",
            Instr::Divide => "Divide",
            Instr::Modulo => "Modulo",
            Instr::Exponent => "Exponent",
            Instr::Nil => "Nil",
            Instr::True => "True",
            Instr::False => "False",
            Instr::Equal => "Equal",
            Instr::Greater => "Greater",
            Instr::GreaterEqual => "GreaterEqual",
            Instr::Less => "Less",
            Instr::LessEqual => "LessEqual",
            Instr::Not => "Not",
            Instr::Pop => "Pop",
            Instr::SetLocal(_) => "SetLocal",
            Instr::GetLocal(_) => "GetLocal",
            Instr::JumpIfFalse(_) => "JumpIfFalse",
            Instr::PopJumpIfFalse(_) => "PopJumpIfFalse",
            Instr::GreaterJumpIfFalse(_) => "GreaterJumpIfFalse",
            Instr::GreaterEqualJumpIfFalse(_) => "GreaterEqualJumpIfFalse",
            Instr::LessJumpIfFalse(_) => "LessJumpIfFalse",
            Instr::LessEqualJumpIfFalse(_) => "LessEqualJumpIfFalse",
            Instr::GreaterConstantJumpIfFalse { .. } => "GreaterConstantJumpIfFalse",
            Instr::GreaterEqualConstantJumpIfFalse { .. } => "GreaterEqualConstantJumpIfFalse",
            Instr::LessConstantJumpIfFalse { .. } => "LessConstantJumpIfFalse",
            Instr::LessEqualConstantJumpIfFalse { .. } => "LessEqualConstantJumpIfFalse",
            Instr::LocalGreaterConstantJumpIfFalse { .. } => "LocalGreaterConstantJumpIfFalse",
            Instr::LocalGreaterEqualConstantJumpIfFalse { .. } => {
                "LocalGreaterEqualConstantJumpIfFalse"
            }
            Instr::LocalLessConstantJumpIfFalse { .. } => "LocalLessConstantJumpIfFalse",
            Instr::LocalLessEqualConstantJumpIfFalse { .. } => "LocalLessEqualConstantJumpIfFalse",
            Instr::Jump(_) => "Jump",
            Instr::Loop(_) => "Loop",
            Instr::Call(_) => "Call",
            Instr::Invoke { .. } => "Invoke",
            Instr::GetBuiltin(_) => "GetBuiltin",
            Instr::GetGlobal(_) => "GetGlobal",
            Instr::SetGlobal(_) => "SetGlobal",
            Instr::GetField(_) => "GetField",
            Instr::SetField(_) => "SetField",
            Instr::GetLocalField { .. } => "GetLocalField",
            Instr::CreateMap(_) => "CreateMap",
            Instr::CreateArray(_) => "CreateArray",
            Instr::CreateSet(_) => "CreateSet",
            Instr::GetIndex => "GetIndex",
            Instr::SetIndex => "SetIndex",
            Instr::GetIterator { .. } => "GetIterator",
            Instr::IteratorNext(_) => "IteratorNext",
            Instr::IteratorDone(_) => "IteratorDone",
            Instr::CreateRange { .. } => "CreateRange",
            Instr::ToString => "ToString",
            Instr::BitwiseAnd => "BitwiseAnd",
            Instr::BitwiseOr => "BitwiseOr",
            Instr::BitwiseXor => "BitwiseXor",
            Instr::BitwiseNot => "BitwiseNot",
            Instr::LeftShift => "LeftShift",
            Instr::RightShift => "RightShift",
            Instr::Closure { .. } => "Closure",
            Instr::GetUpvalue(_) => "GetUpvalue",
            Instr::SetUpvalue(_) => "SetUpvalue",
            Instr::CloseUpvalue => "CloseUpvalue",
            Instr::DefineMethod { .. } => "DefineMethod",
            Instr::DefineBuiltinMethod { .. } => "DefineBuiltinMethod",
            Instr::CheckInitialized => "CheckInitialized",
            Instr::CheckTuple(_) => "CheckTuple",
            Instr::StoreLocal(_) => "StoreLocal",
            Instr::StoreField(_) => "StoreField",
            Instr::StoreLocalField { .. } => "StoreLocalField",
            Instr::AddConstant(_) => "AddConstant",
            Instr::SubtractConstant(_) => "SubtractConstant",
            Instr::GreaterConstant(_) => "GreaterConstant",
            Instr::GreaterEqualConstant(_) => "GreaterEqualConstant",
            Instr::LessConstant(_) => "LessConstant",
            Instr::LessEqualConstant(_) => "LessEqualConstant",
            Instr::ModuloConstant(_) => "ModuloConstant",
            Instr::MultiplyConstant(_) => "MultiplyConstant",
            Instr::GetLocalAddConstant { .. } => "GetLocalAddConstant",
            Instr::GetLocalSubtractConstant { .. } => "GetLocalSubtractConstant",
            Instr::GetLocalMultiplyConstant { .. } => "GetLocalMultiplyConstant",
            Instr::GetLocalModuloConstant { .. } => "GetLocalModuloConstant",
            Instr::IncrementLocal { .. } => "IncrementLocal",
            Instr::AddLocal(_) => "AddLocal",
            Instr::SubtractLocal(_) => "SubtractLocal",
            Instr::MultiplyLocal(_) => "MultiplyLocal",
            Instr::DivideLocal(_) => "DivideLocal",
            Instr::AddLocalField { .. } => "AddLocalField",
            Instr::SubtractLocalField { .. } => "SubtractLocalField",
            Instr::MultiplyLocalField { .. } => "MultiplyLocalField",
            Instr::DivideLocalField { .. } => "DivideLocalField",
            Instr::Dup => "Dup",
            Instr::Dup2 => "Dup2",
            Instr::JumpIfNotNil(_) => "JumpIfNotNil",
            Instr::JumpIfNil(_) => "JumpIfNil",
            Instr::NoMatchArm => "NoMatchArm",
            Instr::EnumConstruct(_) => "EnumConstruct",
            Instr::IsArrayOfLen { .. } => "IsArrayOfLen",
            Instr::IsVariant(_) => "IsVariant",
            Instr::TailCall(_) => "TailCall",
            Instr::TailInvoke { .. } => "TailInvoke",
            Instr::IsNumber => "IsNumber",
            Instr::BeginTry(_) => "BeginTry",
            Instr::EndTry => "EndTry",
            Instr::Throw => "Throw",
            Instr::Invalid(_) => return None,
        })
    }
}

/// Which part of a fusion supplies the fused instruction's location.
#[derive(Clone, Copy)]
enum Keep {
    First,
    Second,
    /// Both parts can fail: the second's location, with the first's kept in
    /// `Chunk::fused_field_lines`.
    Both,
}

/// The instruction that replaces `first` followed by `second`, if they fuse,
/// and which part's location it keeps: the one whose handler can fail, or
/// both when both can.
fn fused(first: Instr, second: Instr, constants: &[Value]) -> Option<(Instr, Keep)> {
    match (first, second) {
        (Instr::GetLocal(slot), Instr::GetField(symbol)) => {
            Some((Instr::GetLocalField { slot, symbol }, Keep::Second))
        }
        (Instr::GetLocal(slot), Instr::Add) => Some((Instr::AddLocal(slot), Keep::Second)),
        (Instr::GetLocal(slot), Instr::Subtract) => {
            Some((Instr::SubtractLocal(slot), Keep::Second))
        }
        (Instr::GetLocal(slot), Instr::Multiply) => {
            Some((Instr::MultiplyLocal(slot), Keep::Second))
        }
        (Instr::GetLocal(slot), Instr::Divide) => Some((Instr::DivideLocal(slot), Keep::Second)),
        (Instr::GetLocalField { slot, symbol }, Instr::Add) => {
            Some((Instr::AddLocalField { slot, symbol }, Keep::Both))
        }
        (Instr::GetLocalField { slot, symbol }, Instr::Subtract) => {
            Some((Instr::SubtractLocalField { slot, symbol }, Keep::Both))
        }
        (Instr::GetLocalField { slot, symbol }, Instr::Multiply) => {
            Some((Instr::MultiplyLocalField { slot, symbol }, Keep::Both))
        }
        (Instr::GetLocalField { slot, symbol }, Instr::Divide) => {
            Some((Instr::DivideLocalField { slot, symbol }, Keep::Both))
        }
        (Instr::GetLocal(slot), Instr::AddConstant(constant)) => {
            Some((Instr::GetLocalAddConstant { slot, constant }, Keep::Second))
        }
        (Instr::GetLocal(slot), Instr::SubtractConstant(constant)) => Some((
            Instr::GetLocalSubtractConstant { slot, constant },
            Keep::Second,
        )),
        (Instr::GetLocal(slot), Instr::MultiplyConstant(constant)) => Some((
            Instr::GetLocalMultiplyConstant { slot, constant },
            Keep::Second,
        )),
        (Instr::GetLocal(slot), Instr::ModuloConstant(constant)) => Some((
            Instr::GetLocalModuloConstant { slot, constant },
            Keep::Second,
        )),
        (Instr::GetLocalAddConstant { slot, constant }, Instr::StoreLocal(target))
            if target == slot =>
        {
            Some((Instr::IncrementLocal { slot, constant }, Keep::First))
        }
        (Instr::SetLocal(slot), Instr::Pop) => Some((Instr::StoreLocal(slot), Keep::First)),
        (Instr::SetField(symbol), Instr::Pop) => Some((Instr::StoreField(symbol), Keep::First)),
        (Instr::Greater, Instr::PopJumpIfFalse(target)) => {
            Some((Instr::GreaterJumpIfFalse(target), Keep::First))
        }
        (Instr::GreaterEqual, Instr::PopJumpIfFalse(target)) => {
            Some((Instr::GreaterEqualJumpIfFalse(target), Keep::First))
        }
        (Instr::Less, Instr::PopJumpIfFalse(target)) => {
            Some((Instr::LessJumpIfFalse(target), Keep::First))
        }
        (Instr::LessEqual, Instr::PopJumpIfFalse(target)) => {
            Some((Instr::LessEqualJumpIfFalse(target), Keep::First))
        }
        (Instr::GreaterConstant(constant), Instr::PopJumpIfFalse(target)) => Some((
            Instr::GreaterConstantJumpIfFalse { constant, target },
            Keep::First,
        )),
        (Instr::GreaterEqualConstant(constant), Instr::PopJumpIfFalse(target)) => Some((
            Instr::GreaterEqualConstantJumpIfFalse { constant, target },
            Keep::First,
        )),
        (Instr::LessConstant(constant), Instr::PopJumpIfFalse(target)) => Some((
            Instr::LessConstantJumpIfFalse { constant, target },
            Keep::First,
        )),
        (Instr::LessEqualConstant(constant), Instr::PopJumpIfFalse(target)) => Some((
            Instr::LessEqualConstantJumpIfFalse { constant, target },
            Keep::First,
        )),
        (Instr::GetLocal(slot), Instr::GreaterConstantJumpIfFalse { constant, target })
            if slot <= u8::MAX as u16 =>
        {
            Some((
                Instr::LocalGreaterConstantJumpIfFalse {
                    slot: slot as u8,
                    constant,
                    target,
                },
                Keep::Second,
            ))
        }
        (Instr::GetLocal(slot), Instr::GreaterEqualConstantJumpIfFalse { constant, target })
            if slot <= u8::MAX as u16 =>
        {
            Some((
                Instr::LocalGreaterEqualConstantJumpIfFalse {
                    slot: slot as u8,
                    constant,
                    target,
                },
                Keep::Second,
            ))
        }
        (Instr::GetLocal(slot), Instr::LessConstantJumpIfFalse { constant, target })
            if slot <= u8::MAX as u16 =>
        {
            Some((
                Instr::LocalLessConstantJumpIfFalse {
                    slot: slot as u8,
                    constant,
                    target,
                },
                Keep::Second,
            ))
        }
        (Instr::GetLocal(slot), Instr::LessEqualConstantJumpIfFalse { constant, target })
            if slot <= u8::MAX as u16 =>
        {
            Some((
                Instr::LocalLessEqualConstantJumpIfFalse {
                    slot: slot as u8,
                    constant,
                    target,
                },
                Keep::Second,
            ))
        }
        (Instr::Constant(index), second)
            if matches!(
                constants.get(index as usize),
                Some(Value::Number(_) | Value::Int(_))
            ) =>
        {
            let fusion = match second {
                Instr::Add => Instr::AddConstant(index),
                Instr::Subtract => Instr::SubtractConstant(index),
                Instr::Multiply => Instr::MultiplyConstant(index),
                Instr::Modulo => Instr::ModuloConstant(index),
                Instr::Greater => Instr::GreaterConstant(index),
                Instr::GreaterEqual => Instr::GreaterEqualConstant(index),
                Instr::Less => Instr::LessConstant(index),
                Instr::LessEqual => Instr::LessEqualConstant(index),
                _ => return None,
            };
            Some((fusion, Keep::Second))
        }
        _ => None,
    }
}

/// Replaces adjacent instructions that `fused` accepts with one, unless the
/// second starts at a jump target. Each fusion is tried again with the
/// instruction before it, so a run of three or more can fuse. A fused
/// instruction keeps the line of the part whose handler can fail, as `fused`
/// says; when both can, the first part's line goes into `field_lines` under
/// the fused instruction's index. Returns each old index's new index, with
/// one extra entry for the end of the code.
fn fuse(
    code: &mut Vec<Instr>,
    instr_lines: &mut Vec<Option<LineInfo>>,
    field_lines: &mut Vec<(u32, Option<LineInfo>)>,
    jump_targets: &[bool],
    constants: &[Value],
) -> Vec<u32> {
    let mut new_code: Vec<Instr> = Vec::with_capacity(code.len());
    let mut new_lines = Vec::with_capacity(instr_lines.len());
    // The old index of each new instruction's first part.
    let mut starts: Vec<usize> = Vec::with_capacity(code.len());
    for (i, &instr) in code.iter().enumerate() {
        new_code.push(instr);
        new_lines.push(instr_lines[i]);
        starts.push(i);
        while new_code.len() >= 2 {
            let last = new_code.len() - 1;
            if jump_targets[starts[last]] {
                break;
            }
            let Some((fusion, keep)) = fused(new_code[last - 1], new_code[last], constants) else {
                break;
            };
            new_code.pop();
            starts.pop();
            let line = new_lines.pop().flatten();
            let first = last - 1;
            new_code[first] = fusion;
            match keep {
                Keep::First => {}
                Keep::Second => new_lines[first] = line,
                Keep::Both => {
                    field_lines.push((first as u32, new_lines[first]));
                    new_lines[first] = line;
                }
            }
        }
    }
    let mut new_index = Vec::with_capacity(code.len() + 1);
    for (new, &start) in starts.iter().enumerate() {
        let end = starts.get(new + 1).copied().unwrap_or(code.len());
        new_index.extend((start..end).map(|_| new as u32));
    }
    new_index.push(new_code.len() as u32);
    *code = new_code;
    *instr_lines = new_lines;
    new_index
}

impl Chunk {
    /// Decodes `instructions` into `code`, with one `instr_lines` entry per
    /// instruction. Runs once, when the chunk becomes immutable.
    pub(crate) fn decode(&mut self) {
        let bytes = &self.instructions;
        let mut code = Vec::new();
        let mut instr_lines = Vec::new();
        let mut closure_upvalues = Vec::new();
        // Byte offset of each jump's target, patched to an instruction index below.
        let mut jumps = Vec::new();
        let mut index_of = vec![u32::MAX; bytes.len() + 1];

        let mut pos = 0;
        while pos < bytes.len() {
            index_of[pos] = code.len() as u32;
            instr_lines.push(self.get_line_info(pos));
            let byte = bytes[pos];
            let Some(op_code) = OpCode::from_u8(byte) else {
                code.push(Instr::Invalid(byte));
                break;
            };
            // Reads past the end give 0; the width check below rejects them.
            let u8_at = |offset: usize| bytes.get(pos + offset).copied().unwrap_or(0);
            let u16_at = |offset: usize| u16::from_le_bytes([u8_at(offset), u8_at(offset + 1)]);
            let u32_at = |offset: usize| {
                u32::from_le_bytes([
                    u8_at(offset),
                    u8_at(offset + 1),
                    u8_at(offset + 2),
                    u8_at(offset + 3),
                ])
            };
            let upvalues_start = closure_upvalues.len();
            let (instr, width) = match op_code {
                OpCode::Return => (Instr::Return, 1),
                OpCode::Constant => (Instr::Constant(u16_at(1)), 3),
                OpCode::Negate => (Instr::Negate, 1),
                OpCode::Add => (Instr::Add, 1),
                OpCode::Subtract => (Instr::Subtract, 1),
                OpCode::Multiply => (Instr::Multiply, 1),
                OpCode::Divide => (Instr::Divide, 1),
                OpCode::Modulo => (Instr::Modulo, 1),
                OpCode::Exponent => (Instr::Exponent, 1),
                OpCode::Nil => (Instr::Nil, 1),
                OpCode::True => (Instr::True, 1),
                OpCode::False => (Instr::False, 1),
                OpCode::Equal => (Instr::Equal, 1),
                OpCode::Greater => (Instr::Greater, 1),
                OpCode::GreaterEqual => (Instr::GreaterEqual, 1),
                OpCode::Less => (Instr::Less, 1),
                OpCode::LessEqual => (Instr::LessEqual, 1),
                OpCode::Not => (Instr::Not, 1),
                OpCode::Pop => (Instr::Pop, 1),
                OpCode::SetLocal => (Instr::SetLocal(u16_at(1)), 3),
                OpCode::GetLocal => (Instr::GetLocal(u16_at(1)), 3),
                OpCode::JumpIfFalse => (Instr::JumpIfFalse(0), 5),
                OpCode::PopJumpIfFalse => (Instr::PopJumpIfFalse(0), 5),
                OpCode::Jump => (Instr::Jump(0), 5),
                OpCode::JumpIfNotNil => (Instr::JumpIfNotNil(0), 5),
                OpCode::JumpIfNil => (Instr::JumpIfNil(0), 5),
                OpCode::Loop => (Instr::Loop(0), 5),
                OpCode::Call => (Instr::Call(u8_at(1)), 2),
                OpCode::Invoke => (
                    Instr::Invoke {
                        method_symbol: u16_at(1),
                        arg_count: u8_at(3),
                    },
                    4,
                ),
                OpCode::GetBuiltin => (Instr::GetBuiltin(u16_at(1)), 3),
                OpCode::GetGlobal => (Instr::GetGlobal(u16_at(1)), 3),
                OpCode::SetGlobal => (Instr::SetGlobal(u16_at(1)), 3),
                OpCode::GetField => (Instr::GetField(u16_at(1)), 3),
                OpCode::SetField => (Instr::SetField(u16_at(1)), 3),
                OpCode::CreateMap => (Instr::CreateMap(u16_at(1)), 3),
                OpCode::CreateArray => (Instr::CreateArray(u16_at(1)), 3),
                OpCode::CreateSet => (Instr::CreateSet(u16_at(1)), 3),
                OpCode::GetIndex => (Instr::GetIndex, 1),
                OpCode::SetIndex => (Instr::SetIndex, 1),
                OpCode::GetIterator => (
                    Instr::GetIterator {
                        pairs: u8_at(1) != 0,
                    },
                    2,
                ),
                OpCode::IteratorNext => (Instr::IteratorNext(u16_at(1)), 3),
                OpCode::IteratorDone => (Instr::IteratorDone(u16_at(1)), 3),
                OpCode::CreateRange => (
                    Instr::CreateRange {
                        inclusive: u8_at(1) != 0,
                    },
                    2,
                ),
                OpCode::ToString => (Instr::ToString, 1),
                OpCode::BitwiseAnd => (Instr::BitwiseAnd, 1),
                OpCode::BitwiseOr => (Instr::BitwiseOr, 1),
                OpCode::BitwiseXor => (Instr::BitwiseXor, 1),
                OpCode::BitwiseNot => (Instr::BitwiseNot, 1),
                OpCode::LeftShift => (Instr::LeftShift, 1),
                OpCode::RightShift => (Instr::RightShift, 1),
                OpCode::Closure => {
                    let upvalue_count = u8_at(3);
                    let upvalues = closure_upvalues.len() as u32;
                    for i in 0..upvalue_count as usize {
                        let entry = 4 + 3 * i;
                        closure_upvalues.push((u8_at(entry) != 0, u16_at(entry + 1)));
                    }
                    (
                        Instr::Closure {
                            const_index: u16_at(1),
                            upvalue_count,
                            upvalues,
                        },
                        4 + 3 * upvalue_count as usize,
                    )
                }
                OpCode::GetUpvalue => (Instr::GetUpvalue(u16_at(1)), 3),
                OpCode::SetUpvalue => (Instr::SetUpvalue(u16_at(1)), 3),
                OpCode::CloseUpvalue => (Instr::CloseUpvalue, 1),
                OpCode::DefineMethod => (
                    Instr::DefineMethod {
                        type_symbol: u16_at(1),
                        method_symbol: u16_at(3),
                        takes_self: u8_at(5) != 0,
                    },
                    6,
                ),
                OpCode::DefineBuiltinMethod => (
                    Instr::DefineBuiltinMethod {
                        type_symbol: u16_at(1),
                        method_symbol: u16_at(3),
                        takes_self: u8_at(5) != 0,
                    },
                    6,
                ),
                OpCode::CheckInitialized => (Instr::CheckInitialized, 1),
                OpCode::CheckTuple => (Instr::CheckTuple(u16_at(1)), 3),
                OpCode::StoreLocalField => (
                    Instr::StoreLocalField {
                        slot: u16_at(1),
                        symbol: u16_at(3),
                    },
                    5,
                ),
                OpCode::Dup => (Instr::Dup, 1),
                OpCode::Dup2 => (Instr::Dup2, 1),
                OpCode::NoMatchArm => (Instr::NoMatchArm, 1),
                OpCode::EnumConstruct => (Instr::EnumConstruct(u16_at(1)), 3),
                OpCode::IsArrayOfLen => (
                    Instr::IsArrayOfLen {
                        length: u16_at(1),
                        at_least: u8_at(3) != 0,
                    },
                    4,
                ),
                OpCode::IsVariant => (Instr::IsVariant(u16_at(1)), 3),
                OpCode::TailCall => (Instr::TailCall(u8_at(1)), 2),
                OpCode::TailInvoke => (
                    Instr::TailInvoke {
                        method_symbol: u16_at(1),
                        arg_count: u8_at(3),
                    },
                    4,
                ),
                OpCode::IsNumber => (Instr::IsNumber, 1),
                OpCode::BeginTry => (Instr::BeginTry(0), 5),
                OpCode::EndTry => (Instr::EndTry, 1),
                OpCode::Throw => (Instr::Throw, 1),
            };
            if pos + width > bytes.len() {
                closure_upvalues.truncate(upvalues_start);
                code.push(Instr::Invalid(byte));
                break;
            }
            let target = match op_code {
                OpCode::Jump
                | OpCode::JumpIfFalse
                | OpCode::PopJumpIfFalse
                | OpCode::JumpIfNotNil
                | OpCode::JumpIfNil
                | OpCode::BeginTry => Some((pos + 5).checked_add(u32_at(1) as usize)),
                OpCode::Loop => Some((pos + 5).checked_sub(u32_at(1) as usize)),
                _ => None,
            };
            if let Some(target) = target {
                jumps.push((code.len(), target, byte));
            }
            code.push(instr);
            pos += width;
        }
        if pos == bytes.len() {
            index_of[pos] = code.len() as u32;
        }

        // Each jump's target as an old instruction index; None if it is no
        // instruction boundary.
        let jumps: Vec<(usize, Option<usize>, u8)> = jumps
            .into_iter()
            .map(|(at, target_offset, byte)| {
                let target = target_offset
                    .and_then(|offset| index_of.get(offset).copied())
                    .filter(|&target| target != u32::MAX)
                    .map(|target| target as usize);
                (at, target, byte)
            })
            .collect();
        let mut jump_targets = vec![false; code.len() + 1];
        for &(_, target, _) in &jumps {
            if let Some(target) = target {
                jump_targets[target] = true;
            }
        }
        let mut fused_field_lines = Vec::new();
        let new_index = fuse(
            &mut code,
            &mut instr_lines,
            &mut fused_field_lines,
            &jump_targets,
            &self.constants.values,
        );

        for (at, target, byte) in jumps {
            let at = new_index[at] as usize;
            let Some(target) = target.map(|target| new_index[target]) else {
                code[at] = Instr::Invalid(byte);
                continue;
            };
            match &mut code[at] {
                Instr::Jump(t)
                | Instr::JumpIfFalse(t)
                | Instr::PopJumpIfFalse(t)
                | Instr::GreaterJumpIfFalse(t)
                | Instr::GreaterEqualJumpIfFalse(t)
                | Instr::LessJumpIfFalse(t)
                | Instr::LessEqualJumpIfFalse(t)
                | Instr::GreaterConstantJumpIfFalse { target: t, .. }
                | Instr::GreaterEqualConstantJumpIfFalse { target: t, .. }
                | Instr::LessConstantJumpIfFalse { target: t, .. }
                | Instr::LessEqualConstantJumpIfFalse { target: t, .. }
                | Instr::LocalGreaterConstantJumpIfFalse { target: t, .. }
                | Instr::LocalGreaterEqualConstantJumpIfFalse { target: t, .. }
                | Instr::LocalLessConstantJumpIfFalse { target: t, .. }
                | Instr::LocalLessEqualConstantJumpIfFalse { target: t, .. }
                | Instr::JumpIfNotNil(t)
                | Instr::JumpIfNil(t)
                | Instr::BeginTry(t)
                | Instr::Loop(t) => *t = target,
                other => unreachable!("recorded jump at a non-jump instruction {other:?}"),
            }
        }

        self.code = code;
        self.instr_lines = instr_lines;
        self.fused_field_lines = fused_field_lines;
        self.closure_upvalues = closure_upvalues;
    }

    /// The source location of instruction `index`, like `get_line_info`
    /// for a byte offset.
    pub(crate) fn instr_line_info(&self, index: usize) -> Option<LineInfo> {
        self.instr_lines.get(index).copied().flatten()
    }

    /// The source location of the field read fused into instruction `index`.
    pub(crate) fn fused_field_line_info(&self, index: usize) -> Option<LineInfo> {
        let index = index as u32;
        self.fused_field_lines
            .binary_search_by_key(&index, |&(at, _)| at)
            .ok()
            .and_then(|i| self.fused_field_lines[i].1)
    }
}
