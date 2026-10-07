use crate::common::opcodes::OpCode;
use crate::common::{Chunk, LineInfo};

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
    /// A byte that is no opcode; executing it is a runtime error. Decoding
    /// stops here, since the width of what follows is unknown.
    Invalid(u8),
}

impl Instr {
    #[cfg(feature = "opcode-stats")]
    pub(crate) fn opcode(self) -> Option<OpCode> {
        Some(match self {
            Instr::Return => OpCode::Return,
            Instr::Constant(_) => OpCode::Constant,
            Instr::Negate => OpCode::Negate,
            Instr::Add => OpCode::Add,
            Instr::Subtract => OpCode::Subtract,
            Instr::Multiply => OpCode::Multiply,
            Instr::Divide => OpCode::Divide,
            Instr::Modulo => OpCode::Modulo,
            Instr::Exponent => OpCode::Exponent,
            Instr::Nil => OpCode::Nil,
            Instr::True => OpCode::True,
            Instr::False => OpCode::False,
            Instr::Equal => OpCode::Equal,
            Instr::Greater => OpCode::Greater,
            Instr::GreaterEqual => OpCode::GreaterEqual,
            Instr::Less => OpCode::Less,
            Instr::LessEqual => OpCode::LessEqual,
            Instr::Not => OpCode::Not,
            Instr::Pop => OpCode::Pop,
            Instr::SetLocal(_) => OpCode::SetLocal,
            Instr::GetLocal(_) => OpCode::GetLocal,
            Instr::JumpIfFalse(_) => OpCode::JumpIfFalse,
            Instr::PopJumpIfFalse(_) => OpCode::PopJumpIfFalse,
            Instr::GreaterJumpIfFalse(_) => OpCode::GreaterJumpIfFalse,
            Instr::GreaterEqualJumpIfFalse(_) => OpCode::GreaterEqualJumpIfFalse,
            Instr::LessJumpIfFalse(_) => OpCode::LessJumpIfFalse,
            Instr::LessEqualJumpIfFalse(_) => OpCode::LessEqualJumpIfFalse,
            Instr::GreaterConstantJumpIfFalse { .. } => OpCode::GreaterConstantJumpIfFalse,
            Instr::GreaterEqualConstantJumpIfFalse { .. } => {
                OpCode::GreaterEqualConstantJumpIfFalse
            }
            Instr::LessConstantJumpIfFalse { .. } => OpCode::LessConstantJumpIfFalse,
            Instr::LessEqualConstantJumpIfFalse { .. } => OpCode::LessEqualConstantJumpIfFalse,
            Instr::Jump(_) => OpCode::Jump,
            Instr::Loop(_) => OpCode::Loop,
            Instr::Call(_) => OpCode::Call,
            Instr::Invoke { .. } => OpCode::Invoke,
            Instr::GetBuiltin(_) => OpCode::GetBuiltin,
            Instr::GetGlobal(_) => OpCode::GetGlobal,
            Instr::SetGlobal(_) => OpCode::SetGlobal,
            Instr::GetField(_) => OpCode::GetField,
            Instr::SetField(_) => OpCode::SetField,
            Instr::GetLocalField { .. } => OpCode::GetLocalField,
            Instr::CreateMap(_) => OpCode::CreateMap,
            Instr::CreateArray(_) => OpCode::CreateArray,
            Instr::CreateSet(_) => OpCode::CreateSet,
            Instr::GetIndex => OpCode::GetIndex,
            Instr::SetIndex => OpCode::SetIndex,
            Instr::GetIterator { .. } => OpCode::GetIterator,
            Instr::IteratorNext(_) => OpCode::IteratorNext,
            Instr::IteratorDone(_) => OpCode::IteratorDone,
            Instr::CreateRange { .. } => OpCode::CreateRange,
            Instr::ToString => OpCode::ToString,
            Instr::BitwiseAnd => OpCode::BitwiseAnd,
            Instr::BitwiseOr => OpCode::BitwiseOr,
            Instr::BitwiseXor => OpCode::BitwiseXor,
            Instr::BitwiseNot => OpCode::BitwiseNot,
            Instr::LeftShift => OpCode::LeftShift,
            Instr::RightShift => OpCode::RightShift,
            Instr::Closure { .. } => OpCode::Closure,
            Instr::GetUpvalue(_) => OpCode::GetUpvalue,
            Instr::SetUpvalue(_) => OpCode::SetUpvalue,
            Instr::CloseUpvalue => OpCode::CloseUpvalue,
            Instr::DefineMethod { .. } => OpCode::DefineMethod,
            Instr::DefineBuiltinMethod { .. } => OpCode::DefineBuiltinMethod,
            Instr::CheckInitialized => OpCode::CheckInitialized,
            Instr::CheckTuple(_) => OpCode::CheckTuple,
            Instr::StoreLocal(_) => OpCode::StoreLocal,
            Instr::StoreField(_) => OpCode::StoreField,
            Instr::StoreLocalField { .. } => OpCode::StoreLocalField,
            Instr::AddConstant(_) => OpCode::AddConstant,
            Instr::SubtractConstant(_) => OpCode::SubtractConstant,
            Instr::GreaterConstant(_) => OpCode::GreaterConstant,
            Instr::GreaterEqualConstant(_) => OpCode::GreaterEqualConstant,
            Instr::LessConstant(_) => OpCode::LessConstant,
            Instr::LessEqualConstant(_) => OpCode::LessEqualConstant,
            Instr::Dup => OpCode::Dup,
            Instr::Dup2 => OpCode::Dup2,
            Instr::JumpIfNotNil(_) => OpCode::JumpIfNotNil,
            Instr::JumpIfNil(_) => OpCode::JumpIfNil,
            Instr::NoMatchArm => OpCode::NoMatchArm,
            Instr::EnumConstruct(_) => OpCode::EnumConstruct,
            Instr::IsArrayOfLen { .. } => OpCode::IsArrayOfLen,
            Instr::IsVariant(_) => OpCode::IsVariant,
            Instr::TailCall(_) => OpCode::TailCall,
            Instr::TailInvoke { .. } => OpCode::TailInvoke,
            Instr::IsNumber => OpCode::IsNumber,
            Instr::Invalid(_) => return None,
        })
    }
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
                OpCode::GreaterJumpIfFalse => (Instr::GreaterJumpIfFalse(0), 5),
                OpCode::GreaterEqualJumpIfFalse => (Instr::GreaterEqualJumpIfFalse(0), 5),
                OpCode::LessJumpIfFalse => (Instr::LessJumpIfFalse(0), 5),
                OpCode::LessEqualJumpIfFalse => (Instr::LessEqualJumpIfFalse(0), 5),
                OpCode::GreaterConstantJumpIfFalse => (
                    Instr::GreaterConstantJumpIfFalse {
                        constant: u16_at(1),
                        target: 0,
                    },
                    7,
                ),
                OpCode::GreaterEqualConstantJumpIfFalse => (
                    Instr::GreaterEqualConstantJumpIfFalse {
                        constant: u16_at(1),
                        target: 0,
                    },
                    7,
                ),
                OpCode::LessConstantJumpIfFalse => (
                    Instr::LessConstantJumpIfFalse {
                        constant: u16_at(1),
                        target: 0,
                    },
                    7,
                ),
                OpCode::LessEqualConstantJumpIfFalse => (
                    Instr::LessEqualConstantJumpIfFalse {
                        constant: u16_at(1),
                        target: 0,
                    },
                    7,
                ),
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
                OpCode::GetLocalField => (
                    Instr::GetLocalField {
                        slot: u16_at(1),
                        symbol: u16_at(3),
                    },
                    5,
                ),
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
                OpCode::StoreLocal => (Instr::StoreLocal(u16_at(1)), 3),
                OpCode::StoreField => (Instr::StoreField(u16_at(1)), 3),
                OpCode::StoreLocalField => (
                    Instr::StoreLocalField {
                        slot: u16_at(1),
                        symbol: u16_at(3),
                    },
                    5,
                ),
                OpCode::AddConstant => (Instr::AddConstant(u16_at(1)), 3),
                OpCode::SubtractConstant => (Instr::SubtractConstant(u16_at(1)), 3),
                OpCode::GreaterConstant => (Instr::GreaterConstant(u16_at(1)), 3),
                OpCode::GreaterEqualConstant => (Instr::GreaterEqualConstant(u16_at(1)), 3),
                OpCode::LessConstant => (Instr::LessConstant(u16_at(1)), 3),
                OpCode::LessEqualConstant => (Instr::LessEqualConstant(u16_at(1)), 3),
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
                | OpCode::GreaterJumpIfFalse
                | OpCode::GreaterEqualJumpIfFalse
                | OpCode::LessJumpIfFalse
                | OpCode::LessEqualJumpIfFalse
                | OpCode::JumpIfNotNil
                | OpCode::JumpIfNil => Some((pos + 5).checked_add(u32_at(1) as usize)),
                OpCode::GreaterConstantJumpIfFalse
                | OpCode::GreaterEqualConstantJumpIfFalse
                | OpCode::LessConstantJumpIfFalse
                | OpCode::LessEqualConstantJumpIfFalse => {
                    Some((pos + 7).checked_add(u32_at(3) as usize))
                }
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

        for (at, target_offset, byte) in jumps {
            let target = target_offset
                .and_then(|offset| index_of.get(offset).copied())
                .filter(|&target| target != u32::MAX);
            let Some(target) = target else {
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
                | Instr::JumpIfNotNil(t)
                | Instr::JumpIfNil(t)
                | Instr::Loop(t) => *t = target,
                other => unreachable!("recorded jump at a non-jump instruction {other:?}"),
            }
        }

        self.code = code;
        self.instr_lines = instr_lines;
        self.closure_upvalues = closure_upvalues;
    }

    /// The source location of instruction `index`, like `get_line_info`
    /// for a byte offset.
    pub(crate) fn instr_line_info(&self, index: usize) -> Option<LineInfo> {
        self.instr_lines.get(index).copied().flatten()
    }
}
