use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};
use crate::common::method_registry::BUILTIN_TYPE_NAMES;

/// Code generator for the multi-pass compiler
/// Generates bytecode from AST using the semantic pass's resolutions
use crate::common::opcodes::OpCode;
use crate::common::{Chunk, SourceLocation, Value};
use crate::compiler::ast::{
    BinaryOp, Binding, Expr, IfExprElse, MatchArm, MatchArmBody, MatchPattern, NodeId, PathStep,
    Pattern, Stmt, UnaryOp,
};
use crate::compiler::global_env::GlobalEnv;
use crate::compiler::resolutions::{Capture, DeclId, EnumVariantAccess, Res, Resolutions};
use crate::{int, number, string};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

struct Local {
    depth: u32,
    is_captured: bool,
}

impl Local {
    fn new(depth: u32, is_captured: bool) -> Self {
        Local { depth, is_captured }
    }
}

struct LoopContext {
    #[allow(dead_code)]
    loop_start: u32,
    break_jumps: Vec<u32>,
    continue_jumps: Vec<u32>,
    /// Locals deeper than this are popped by a break/continue before it jumps.
    depth: u32,
    /// Operand-stack height when the loop was entered; break/continue pop
    /// down to this, which also accounts for transient values above the
    /// loop's locals (e.g. an if-expression branch's own transients).
    entry_stack_height: u32,
    /// Number of enclosing try bodies when the loop was entered; break/continue
    /// end the ones entered since.
    try_depth: u32,
}

enum LoopExit {
    Break,
    Continue,
}

/// Key under which a deduplicable constant is looked up. Strings are keyed
/// by content; numbers by their bit pattern, so `0.0` and `-0.0` (whose
/// values compare equal) stay distinct pool entries.
#[derive(PartialEq, Eq, Hash)]
enum ConstantKey {
    String(Rc<String>),
    Number(u64),
    Int(i64),
}

struct FunctionCompiler {
    chunk: Chunk,
    locals: Vec<Local>,
    scope_depth: u32,
    loop_contexts: Vec<LoopContext>,
    /// Number of try bodies enclosing the code being generated.
    try_depth: u32,
    reported_overflows: HashSet<&'static str>,
    constant_keys: HashMap<ConstantKey, u32>,
    /// Operand-stack height, tracked by applying each emitted opcode's
    /// `stack_effect`. Equal to `locals.len() + transient_offset` at every
    /// statement boundary, checked by `assert_stack_height`.
    stack_height: u32,
    /// Transient values sitting below the current statement's locals (e.g.
    /// a callee and already-evaluated arguments while a later argument, an
    /// if-expression, generates its own branches). Saved and restored
    /// around each if-expression branch.
    transient_offset: u32,
}

impl FunctionCompiler {
    fn new(name: &str, symbols: Rc<[Rc<str>]>, file: Option<Rc<str>>) -> Self {
        let mut chunk = Chunk::new(name);
        chunk.symbols = symbols;
        chunk.file = file;
        FunctionCompiler {
            chunk,
            locals: Vec::new(),
            scope_depth: 0,
            loop_contexts: Vec::new(),
            try_depth: 0,
            reported_overflows: HashSet::new(),
            constant_keys: HashMap::new(),
            stack_height: 0,
            transient_offset: 0,
        }
    }

    /// Drops locals declared deeper than `depth`, returning whether each one
    /// was captured (top-most local first), so the caller can emit
    /// CloseUpvalue instead of a plain Pop for it.
    fn pop_locals_above(&mut self, depth: u32) -> Vec<bool> {
        let mut captured = Vec::new();
        while let Some(local) = self.locals.last() {
            if local.depth <= depth {
                break;
            }
            captured.push(local.is_captured);
            self.locals.pop();
        }
        captured
    }

    /// Same as `pop_locals_above`, without removing the locals: for a
    /// break/continue jump, which unwinds the runtime stack early but
    /// leaves the compile-time locals in scope for the code that follows.
    fn captured_flags_above(&self, depth: u32) -> Vec<bool> {
        let mut captured = Vec::new();
        for local in self.locals.iter().rev() {
            if local.depth <= depth {
                break;
            }
            captured.push(local.is_captured);
        }
        captured
    }
}

pub struct CodeGenerator<'a> {
    /// One entry per level of function nesting, outermost (the script) first.
    functions: Vec<FunctionCompiler>,
    errors: Vec<CompilationError>,
    resolutions: &'a Resolutions,
    /// Slot of each declaration in the `locals` of the function that owns it.
    /// `DeclId`s are unique program-wide, so one map serves every function.
    decl_slots: HashMap<DeclId, u32>,
    /// Closing-brace location of each `fn`/lambda body, by `NodeId`.
    end_locations: &'a HashMap<NodeId, SourceLocation>,
    /// Built once from `resolutions` and `Rc::clone`d into every chunk, so
    /// every chunk of this compile shares one allocation.
    symbols: Rc<[Rc<str>]>,
    /// The unit's source file, cloned into every chunk like `symbols`.
    file: Option<Rc<str>>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(
        resolutions: &'a Resolutions,
        end_locations: &'a HashMap<NodeId, SourceLocation>,
    ) -> Self {
        let symbols = resolutions.symbol_names();
        CodeGenerator {
            functions: vec![FunctionCompiler::new("main", symbols.clone(), None)],
            end_locations,
            errors: Vec::new(),
            resolutions,
            decl_slots: HashMap::new(),
            symbols,
            file: None,
        }
    }

    /// Names the source file of every chunk this generator creates.
    pub(crate) fn set_file(&mut self, file: Option<Rc<str>>) {
        self.functions[0].chunk.file = file.clone();
        self.file = file;
    }

    /// Seeds the script frame with one placeholder local per global an
    /// earlier REPL line already defined, so this line's new globals get
    /// slots starting right after them, and restores where each of those
    /// earlier globals lives.
    pub(crate) fn seed(&mut self, env: &GlobalEnv) {
        self.decl_slots = env.decl_slots.clone();
        let script = &mut self.functions[0];
        for _ in 0..env.slot_count {
            script.locals.push(Local::new(0, false));
        }
        // These globals are already on the stack from an earlier REPL line,
        // not pushed by any opcode this compile emits.
        script.stack_height = env.slot_count;
    }

    pub(crate) fn into_decl_slots(self) -> HashMap<DeclId, u32> {
        self.decl_slots
    }

    #[allow(clippy::expect_used)]
    pub fn generate(
        &mut self,
        statements: &[Stmt],
        eof_location: SourceLocation,
    ) -> CompilationResult<Chunk> {
        // First: allocate one slot per top-level declaration, in statement
        // order, so every slot is known before any body is compiled. A
        // val/var slot starts out holding the uninitialized sentinel, which
        // GetGlobal/SetGlobal reject until its statement runs.
        for stmt in statements {
            match stmt.unexported() {
                Stmt::Fn { id, location, .. } => {
                    self.emit_op_code(OpCode::Nil, *location);
                    let decl = self.resolutions.decl(*id);
                    self.bind_decl_local(decl, *location);
                }
                Stmt::Struct {
                    name,
                    fields,
                    id,
                    location,
                    ..
                } => {
                    // Create the struct value
                    let field_names = fields.iter().map(|f| f.name.clone()).collect();
                    let field_symbols = fields
                        .iter()
                        .map(|f| self.resolutions.symbol(&f.name))
                        .collect();
                    let name_symbol = self.resolutions.symbol(name);
                    let struct_value =
                        Value::new_struct(name.clone(), field_names, field_symbols, name_symbol);
                    self.emit_constant(struct_value, *location);
                    let decl = self.resolutions.decl(*id);
                    self.bind_decl_local(decl, *location);
                }
                Stmt::Val { pattern, .. } | Stmt::Var { pattern, .. } => {
                    for binding in pattern.bindings() {
                        let sentinel = Value::Uninitialized(Rc::new(binding.name.clone()));
                        self.emit_constant(sentinel, binding.location);
                        let decl = self.resolutions.decl(binding.id);
                        self.bind_decl_local(decl, binding.location);
                    }
                }
                _ => {}
            }
        }

        // Then: compile every top-level fn body into a closure and store it
        // into its pre-allocated slot, so a body can call - or be called by
        // - any other top-level function regardless of declaration order.
        for stmt in statements {
            if let Stmt::Fn {
                name,
                body,
                id,
                location,
                ..
            } = stmt.unexported()
            {
                self.generate_closure(*id, name, body, *location);
                let slot = self.decl_slot(self.resolutions.decl(*id));
                self.emit_index_op(OpCode::SetLocal, slot, "locals", *location);
                self.emit_op_code(OpCode::Pop, *location);
            }
        }

        // Then: Compile impl-block methods into closures and register them,
        // so a method call textually before its `impl` block still works.
        for stmt in statements {
            if let Stmt::Impl {
                type_name,
                type_id,
                methods,
                ..
            } = stmt
            {
                let is_builtin_type = BUILTIN_TYPE_NAMES.contains(&type_name.as_str());
                for method in methods {
                    if let Stmt::Fn {
                        name,
                        params,
                        body,
                        id,
                        location,
                    } = method
                    {
                        if !is_builtin_type {
                            self.emit_variable_get(*type_id, *location);
                        }
                        self.generate_closure(*id, name, body, *location);
                        let takes_self = params.first().map(String::as_str) == Some("self");
                        let op_code = if is_builtin_type {
                            OpCode::DefineBuiltinMethod
                        } else {
                            OpCode::DefineMethod
                        };
                        self.emit_define_method(op_code, type_name, name, takes_self, *location);
                    }
                }
            }
        }

        for stmt in statements {
            self.generate_stmt(stmt);
            self.assert_stack_height();
        }

        // Emit final return
        self.emit_return(eof_location);

        if self.errors.is_empty() {
            Ok(self
                .functions
                .pop()
                .expect("generate pushed the top-level function compiler before compiling")
                .chunk)
        } else {
            Err(self.errors.clone())
        }
    }

    // ===== Helper Methods =====

    #[allow(clippy::expect_used)]
    fn current(&mut self) -> &mut FunctionCompiler {
        self.functions
            .last_mut()
            .expect("a function compiler is always on the stack while generating code")
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.current().chunk
    }

    fn emit_op_code(&mut self, op_code: OpCode, location: SourceLocation) {
        self.current_chunk()
            .write_op_code(op_code, location.line, location.column);
        self.adjust_stack_height(op_code.stack_effect());
    }

    /// Applies `delta` to the current function's tracked operand-stack
    /// height. Called from `emit_op_code` for every opcode's own effect,
    /// and separately for the handful of opcodes (`Call`, `Invoke`,
    /// `CreateArray`, `CreateMap`, `CreateSet`) whose effect depends on a
    /// count known only at the emit site.
    fn adjust_stack_height(&mut self, delta: i32) {
        let compiler = self.current();
        compiler.stack_height = (compiler.stack_height as i32 + delta) as u32;
    }

    /// Verifies the tracked stack height still matches the live locals
    /// (hidden for-in locals are locals too), so a wrong stack-effect entry
    /// fails loudly instead of silently corrupting a later local's slot.
    fn assert_stack_height(&mut self) {
        // A reported error (e.g. a count or index too large for its operand
        // width) can leave a statement's bytecode incomplete, since the
        // whole chunk is discarded once any error is recorded; only check
        // the invariant while the generated code is still meant to be real.
        if !self.errors.is_empty() {
            return;
        }
        let compiler = self.current();
        debug_assert_eq!(
            compiler.stack_height,
            compiler.locals.len() as u32 + compiler.transient_offset,
            "stack height drifted from the live locals count"
        );
    }

    /// Verifies `index` fits the u16 operand width, reporting at most one
    /// "too many `kind`" compile error per function instead of one per
    /// occurrence.
    fn checked_index(
        &mut self,
        index: u32,
        kind: &'static str,
        location: SourceLocation,
    ) -> Option<u16> {
        let count = index as usize + 1;
        if count > u16::MAX as usize {
            if self.current().reported_overflows.insert(kind) {
                let message = format!(
                    "too many {} in one function (maximum is {})",
                    kind,
                    u16::MAX
                );
                self.errors.push(CompilationError::new(
                    CompilationPhase::Codegen,
                    CompilationErrorKind::LimitExceeded,
                    message,
                    location,
                ));
            }
            None
        } else {
            Some(index as u16)
        }
    }

    /// Emits `op_code` followed by `index` as a checked u16 operand.
    fn emit_index_op(
        &mut self,
        op_code: OpCode,
        index: u32,
        kind: &'static str,
        location: SourceLocation,
    ) {
        if let Some(index) = self.checked_index(index, kind, location) {
            self.emit_op_code(op_code, location);
            self.current_chunk().write_u16(index);
        }
    }

    /// Emits `IsArrayOfLen`: exactly `length` elements, or at least that
    /// many when `at_least`.
    fn emit_array_length_test(&mut self, length: u32, at_least: bool, location: SourceLocation) {
        if let Some(length) = self.checked_index(length, "array pattern elements", location) {
            self.emit_op_code(OpCode::IsArrayOfLen, location);
            self.current_chunk().write_u16(length);
            self.current_chunk().write_u8(u8::from(at_least));
        }
    }

    fn emit_store_local_field(&mut self, slot: u32, symbol: u32, location: SourceLocation) {
        let Some(slot) = self.checked_index(slot, "locals", location) else {
            return;
        };
        let Some(symbol) = self.checked_index(symbol, "symbols", location) else {
            return;
        };
        self.emit_op_code(OpCode::StoreLocalField, location);
        self.current_chunk().write_u16(slot);
        self.current_chunk().write_u16(symbol);
    }

    /// Slot of `decl` in the `locals` of the function that owns it.
    fn decl_slot(&self, decl: DeclId) -> u32 {
        *self
            .decl_slots
            .get(&decl)
            .unwrap_or_else(|| panic!("no slot recorded for {:?}", decl))
    }

    /// Pushes a new local bound to `decl`, capturing whether it's captured
    /// from the resolutions, and records its slot.
    fn bind_local(&mut self, decl: DeclId) {
        let is_captured = self.resolutions.is_captured(decl);
        let depth = self.current().scope_depth;
        let local = Local::new(depth, is_captured);
        self.current().locals.push(local);
        let slot = self.current().stack_height - 1;
        self.decl_slots.insert(decl, slot);
    }

    /// Pushes a new local bound to `decl` on top of the current function's
    /// stack. The value it binds must already be on the stack at this slot.
    fn bind_decl_local(&mut self, decl: DeclId, location: SourceLocation) {
        self.bind_local(decl);
        let slot = self.decl_slot(decl);
        self.checked_index(slot, "locals", location);
    }

    fn emit_variable_get(&mut self, id: NodeId, location: SourceLocation) {
        let (op_code, index, kind) = match self.resolutions.res(id) {
            Res::Local(decl) => (OpCode::GetLocal, self.decl_slot(decl), "locals"),
            Res::Global(decl) => (OpCode::GetGlobal, self.decl_slot(decl), "globals"),
            Res::Upvalue(index) => (OpCode::GetUpvalue, index, "upvalues"),
            Res::Builtin(index) => (OpCode::GetBuiltin, index, "builtins"),
        };
        self.emit_index_op(op_code, index, kind, location);
        if self.resolutions.is_checked(id) {
            self.emit_op_code(OpCode::CheckInitialized, location);
        }
    }

    fn emit_variable_set(&mut self, id: NodeId, location: SourceLocation) {
        let (op_code, index, kind) = match self.resolutions.res(id) {
            Res::Local(decl) => (OpCode::SetLocal, self.decl_slot(decl), "locals"),
            Res::Global(decl) => (OpCode::SetGlobal, self.decl_slot(decl), "globals"),
            Res::Upvalue(index) => (OpCode::SetUpvalue, index, "upvalues"),
            Res::Builtin(_) => unreachable!("the semantic pass rejects assignment to a builtin"),
        };
        self.emit_index_op(op_code, index, kind, location);
    }

    fn emit_upvalue_metadata(&mut self, captures: &[Capture], location: SourceLocation) {
        if self
            .check_count_limit(
                captures.len(),
                u8::MAX as usize,
                || {
                    format!(
                        "function captures too many variables: {} (maximum is {})",
                        captures.len(),
                        u8::MAX
                    )
                },
                location,
            )
            .is_none()
        {
            return;
        }
        self.current_chunk().write_u8(captures.len() as u8);

        for capture in captures {
            let (is_local, index) = match *capture {
                Capture::Local(decl) => (true, self.decl_slot(decl)),
                Capture::Upvalue(index) => (false, index),
            };
            let kind = if is_local { "locals" } else { "upvalues" };
            let Some(index) = self.checked_index(index, kind, location) else {
                return;
            };
            self.current_chunk().write_u8(if is_local { 1 } else { 0 });
            self.current_chunk().write_u16(index);
        }
    }

    /// Verifies a count fits within `max` before it is narrowed into a
    /// bytecode operand, reporting a compile error naming the limit instead
    /// of letting the narrowing cast wrap silently.
    fn check_count_limit(
        &mut self,
        count: usize,
        max: usize,
        message: impl FnOnce() -> String,
        location: SourceLocation,
    ) -> Option<()> {
        if count > max {
            self.errors.push(CompilationError::new(
                CompilationPhase::Codegen,
                CompilationErrorKind::LimitExceeded,
                message(),
                location,
            ));
            None
        } else {
            Some(())
        }
    }

    /// Adds `value` to the current function's constant pool, reusing the
    /// existing index for a repeated string or number.
    pub(super) fn add_constant(&mut self, value: Value) -> u32 {
        let key = match &value {
            Value::String(s) => Some(ConstantKey::String(Rc::clone(s))),
            Value::Number(n) => Some(ConstantKey::Number(n.to_bits())),
            Value::Int(i) => Some(ConstantKey::Int(*i)),
            _ => None,
        };
        let Some(key) = key else {
            return self.current_chunk().add_constant(value);
        };
        if let Some(&index) = self.current().constant_keys.get(&key) {
            return index;
        }
        let index = self.current_chunk().add_constant(value);
        self.current().constant_keys.insert(key, index);
        index
    }

    fn emit_constant(&mut self, value: Value, location: SourceLocation) {
        let index = self.add_constant(value);
        self.emit_index_op(OpCode::Constant, index, "constants", location);
    }

    fn emit_enum_variant_constant(
        &mut self,
        enum_name: &str,
        variant_name: &str,
        ordinal: u16,
        fields: &[Rc<str>],
        location: SourceLocation,
    ) {
        let value = self.enum_variant_value(enum_name, variant_name, ordinal, fields);
        self.emit_constant(value, location);
    }

    fn enum_variant_value(
        &self,
        enum_name: &str,
        variant_name: &str,
        ordinal: u16,
        fields: &[Rc<str>],
    ) -> Value {
        if fields.is_empty() {
            return Value::new_enum_variant(
                enum_name.to_string(),
                variant_name.to_string(),
                ordinal,
            );
        }
        let field_symbols = fields.iter().map(|f| self.resolutions.symbol(f)).collect();
        Value::new_enum_variant_template(
            enum_name.to_string(),
            variant_name.to_string(),
            ordinal,
            field_symbols,
        )
    }

    /// Emits `EnumConstruct`, popping one value per payload field and
    /// pushing the variant.
    fn emit_enum_construct(&mut self, access: &EnumVariantAccess, location: SourceLocation) {
        let template = self.enum_variant_value(
            &access.enum_name,
            &access.variant_name,
            access.ordinal,
            &access.fields,
        );
        let index = self.add_constant(template);
        self.emit_index_op(OpCode::EnumConstruct, index, "constants", location);
        self.adjust_stack_height(1 - access.fields.len() as i32);
    }

    fn emit_return(&mut self, location: SourceLocation) {
        self.emit_op_code(OpCode::Nil, location);
        self.emit_op_code(OpCode::Return, location);
    }

    fn emit_jump(&mut self, op_code: OpCode, location: SourceLocation) -> u32 {
        let offset = self
            .current_chunk()
            .emit_jump(op_code, location.line, location.column);
        self.adjust_stack_height(op_code.stack_effect());
        offset
    }

    /// Emits `condition` and a jump, taken when it is false-like, that pops
    /// it on both paths. A `<`, `<=`, `>` or `>=` condition fuses into the
    /// jump. Returns the jump to patch.
    fn generate_condition_jump(&mut self, condition: &Expr, location: SourceLocation) -> u32 {
        let mut condition = condition;
        while let Expr::Grouping { expr, .. } = condition {
            condition = expr;
        }
        if let Expr::Binary {
            left,
            operator,
            right,
            location: compare_location,
        } = condition
        {
            let fused = match operator {
                BinaryOp::Greater => Some((
                    OpCode::GreaterJumpIfFalse,
                    OpCode::GreaterConstantJumpIfFalse,
                )),
                BinaryOp::GreaterEqual => Some((
                    OpCode::GreaterEqualJumpIfFalse,
                    OpCode::GreaterEqualConstantJumpIfFalse,
                )),
                BinaryOp::Less => Some((OpCode::LessJumpIfFalse, OpCode::LessConstantJumpIfFalse)),
                BinaryOp::LessEqual => Some((
                    OpCode::LessEqualJumpIfFalse,
                    OpCode::LessEqualConstantJumpIfFalse,
                )),
                _ => None,
            };
            if let Some((op_code, constant_op_code)) = fused {
                self.generate_expr(left);
                if let Some(constant) = Self::number_literal(right) {
                    return self.emit_constant_jump(constant_op_code, constant, *compare_location);
                }
                self.generate_expr(right);
                return self.emit_jump(op_code, *compare_location);
            }
        }
        self.generate_expr(condition);
        self.emit_jump(OpCode::PopJumpIfFalse, location)
    }

    fn emit_constant_jump(
        &mut self,
        op_code: OpCode,
        constant: Value,
        location: SourceLocation,
    ) -> u32 {
        let index = self.add_constant(constant);
        // An index past u16 is reported as a compile error; 0 only keeps the
        // jump patchable.
        let index = self
            .checked_index(index, "constants", location)
            .unwrap_or(0);
        let offset =
            self.current_chunk()
                .emit_constant_jump(op_code, index, location.line, location.column);
        self.adjust_stack_height(op_code.stack_effect());
        offset
    }

    /// The value of a number literal, which a `*Constant` opcode takes from
    /// the constant pool instead of the stack.
    fn number_literal(expr: &Expr) -> Option<Value> {
        match expr {
            Expr::Number { value, .. } => Some(number!(*value)),
            Expr::Int { value, .. } => Some(int!(*value)),
            _ => None,
        }
    }

    fn patch_jump(&mut self, offset: u32) {
        self.current_chunk().patch_jump(offset);
    }

    fn emit_loop(&mut self, loop_start: u32, location: SourceLocation) {
        self.current_chunk()
            .emit_loop(loop_start, location.line, location.column);
    }

    // ===== Statement Generation =====

    fn generate_variable_declaration(
        &mut self,
        id: NodeId,
        initializer: &Option<Expr>,
        location: SourceLocation,
    ) {
        // Generate initializer or nil
        if let Some(init) = initializer {
            self.generate_expr(init);
        } else {
            self.emit_op_code(OpCode::Nil, location);
        }

        let decl = self.resolutions.decl(id);
        if self.current().scope_depth == 0 {
            // Top level: store into the slot generate()'s prologue pre-allocated.
            let slot = self.decl_slot(decl);
            self.emit_index_op(OpCode::SetLocal, slot, "locals", location);
            self.emit_op_code(OpCode::Pop, location);
        } else {
            self.bind_decl_local(decl, location);
        }
    }

    /// `val (a, _, c) = expr` / `var (...)`. The checked value is held in a
    /// hidden local so each name can fetch its element without disturbing
    /// one already bound; `_` positions are skipped entirely. At depth 0
    /// the hidden local is dropped right after binding, so no slot leaks.
    fn generate_tuple_declaration(
        &mut self,
        slots: &[Option<Binding>],
        initializer: &Expr,
        location: SourceLocation,
    ) {
        self.generate_expr(initializer);
        self.emit_index_op(
            OpCode::CheckTuple,
            slots.len() as u32,
            "tuple pattern names",
            location,
        );

        let is_top_level = self.current().scope_depth == 0;
        let depth = self.current().scope_depth;
        self.current().locals.push(Local::new(depth, false));
        let hidden_slot = self.current().stack_height - 1;

        if is_top_level {
            for (index, slot) in slots.iter().enumerate() {
                let Some(binding) = slot else {
                    continue;
                };

                self.emit_index_op(OpCode::GetLocal, hidden_slot, "locals", location);
                self.emit_constant(int!(index as i64), location);
                self.emit_op_code(OpCode::GetIndex, location);

                let decl = self.resolutions.decl(binding.id);
                let slot = self.decl_slot(decl);
                self.emit_index_op(OpCode::SetLocal, slot, "locals", location);
                self.emit_op_code(OpCode::Pop, location);
            }
            self.current().locals.pop();
            self.emit_op_code(OpCode::Pop, location);
        } else {
            self.bind_tuple_slots(slots, hidden_slot, location);
        }
    }

    /// Reads each named slot's element from the tuple value held in
    /// `hidden_slot` and binds it as a fresh local, skipping `_` positions.
    fn bind_tuple_slots(
        &mut self,
        slots: &[Option<Binding>],
        hidden_slot: u32,
        location: SourceLocation,
    ) {
        for (index, slot) in slots.iter().enumerate() {
            let Some(binding) = slot else {
                continue;
            };
            self.emit_index_op(OpCode::GetLocal, hidden_slot, "locals", location);
            self.emit_constant(int!(index as i64), location);
            self.emit_op_code(OpCode::GetIndex, location);

            let decl = self.resolutions.decl(binding.id);
            self.bind_decl_local(decl, location);
        }
    }

    fn generate_fn_stmt(
        &mut self,
        id: NodeId,
        name: &str,
        body: &[Stmt],
        location: SourceLocation,
    ) {
        if self.current().scope_depth == 0 {
            // Top level: already compiled and stored by generate()'s prologue.
            return;
        }

        self.generate_closure(id, name, body, location);

        let slot = self.decl_slot(self.resolutions.decl(id));
        self.emit_index_op(OpCode::SetLocal, slot, "locals", location);
        self.emit_op_code(OpCode::Pop, location); // Pop the function value from the stack
    }

    /// Pushes an uninitialized sentinel slot for each `fn` in a statement
    /// list before any statement runs, so siblings can resolve each other's
    /// slots regardless of call order.
    fn hoist_block_functions(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            if let Stmt::Fn {
                name, id, location, ..
            } = stmt
            {
                let sentinel = Value::Uninitialized(Rc::new(name.clone()));
                self.emit_constant(sentinel, *location);
                let decl = self.resolutions.decl(*id);
                self.bind_decl_local(decl, *location);
            }
        }
    }

    /// Compiles `params`/`body` into a closure and leaves it on top of the
    /// stack. Shared by named function declarations, which then store it
    /// into the variable defined for the name, and lambda expressions,
    /// which leave it as their expression value.
    #[allow(clippy::expect_used)]
    fn generate_closure(
        &mut self,
        id: NodeId,
        name: &str,
        body: &[Stmt],
        location: SourceLocation,
    ) {
        self.functions.push(FunctionCompiler::new(
            &format!("function_{}", name),
            self.symbols.clone(),
            self.file.clone(),
        ));

        // Enter function scope
        self.current().scope_depth += 1;

        // Define parameters as local variables in the function scope. The
        // caller already pushed them onto the stack before Call/Invoke ran,
        // so no opcode in this chunk accounts for them; do it here instead.
        let resolutions = self.resolutions;
        for &decl in &resolutions.function(id).params {
            self.current().stack_height += 1;
            self.bind_local(decl);
        }

        // Compile function body
        self.hoist_block_functions(body);
        let end_location = *self
            .end_locations
            .get(&id)
            .unwrap_or_else(|| panic!("no end location recorded for {:?}", id));
        match body.split_last() {
            Some((last, init)) => {
                for stmt in init {
                    self.generate_stmt(stmt);
                    self.assert_stack_height();
                }
                if let Stmt::Expression { expr, .. } = last {
                    self.generate_expr_in_tail(expr, true);
                    self.emit_op_code(OpCode::Return, end_location);
                } else {
                    self.generate_stmt(last);
                    self.emit_return(end_location);
                }
            }
            None => self.emit_return(end_location),
        }

        let compiler = self
            .functions
            .pop()
            .expect("this function pushed a function compiler above");
        let arity = resolutions.function(id).params.len();
        let function_value = Value::new_function(name.to_string(), arity as u8, compiler.chunk);

        // Wrap the function in a closure.
        let const_index = self.current_chunk().add_constant(function_value);
        self.emit_index_op(OpCode::Closure, const_index, "constants", location);

        self.emit_upvalue_metadata(&resolutions.function(id).upvalues, location);
    }

    /// Stores the value on top of the stack into `id`'s target without
    /// leaving it on the stack, used for assignment as a statement.
    fn generate_store_without_push(&mut self, id: NodeId, location: SourceLocation) {
        self.emit_variable_set(id, location);
        self.emit_op_code(OpCode::Pop, location);
    }

    /// Generates the new value of a compound assignment: reads `read_id`,
    /// then applies `operator` against `value`.
    fn generate_compound_assign_value(
        &mut self,
        read_id: NodeId,
        operator: &BinaryOp,
        value: &Expr,
        location: SourceLocation,
    ) {
        self.emit_variable_get(read_id, location);
        self.generate_binary_op_tail(operator, value, location);
    }

    /// Generates `object; Dup; GetField field; value; operator`, leaving
    /// `[.., instance, result]` on the stack so the caller can finish with
    /// `SetField` (followed by `Pop` in statement position); `object`
    /// evaluates exactly once. Returns the field's symbol id.
    fn generate_field_compound_assign_value(
        &mut self,
        object: &Expr,
        field: &str,
        operator: &BinaryOp,
        value: &Expr,
        location: SourceLocation,
        operator_location: SourceLocation,
    ) -> u16 {
        self.generate_expr(object);
        self.emit_op_code(OpCode::Dup, location);
        let symbol = self.resolutions.symbol(field);
        self.emit_index_op(OpCode::GetField, symbol as u32, "symbols", location);
        self.generate_binary_op_tail(operator, value, operator_location);
        symbol
    }

    /// Generates `object; index; Dup2; GetIndex; value; operator`, leaving
    /// `[.., object, index, result]` on the stack so the caller can finish
    /// with `SetIndex`; `object` and `index` each evaluate exactly once.
    fn generate_index_compound_assign_value(
        &mut self,
        object: &Expr,
        index: &Expr,
        operator: &BinaryOp,
        value: &Expr,
        location: SourceLocation,
        operator_location: SourceLocation,
    ) {
        self.generate_expr(object);
        self.generate_expr(index);
        self.emit_op_code(OpCode::Dup2, location);
        self.emit_op_code(OpCode::GetIndex, location);
        self.generate_binary_op_tail(operator, value, operator_location);
    }

    fn generate_expression_stmt(&mut self, expr: &Expr, location: SourceLocation) {
        match expr {
            Expr::Assign {
                value,
                id,
                location,
                ..
            } => {
                self.generate_expr(value);
                self.generate_store_without_push(*id, *location);
            }
            Expr::CompoundAssign {
                name: _,
                operator,
                value,
                read_id,
                write_id,
                location,
            } => {
                self.generate_compound_assign_value(*read_id, operator, value, *location);
                self.generate_store_without_push(*write_id, *location);
            }
            Expr::SetField {
                object,
                field,
                value,
                location,
            } => {
                let symbol = self.resolutions.symbol(field);
                if let Expr::Variable { id, .. } = object.as_ref() {
                    if let Res::Local(decl) = self.resolutions.res(*id) {
                        // StoreLocalField reads the local after evaluating the value, so the
                        // value must not be able to reassign it.
                        if !self.resolutions.is_checked(*id) && self.resolutions.is_immutable(decl)
                        {
                            let slot = self.decl_slot(decl);
                            self.generate_expr(value);
                            self.emit_store_local_field(slot, symbol as u32, *location);
                            return;
                        }
                    }
                }
                self.generate_expr(object);
                self.generate_expr(value);
                self.emit_index_op(OpCode::SetField, symbol as u32, "symbols", *location);
                self.emit_op_code(OpCode::Pop, *location);
            }
            Expr::CompoundAssignField {
                object,
                field,
                operator,
                value,
                location,
                operator_location,
            } => {
                let symbol = self.generate_field_compound_assign_value(
                    object,
                    field,
                    operator,
                    value,
                    *location,
                    *operator_location,
                );
                self.emit_index_op(OpCode::SetField, symbol as u32, "symbols", *location);
                self.emit_op_code(OpCode::Pop, *location);
            }
            _ => {
                self.generate_expr(expr);
                self.emit_op_code(OpCode::Pop, location);
            }
        }
    }

    fn generate_block_stmt(&mut self, statements: &[Stmt], location: SourceLocation) {
        self.current().scope_depth += 1;
        self.hoist_block_functions(statements);
        for stmt in statements {
            self.generate_stmt(stmt);
            self.assert_stack_height();
        }
        self.end_scope(location);
    }

    fn end_scope(&mut self, location: SourceLocation) {
        self.current().scope_depth -= 1;
        let captured = self.discard_locals_above_current_depth();
        self.emit_scope_exit(&captured, location);
    }

    fn discard_locals_above_current_depth(&mut self) -> Vec<bool> {
        let scope_depth = self.current().scope_depth;
        self.current().pop_locals_above(scope_depth)
    }

    /// Emits one instruction per popped local, top-most first: CloseUpvalue
    /// for one a nested function captured, Pop otherwise.
    fn emit_scope_exit(&mut self, captured: &[bool], location: SourceLocation) {
        for &is_captured in captured {
            let op_code = if is_captured {
                OpCode::CloseUpvalue
            } else {
                OpCode::Pop
            };
            self.emit_op_code(op_code, location);
        }
    }

    fn always_exits(stmt: &Stmt) -> bool {
        match stmt {
            Stmt::Break { .. }
            | Stmt::Continue { .. }
            | Stmt::Return { .. }
            | Stmt::Throw { .. } => true,
            Stmt::Block { statements, .. } => statements.last().is_some_and(Self::always_exits),
            _ => false,
        }
    }

    fn generate_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
        location: SourceLocation,
    ) {
        let then_jump = self.generate_condition_jump(condition, location);
        // then_jump's target below is reached only via the false path, at
        // this height. then_branch's own height doesn't apply there (its end
        // is reached by a jump, or not at all when it always exits), so save
        // this to restore there.
        let false_path_height = self.current().stack_height;
        self.generate_stmt(then_branch);
        let else_jump = if else_branch.is_none() || Self::always_exits(then_branch) {
            None
        } else {
            Some(self.emit_jump(OpCode::Jump, location))
        };

        self.patch_jump(then_jump);
        self.current().stack_height = false_path_height;

        if let Some(else_stmt) = else_branch {
            self.generate_stmt(else_stmt);
        }
        if let Some(else_jump) = else_jump {
            self.patch_jump(else_jump);
        }
    }

    #[allow(clippy::expect_used)]
    fn generate_while_stmt(&mut self, condition: &Expr, body: &Stmt, location: SourceLocation) {
        let loop_start = self.current_chunk().instruction_count() as u32;

        // Push loop context for break/continue tracking
        let depth = self.current().scope_depth;
        let entry_stack_height = self.current().stack_height;
        let try_depth = self.current().try_depth;
        self.current().loop_contexts.push(LoopContext {
            loop_start,
            break_jumps: Vec::new(),
            continue_jumps: Vec::new(),
            depth,
            entry_stack_height,
            try_depth,
        });

        let exit_jump = self.generate_condition_jump(condition, location);
        // exit_jump, when taken, lands after the Loop below at this height.
        // Loop is a back-edge, not a fallthrough, so save this now and
        // restore it after Loop: the body's own height is not what the exit
        // lands with.
        let exit_height = self.current().stack_height;

        self.generate_stmt(body);

        // Pop loop context; continue jumps land here, before the Loop back.
        let loop_context = self
            .current()
            .loop_contexts
            .pop()
            .expect("this function pushed a loop context above");
        for continue_jump in loop_context.continue_jumps {
            self.patch_jump(continue_jump);
        }

        self.emit_loop(loop_start, location);
        self.current().stack_height = exit_height;

        self.patch_jump(exit_jump);

        // Patch all break jumps
        for break_jump in loop_context.break_jumps {
            self.patch_jump(break_jump);
        }
    }

    fn generate_return_stmt(&mut self, value: &Option<Expr>, location: SourceLocation) {
        match value {
            Some(value) => self.generate_expr_in_tail(value, true),
            None => self.emit_op_code(OpCode::Nil, location),
        }
        for _ in 0..self.current().try_depth {
            self.emit_op_code(OpCode::EndTry, location);
        }
        self.emit_op_code(OpCode::Return, location);
    }

    /// Compiles `expr`; when `tail` is set, the value is the function's
    /// return value, so a call here reuses the caller's frame and the
    /// branches of a conditional, if, or match are tail positions too.
    fn generate_expr_in_tail(&mut self, expr: &Expr, tail: bool) {
        let in_function = self.functions.len() > 1;
        match expr {
            Expr::Call {
                callee,
                arguments,
                id,
                location,
            } if tail && in_function && self.current().try_depth == 0 => match callee.as_ref() {
                Expr::GetField {
                    object,
                    field,
                    optional: false,
                    ..
                } => self.generate_method_call_expr(
                    *id,
                    object,
                    field,
                    arguments,
                    OpCode::TailInvoke,
                    *location,
                ),
                Expr::GetField { .. } => self.generate_expr(expr),
                _ => self.generate_call_expr(*id, callee, arguments, OpCode::TailCall, *location),
            },
            Expr::Conditional {
                condition,
                then_expr,
                else_expr,
                location,
            } => self.generate_conditional_expr(condition, then_expr, else_expr, tail, *location),
            Expr::If {
                condition,
                then_branch,
                else_branch,
                location,
            } => self.generate_if_expr(condition, then_branch, else_branch, tail, *location),
            Expr::Match {
                scrutinee,
                arms,
                location,
            } => self.generate_match_expr(scrutinee, arms, tail, *location),
            Expr::Grouping { expr, .. } => self.generate_expr_in_tail(expr, tail),
            _ => self.generate_expr(expr),
        }
    }

    // Leaves the locals in place; end_scope still owns them on fall-through.
    // Pops everything down to the loop's entry height, not just registered
    // locals, since a break/continue inside an if-expression branch can also
    // leave transient values interleaved with them.
    fn emit_loop_exit_pops(
        &mut self,
        depth: u32,
        entry_stack_height: u32,
        location: SourceLocation,
    ) {
        let any_captured = self
            .current()
            .captured_flags_above(depth)
            .iter()
            .any(|c| *c);
        let total_pops = self.current().stack_height - entry_stack_height;
        let op_code = if any_captured {
            OpCode::CloseUpvalue
        } else {
            OpCode::Pop
        };
        for _ in 0..total_pops {
            self.emit_op_code(op_code, location);
        }
    }

    #[allow(clippy::expect_used)]
    fn generate_loop_exit_stmt(&mut self, exit: LoopExit, location: SourceLocation) {
        // Emit a Jump opcode and record it for later patching. For continue,
        // this allows jumping to the right place, just before the Loop
        // instruction.
        let (depth, entry_stack_height, try_depth) = {
            let context = self
                .current()
                .loop_contexts
                .last()
                .expect("semantic pass guarantees a loop context");
            (context.depth, context.entry_stack_height, context.try_depth)
        };
        for _ in try_depth..self.current().try_depth {
            self.emit_op_code(OpCode::EndTry, location);
        }
        self.emit_loop_exit_pops(depth, entry_stack_height, location);

        let jump_index = self.emit_jump(OpCode::Jump, location);
        // The pops above tracked height down to the loop's entry height,
        // matching the real stack once this jump is taken. But locals above
        // that depth are still in scope here (break/continue doesn't remove
        // them), so resync to keep height matching locals (plus any
        // transient offset) for whatever code follows in this block,
        // reachable or not.
        self.current().stack_height =
            self.current().locals.len() as u32 + self.current().transient_offset;
        let context = self
            .current()
            .loop_contexts
            .last_mut()
            .expect("semantic pass guarantees a loop context");
        let jumps = match exit {
            LoopExit::Break => &mut context.break_jumps,
            LoopExit::Continue => &mut context.continue_jumps,
        };
        jumps.push(jump_index);
    }

    #[allow(clippy::expect_used)]
    fn generate_for_in_stmt(
        &mut self,
        pattern: &Pattern,
        collection: &Expr,
        body: &Stmt,
        location: SourceLocation,
    ) {
        // For-in loop code generation strategy: uses iterator opcodes.
        // The iterator state lives in two hidden locals (collection, index) below
        // the loop variable (or, for `for (a, b) in coll`, below the hidden pair
        // local and the names destructured from it).
        //
        // Bytecode structure:
        //   <evaluate collection>
        //   GetIterator              ; pop collection, push [iterable collection, index 0]
        //   loop_start:
        //   IteratorDone slot        ; pushes true if more, false if done
        //   PopJumpIfFalse exit_jump ; pop it; if false (done), exit loop
        //   IteratorNext slot        ; push collection[index], slot+1 index += 1
        //   <body with loop variable>       ; break/continue pop the loop variable
        //                                    ; and any body locals before jumping
        //   Pop                      ; Pop the loop variable value
        //   Loop loop_start          ; Jump back
        //   exit_jump:
        //   <break lands here>
        //   Pop, Pop                 ; pop the two hidden iterator slots

        // Evaluate the collection expression
        self.generate_expr(collection);

        // Convert collection to iterator: pushes the iterable collection and
        // the starting index as two hidden locals (see OpCode::GetIterator).
        self.emit_op_code(OpCode::GetIterator, location);
        self.current_chunk()
            .write_u8(if matches!(pattern, Pattern::Tuple(_)) {
                1
            } else {
                0
            });

        // Enter a block scope owning the two hidden iterator slots.
        self.current().scope_depth += 1;
        let hidden_depth = self.current().scope_depth;
        self.current().locals.push(Local::new(hidden_depth, false));
        self.current().locals.push(Local::new(hidden_depth, false));
        let iterator_slot = self.current().stack_height - 2;

        // Enter a nested scope for the loop variable and body, so break and
        // continue never pop the hidden iterator slots.
        self.current().scope_depth += 1;

        // Mark the start of the loop
        let loop_start = self.current_chunk().instruction_count() as u32;

        // Push loop context for break/continue tracking
        // - 1 so break/continue also pop the loop variable itself.
        let depth = self.current().scope_depth - 1;
        let entry_stack_height = self.current().stack_height;
        let try_depth = self.current().try_depth;
        self.current().loop_contexts.push(LoopContext {
            loop_start,
            break_jumps: Vec::new(),
            continue_jumps: Vec::new(),
            depth,
            entry_stack_height,
            try_depth,
        });

        // Check if iterator has more elements (pushes true if more, false if done)
        self.emit_index_op(OpCode::IteratorDone, iterator_slot, "locals", location);

        let exit_jump = self.emit_jump(OpCode::PopJumpIfFalse, location);
        // See generate_while_stmt: exit_jump lands after the Loop below,
        // which is a back-edge rather than a fallthrough, at this height
        // rather than the body's.
        let exit_height = self.current().stack_height;

        // Get next value from iterator (pushes value)
        self.emit_index_op(OpCode::IteratorNext, iterator_slot, "locals", location);

        match pattern {
            Pattern::Name(binding) => {
                // Define the loop variable (value is already on stack from IteratorNext)
                self.bind_decl_local(self.resolutions.decl(binding.id), location);
            }
            Pattern::Tuple(slots) => {
                // Hold the element in a hidden local, then define each name
                // from it, fresh every iteration.
                self.emit_index_op(
                    OpCode::CheckTuple,
                    slots.len() as u32,
                    "tuple pattern names",
                    location,
                );
                let pair_depth = self.current().scope_depth;
                self.current().locals.push(Local::new(pair_depth, false));
                let pair_slot = self.current().stack_height - 1;

                self.bind_tuple_slots(slots, pair_slot, location);
            }
        }

        // Generate the loop body
        self.generate_stmt(body);

        // Pop the loop variable (or hidden pair local and destructured
        // names), each with CloseUpvalue instead of Pop if captured.
        let captured = self.current().pop_locals_above(depth);
        self.emit_scope_exit(&captured, location);

        // Patch all continue jumps to point here (just before the Loop)
        // This allows continue to properly skip to the next iteration
        let loop_context = self
            .current()
            .loop_contexts
            .pop()
            .expect("this function pushed a loop context above");
        for continue_jump in loop_context.continue_jumps {
            self.patch_jump(continue_jump);
        }

        // Jump back to loop start (will push next value)
        self.emit_loop(loop_start, location);
        self.current().stack_height = exit_height;

        // Patch the exit jump
        self.patch_jump(exit_jump);

        // Patch all break jumps
        for break_jump in loop_context.break_jumps {
            self.patch_jump(break_jump);
        }

        // Exit the inner scope. Its locals' runtime slots and Local entries
        // were already popped above.
        self.current().scope_depth -= 1;

        // Exit the outer scope, popping the two hidden iterator slots.
        self.end_scope(location);
    }

    fn generate_try_stmt(
        &mut self,
        body: &Stmt,
        catch_binding: &Binding,
        catch_body: &Stmt,
        location: SourceLocation,
    ) {
        // Bytecode structure:
        //   BeginTry catch_target
        //   <body>
        //   EndTry
        //   Jump end
        //   catch_target:            ; the VM pushes the caught value here
        //   <catch body with the caught value as a local>
        //   Pop                      ; the caught value
        //   end:
        let catch_jump = self.emit_jump(OpCode::BeginTry, location);
        self.current().try_depth += 1;
        self.generate_stmt(body);
        self.current().try_depth -= 1;
        self.emit_op_code(OpCode::EndTry, location);
        let end_jump = self.emit_jump(OpCode::Jump, location);

        self.patch_jump(catch_jump);
        self.adjust_stack_height(1);
        self.current().scope_depth += 1;
        self.bind_decl_local(self.resolutions.decl(catch_binding.id), location);
        self.generate_stmt(catch_body);
        self.end_scope(location);

        self.patch_jump(end_jump);
    }

    fn generate_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Val {
                pattern,
                initializer,
                location,
            }
            | Stmt::Var {
                pattern,
                initializer,
                location,
            } => match (pattern, initializer) {
                (Pattern::Tuple(slots), Some(initializer)) => {
                    self.generate_tuple_declaration(slots, initializer, *location);
                }
                (Pattern::Name(binding), _) => {
                    self.generate_variable_declaration(binding.id, initializer, binding.location);
                }
                (Pattern::Tuple(_), None) => {
                    unreachable!("tuple pattern always has an initializer")
                }
            },
            Stmt::Fn {
                name,
                body,
                id,
                location,
                ..
            } => {
                self.generate_fn_stmt(*id, name, body, *location);
            }
            Stmt::Struct { .. } => {
                // Struct was already defined, nothing to do here
            }
            Stmt::Enum { .. } => {}
            Stmt::Impl { .. } => {
                // Methods were already compiled and registered in generate()'s pre-pass.
            }
            Stmt::Expression { expr, location } => {
                self.generate_expression_stmt(expr, *location);
            }
            Stmt::Block {
                statements,
                location,
            } => {
                self.generate_block_stmt(statements, *location);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                location,
            } => {
                self.generate_if_stmt(condition, then_branch, else_branch, *location);
            }
            Stmt::While {
                condition,
                body,
                location,
            } => {
                self.generate_while_stmt(condition, body, *location);
            }
            Stmt::Return { value, location } => {
                self.generate_return_stmt(value, *location);
            }
            Stmt::Import { .. } => {}
            Stmt::Export { declaration, .. } => self.generate_stmt(declaration),
            Stmt::Break { location } => {
                self.generate_loop_exit_stmt(LoopExit::Break, *location);
            }
            Stmt::Continue { location } => {
                self.generate_loop_exit_stmt(LoopExit::Continue, *location);
            }
            Stmt::ForIn {
                pattern,
                collection,
                body,
                location,
            } => {
                self.generate_for_in_stmt(pattern, collection, body, *location);
            }
            Stmt::Try {
                body,
                catch_binding,
                catch_body,
                location,
            } => {
                self.generate_try_stmt(body, catch_binding, catch_body, *location);
            }
            Stmt::Throw { value, location } => {
                self.generate_expr(value);
                self.emit_op_code(OpCode::Throw, *location);
            }
        }
    }

    // ===== Expression Generation =====

    fn generate_string_interpolation_expr(
        &mut self,
        parts: &[crate::compiler::ast::InterpolationPart],
        location: SourceLocation,
    ) {
        use crate::compiler::ast::InterpolationPart;

        // Generate code for each part and concatenate them
        let mut first = true;
        for part in parts {
            match part {
                InterpolationPart::Literal { value, .. } => {
                    self.emit_constant(string!(value.as_str()), location);
                }
                InterpolationPart::Expression(expr) => {
                    // Generate the expression
                    self.generate_expr(expr);
                    // Convert to string using ToString opcode
                    self.emit_op_code(OpCode::ToString, location);
                }
            }

            // Concatenate with previous parts (skip for first part)
            if !first {
                self.emit_op_code(OpCode::Add, location);
            }
            first = false;
        }

        // If there are no parts, emit an empty string
        if parts.is_empty() {
            self.emit_constant(string!(""), location);
        }
    }

    fn generate_binary_expr(
        &mut self,
        left: &Expr,
        operator: &BinaryOp,
        right: &Expr,
        location: SourceLocation,
    ) {
        // Handle short-circuit operators specially
        match operator {
            BinaryOp::And => {
                // For `a && b`:
                // 1. Evaluate left operand
                self.generate_expr(left);
                // 2. If false, skip right operand and result is false
                let end_jump = self.emit_jump(OpCode::JumpIfFalse, location);
                // 3. Left was true, pop it and evaluate right
                self.emit_op_code(OpCode::Pop, location);
                self.generate_expr(right);
                // 4. Patch jump to end (if left was false, we skip here with false on stack)
                self.patch_jump(end_jump);
            }
            BinaryOp::Or => {
                // For `a || b`:
                // 1. Evaluate left operand
                self.generate_expr(left);
                // 2. If false, jump to evaluate right operand
                let else_jump = self.emit_jump(OpCode::JumpIfFalse, location);
                // 3. Left was true, jump to end with true result
                let end_jump = self.emit_jump(OpCode::Jump, location);
                // 4. Patch else jump (left was false, need to evaluate right)
                self.patch_jump(else_jump);
                // 5. Pop false value and evaluate right
                self.emit_op_code(OpCode::Pop, location);
                self.generate_expr(right);
                // 6. Patch end jump (left was true, skip right evaluation)
                self.patch_jump(end_jump);
            }
            BinaryOp::NilCoalesce => {
                // For `a ?? b`:
                // 1. Evaluate left operand
                self.generate_expr(left);
                // 2. If not nil, jump to end with left result
                let end_jump = self.emit_jump(OpCode::JumpIfNotNil, location);
                // 3. Left was nil, pop it and evaluate right
                self.emit_op_code(OpCode::Pop, location);
                self.generate_expr(right);
                // 4. Patch end jump (left was not nil, skip right evaluation)
                self.patch_jump(end_jump);
            }
            _ => {
                self.generate_expr(left);
                self.generate_binary_op_tail(operator, right, location);
            }
        }
    }

    /// Emits `right`'s code followed by `operator`, applied to the operand
    /// already on top of the stack.
    fn generate_binary_op_tail(
        &mut self,
        operator: &BinaryOp,
        right: &Expr,
        location: SourceLocation,
    ) {
        self.generate_expr(right);

        match operator {
            BinaryOp::Add => self.emit_op_code(OpCode::Add, location),
            BinaryOp::Subtract => self.emit_op_code(OpCode::Subtract, location),
            BinaryOp::Multiply => self.emit_op_code(OpCode::Multiply, location),
            BinaryOp::Divide => self.emit_op_code(OpCode::Divide, location),
            BinaryOp::Modulo => self.emit_op_code(OpCode::Modulo, location),
            BinaryOp::Exponent => self.emit_op_code(OpCode::Exponent, location),
            BinaryOp::Equal => self.emit_op_code(OpCode::Equal, location),
            BinaryOp::NotEqual => {
                self.emit_op_code(OpCode::Equal, location);
                self.emit_op_code(OpCode::Not, location);
            }
            BinaryOp::Greater => self.emit_op_code(OpCode::Greater, location),
            BinaryOp::GreaterEqual => self.emit_op_code(OpCode::GreaterEqual, location),
            BinaryOp::Less => self.emit_op_code(OpCode::Less, location),
            BinaryOp::LessEqual => self.emit_op_code(OpCode::LessEqual, location),
            BinaryOp::BitwiseAnd => self.emit_op_code(OpCode::BitwiseAnd, location),
            BinaryOp::BitwiseOr => self.emit_op_code(OpCode::BitwiseOr, location),
            BinaryOp::BitwiseXor => self.emit_op_code(OpCode::BitwiseXor, location),
            BinaryOp::LeftShift => self.emit_op_code(OpCode::LeftShift, location),
            BinaryOp::RightShift => self.emit_op_code(OpCode::RightShift, location),
            BinaryOp::And | BinaryOp::Or | BinaryOp::NilCoalesce => unreachable!(),
        }
    }

    fn generate_call_expr(
        &mut self,
        id: NodeId,
        callee: &Expr,
        arguments: &[Expr],
        call_op: OpCode,
        location: SourceLocation,
    ) {
        let Some(index) = self.resolutions.native(id) else {
            self.generate_regular_call_expr(callee, arguments, call_op, location);
            return;
        };
        self.generate_native_call_expr(index, arguments, call_op, location);
    }

    fn generate_regular_call_expr(
        &mut self,
        callee: &Expr,
        arguments: &[Expr],
        call_op: OpCode,
        location: SourceLocation,
    ) {
        // Unified calling convention: [callable, args...]
        self.generate_expr(callee);

        for arg in arguments {
            self.generate_expr(arg);
        }

        self.emit_call(call_op, arguments.len() as u8, location);
    }

    /// Emits a call dispatched by registry index, known at compile time: a
    /// global function, a constructor, or a static method. The callable is
    /// pushed first (no callee/receiver is loaded), then the arguments.
    fn generate_native_call_expr(
        &mut self,
        index: usize,
        arguments: &[Expr],
        call_op: OpCode,
        location: SourceLocation,
    ) {
        let label = crate::common::method_registry::native_label(index);
        self.push_native_callable_by_index(label, index, arguments.len() as u8, location);

        for arg in arguments {
            self.generate_expr(arg);
        }

        self.emit_call(call_op, arguments.len() as u8, location);
    }

    fn generate_method_call_expr(
        &mut self,
        id: NodeId,
        object: &Expr,
        method: &str,
        arguments: &[Expr],
        invoke_op: OpCode,
        location: SourceLocation,
    ) {
        if let Some(access) = self.resolutions.enum_construct(id) {
            for arg in arguments {
                self.generate_expr(arg);
            }
            self.emit_enum_construct(access, location);
            return;
        }
        if let Some(access) = self.resolutions.enum_values_access(id) {
            for (ordinal, variant_name) in access.variants.iter().enumerate() {
                self.emit_enum_variant_constant(
                    &access.enum_name,
                    variant_name,
                    ordinal as u16,
                    &[],
                    location,
                );
            }
            self.emit_create_array(access.variants.len() as u16, location);
            return;
        }
        if let Expr::Variable { id: object_id, .. } = object {
            if let Some(slot) = self.resolutions.module_member(*object_id) {
                let call_op = match invoke_op {
                    OpCode::TailInvoke => OpCode::TailCall,
                    _ => OpCode::Call,
                };
                self.emit_index_op(OpCode::GetGlobal, slot, "globals", location);
                for arg in arguments {
                    self.generate_expr(arg);
                }
                self.emit_call(call_op, arguments.len() as u8, location);
                return;
            }
        }
        match self.resolutions.native(id) {
            Some(index) => self.generate_native_call_expr(index, arguments, OpCode::Call, location),
            None => self.generate_instance_method_call_expr(
                object, method, arguments, false, invoke_op, location,
            ),
        }
    }

    fn generate_instance_method_call_expr(
        &mut self,
        callee: &Expr,
        method: &str,
        arguments: &[Expr],
        optional: bool,
        invoke_op: OpCode,
        location: SourceLocation,
    ) {
        // Instance method call: arr.push(x), str.size(), etc.
        // Type is unknown at compile time, so dispatch by name at runtime.
        self.generate_expr(callee);
        let end_jump = optional.then(|| self.emit_jump(OpCode::JumpIfNil, location));

        for arg in arguments {
            self.generate_expr(arg);
        }

        self.emit_invoke(invoke_op, method, arguments.len() as u8, location);
        if let Some(end_jump) = end_jump {
            self.patch_jump(end_jump);
        }
    }

    fn generate_array_literal_expr(&mut self, elements: &[Expr], location: SourceLocation) {
        if self
            .check_count_limit(
                elements.len(),
                u16::MAX as usize,
                || {
                    format!(
                        "array literal too large: {} elements (maximum is {})",
                        elements.len(),
                        u16::MAX
                    )
                },
                location,
            )
            .is_none()
        {
            return;
        }

        // Generate code for all elements
        for element in elements {
            self.generate_expr(element);
        }

        // Emit CreateArray with the count of elements
        self.emit_create_array(elements.len() as u16, location);
    }

    fn generate_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number {
                value, location, ..
            } => {
                self.emit_constant(number!(*value), *location);
            }
            Expr::Int {
                value, location, ..
            } => {
                self.emit_constant(int!(*value), *location);
            }
            Expr::String {
                value, location, ..
            } => {
                self.emit_constant(string!(value.as_str()), *location);
            }
            Expr::StringInterpolation { parts, location } => {
                self.generate_string_interpolation_expr(parts, *location);
            }
            Expr::Boolean { value, location } => {
                if *value {
                    self.emit_op_code(OpCode::True, *location);
                } else {
                    self.emit_op_code(OpCode::False, *location);
                }
            }
            Expr::Nil { location } => {
                self.emit_op_code(OpCode::Nil, *location);
            }
            Expr::Variable { id, location, .. } => {
                self.emit_variable_get(*id, *location);
            }
            Expr::Assign {
                value,
                id,
                location,
                ..
            } => {
                self.generate_expr(value);
                self.emit_variable_set(*id, *location);
            }
            Expr::CompoundAssign {
                name: _,
                operator,
                value,
                read_id,
                write_id,
                location,
            } => {
                self.generate_compound_assign_value(*read_id, operator, value, *location);
                self.emit_variable_set(*write_id, *location);
            }
            Expr::Binary {
                left,
                operator,
                right,
                location,
            } => {
                self.generate_binary_expr(left, operator, right, *location);
            }
            Expr::Unary {
                operator,
                operand,
                location,
            } => {
                self.generate_expr(operand);
                match operator {
                    UnaryOp::Negate => self.emit_op_code(OpCode::Negate, *location),
                    UnaryOp::Not => self.emit_op_code(OpCode::Not, *location),
                    UnaryOp::BitwiseNot => self.emit_op_code(OpCode::BitwiseNot, *location),
                }
            }
            Expr::Call {
                callee,
                arguments,
                id,
                location,
            } => {
                if let Expr::GetField {
                    object,
                    field,
                    optional,
                    ..
                } = callee.as_ref()
                {
                    if *optional {
                        self.generate_instance_method_call_expr(
                            object,
                            field,
                            arguments,
                            true,
                            OpCode::Invoke,
                            *location,
                        );
                    } else {
                        self.generate_method_call_expr(
                            *id,
                            object,
                            field,
                            arguments,
                            OpCode::Invoke,
                            *location,
                        );
                    }
                } else {
                    self.generate_call_expr(*id, callee, arguments, OpCode::Call, *location);
                }
            }
            Expr::GetField {
                object,
                field,
                optional,
                location,
            } => {
                if *optional {
                    self.generate_expr(object);
                    let end_jump = self.emit_jump(OpCode::JumpIfNil, *location);
                    let symbol = self.resolutions.symbol(field);
                    self.emit_index_op(OpCode::GetField, symbol as u32, "symbols", *location);
                    self.patch_jump(end_jump);
                    return;
                }
                if let Expr::Variable { id, .. } = object.as_ref() {
                    if let Some(slot) = self.resolutions.module_member(*id) {
                        self.emit_index_op(OpCode::GetGlobal, slot, "globals", *location);
                        return;
                    }
                    if let Some(index) = self.resolutions.native(*id) {
                        let label = crate::common::method_registry::native_label(index);
                        let arity = crate::common::method_registry::native_arity(index);
                        self.push_native_callable_by_index(label, index, arity, *location);
                        return;
                    }
                    if let Some(access) = self.resolutions.enum_variant_access(*id) {
                        self.emit_enum_variant_constant(
                            &access.enum_name,
                            &access.variant_name,
                            access.ordinal,
                            &access.fields,
                            *location,
                        );
                        return;
                    }
                }
                // A variant of an exported enum, e.g. utils.Color.Red: the
                // access is recorded on the node naming the module.
                if let Expr::GetField { object: module, .. } = object.as_ref() {
                    if let Expr::Variable { id, .. } = module.as_ref() {
                        if let Some(access) = self.resolutions.enum_variant_access(*id) {
                            self.emit_enum_variant_constant(
                                &access.enum_name,
                                &access.variant_name,
                                access.ordinal,
                                &access.fields,
                                *location,
                            );
                            return;
                        }
                    }
                }
                let symbol = self.resolutions.symbol(field);
                self.generate_expr(object);
                self.emit_index_op(OpCode::GetField, symbol as u32, "symbols", *location);
            }
            Expr::SetField {
                object,
                field,
                value,
                location,
            } => {
                self.generate_expr(object);
                self.generate_expr(value);
                let symbol = self.resolutions.symbol(field);
                self.emit_index_op(OpCode::SetField, symbol as u32, "symbols", *location);
            }
            Expr::CompoundAssignField {
                object,
                field,
                operator,
                value,
                location,
                operator_location,
            } => {
                let symbol = self.generate_field_compound_assign_value(
                    object,
                    field,
                    operator,
                    value,
                    *location,
                    *operator_location,
                );
                self.emit_index_op(OpCode::SetField, symbol as u32, "symbols", *location);
            }
            Expr::Grouping { expr, .. } => {
                self.generate_expr(expr);
            }
            Expr::MapLiteral { entries, location } => {
                if self
                    .check_count_limit(
                        entries.len(),
                        u16::MAX as usize,
                        || {
                            format!(
                                "map literal too large: {} entries (maximum is {})",
                                entries.len(),
                                u16::MAX
                            )
                        },
                        *location,
                    )
                    .is_none()
                {
                    return;
                }

                for (key, value) in entries {
                    self.generate_expr(key);
                    self.generate_expr(value);
                }
                self.emit_create_map(entries.len() as u16, *location);
            }
            Expr::ArrayLiteral { elements, location } => {
                self.generate_array_literal_expr(elements, *location);
            }
            Expr::SetLiteral { elements, location } => {
                if self
                    .check_count_limit(
                        elements.len(),
                        u16::MAX as usize,
                        || {
                            format!(
                                "set literal too large: {} elements (maximum is {})",
                                elements.len(),
                                u16::MAX
                            )
                        },
                        *location,
                    )
                    .is_none()
                {
                    return;
                }

                for element in elements {
                    self.generate_expr(element);
                }
                self.emit_create_set(elements.len() as u16, *location);
            }
            Expr::Index {
                object,
                index,
                location,
            } => {
                self.generate_expr(object);
                self.generate_expr(index);
                self.emit_op_code(OpCode::GetIndex, *location);
            }
            Expr::IndexAssign {
                object,
                index,
                value,
                location,
            } => {
                self.generate_expr(object);
                self.generate_expr(index);
                self.generate_expr(value);
                self.emit_op_code(OpCode::SetIndex, *location);
            }
            Expr::CompoundAssignIndex {
                object,
                index,
                operator,
                value,
                location,
                operator_location,
            } => {
                self.generate_index_compound_assign_value(
                    object,
                    index,
                    operator,
                    value,
                    *location,
                    *operator_location,
                );
                self.emit_op_code(OpCode::SetIndex, *location);
            }
            Expr::Range {
                start,
                end,
                inclusive,
                location,
            } => {
                self.generate_expr(start);
                self.generate_expr(end);
                self.emit_op_code(OpCode::CreateRange, *location);
                self.current_chunk()
                    .write_u8(if *inclusive { 1 } else { 0 });
            }
            Expr::Conditional {
                condition,
                then_expr,
                else_expr,
                location,
            } => {
                self.generate_conditional_expr(condition, then_expr, else_expr, false, *location);
            }
            Expr::Function {
                body, id, location, ..
            } => {
                self.generate_closure(*id, "anonymous", body, *location);
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
                location,
            } => {
                self.generate_if_expr(condition, then_branch, else_branch, false, *location);
            }
            Expr::Match {
                scrutinee,
                arms,
                location,
            } => {
                self.generate_match_expr(scrutinee, arms, false, *location);
            }
        }
    }

    fn generate_conditional_expr(
        &mut self,
        condition: &Expr,
        then_expr: &Expr,
        else_expr: &Expr,
        tail: bool,
        location: SourceLocation,
    ) {
        let else_jump = self.generate_condition_jump(condition, location);
        let false_path_height = self.current().stack_height;
        self.generate_expr_in_tail(then_expr, tail);
        let end_jump = self.emit_jump(OpCode::Jump, location);

        self.patch_jump(else_jump);
        self.current().stack_height = false_path_height;
        self.generate_expr_in_tail(else_expr, tail);
        self.patch_jump(end_jump);
    }

    /// Compiles `if cond { ... } else ...` in expression position: a hidden
    /// local reserved before the branches holds the result; each branch
    /// runs in its own nested scope, which is torn down (popping the
    /// branch's own locals / closing their upvalues) right after storing
    /// its value into the hidden local. That leaves the hidden local's
    /// value sitting on top of the stack with nothing above it, so it's
    /// already the if-expression's value - dropping it from the compiler's
    /// local bookkeeping (without popping it at runtime) hands it to
    /// whatever uses the expression next, the same way any other
    /// expression's value is handed off. Mirrors the hidden locals for-in
    /// keeps for its iterator state - no new opcode.
    fn generate_if_expr(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &IfExprElse,
        tail: bool,
        location: SourceLocation,
    ) {
        self.current().scope_depth += 1;
        let hidden_depth = self.current().scope_depth;
        self.emit_op_code(OpCode::Nil, location);
        self.current().locals.push(Local::new(hidden_depth, false));
        let hidden_slot = self.current().stack_height - 1;

        let then_jump = self.generate_condition_jump(condition, location);
        // See generate_if_stmt: then_jump lands here, not at the height the
        // then-branch leaves behind.
        let false_path_height = self.current().stack_height;
        self.generate_if_expr_branch(then_branch, hidden_slot, tail);
        let else_jump = self.emit_jump(OpCode::Jump, location);

        self.patch_jump(then_jump);
        self.current().stack_height = false_path_height;

        match else_branch {
            IfExprElse::If(expr) => {
                self.generate_expr_in_tail(expr, tail);
                self.emit_index_op(OpCode::SetLocal, hidden_slot, "locals", location);
                self.emit_op_code(OpCode::Pop, location);
            }
            IfExprElse::Block(stmt) => {
                self.generate_if_expr_branch(stmt, hidden_slot, tail);
            }
        }
        self.patch_jump(else_jump);

        self.current().scope_depth -= 1;
        self.current().locals.pop();
    }

    /// Compiles one `{ ... }` branch of an if-expression: its last
    /// expression statement becomes the branch's value, stored into
    /// `hidden_slot`; any other kind of last statement (or an empty
    /// branch) leaves `nil`.
    fn generate_if_expr_branch(&mut self, branch: &Stmt, hidden_slot: u32, tail: bool) {
        let Stmt::Block {
            statements,
            location: branch_location,
        } = branch
        else {
            unreachable!("if-expression branches are always blocks")
        };
        let branch_location = *branch_location;

        self.current().scope_depth += 1;
        self.hoist_block_functions(statements);
        let previous_offset = self.current().transient_offset;
        self.current().transient_offset =
            self.current().stack_height - self.current().locals.len() as u32;
        match statements.split_last() {
            Some((last, init)) => {
                for stmt in init {
                    self.generate_stmt(stmt);
                    self.assert_stack_height();
                }
                if let Stmt::Expression { expr, .. } = last {
                    self.generate_expr_in_tail(expr, tail);
                } else {
                    self.generate_stmt(last);
                    self.assert_stack_height();
                    self.emit_op_code(OpCode::Nil, branch_location);
                }
            }
            None => self.emit_op_code(OpCode::Nil, branch_location),
        }
        self.emit_index_op(OpCode::SetLocal, hidden_slot, "locals", branch_location);
        self.emit_op_code(OpCode::Pop, branch_location);
        self.end_scope(branch_location);
        self.current().transient_offset = previous_offset;
    }

    /// Compiles `match scrutinee { pattern, pattern -> body ... }`. A hidden
    /// local holds the scrutinee; the matching arm's value overwrites it.
    fn generate_match_expr(
        &mut self,
        scrutinee: &Expr,
        arms: &[MatchArm],
        tail: bool,
        location: SourceLocation,
    ) {
        self.current().scope_depth += 1;
        let hidden_depth = self.current().scope_depth;
        self.generate_expr(scrutinee);
        self.current().locals.push(Local::new(hidden_depth, false));
        let hidden_slot = self.current().stack_height - 1;

        let mut end_jumps = Vec::new();
        for arm in arms {
            self.current().scope_depth += 1;
            let slots = self.bind_match_arm_placeholders(arm);
            self.generate_match_arm_test(hidden_slot, &arm.patterns, &slots, arm.location);
            let next_arm_jump = self.emit_jump(OpCode::PopJumpIfFalse, arm.location);
            let false_path_height = self.current().stack_height;
            let guard_false = self.generate_match_arm_body(arm, hidden_slot, tail);
            self.current().scope_depth -= 1;
            let captured = self.discard_locals_above_current_depth();
            self.emit_scope_exit(&captured, arm.location);
            end_jumps.push(self.emit_jump(OpCode::Jump, arm.location));

            let mut guard_failed_jump = None;
            if let Some((jump, guard_height)) = guard_false {
                self.patch_jump(jump);
                self.current().stack_height = guard_height;
                self.emit_scope_exit(&captured, arm.location);
                guard_failed_jump = Some(self.emit_jump(OpCode::Jump, arm.location));
            }

            self.patch_jump(next_arm_jump);
            self.current().stack_height = false_path_height;
            self.emit_scope_exit(&captured, arm.location);
            if let Some(jump) = guard_failed_jump {
                self.patch_jump(jump);
            }
        }

        self.emit_index_op(OpCode::GetLocal, hidden_slot, "locals", location);
        self.emit_op_code(OpCode::NoMatchArm, location);

        for jump in end_jumps {
            self.patch_jump(jump);
        }

        self.current().scope_depth -= 1;
        self.current().locals.pop();
    }

    /// Pushes a nil local for each name the arm binds, so the test can fill
    /// in whichever alternative matched. Returns each name with its slot.
    fn bind_match_arm_placeholders(&mut self, arm: &MatchArm) -> Vec<(String, u32)> {
        let mut slots = Vec::new();
        for (binding, _) in arm.patterns[0].bindings() {
            self.emit_op_code(OpCode::Nil, binding.location);
            let decl = self.resolutions.decl(binding.id);
            self.bind_decl_local(decl, binding.location);
            slots.push((binding.name.clone(), self.decl_slot(decl)));
        }
        slots
    }

    /// Runs the arm's guard and body inside the arm's binding scope.
    /// Returns the jump taken when the guard is false and the stack height
    /// it is taken at.
    fn generate_match_arm_body(
        &mut self,
        arm: &MatchArm,
        hidden_slot: u32,
        tail: bool,
    ) -> Option<(u32, u32)> {
        let guard_false = arm.guard.as_ref().map(|guard| {
            let jump = self.generate_condition_jump(guard, arm.location);
            let guard_height = self.current().stack_height;
            (jump, guard_height)
        });
        self.generate_match_arm_value(&arm.body, hidden_slot, tail, arm.location);
        guard_false
    }

    /// Stores an arm's body value into `hidden_slot`, the same way an
    /// if-expression branch stores its own result.
    fn generate_match_arm_value(
        &mut self,
        body: &MatchArmBody,
        hidden_slot: u32,
        tail: bool,
        location: SourceLocation,
    ) {
        match body {
            MatchArmBody::Expr(expr) => {
                self.generate_expr_in_tail(expr, tail);
                self.emit_index_op(OpCode::SetLocal, hidden_slot, "locals", location);
                self.emit_op_code(OpCode::Pop, location);
            }
            MatchArmBody::Block(stmt) => self.generate_if_expr_branch(stmt, hidden_slot, tail),
        }
    }

    /// Leaves a boolean on the stack: whether the value in `hidden_slot`
    /// matches any of `patterns`, tested left to right with `||`'s
    /// short-circuit (mirrors `generate_binary_expr`'s `BinaryOp::Or`). The
    /// matching alternative stores what it binds into `slots`.
    fn generate_match_arm_test(
        &mut self,
        hidden_slot: u32,
        patterns: &[MatchPattern],
        slots: &[(String, u32)],
        location: SourceLocation,
    ) {
        let Some((first, rest)) = patterns.split_first() else {
            unreachable!("a match arm always has at least one pattern")
        };
        self.generate_match_alternative(hidden_slot, first, slots, location);
        for pattern in rest {
            let else_jump = self.emit_jump(OpCode::JumpIfFalse, location);
            let end_jump = self.emit_jump(OpCode::Jump, location);
            self.patch_jump(else_jump);
            self.emit_op_code(OpCode::Pop, location);
            self.generate_match_alternative(hidden_slot, pattern, slots, location);
            self.patch_jump(end_jump);
        }
    }

    /// Leaves a boolean on the stack: whether the value in `hidden_slot`
    /// matches `pattern`. When it does, the names the pattern binds are
    /// stored into their `slots` first.
    fn generate_match_alternative(
        &mut self,
        hidden_slot: u32,
        pattern: &MatchPattern,
        slots: &[(String, u32)],
        location: SourceLocation,
    ) {
        self.generate_match_pattern_test(hidden_slot, &[], pattern, location);
        let bindings = pattern.bindings();
        if bindings.is_empty() {
            return;
        }
        let no_match_jump = self.emit_jump(OpCode::JumpIfFalse, location);
        self.emit_op_code(OpCode::Pop, location); // Pop the test result
        for (binding, path) in bindings {
            let Some((_, slot)) = slots.iter().find(|(name, _)| *name == binding.name) else {
                unreachable!("an alternative binds the names the arm declared")
            };
            self.emit_match_subject(hidden_slot, &path, binding.location);
            self.emit_index_op(OpCode::SetLocal, *slot, "locals", binding.location);
            self.emit_op_code(OpCode::Pop, binding.location);
        }
        self.emit_op_code(OpCode::True, location);
        self.patch_jump(no_match_jump);
    }

    /// Pushes the value `path` leads to from the value in `hidden_slot`:
    /// the element at each index in turn, for a rest step a new array of
    /// the elements it covers, and for a field step that variant field.
    fn emit_match_subject(
        &mut self,
        hidden_slot: u32,
        path: &[PathStep],
        location: SourceLocation,
    ) {
        self.emit_index_op(OpCode::GetLocal, hidden_slot, "locals", location);
        for &step in path {
            match step {
                PathStep::Index(index) => {
                    self.emit_constant(Value::Int(index), location);
                    self.emit_op_code(OpCode::GetIndex, location);
                }
                PathStep::Rest { before, after: 0 } => {
                    self.emit_constant(Value::Int(before as i64), location);
                    self.emit_invoke(OpCode::Invoke, "drop", 1, location);
                }
                PathStep::Rest { before, after } => {
                    self.emit_constant(Value::Int(before as i64), location);
                    self.emit_constant(Value::Int(-(after as i64)), location);
                    self.emit_invoke(OpCode::Invoke, "slice", 2, location);
                }
                PathStep::Field { variant, index } => {
                    let Some(access) = self.resolutions.enum_variant_access(variant) else {
                        unreachable!("a variant pattern resolves to an enum variant")
                    };
                    let symbol = self.resolutions.symbol(&access.fields[index]);
                    self.emit_index_op(OpCode::GetField, symbol as u32, "symbols", location);
                }
            }
        }
    }

    /// Leaves a boolean on the stack: whether the value `path` leads to
    /// from `hidden_slot` matches one pattern. A wildcard always matches; a
    /// range is tested by containment, and only a number can be in one; an
    /// array pattern needs an array of exactly its length whose elements
    /// match; a variant pattern needs that variant with matching fields;
    /// anything else is tested with `==`.
    fn generate_match_pattern_test(
        &mut self,
        hidden_slot: u32,
        path: &[PathStep],
        pattern: &MatchPattern,
        location: SourceLocation,
    ) {
        match pattern {
            MatchPattern::Wildcard(_) | MatchPattern::Binding(_) | MatchPattern::Rest { .. } => {
                self.emit_op_code(OpCode::True, location)
            }
            MatchPattern::Array {
                elements,
                location: array_location,
            } => {
                self.emit_match_subject(hidden_slot, path, *array_location);
                let has_rest = elements
                    .iter()
                    .any(|element| matches!(element, MatchPattern::Rest { .. }));
                let length = elements.len() - usize::from(has_rest);
                self.emit_array_length_test(length as u32, has_rest, *array_location);
                let steps = PathStep::for_elements(elements);
                self.generate_match_element_tests(
                    hidden_slot,
                    path,
                    &steps,
                    elements,
                    *array_location,
                );
            }
            MatchPattern::Variant { target, fields } => {
                let Expr::GetField {
                    object, location, ..
                } = target
                else {
                    unreachable!("a variant pattern's target is an enum variant access")
                };
                let Expr::Variable { id, .. } = object.as_ref() else {
                    unreachable!("a variant pattern's target is an enum variant access")
                };
                let Some(access) = self.resolutions.enum_variant_access(*id).cloned() else {
                    unreachable!("a variant pattern resolves to an enum variant")
                };
                self.emit_match_subject(hidden_slot, path, *location);
                let template = self.enum_variant_value(
                    &access.enum_name,
                    &access.variant_name,
                    access.ordinal,
                    &access.fields,
                );
                let index = self.add_constant(template);
                self.emit_index_op(OpCode::IsVariant, index, "constants", *location);
                let steps: Vec<PathStep> = (0..fields.len())
                    .map(|index| PathStep::Field {
                        variant: *id,
                        index,
                    })
                    .collect();
                self.generate_match_element_tests(hidden_slot, path, &steps, fields, *location);
            }
            MatchPattern::Expr(Expr::Range {
                start,
                end,
                inclusive,
                location: range_location,
            }) => {
                self.emit_match_subject(hidden_slot, path, *range_location);
                self.emit_op_code(OpCode::IsNumber, *range_location);
                let not_number_jump = self.emit_jump(OpCode::JumpIfFalse, *range_location);
                self.emit_op_code(OpCode::Pop, *range_location);
                self.emit_match_subject(hidden_slot, path, *range_location);
                self.generate_expr(start);
                self.emit_op_code(OpCode::GreaterEqual, *range_location);
                let end_jump = self.emit_jump(OpCode::JumpIfFalse, *range_location);
                self.emit_op_code(OpCode::Pop, *range_location);
                self.emit_match_subject(hidden_slot, path, *range_location);
                self.generate_expr(end);
                let op_code = if *inclusive {
                    OpCode::LessEqual
                } else {
                    OpCode::Less
                };
                self.emit_op_code(op_code, *range_location);
                self.patch_jump(end_jump);
                self.patch_jump(not_number_jump);
            }
            MatchPattern::Expr(expr) => {
                self.emit_match_subject(hidden_slot, path, location);
                self.generate_expr(expr);
                self.emit_op_code(OpCode::Equal, location);
            }
        }
    }

    /// Continues a container test already on the stack: for each refutable
    /// element, in turn, short-circuits on a false result and otherwise
    /// replaces it with whether the element at its step matches.
    fn generate_match_element_tests(
        &mut self,
        hidden_slot: u32,
        path: &[PathStep],
        steps: &[PathStep],
        elements: &[MatchPattern],
        location: SourceLocation,
    ) {
        let mut fail_jumps = Vec::new();
        for (step, element) in steps.iter().zip(elements) {
            if element.is_irrefutable() {
                continue;
            }
            fail_jumps.push(self.emit_jump(OpCode::JumpIfFalse, location));
            self.emit_op_code(OpCode::Pop, location);
            let mut element_path = path.to_vec();
            element_path.push(*step);
            self.generate_match_pattern_test(hidden_slot, &element_path, element, location);
        }
        for jump in fail_jumps {
            self.patch_jump(jump);
        }
    }

    /// Pushes the placeholder native callable for a call dispatched by
    /// registry index.
    fn push_native_callable_by_index(
        &mut self,
        type_name: String,
        index: usize,
        arity: u8,
        location: SourceLocation,
    ) {
        let callable = Value::new_native_function(type_name, arity, index as u32);
        self.emit_constant(callable, location);
    }

    fn emit_call(&mut self, call_op: OpCode, argc: u8, location: SourceLocation) {
        self.emit_op_code(call_op, location);
        self.current_chunk().write_u8(argc);
        // Pops the callable and all argc arguments, pushes one result.
        self.adjust_stack_height(-(argc as i32));
    }

    /// Emits `Invoke` or `TailInvoke`: a method call dispatched by name at
    /// runtime. The stack must already hold `[receiver, args...]`.
    fn emit_invoke(
        &mut self,
        op_code: OpCode,
        method_name: &str,
        argc: u8,
        location: SourceLocation,
    ) {
        let symbol = self.resolutions.symbol(method_name);
        self.emit_op_code(op_code, location);
        self.current_chunk().write_u16(symbol);
        self.current_chunk().write_u8(argc);
        // Pops the receiver and all argc arguments, pushes one result.
        self.adjust_stack_height(-(argc as i32));
    }

    /// Emits `CreateArray`, popping `count` elements and pushing the array.
    fn emit_create_array(&mut self, count: u16, location: SourceLocation) {
        self.emit_op_code(OpCode::CreateArray, location);
        self.current_chunk().write_u16(count);
        self.adjust_stack_height(1 - count as i32);
    }

    /// Emits `CreateMap`, popping `count` key/value pairs and pushing the map.
    fn emit_create_map(&mut self, count: u16, location: SourceLocation) {
        self.emit_op_code(OpCode::CreateMap, location);
        self.current_chunk().write_u16(count);
        self.adjust_stack_height(1 - 2 * count as i32);
    }

    /// Emits `CreateSet`, popping `count` elements and pushing the set.
    fn emit_create_set(&mut self, count: u16, location: SourceLocation) {
        self.emit_op_code(OpCode::CreateSet, location);
        self.current_chunk().write_u16(count);
        self.adjust_stack_height(1 - count as i32);
    }

    /// Emits `DefineMethod` or `DefineBuiltinMethod`, popping the closure
    /// left on top of the stack by a preceding `generate_closure` call (and,
    /// for `DefineMethod`, the struct value below it) and registering it
    /// under `method_symbol`, along with whether it takes `self`.
    fn emit_define_method(
        &mut self,
        op_code: OpCode,
        type_name: &str,
        method_name: &str,
        takes_self: bool,
        location: SourceLocation,
    ) {
        let type_symbol = self.resolutions.symbol(type_name);
        let method_symbol = self.resolutions.symbol(method_name);
        self.emit_op_code(op_code, location);
        self.current_chunk().write_u16(type_symbol);
        self.current_chunk().write_u16(method_symbol);
        self.current_chunk().write_u8(takes_self as u8);
    }
}
