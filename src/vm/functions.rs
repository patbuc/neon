use crate::common::constants::{MAX_FRAMES, MAX_NATIVE_CALL_DEPTH};
use crate::common::method_registry::NativeCallable;
use crate::common::runtime_error::RuntimeError;
use crate::common::{
    compare_int_and_float, f64_fits_i64, MapKey, NativeCallError, NativeContext, ObjInstance,
    ObjNativeFunction, ObjStruct, Value,
};
use crate::common::{find_method_entry, ObjClosure, Upvalue};
use crate::vm::VirtualMachine;
use crate::{boolean, int, is_false_like, number, string};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::rc::Rc;

pub(in crate::vm) type OpResult = std::result::Result<(), RuntimeError>;

/// Coerces an index `Value` to a number, normalizes a negative index against
/// `len`, and bounds-checks it. Shared by the Array, Range and String arms of
/// `op_get_index`.
fn normalize_index(index_value: Value, len: i64, type_name: &str) -> Result<usize, String> {
    let index = match index_value {
        Value::Number(n) => n as i64,
        Value::Int(i) => i,
        _ => {
            return Err(format!(
                "{} index must be a number, got {}.",
                type_name, index_value
            ));
        }
    };

    let actual_index = if index < 0 { len + index } else { index };

    if actual_index < 0 || actual_index >= len {
        return Err(format!(
            "{} index out of bounds: index {} (normalized: {}) on {} of length {}.",
            type_name,
            index,
            actual_index,
            type_name.to_lowercase(),
            len
        ));
    }

    Ok(actual_index as usize)
}

pub(in crate::vm) enum Comparison {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

/// Registry index for the print() function (always at index 0)
#[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
const PRINT_METHOD_INDEX: u32 = 0;

/// Symbol ids for builtin types, matching `BUILTIN_TYPE_NAMES`'s order
/// (which is also the order `Resolutions` seeds them in, so these ids
/// line up with the ones codegen and the symbol table use).
const ARRAY_SYMBOL: u16 = 0;
const STRING_SYMBOL: u16 = 1;
const MAP_SYMBOL: u16 = 2;
const SET_SYMBOL: u16 = 3;
const NUMBER_SYMBOL: u16 = 4;
const BOOLEAN_SYMBOL: u16 = 5;
const FILE_SYMBOL: u16 = 6;
const RANGE_SYMBOL: u16 = 7;
const PRIORITY_QUEUE_SYMBOL: u16 = 8;

/// A receiver's type name for method dispatch: a fixed symbol id for
/// builtin types, or the struct definition for an instance (cloning the
/// `Rc` is a refcount bump, not a string allocation).
enum TypeName {
    Builtin(u16),
    Struct(Rc<ObjStruct>),
}

impl TypeName {
    fn as_str(&self) -> &str {
        match self {
            TypeName::Builtin(symbol) => {
                crate::common::method_registry::BUILTIN_TYPE_NAMES[*symbol as usize]
            }
            TypeName::Struct(r#struct) => &r#struct.name,
        }
    }
}

impl std::fmt::Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Outcome of looking up a by-name call in the user method table.
enum MethodDispatch {
    /// No user method matches; fall through to native dispatch.
    NotFound,
    /// The call used the wrong form for the method (static vs. instance).
    Mismatch(RuntimeError),
    /// The closure to call, its effective argument count, and whether to
    /// exclude `self` from an arity-mismatch message.
    Found(Rc<ObjClosure>, usize, bool),
}

impl VirtualMachine {
    #[inline(always)]
    pub(in crate::vm) fn op_to_string(&mut self) {
        let value = self.pop();
        let string_value = string!(value.to_string());
        self.push(string_value);
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_not(&mut self) {
        // [.., operand] -> [.., result]
        let slot = self.stack.last_mut().expect("operand is on the stack");
        *slot = boolean!(is_false_like!(*slot));
    }

    /// Stack: `[.., field values...]` -> `[.., variant]`.
    pub(in crate::vm) fn op_enum_construct(&mut self, index: u16) -> OpResult {
        let Value::EnumVariant(template) = self.chunk.read_constant(index as usize) else {
            return Err(self.runtime_error("Enum constructor constant is not an enum variant."));
        };
        let fields = self
            .stack
            .split_off(self.stack.len() - template.field_symbols.len());
        self.push(Value::EnumVariant(Rc::new(template.with_fields(fields))));
        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_is_number(&mut self) {
        // [.., value] -> [.., is_number]
        let slot = self.stack.last_mut().expect("operand is on the stack");
        let is_number = matches!(slot, Value::Number(_) | Value::Int(_));
        *slot = boolean!(is_number);
    }

    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_is_variant(&mut self, index: u16) -> OpResult {
        // [.., value] -> [.., is_variant]
        let Value::EnumVariant(template) = self.chunk.read_constant(index as usize) else {
            return Err(self.runtime_error("Variant pattern constant is not an enum variant."));
        };
        let slot = self.stack.last_mut().expect("operand is on the stack");
        let matches = matches!(slot, Value::EnumVariant(variant)
            if variant.ordinal == template.ordinal && variant.enum_name == template.enum_name);
        *slot = boolean!(matches);
        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_is_array_of_len(&mut self, length: u16, at_least: bool) {
        // [.., value] -> [.., is_array_of_len]
        let length = length as usize;
        let slot = self.stack.last_mut().expect("operand is on the stack");
        let matches = matches!(slot, Value::Array(array) if {
            let actual = array.borrow().len();
            if at_least { actual >= length } else { actual == length }
        });
        *slot = boolean!(matches);
    }

    #[inline(always)]
    pub(in crate::vm) fn op_call(&mut self, arg_count: u8) -> OpResult {
        let arg_count = arg_count as usize;
        self.ip += 1;

        self.check_frame_limit()?;

        let callable_index = self.stack.len() - 1 - arg_count;
        let callable_value = std::mem::replace(&mut self.stack[callable_index], Value::Nil);
        match callable_value {
            Value::Closure(closure) => self.call_closure(arg_count, closure),
            other => self.dispatch_call_value(other, arg_count),
        }
    }

    /// TailCall: `Call` followed by `Return`, except that a closure callee
    /// takes over the running frame. Returns whether it did; otherwise the
    /// callee's result is on the stack for the caller to return.
    #[inline(never)]
    pub(in crate::vm) fn op_tail_call(&mut self, arg_count: u8) -> Result<bool, RuntimeError> {
        let arg_count = arg_count as usize;
        self.ip += 1;

        let callable_index = self.stack.len() - 1 - arg_count;
        let callable_value = std::mem::replace(&mut self.stack[callable_index], Value::Nil);
        let Value::Closure(closure) = callable_value else {
            self.check_frame_limit()?;
            self.dispatch_call_value(callable_value, arg_count)?;
            return Ok(false);
        };

        self.reuse_frame(callable_index, arg_count, closure, false)?;
        Ok(true)
    }

    /// TailInvoke: `Invoke` followed by `Return`, except that a user-defined
    /// method or a closure in an instance field takes over the running frame.
    /// Returns whether it did; otherwise the callee's result is on the stack
    /// for the caller to return.
    #[inline(never)]
    pub(in crate::vm) fn op_tail_invoke(
        &mut self,
        method_symbol: u16,
        arg_count: u8,
    ) -> Result<bool, RuntimeError> {
        let arg_count = arg_count as usize;
        self.ip += 1;

        let receiver_index = self.stack.len() - arg_count - 1;
        let receiver = self.stack[receiver_index].clone();
        let type_name = self.get_type_name(&receiver);
        if let Some(type_name) = &type_name {
            let is_static_call = matches!(receiver, Value::Struct(_));
            match self.dispatch_user_method(
                type_name,
                is_static_call,
                receiver_index,
                arg_count,
                method_symbol,
            ) {
                MethodDispatch::Found(closure, arg_count, exclude_self) => {
                    self.reuse_frame(receiver_index, arg_count, closure, exclude_self)?;
                    return Ok(true);
                }
                MethodDispatch::Mismatch(e) => return Err(e),
                MethodDispatch::NotFound => {}
            }
        }

        self.check_frame_limit()?;
        match self.dispatch_native_method_or_field(
            &receiver,
            type_name,
            receiver_index,
            method_symbol,
            arg_count,
        )? {
            Some(Value::Closure(closure)) => {
                self.reuse_frame(receiver_index, arg_count, closure, false)?;
                Ok(true)
            }
            Some(callable) => {
                self.dispatch_call_value(callable, arg_count)?;
                Ok(false)
            }
            None => Ok(false),
        }
    }

    /// Replaces the running frame with a call to `closure` whose callee slot
    /// is `callable_index`, followed by its `arg_count` arguments.
    #[allow(clippy::expect_used)]
    fn reuse_frame(
        &mut self,
        callable_index: usize,
        arg_count: usize,
        closure: Rc<ObjClosure>,
        exclude_self: bool,
    ) -> OpResult {
        let arity = closure.function.arity;
        if arg_count != arity as usize {
            return Err(self.arity_error(arg_count, arity, exclude_self, &closure.function.name));
        }

        // [.., frame slots..., callee, args...] -> [.., callee, args...]
        let slot_start = self.current_frame().slot_start as usize;
        self.close_upvalues_above(slot_start + 1);
        self.stack.drain(slot_start..callable_index);

        self.ip = 0;
        self.chunk = Rc::clone(&closure.function.chunk);
        self.call_frames
            .last_mut()
            .expect("a tail call runs inside a function's frame")
            .closure = closure;
        Ok(())
    }

    /// Invoke: a method call dispatched by name at runtime. Stack before:
    /// `[receiver, args...]`, argc excluding the receiver.
    #[inline(always)]
    pub(in crate::vm) fn op_invoke(&mut self, method_symbol: u16, arg_count: u8) -> OpResult {
        let arg_count = arg_count as usize;
        self.ip += 1;

        self.check_frame_limit()?;

        self.dispatch_invoke(method_symbol, arg_count)
    }

    /// Errors with "Stack overflow" if the call frame stack is already at
    /// its limit.
    fn check_frame_limit(&self) -> OpResult {
        if self.call_frames.len() >= MAX_FRAMES {
            Err(self.call_error("Stack overflow"))
        } else {
            Ok(())
        }
    }

    /// Dispatches a call: the stack must already hold `[callable, args...]`.
    /// Used by `call_value`'s re-entrant native-to-Neon calls.
    fn dispatch_call(&mut self, arg_count: usize) -> OpResult {
        // Nothing reads the callee's slot again; locals start above it.
        let callable_index = self.stack.len() - 1 - arg_count;
        let callable_value = std::mem::replace(&mut self.stack[callable_index], Value::Nil);
        self.dispatch_call_value(callable_value, arg_count)
    }

    /// Dispatches an already-extracted callable value (its stack slot has
    /// already been replaced with `Value::Nil`).
    fn dispatch_call_value(&mut self, callable_value: Value, arg_count: usize) -> OpResult {
        let result = match callable_value {
            Value::Closure(closure) => return self.call_closure(arg_count, closure),
            Value::Struct(r#struct) => return self.instantiate_struct(arg_count, r#struct),
            Value::NativeFunction(callable) => {
                match self.call_native_function(arg_count, &callable) {
                    Ok(value) => value,
                    Err(NativeCallError::Message(error)) => return Err(self.call_error(error)),
                    Err(NativeCallError::Runtime(e)) => return Err(e),
                }
            }
            Value::EnumVariant(template) if template.is_template() => {
                let expected = template.field_symbols.len();
                if arg_count != expected {
                    return Err(self.call_error(format!(
                        "Expected {} arguments but got {} for '{}'.",
                        expected, arg_count, template.variant_name
                    )));
                }
                let fields = self.stack[self.stack.len() - arg_count..].to_vec();
                Value::EnumVariant(Rc::new(template.with_fields(fields)))
            }
            _ => {
                return Err(self.call_error("Value is not callable"));
            }
        };

        // Pop callable and arguments, then push result
        let stack_len = self.stack.len();
        self.stack.truncate(stack_len - arg_count - 1);
        self.stack.push(result);
        Ok(())
    }

    /// Dispatches a method call by symbol: the stack holds
    /// `[receiver, args...]`, `arg_count` excluding the receiver. Tries, in
    /// order, a user-defined method, a native method, and a callable
    /// instance field; otherwise reports an unknown method.
    fn dispatch_invoke(&mut self, method_symbol: u16, arg_count: usize) -> OpResult {
        let receiver_index = self.stack.len() - arg_count - 1;
        let receiver = self.stack[receiver_index].clone();
        let type_name = self.get_type_name(&receiver);

        if let Some(type_name) = &type_name {
            let is_static_call = matches!(receiver, Value::Struct(_));
            match self.dispatch_user_method(
                type_name,
                is_static_call,
                receiver_index,
                arg_count,
                method_symbol,
            ) {
                MethodDispatch::Found(closure, arg_count, exclude_self) => {
                    return self.call_closure_with(arg_count, closure, exclude_self);
                }
                MethodDispatch::Mismatch(e) => return Err(e),
                MethodDispatch::NotFound => {}
            }
        }

        match self.dispatch_native_method_or_field(
            &receiver,
            type_name,
            receiver_index,
            method_symbol,
            arg_count,
        )? {
            Some(callable) => self.dispatch_call_value(callable, arg_count),
            None => Ok(()),
        }
    }

    /// The rest of `dispatch_invoke` once no user method matched: runs a
    /// native method, leaving its result on the stack, or returns a
    /// callable instance field for the caller to call, with the receiver's
    /// slot cleared to become the callee slot.
    fn dispatch_native_method_or_field(
        &mut self,
        receiver: &Value,
        type_name: Option<TypeName>,
        receiver_index: usize,
        method_symbol: u16,
        arg_count: usize,
    ) -> Result<Option<Value>, RuntimeError> {
        if let Some(type_name) = &type_name {
            let native = match type_name {
                TypeName::Builtin(type_symbol) => self
                    .native_methods
                    .get(method_symbol as usize)
                    .and_then(|natives| natives[*type_symbol as usize]),
                TypeName::Struct(_) => None,
            };
            if let Some(native) = native {
                let result = match self.run_native_callable(
                    native,
                    receiver_index,
                    receiver_index + arg_count + 1,
                ) {
                    Ok(value) => value,
                    Err(NativeCallError::Message(error)) => return Err(self.call_error(error)),
                    Err(NativeCallError::Runtime(e)) => return Err(e),
                };
                self.stack.truncate(receiver_index);
                self.push(result);
                return Ok(None);
            }
        }

        if let Value::Instance(inst) = receiver {
            let field_value = inst.borrow().field(method_symbol).cloned();
            if let Some(field_value) = field_value {
                self.stack[receiver_index] = Value::Nil;
                return Ok(Some(field_value));
            }
        }

        Err(self.call_error(match type_name {
            Some(type_name) => format!(
                "Unknown method '{}' for type {}",
                self.symbol_name(method_symbol),
                type_name
            ),
            None => "Cannot determine type of receiver for method call".to_string(),
        }))
    }

    /// Runs a native callable with the stack range `args_start..args_end`
    /// as its arguments.
    fn run_native_callable(
        &mut self,
        native: &'static NativeCallable,
        args_start: usize,
        args_end: usize,
    ) -> std::result::Result<Value, NativeCallError> {
        match *native {
            NativeCallable::InstanceMethodWithVm { function, .. }
            | NativeCallable::ConstructorWithVm { function, .. } => {
                let args: Vec<Value> = self.stack[args_start..args_end].to_vec();
                function(self, &args)
            }
            NativeCallable::StaticMethod { function, .. }
            | NativeCallable::InstanceMethod { function, .. } => {
                function(&self.stack[args_start..args_end]).map_err(NativeCallError::Message)
            }
        }
    }

    /// Lets a native call a Neon value (closure, lambda, or native) with
    /// the given arguments, running the dispatch loop re-entrantly until
    /// that call returns.
    pub(crate) fn call_value(
        &mut self,
        callee: Value,
        args: &[Value],
    ) -> std::result::Result<Value, NativeCallError> {
        self.check_frame_limit().map_err(NativeCallError::Runtime)?;
        if self.native_call_depth >= MAX_NATIVE_CALL_DEPTH {
            return Err(NativeCallError::Runtime(self.call_error("Stack overflow")));
        }

        let frame_depth = self.call_frames.len();
        self.native_call_depth += 1;

        self.push(callee);
        for arg in args {
            self.push(arg.clone());
        }

        let outcome = match self.dispatch_call(args.len()) {
            Err(e) => Err(e),
            Ok(()) if self.call_frames.len() > frame_depth => self.run_until(frame_depth),
            Ok(()) => Ok(()),
        };

        self.native_call_depth -= 1;

        match outcome {
            Ok(()) => Ok(self.pop()),
            Err(e) => Err(NativeCallError::Runtime(e)),
        }
    }

    /// Looks up `method_symbol` among the user methods of `type_name` (the
    /// struct's own methods, or the builtin type's `impl` methods) and
    /// checks the call form (static vs. instance) against the method's
    /// definition. For an instance call, inserts a `Nil` callee slot below
    /// the receiver so the receiver becomes `self`.
    fn dispatch_user_method(
        &mut self,
        type_name: &TypeName,
        is_static_call: bool,
        receiver_index: usize,
        arg_count: usize,
        method_symbol: u16,
    ) -> MethodDispatch {
        let method = match type_name {
            TypeName::Struct(r#struct) => r#struct.find_method(method_symbol),
            TypeName::Builtin(type_symbol) => {
                find_method_entry(&self.builtin_methods[*type_symbol as usize], method_symbol)
            }
        };
        let Some((_, closure, takes_self)) = method else {
            return MethodDispatch::NotFound;
        };

        match (is_static_call, takes_self) {
            (true, true) => MethodDispatch::Mismatch(self.call_error(format!(
                "Method '{}' needs an instance; call it on a {} value",
                self.symbol_name(method_symbol),
                type_name
            ))),
            (false, false) => {
                let method_name = self.symbol_name(method_symbol);
                MethodDispatch::Mismatch(self.call_error(format!(
                    "Method '{}' is static; call it as {}.{}()",
                    method_name, type_name, method_name
                )))
            }
            (true, false) => MethodDispatch::Found(closure, arg_count, false),
            (false, true) => {
                self.stack.insert(receiver_index, Value::Nil);
                MethodDispatch::Found(closure, arg_count + 1, true)
            }
        }
    }

    fn call_native_function(
        &mut self,
        arg_count: usize,
        callable: &Rc<ObjNativeFunction>,
    ) -> std::result::Result<Value, NativeCallError> {
        let native_callable = self
            .lookup_native_method_by_index(callable)
            .map_err(NativeCallError::Message)?;

        let stack_len = self.stack.len();
        let args_start = stack_len - arg_count;
        let args_end = stack_len;

        #[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
        {
            if callable.method_index == PRINT_METHOD_INDEX {
                let args = &self.stack[args_start..args_end];
                if !args.is_empty() {
                    use crate::common::stdlib::system_functions::format_print_args;
                    let line = format_print_args(args);
                    use std::fmt::Write;
                    writeln!(self.string_buffer, "{}", line).ok();
                }
            }
        }
        self.run_native_callable(native_callable, args_start, args_end)
    }

    fn instantiate_struct(&mut self, arg_count: usize, r#struct: Rc<ObjStruct>) -> OpResult {
        if arg_count != r#struct.fields.len() {
            return Err(self.call_error(format!(
                "Expected {} fields but got {}.",
                r#struct.fields.len(),
                arg_count
            )));
        }

        let stack_len = self.stack.len();

        // Unified calling convention: [struct_obj, args...]
        let stack_slice = &self.stack[stack_len - arg_count..stack_len];
        let fields = stack_slice.to_vec();

        let instance = ObjInstance { r#struct, fields };

        // Pop arguments and struct object from stack
        let n = arg_count + 1;
        let start = self.stack.len().saturating_sub(n);
        self.stack.drain(start..);

        // Push the new instance
        self.push(Value::new_instance(instance));

        // IP already incremented by fn_call_unified
        Ok(())
    }

    #[inline(always)]
    fn call_closure(&mut self, arg_count: usize, closure: Rc<ObjClosure>) -> OpResult {
        self.call_closure_with(arg_count, closure, false)
    }

    /// Calls a closure; `exclude_self` leaves the leading `self` argument
    /// out of an arity-mismatch message.
    #[inline(always)]
    fn call_closure_with(
        &mut self,
        arg_count: usize,
        closure: Rc<ObjClosure>,
        exclude_self: bool,
    ) -> OpResult {
        let arity = closure.function.arity;
        if arg_count != arity as usize {
            return Err(self.arity_error(arg_count, arity, exclude_self, &closure.function.name));
        }

        let slot_start = self.stack.len() as isize - arg_count as isize - 1;

        self.push_frame(closure, slot_start);
        Ok(())
    }

    #[cold]
    fn arity_error(
        &self,
        arg_count: usize,
        arity: u8,
        exclude_self: bool,
        function_name: &str,
    ) -> RuntimeError {
        let (expected, got) = if exclude_self {
            (arity - 1, arg_count - 1)
        } else {
            (arity, arg_count)
        };
        self.call_error(format!(
            "Expected {} arguments but got {} for '{}'.",
            expected, got, function_name
        ))
    }

    #[inline(always)]
    pub(in crate::vm) fn op_return(&mut self) {
        let return_value = self.pop();
        let slot_start = self.current_frame().slot_start;
        self.pop_frame();

        // A local captured by a closure that outlives this call must keep
        // its value once this frame's stack slots go away.
        self.close_upvalues_above((slot_start + 1) as usize);

        if self.call_frames.is_empty() {
            self.push(return_value);
            return;
        }

        self.stack.truncate(slot_start as usize);
        self.push(return_value);
    }

    /// Applies `wanted` to an already-computed ordering of `a <=> b`. `None`
    /// (NaN) never matches, per IEEE comparison semantics.
    #[inline(always)]
    fn ordering_matches(ordering: Option<std::cmp::Ordering>, wanted: &Comparison) -> bool {
        use std::cmp::Ordering;
        match ordering {
            None => false,
            Some(Ordering::Greater) => {
                matches!(wanted, Comparison::Greater | Comparison::GreaterEqual)
            }
            Some(Ordering::Less) => matches!(wanted, Comparison::Less | Comparison::LessEqual),
            Some(Ordering::Equal) => {
                matches!(wanted, Comparison::GreaterEqual | Comparison::LessEqual)
            }
        }
    }

    /// Whether `a <wanted> b`, or the runtime error message when the
    /// operands can't be compared.
    #[inline(always)]
    fn compare_values(a: &Value, b: &Value, wanted: &Comparison) -> Result<bool, String> {
        let is_match = match (a, b) {
            (Value::Number(x), Value::Number(y)) => match wanted {
                Comparison::Greater => *x > *y,
                Comparison::GreaterEqual => *x >= *y,
                Comparison::Less => *x < *y,
                Comparison::LessEqual => *x <= *y,
            },
            (Value::Int(x), Value::Int(y)) => Self::ordering_matches(Some(x.cmp(y)), wanted),
            (Value::Int(i), Value::Number(n)) => {
                Self::ordering_matches(compare_int_and_float(*i, *n), wanted)
            }
            (Value::Number(n), Value::Int(i)) => Self::ordering_matches(
                compare_int_and_float(*i, *n).map(std::cmp::Ordering::reverse),
                wanted,
            ),
            (Value::String(sa), Value::String(sb)) => match wanted {
                Comparison::Greater => **sa > **sb,
                Comparison::GreaterEqual => **sa >= **sb,
                Comparison::Less => **sa < **sb,
                Comparison::LessEqual => **sa <= **sb,
            },
            _ => {
                return Err(format!(
                    "Operands of a comparison must be two numbers or two strings, got {} and {}",
                    a.type_name(),
                    b.type_name()
                ));
            }
        };
        Ok(is_match)
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_compare(&mut self, wanted: Comparison) -> OpResult {
        // [.., a, b] -> [.., result]
        let b = self.pop();
        let a = self
            .stack
            .last_mut()
            .expect("binary operand a is on the stack below b");
        let is_match = match Self::compare_values(a, &b, &wanted) {
            Ok(is_match) => is_match,
            Err(message) => return Err(self.runtime_error(message)),
        };
        b.discard();
        std::mem::replace(a, boolean!(is_match)).discard();
        Ok(())
    }

    /// Pops both operands and returns whether `a <wanted> b`, for a
    /// comparison fused into a conditional jump.
    #[inline(always)]
    pub(in crate::vm) fn op_compare_and_pop(
        &mut self,
        wanted: Comparison,
    ) -> Result<bool, RuntimeError> {
        // [.., a, b] -> [..]
        let b = self.pop();
        let a = self.pop();
        let is_match = match Self::compare_values(&a, &b, &wanted) {
            Ok(is_match) => is_match,
            Err(message) => return Err(self.runtime_error(message)),
        };
        b.discard();
        a.discard();
        Ok(is_match)
    }

    /// Like `op_compare_and_pop`, with constant `index` as the right operand.
    #[inline(always)]
    pub(in crate::vm) fn op_compare_constant_and_pop(
        &mut self,
        index: u16,
        wanted: Comparison,
    ) -> Result<bool, RuntimeError> {
        // [.., a] -> [..]
        let a = self.pop();
        let is_match = match Self::compare_values(&a, self.chunk.constant(index as usize), &wanted)
        {
            Ok(is_match) => is_match,
            Err(message) => return Err(self.runtime_error(message)),
        };
        a.discard();
        Ok(is_match)
    }

    #[inline(always)]
    pub(in crate::vm) fn op_equal(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(boolean!(a == b));
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_divide(&mut self) -> OpResult {
        // [.., a, b] -> [.., result]; division is always float, even for
        // two ints.
        let b = self.pop();
        let slot = self
            .stack
            .last_mut()
            .expect("binary operand a is on the stack below b");
        match (&mut *slot, &b) {
            (Value::Number(x), Value::Number(y)) => *x /= *y,
            (Value::Int(x), Value::Number(y)) => *slot = number!(*x as f64 / *y),
            (Value::Number(x), Value::Int(y)) => *x /= *y as f64,
            (Value::Int(x), Value::Int(y)) => *slot = number!(*x as f64 / *y as f64),
            _ => return Err(self.binary_number_op_error("/", &b)),
        }
        std::mem::forget(b);
        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_modulo(&mut self) -> OpResult {
        // [.., a, b] -> [.., result]
        let b = self.pop();
        let slot = self
            .stack
            .last_mut()
            .expect("binary operand a is on the stack below b");
        match (&mut *slot, &b) {
            (Value::Int(x), Value::Int(y)) => {
                if *y == 0 {
                    return Err(self.modulo_by_zero_error());
                }
                // i64::MIN % -1 would overflow via the hardware instruction;
                // the true remainder is always 0 when dividing by -1.
                *x = if *y == -1 { 0 } else { *x % *y };
            }
            (Value::Number(x), Value::Int(y)) => *x %= *y as f64,
            (Value::Int(x), Value::Number(y)) => *slot = number!(*x as f64 % *y),
            (Value::Number(x), Value::Number(y)) => *x %= *y,
            _ => return Err(self.binary_number_op_error("%", &b)),
        }
        std::mem::forget(b);
        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_exponent(&mut self) -> OpResult {
        // [.., base, exponent] -> [.., result]
        let b = self.pop();
        let slot = self
            .stack
            .last_mut()
            .expect("binary operand a is on the stack below b");
        match (&mut *slot, &b) {
            (Value::Int(base), Value::Int(exp)) => {
                if *exp >= 0 {
                    let result = match u32::try_from(*exp) {
                        Ok(e) => base.checked_pow(e),
                        // The exponent itself overflows u32; only bases whose
                        // magnitude never changes under repeated
                        // multiplication can be answered without computing it.
                        Err(_) => match *base {
                            0 => Some(0),
                            1 => Some(1),
                            -1 => Some(if exp % 2 == 0 { 1 } else { -1 }),
                            _ => None,
                        },
                    };
                    match result {
                        Some(r) => *slot = int!(r),
                        None => return Err(self.overflow_error("**")),
                    }
                } else {
                    *slot = number!((*base as f64).powf(*exp as f64));
                }
            }
            (Value::Number(base), Value::Number(exp)) => *base = base.powf(*exp),
            (Value::Int(base), Value::Number(exp)) => {
                *slot = number!((*base as f64).powf(*exp));
            }
            (Value::Number(base), Value::Int(exp)) => *base = base.powf(*exp as f64),
            _ => return Err(self.binary_number_op_error("**", &b)),
        }
        std::mem::forget(b);
        Ok(())
    }

    #[cold]
    #[inline(never)]
    fn binary_number_op_error(&self, op: &str, b: &Value) -> RuntimeError {
        self.runtime_error(format!(
            "Operands of '{}' must be numbers, got {} and {}",
            op,
            self.peek(0).type_name(),
            b.type_name()
        ))
    }

    #[cold]
    #[inline(never)]
    fn overflow_error(&self, op: &str) -> RuntimeError {
        self.runtime_error(format!("integer overflow in {}", op))
    }

    #[cold]
    #[inline(never)]
    fn modulo_by_zero_error(&self) -> RuntimeError {
        self.runtime_error("modulo by zero")
    }

    /// Helper: Convert f64 to i64 for bitwise operations
    #[inline(always)]
    fn to_integer(value: f64) -> i64 {
        if value.is_nan() || value.is_infinite() {
            0
        } else {
            value.trunc() as i64
        }
    }

    /// Helper: the operand of a bitwise op as an `i64` (`Int` directly,
    /// `Number` truncated), or `None` if it's neither.
    #[inline(always)]
    fn as_bitwise_operand(value: &Value) -> Option<i64> {
        match value {
            Value::Int(i) => Some(*i),
            Value::Number(n) => Some(Self::to_integer(*n)),
            _ => None,
        }
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    fn binary_bitwise_op(&mut self, op: &str, f: impl Fn(i64, i64) -> i64) -> OpResult {
        // [.., a, b] -> [.., result]
        let b = self.pop();
        let a = match Self::as_bitwise_operand(
            self.stack
                .last()
                .expect("binary operand a is on the stack below b"),
        ) {
            Some(a) => a,
            None => return Err(self.binary_number_op_error(op, &b)),
        };
        let result = match Self::as_bitwise_operand(&b) {
            Some(bi) => f(a, bi),
            None => return Err(self.binary_number_op_error(op, &b)),
        };
        std::mem::forget(b);
        let slot = self
            .stack
            .last_mut()
            .expect("binary operand a is on the stack below b");
        *slot = int!(result);
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_bitwise_and(&mut self) -> OpResult {
        self.binary_bitwise_op("&", |a, b| a & b)
    }

    #[inline(always)]
    pub(in crate::vm) fn op_bitwise_or(&mut self) -> OpResult {
        self.binary_bitwise_op("|", |a, b| a | b)
    }

    #[inline(always)]
    pub(in crate::vm) fn op_bitwise_xor(&mut self) -> OpResult {
        self.binary_bitwise_op("^", |a, b| a ^ b)
    }

    #[inline(always)]
    pub(in crate::vm) fn op_bitwise_not(&mut self) -> OpResult {
        if let Some(int_val) = Self::as_bitwise_operand(self.peek(0)) {
            self.pop();
            self.push(int!(!int_val));
            return Ok(());
        }
        Err(self.runtime_error("Operand must be a number for bitwise NOT"))
    }

    #[inline(always)]
    pub(in crate::vm) fn op_left_shift(&mut self) -> OpResult {
        self.binary_bitwise_op("<<", |a, b| {
            let shift_amount = (b & 0x3F) as u32; // mask to 0-63
            a << shift_amount
        })
    }

    #[inline(always)]
    pub(in crate::vm) fn op_right_shift(&mut self) -> OpResult {
        self.binary_bitwise_op(">>", |a, b| {
            let shift_amount = (b & 0x3F) as u32; // mask to 0-63
            a >> shift_amount
        })
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_multiply(&mut self) -> OpResult {
        // [.., a, b] -> [.., result]
        let b = self.pop();
        let slot = self
            .stack
            .last_mut()
            .expect("binary operand a is on the stack below b");
        match (&mut *slot, &b) {
            (Value::Number(x), Value::Number(y)) => *x *= *y,
            (Value::Int(x), Value::Int(y)) => match x.checked_mul(*y) {
                Some(r) => *x = r,
                None => return Err(self.overflow_error("*")),
            },
            (Value::Int(x), Value::Number(y)) => *slot = number!(*x as f64 * *y),
            (Value::Number(x), Value::Int(y)) => *x *= *y as f64,
            _ => return Err(self.binary_number_op_error("*", &b)),
        }
        std::mem::forget(b);
        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_subtract(&mut self) -> OpResult {
        // [.., a, b] -> [.., result]
        let b = self.pop();
        let slot = self
            .stack
            .last_mut()
            .expect("binary operand a is on the stack below b");
        match (&mut *slot, &b) {
            (Value::Number(x), Value::Number(y)) => *x -= *y,
            (Value::Int(x), Value::Int(y)) => match x.checked_sub(*y) {
                Some(r) => *x = r,
                None => return Err(self.overflow_error("-")),
            },
            (Value::Int(x), Value::Number(y)) => *slot = number!(*x as f64 - *y),
            (Value::Number(x), Value::Int(y)) => *x -= *y as f64,
            _ => return Err(self.binary_number_op_error("-", &b)),
        }
        std::mem::forget(b);
        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_add(&mut self) -> OpResult {
        // [.., a, b] -> [.., result]
        let b = self.pop();
        let slot = self
            .stack
            .last_mut()
            .expect("binary operand a is on the stack below b");
        match (&mut *slot, &b) {
            (Value::Number(x), Value::Number(y)) => {
                *x += *y;
                std::mem::forget(b);
            }
            (Value::Int(x), Value::Int(y)) => match x.checked_add(*y) {
                Some(r) => {
                    *x = r;
                    std::mem::forget(b);
                }
                None => return Err(self.overflow_error("+")),
            },
            (Value::Int(x), Value::Number(y)) => {
                *slot = number!(*x as f64 + *y);
                std::mem::forget(b);
            }
            (Value::Number(x), Value::Int(y)) => {
                *x += *y as f64;
                std::mem::forget(b);
            }
            (Value::String(x), Value::String(y)) => {
                let mut combined = String::with_capacity(x.len() + y.len());
                combined.push_str(x);
                combined.push_str(y);
                *slot = string!(combined);
            }
            _ => {
                return Err(self.runtime_error("Operands must be two numbers or two strings"));
            }
        }
        Ok(())
    }

    /// `Add` with a number constant as the right operand. A non-number left
    /// operand, or a mix the fast path doesn't cover, goes through `op_add`
    /// so its result and error stay identical.
    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_add_constant(&mut self, index: u16) -> OpResult {
        let index = index as usize;
        let slot = self.stack.last_mut().expect("operand is on the stack");
        match (&mut *slot, self.chunk.constant(index)) {
            (Value::Number(a), &Value::Number(c)) => {
                *a += c;
                return Ok(());
            }
            (Value::Int(a), &Value::Int(c)) => {
                return match a.checked_add(c) {
                    Some(r) => {
                        *a = r;
                        Ok(())
                    }
                    None => Err(self.overflow_error("+")),
                };
            }
            (Value::Number(a), &Value::Int(c)) => {
                *a += c as f64;
                return Ok(());
            }
            _ => {}
        }
        let constant = self.chunk.read_constant(index);
        self.push(constant);
        self.op_add()?;
        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_modulo_constant(&mut self, index: u16) -> OpResult {
        let index = index as usize;
        let slot = self.stack.last_mut().expect("operand is on the stack");
        match (&mut *slot, self.chunk.constant(index)) {
            (Value::Int(a), &Value::Int(c)) => {
                if let Some(r) = a.checked_rem(c) {
                    *a = r;
                    return Ok(());
                }
            }
            (Value::Number(a), &Value::Number(c)) => {
                *a %= c;
                return Ok(());
            }
            _ => {}
        }
        let constant = self.chunk.read_constant(index);
        self.push(constant);
        self.op_modulo()?;
        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_multiply_constant(&mut self, index: u16) -> OpResult {
        let index = index as usize;
        let slot = self.stack.last_mut().expect("operand is on the stack");
        match (&mut *slot, self.chunk.constant(index)) {
            (Value::Number(a), &Value::Number(c)) => {
                *a *= c;
                return Ok(());
            }
            (Value::Int(a), &Value::Int(c)) => {
                return match a.checked_mul(c) {
                    Some(r) => {
                        *a = r;
                        Ok(())
                    }
                    None => Err(self.overflow_error("*")),
                };
            }
            _ => {}
        }
        let constant = self.chunk.read_constant(index);
        self.push(constant);
        self.op_multiply()?;
        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_subtract_constant(&mut self, index: u16) -> OpResult {
        let index = index as usize;
        let slot = self.stack.last_mut().expect("operand is on the stack");
        match (&mut *slot, self.chunk.constant(index)) {
            (Value::Number(a), &Value::Number(c)) => {
                *a -= c;
                return Ok(());
            }
            (Value::Int(a), &Value::Int(c)) => {
                return match a.checked_sub(c) {
                    Some(r) => {
                        *a = r;
                        Ok(())
                    }
                    None => Err(self.overflow_error("-")),
                };
            }
            (Value::Number(a), &Value::Int(c)) => {
                *a -= c as f64;
                return Ok(());
            }
            _ => {}
        }
        let constant = self.chunk.read_constant(index);
        self.push(constant);
        self.op_subtract()?;
        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_compare_constant(
        &mut self,
        index: u16,
        wanted: Comparison,
    ) -> OpResult {
        let index = index as usize;
        let slot = self.stack.last_mut().expect("operand is on the stack");
        match (&*slot, self.chunk.constant(index)) {
            (Value::Number(a), &Value::Number(c)) => {
                let is_match = match wanted {
                    Comparison::Greater => *a > c,
                    Comparison::GreaterEqual => *a >= c,
                    Comparison::Less => *a < c,
                    Comparison::LessEqual => *a <= c,
                };
                std::mem::replace(slot, boolean!(is_match)).discard();
                return Ok(());
            }
            (Value::Int(a), &Value::Int(c)) => {
                let is_match = Self::ordering_matches(Some(a.cmp(&c)), &wanted);
                std::mem::replace(slot, boolean!(is_match)).discard();
                return Ok(());
            }
            (Value::Number(a), &Value::Int(c)) => {
                let is_match = Self::ordering_matches(
                    compare_int_and_float(c, *a).map(std::cmp::Ordering::reverse),
                    &wanted,
                );
                std::mem::replace(slot, boolean!(is_match)).discard();
                return Ok(());
            }
            _ => {}
        }
        let constant = self.chunk.read_constant(index);
        self.push(constant);
        self.op_compare(wanted)?;
        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_negate(&mut self) -> OpResult {
        // [.., operand] -> [.., result]
        let slot = self.stack.last_mut().expect("operand is on the stack");
        if let Value::Number(n) = *slot {
            *slot = number!(-n);
            return Ok(());
        }
        if let Value::Int(i) = *slot {
            return match i.checked_neg() {
                Some(r) => {
                    *slot = int!(r);
                    Ok(())
                }
                None => Err(self.overflow_error("-")),
            };
        }
        Err(self.runtime_error("Operand must be a number"))
    }

    #[inline(always)]
    pub(in crate::vm) fn op_constant(&mut self, index: u16) {
        let constant = self.chunk.read_constant(index as usize);
        self.push(constant);
    }

    #[inline(always)]
    pub(in crate::vm) fn op_set_local(&mut self, index: u16) -> OpResult {
        let (index, absolute_index) = self.read_local_slot(index);
        let value = self.peek(0).copy_or_clone();
        let Some(slot) = self.stack.get_mut(absolute_index) else {
            return Err(self.runtime_error(format!("Invalid local slot {}", index)));
        };
        std::mem::replace(slot, value).discard();
        Ok(())
    }

    /// Statement-position `SetLocal`: moves the top of stack into the slot
    /// instead of copying it there and leaving it pushed.
    #[inline(always)]
    pub(in crate::vm) fn op_store_local(&mut self, index: u16) -> OpResult {
        let (index, absolute_index) = self.read_local_slot(index);
        let value = self.pop();
        let Some(slot) = self.stack.get_mut(absolute_index) else {
            return Err(self.runtime_error(format!("Invalid local slot {}", index)));
        };
        std::mem::replace(slot, value).discard();
        Ok(())
    }

    /// Wraps a function constant in a closure, capturing the upvalues its
    /// run in `Chunk::closure_upvalues` describes.
    #[inline(always)]
    pub(in crate::vm) fn op_closure(
        &mut self,
        const_index: u16,
        upvalue_count: u8,
        upvalues_start: u32,
    ) -> OpResult {
        let function = match self.chunk.read_constant(const_index as usize) {
            Value::Function(function) => function,
            _ => unreachable!("Closure operand must reference a function constant"),
        };

        let upvalues_start = upvalues_start as usize;
        let mut upvalues = Vec::with_capacity(upvalue_count as usize);
        for i in 0..upvalue_count as usize {
            let (is_local, index) = self.chunk.closure_upvalues[upvalues_start + i];
            let index = index as usize;

            let upvalue = if is_local {
                self.capture_upvalue(self.frame_base + index)
            } else {
                if index >= self.current_frame().closure.upvalues.len() {
                    return Err(self.runtime_error(format!("Invalid upvalue index {}", index)));
                }
                Rc::clone(&self.current_frame().closure.upvalues[index])
            };
            upvalues.push(upvalue);
        }

        self.push(Value::new_closure(function, upvalues));
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_get_upvalue(&mut self, index: u16) -> OpResult {
        let index = index as usize;
        if index >= self.current_frame().closure.upvalues.len() {
            return Err(self.runtime_error(format!("Invalid upvalue index {}", index)));
        }
        let upvalue = Rc::clone(&self.current_frame().closure.upvalues[index]);
        let value = match &*upvalue.borrow() {
            Upvalue::Open(stack_index) => self.stack[*stack_index].clone(),
            Upvalue::Closed(value) => value.clone(),
        };
        self.push(value);
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_set_upvalue(&mut self, index: u16) -> OpResult {
        let index = index as usize;
        if index >= self.current_frame().closure.upvalues.len() {
            return Err(self.runtime_error(format!("Invalid upvalue index {}", index)));
        }
        let value = self.peek(0).clone();
        let upvalue = Rc::clone(&self.current_frame().closure.upvalues[index]);
        let stack_index = match &*upvalue.borrow() {
            Upvalue::Open(stack_index) => Some(*stack_index),
            Upvalue::Closed(_) => None,
        };
        match stack_index {
            Some(stack_index) => self.stack[stack_index] = value,
            None => *upvalue.borrow_mut() = Upvalue::Closed(value),
        }
        Ok(())
    }

    /// Closes the upvalue (if any) pointing at the current top-of-stack
    /// slot, then pops it. Used at block exit for a captured local, in
    /// place of a plain Pop.
    #[inline(always)]
    pub(in crate::vm) fn op_close_upvalue(&mut self) {
        let top_index = self.stack.len() - 1;
        self.close_upvalues_above(top_index);
        self.pop();
    }

    /// Returns the open upvalue for `stack_index`, reusing one already
    /// open for that exact slot so two closures capturing the same local
    /// share writes to it.
    fn capture_upvalue(&mut self, stack_index: usize) -> Rc<RefCell<Upvalue>> {
        for existing in &self.open_upvalues {
            if let Upvalue::Open(index) = *existing.borrow() {
                if index == stack_index {
                    return Rc::clone(existing);
                }
            }
        }
        let upvalue = Rc::new(RefCell::new(Upvalue::Open(stack_index)));
        self.open_upvalues.push(Rc::clone(&upvalue));
        upvalue
    }

    /// Closes every open upvalue pointing at `stack_index` or higher,
    /// copying its live stack value out so it survives that slot being
    /// reused or the stack being truncated.
    pub(in crate::vm) fn close_upvalues_above(&mut self, stack_index: usize) {
        if self.open_upvalues.is_empty() {
            return;
        }
        let stack = &self.stack;
        self.open_upvalues.retain(|upvalue| {
            let index = match *upvalue.borrow() {
                Upvalue::Open(index) => index,
                Upvalue::Closed(_) => return false,
            };
            if index < stack_index {
                return true;
            }
            *upvalue.borrow_mut() = Upvalue::Closed(stack[index].clone());
            false
        });
    }

    /// Returns a local-slot operand with its absolute stack index.
    #[inline(always)]
    fn read_local_slot(&self, index: u16) -> (usize, usize) {
        let index = index as usize;
        let absolute_index = self.frame_base + index;
        (index, absolute_index)
    }

    #[inline(always)]
    pub(in crate::vm) fn op_get_local(&mut self, index: u16) -> OpResult {
        let (index, absolute_index) = self.read_local_slot(index);
        let Some(value) = self.stack.get(absolute_index) else {
            return Err(self.runtime_error(format!("Invalid local slot {}", index)));
        };
        self.push(value.copy_or_clone());
        Ok(())
    }

    pub(in crate::vm) fn op_get_builtin(&mut self, index: u16) -> OpResult {
        let index = index as usize;
        if let Some(value) = self.builtin.get(index) {
            self.push(value.clone());
        } else {
            return Err(self.runtime_error(format!("Built-in global at index {} not found", index)));
        }
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_get_global(&mut self, index: u16) -> OpResult {
        let index = index as usize;

        // Regular global variables are in the script frame
        // Script frame has slot_start = -1, so globals start at index 0
        let script_frame = &self.call_frames[0];
        let absolute_index = (script_frame.slot_start + 1 + index as isize) as usize;

        // Make sure we don't go out of bounds
        if absolute_index >= self.stack.len() {
            return Err(self.runtime_error(format!(
                "Global variable index {} out of bounds (stack size: {})",
                absolute_index,
                self.stack.len()
            )));
        }

        let value = &self.stack[absolute_index];
        if let Some(message) = Self::uninitialized_error(value) {
            return Err(self.runtime_error(message));
        }

        self.push(value.clone());
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_set_global(&mut self, index: u16) -> OpResult {
        let index = index as usize;
        // Global variables are always in the script frame (first frame)
        // Script frame has slot_start = -1, so globals start at index 0
        let script_frame = &self.call_frames[0];
        let absolute_index = (script_frame.slot_start + 1 + index as isize) as usize;

        if absolute_index >= self.stack.len() {
            return Err(self.runtime_error(format!(
                "Global variable index {} out of bounds (stack size: {})",
                absolute_index,
                self.stack.len()
            )));
        }

        if let Some(message) = Self::uninitialized_error(&self.stack[absolute_index]) {
            return Err(self.runtime_error(message));
        }

        self.stack[absolute_index] = self.peek(0).clone();
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_check_initialized(&mut self) -> OpResult {
        if let Some(message) = self.stack.last().and_then(Self::uninitialized_error) {
            return Err(self.runtime_error(message));
        }
        Ok(())
    }

    /// Pops the scrutinee value and raises "No match arm for <value>",
    /// formatted like `print`. Emitted after every `match` arm has been
    /// tested and none matched.
    pub(in crate::vm) fn op_no_match_arm(&mut self) -> OpResult {
        let value = self.pop();
        Err(self.runtime_error(format!("No match arm for {value}")))
    }

    /// Peeks the top of the stack and errors unless it holds an Array of
    /// exactly `n` elements; otherwise a no-op.
    #[inline(always)]
    pub(in crate::vm) fn op_check_tuple(&mut self, n: u16) -> OpResult {
        let n = n as usize;

        match self.peek(0) {
            Value::Array(array_ref) => {
                let len = array_ref.borrow().len();
                if len != n {
                    return Err(self.runtime_error(format!(
                        "Cannot destructure array of size {} into {} names",
                        len, n
                    )));
                }
                Ok(())
            }
            other => Err(self.runtime_error(format!(
                "Cannot destructure {} into {} names",
                other.type_name(),
                n
            ))),
        }
    }

    /// The "used before initialization" message for `value`, if it's the
    /// uninitialized sentinel. A free function (no `self`) so callers can
    /// build it while still holding an immutable borrow of the stack.
    fn uninitialized_error(value: &Value) -> Option<String> {
        match value {
            Value::Uninitialized(name) => {
                Some(format!("variable '{}' used before initialization", name))
            }
            _ => None,
        }
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_get_field(&mut self, symbol: u16) -> OpResult {
        let value = match self.peek(0) {
            Value::Instance(instance_ref) => match instance_ref.borrow().field(symbol) {
                Some(value) => value.copy_or_clone(),
                None => {
                    let name = self.symbol_name(symbol);
                    return Err(self.runtime_error(format!("Undefined field '{}'.", name)));
                }
            },
            Value::EnumVariant(variant) => match variant.field(symbol) {
                Some(value) => value.copy_or_clone(),
                None => {
                    let name = self.symbol_name(symbol);
                    return Err(self.runtime_error(format!("Undefined field '{}'.", name)));
                }
            },
            _ => return Err(self.runtime_error("Only instances have fields.")),
        };

        // Plain assignment drops the old top while `value` is still live,
        // which makes the compiler spill a slow unwind copy of it.
        drop(std::mem::replace(
            self.stack.last_mut().expect("operand is on the stack"),
            value,
        ));

        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_get_local_field(&mut self, index: u16, symbol: u16) -> OpResult {
        let index = index as usize;
        let absolute_index = self.frame_base + index;
        if absolute_index >= self.stack.len() {
            return Err(self.runtime_error(format!("Invalid local slot {}", index)));
        }

        let value = match &self.stack[absolute_index] {
            Value::Instance(instance_ref) => match instance_ref.borrow().field(symbol) {
                Some(value) => value.copy_or_clone(),
                None => {
                    let name = self.symbol_name(symbol);
                    return Err(self.runtime_error(format!("Undefined field '{}'.", name)));
                }
            },
            Value::EnumVariant(variant) => match variant.field(symbol) {
                Some(value) => value.copy_or_clone(),
                None => {
                    let name = self.symbol_name(symbol);
                    return Err(self.runtime_error(format!("Undefined field '{}'.", name)));
                }
            },
            _ => return Err(self.runtime_error("Only instances have fields.")),
        };

        self.push(value);
        Ok(())
    }

    /// Stack: `[.., value]` -> `[..]`.
    #[inline(always)]
    pub(in crate::vm) fn op_store_local_field(&mut self, index: u16, symbol: u16) -> OpResult {
        let index = index as usize;
        let absolute_index = self.frame_base + index;
        if absolute_index >= self.stack.len() {
            return Err(self.runtime_error(format!("Invalid local slot {}", index)));
        }

        let value = self.pop();
        match &self.stack[absolute_index] {
            Value::Instance(instance_ref) => {
                let mut instance = instance_ref.borrow_mut();
                let Some(field_index) = instance.r#struct.field_index(symbol) else {
                    let name = self.symbol_name(symbol);
                    return Err(self.runtime_error(format!("Undefined field '{}'.", name)));
                };
                drop(std::mem::replace(&mut instance.fields[field_index], value));
            }
            Value::EnumVariant(_) => return Err(self.enum_field_assign_error(symbol)),
            _ => return Err(self.runtime_error("Only instances have fields.")),
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn op_set_field(&mut self, symbol: u16) -> OpResult {
        // [.., instance, value] -> [.., value]
        let value = self.pop();
        match self.peek(0) {
            Value::Instance(instance_ref) => {
                let mut instance = instance_ref.borrow_mut();
                let Some(index) = instance.r#struct.field_index(symbol) else {
                    let name = self.symbol_name(symbol);
                    return Err(self.runtime_error(format!("Undefined field '{}'.", name)));
                };
                instance.fields[index] = value.copy_or_clone();
            }
            Value::EnumVariant(_) => return Err(self.enum_field_assign_error(symbol)),
            _ => return Err(self.runtime_error("Only instances have fields.")),
        }

        // Plain assignment drops the old top while `value` is still live,
        // which makes the compiler spill a slow unwind copy of it.
        drop(std::mem::replace(
            self.stack.last_mut().expect("operand is on the stack"),
            value,
        ));

        Ok(())
    }

    /// Statement-position `SetField`: unlike `op_set_field`, fully
    /// consumes the instance and value instead of leaving the value
    /// pushed. Stack: `[.., instance, value]` -> `[..]`.
    #[inline(always)]
    pub(in crate::vm) fn op_store_field(&mut self, symbol: u16) -> OpResult {
        let value = self.pop();
        let instance = self.pop();
        match instance {
            Value::Instance(instance_ref) => {
                let mut instance = instance_ref.borrow_mut();
                let Some(index) = instance.r#struct.field_index(symbol) else {
                    let name = self.symbol_name(symbol);
                    return Err(self.runtime_error(format!("Undefined field '{}'.", name)));
                };
                drop(std::mem::replace(&mut instance.fields[index], value));
            }
            Value::EnumVariant(_) => return Err(self.enum_field_assign_error(symbol)),
            _ => return Err(self.runtime_error("Only instances have fields.")),
        }

        Ok(())
    }

    fn enum_field_assign_error(&self, symbol: u16) -> RuntimeError {
        let name = self.symbol_name(symbol);
        self.runtime_error(format!(
            "Cannot assign to field '{}' of an enum variant.",
            name
        ))
    }

    /// Looks up an interned name by symbol id, for use on an error path only.
    fn symbol_name(&self, symbol: u16) -> Rc<str> {
        self.chunk.symbols[symbol as usize].clone()
    }

    #[inline(always)]
    pub(in crate::vm) fn op_create_map(&mut self, count: u16) -> OpResult {
        let count = count as usize;

        let stack_len = self.stack.len();
        let pairs_start = stack_len - (count * 2);

        let mut map = IndexMap::with_capacity(count);
        for i in 0..count {
            let key_value = &self.stack[pairs_start + 2 * i];
            let value = &self.stack[pairs_start + 2 * i + 1];

            let key =
                MapKey::from_value(key_value, "map key").map_err(|m| self.runtime_error(m))?;

            map.insert(key, value.clone());
        }

        self.stack.drain(pairs_start..);

        self.push(Value::new_map(map));

        Ok(())
    }

    pub(in crate::vm) fn op_create_array(&mut self, count: u16) {
        let count = count as usize;

        let stack_len = self.stack.len();
        let elements_start = stack_len - count;

        let elements: Vec<Value> = self.stack[elements_start..stack_len].to_vec();

        self.stack.drain(elements_start..);

        self.push(Value::new_array(elements));
    }

    #[inline(always)]
    pub(in crate::vm) fn op_create_set(&mut self, count: u16) -> OpResult {
        let count = count as usize;

        let stack_len = self.stack.len();
        let elements_start = stack_len - count;

        let mut set = std::collections::BTreeSet::new();
        for i in 0..count {
            let element_value = &self.stack[elements_start + i];

            let key = MapKey::from_value(element_value, "set element")
                .map_err(|m| self.runtime_error(m))?;

            set.insert(key);
        }

        self.stack.drain(elements_start..);

        self.push(Value::new_set(set));

        Ok(())
    }

    pub(in crate::vm) fn op_create_range(&mut self, inclusive: bool) -> OpResult {
        let end_value = self.pop();
        let start_value = self.pop();

        let start = self.range_bound(start_value, "start")?;
        let end = self.range_bound(end_value, "end")?;

        let range = Value::new_range(start, end, inclusive);
        if let Value::Range(r) = &range {
            if r.len() > Self::MAX_RANGE_BOUND {
                return Err(self.runtime_error(format!(
                    "Range must have at most 2^53 elements, got {}",
                    range
                )));
            }
        }

        self.push(range);

        Ok(())
    }

    const MAX_RANGE_BOUND: i64 = 1 << 53;

    fn range_bound(&self, value: Value, label: &str) -> std::result::Result<i64, RuntimeError> {
        match value {
            Value::Int(i) => Ok(i),
            Value::Number(n) => {
                if n.fract() != 0.0 {
                    return Err(self
                        .runtime_error(format!("Range {} must be an integer, got {}", label, n)));
                }
                if !f64_fits_i64(n) {
                    return Err(self.runtime_error(format!(
                        "Range {} must fit in a 64-bit integer, got {}",
                        label, n
                    )));
                }
                Ok(n as i64)
            }
            _ => {
                Err(self.runtime_error(format!("Range {} must be a number, got {}", label, value)))
            }
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn op_get_index(&mut self) -> OpResult {
        let index_value = self.pop();
        let collection_value = self.pop();

        match &collection_value {
            Value::Map(map_ref) => {
                let key = MapKey::from_value(&index_value, "map key")
                    .map_err(|m| self.runtime_error(m))?;

                let map = map_ref.borrow();
                let result = map.get(&key).cloned().unwrap_or(Value::Nil);
                self.push(result);
                Ok(())
            }
            Value::Array(array_ref) => {
                let array = array_ref.borrow();
                let len = array.len() as i64;
                let actual_index = normalize_index(index_value, len, "Array")
                    .map_err(|m| self.runtime_error(m))?;

                let result = array[actual_index].clone();
                self.push(result);
                Ok(())
            }
            Value::Range(range) => {
                let len = range.len();
                let actual_index = normalize_index(index_value, len, "Range")
                    .map_err(|m| self.runtime_error(m))?;

                self.push(Value::Int(range.get(actual_index as i64)));
                Ok(())
            }
            Value::String(s) => {
                let len = s.chars().count() as i64;
                let actual_index = normalize_index(index_value, len, "String")
                    .map_err(|m| self.runtime_error(m))?;

                let ch = s.chars().nth(actual_index).ok_or_else(|| {
                    self.runtime_error(format!(
                        "String index out of bounds: index {} (normalized: {}) on string of length {}.",
                        actual_index, actual_index, len
                    ))
                })?;
                self.push(string!(ch.to_string()));
                Ok(())
            }
            _ => Err(self.runtime_error(format!(
                "Only arrays, maps, ranges, and strings support index access, got {}.",
                collection_value
            ))),
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn op_set_index(&mut self) -> OpResult {
        let value = self.pop();
        let index_value = self.pop();
        let collection_value = self.pop();

        match &collection_value {
            Value::Map(map_ref) => {
                let key = MapKey::from_value(&index_value, "map key")
                    .map_err(|m| self.runtime_error(m))?;

                let mut map = map_ref.borrow_mut();
                map.insert(key, value.clone());

                self.push(value);
                Ok(())
            }
            Value::Array(array_ref) => {
                let index = match index_value {
                    Value::Number(n) => n as i64,
                    Value::Int(i) => i,
                    _ => {
                        return Err(self.runtime_error(format!(
                            "Array index must be a number, got {}.",
                            index_value
                        )));
                    }
                };

                let mut array = array_ref.borrow_mut();
                let len = array.len() as i64;

                let actual_index = if index < 0 { len + index } else { index };

                if actual_index < 0 || actual_index >= len {
                    return Err(self.runtime_error(format!(
                        "Array index out of bounds: index {} (normalized: {}) on array of length {}.",
                        index, actual_index, len
                    )));
                }

                array[actual_index as usize] = value.clone();

                self.push(value);
                Ok(())
            }
            Value::Range(_) => Err(self.runtime_error(
                "Cannot assign to an index of a range: ranges are immutable.".to_string(),
            )),
            Value::String(_) => Err(self.runtime_error(
                "Cannot assign to an index of a string: strings are immutable.".to_string(),
            )),
            _ => Err(self.runtime_error(format!(
                "Only arrays and maps support index assignment, got {}.",
                collection_value
            ))),
        }
    }

    /// GetIterator: Convert a collection to an iterator
    /// Pops the collection and pushes two hidden locals: the iterable
    /// collection (arrays and ranges as-is, map keys or set elements
    /// collected into a new array) followed by the starting index, 0.
    #[inline(always)]
    pub(in crate::vm) fn op_get_iterator(&mut self, pairs: bool) -> OpResult {
        let collection = self.pop();

        let iterator_value = match &collection {
            Value::Array(_) => collection,
            Value::Range(_) => collection,
            Value::Map(map_ref) => {
                let map = map_ref.borrow();
                if pairs {
                    let entries: Vec<Value> = map
                        .iter()
                        .map(|(key, value)| Value::new_array(vec![key.to_value(), value.clone()]))
                        .collect();

                    Value::new_array(entries)
                } else {
                    let keys: Vec<Value> = map.keys().map(MapKey::to_value).collect();

                    Value::new_array(keys)
                }
            }
            Value::Set(set_ref) => {
                let set = set_ref.borrow();
                let elements: Vec<Value> = set.iter().map(MapKey::to_value).collect();

                Value::new_array(elements)
            }
            Value::String(s) => crate::common::stdlib::string_functions::string_chars_array(s),
            _ => {
                return Err(self.runtime_error(format!(
                    "Cannot iterate over type: {}. Only arrays, maps, sets, ranges, and strings are iterable.",
                    collection
                )));
            }
        };

        self.push(iterator_value);
        self.push(int!(0));
        Ok(())
    }

    /// Resolves the u16 slot operand for an iterator opcode to the absolute
    /// index of its first hidden slot (collection), checking that both it and
    /// the index slot right after it are in range.
    #[inline(always)]
    fn read_iterator_slot(&self, index: u16) -> std::result::Result<usize, RuntimeError> {
        let (index, slot) = self.read_local_slot(index);
        if slot + 1 >= self.stack.len() {
            return Err(self.runtime_error(format!("Invalid local slot {}", index)));
        }
        Ok(slot)
    }

    /// IteratorDone: Check if iteration is complete for the hidden iterator
    /// slots starting at the given local slot (collection, then index).
    /// Pushes false if done (no more elements), true if not done (more elements remain)
    /// This inverted logic allows PopJumpIfFalse to exit the loop when done
    #[inline(always)]
    pub(in crate::vm) fn op_iterator_done(&mut self, slot: u16) -> OpResult {
        let slot = self.read_iterator_slot(slot)?;
        let index = match &self.stack[slot + 1] {
            Value::Int(i) => *i,
            other => {
                return Err(self.runtime_error(format!(
                    "Invalid iterator index, got {}.",
                    other.type_name()
                )));
            }
        };
        let has_more = match &self.stack[slot] {
            Value::Array(array_ref) => index < array_ref.borrow().len() as i64,
            Value::Range(range) => index < range.len(),
            other => {
                return Err(self.runtime_error(format!(
                    "Invalid iterator collection, got {}.",
                    other.type_name()
                )));
            }
        };

        self.push(boolean!(has_more));
        Ok(())
    }

    /// IteratorNext: Get the next element from the iterator held in the
    /// hidden slots starting at the given local slot (collection, then index).
    /// Pushes the next value onto the stack and advances the index slot.
    #[inline(always)]
    pub(in crate::vm) fn op_iterator_next(&mut self, slot: u16) -> OpResult {
        let slot = self.read_iterator_slot(slot)?;
        let index = match &self.stack[slot + 1] {
            Value::Int(i) => *i,
            other => {
                return Err(self.runtime_error(format!(
                    "Invalid iterator index, got {}.",
                    other.type_name()
                )));
            }
        };
        let value = match &self.stack[slot] {
            Value::Array(array_ref) => {
                let array = array_ref.borrow();
                if index >= array.len() as i64 {
                    return Err(self.runtime_error("Iterator exhausted"));
                }
                array[index as usize].clone()
            }
            Value::Range(range) => {
                if index >= range.len() {
                    return Err(self.runtime_error("Iterator exhausted"));
                }
                Value::Int(range.get(index))
            }
            other => {
                return Err(self.runtime_error(format!(
                    "Invalid iterator collection, got {}.",
                    other.type_name()
                )));
            }
        };

        self.stack[slot + 1] = int!(index + 1);
        self.push(value);
        Ok(())
    }

    /// Helper: Extract type name from a value for method dispatch
    fn get_type_name(&self, value: &Value) -> Option<TypeName> {
        match value {
            Value::Array(_) => Some(TypeName::Builtin(ARRAY_SYMBOL)),
            Value::String(_) => Some(TypeName::Builtin(STRING_SYMBOL)),
            Value::Map(_) => Some(TypeName::Builtin(MAP_SYMBOL)),
            Value::Set(_) => Some(TypeName::Builtin(SET_SYMBOL)),
            Value::File(_) => Some(TypeName::Builtin(FILE_SYMBOL)),
            Value::Range(_) => Some(TypeName::Builtin(RANGE_SYMBOL)),
            Value::PriorityQueue(_) => Some(TypeName::Builtin(PRIORITY_QUEUE_SYMBOL)),
            Value::Instance(inst) => Some(TypeName::Struct(Rc::clone(&inst.borrow().r#struct))),
            // The struct value itself (e.g. `Point` in `Point.origin()`)
            // dispatches static methods under the struct's own name.
            Value::Struct(r#struct) => Some(TypeName::Struct(Rc::clone(r#struct))),
            Value::Number(_) => Some(TypeName::Builtin(NUMBER_SYMBOL)),
            Value::Int(_) => Some(TypeName::Builtin(NUMBER_SYMBOL)),
            Value::Boolean(_) => Some(TypeName::Builtin(BOOLEAN_SYMBOL)),
            _ => None,
        }
    }

    /// Helper: Look up native method by index
    fn lookup_native_method_by_index(
        &mut self,
        callable: &Rc<ObjNativeFunction>,
    ) -> std::result::Result<&'static NativeCallable, String> {
        match crate::common::method_registry::get_native_method_by_index(
            callable.method_index as usize,
        ) {
            None => Err(format!(
                "Unknown method index '{}' for native method call",
                callable.method_index
            )),
            Some(callable) => Ok(callable),
        }
    }

    /// DefineMethod: pops the closure left on top of the stack by a
    /// preceding Closure op and the struct value below it, and appends the
    /// closure to the struct's methods along with whether it takes `self`.
    #[inline(always)]
    pub(in crate::vm) fn op_define_method(&mut self, method_symbol: u16, takes_self: bool) {
        let closure_value = self.pop();
        let Value::Closure(closure) = closure_value else {
            unreachable!("DefineMethod expects a closure on top of the stack")
        };
        let Value::Struct(r#struct) = self.pop() else {
            unreachable!("DefineMethod expects a struct below the closure")
        };
        r#struct
            .methods
            .borrow_mut()
            .push((method_symbol, closure, takes_self));
        if let Some(journal) = &mut self.method_journal {
            journal.push(r#struct);
        }
    }

    /// DefineBuiltinMethod: pops the closure left on top of the stack by a
    /// preceding Closure op and registers it under the builtin type symbol,
    /// along with whether the method takes `self`.
    #[inline(always)]
    pub(in crate::vm) fn op_define_builtin_method(
        &mut self,
        type_symbol: u16,
        method_symbol: u16,
        takes_self: bool,
    ) {
        let Value::Closure(closure) = self.pop() else {
            unreachable!("DefineBuiltinMethod expects a closure on top of the stack")
        };
        self.builtin_methods[type_symbol as usize].push((method_symbol, closure, takes_self));
    }
}

impl NativeContext for VirtualMachine {
    fn call_value(&mut self, callee: Value, args: &[Value]) -> Result<Value, NativeCallError> {
        VirtualMachine::call_value(self, callee, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::method_registry::BUILTIN_TYPE_NAMES;

    #[test]
    fn builtin_symbol_consts_match_builtin_type_names() {
        assert_eq!(BUILTIN_TYPE_NAMES[ARRAY_SYMBOL as usize], "Array");
        assert_eq!(BUILTIN_TYPE_NAMES[STRING_SYMBOL as usize], "String");
        assert_eq!(BUILTIN_TYPE_NAMES[MAP_SYMBOL as usize], "Map");
        assert_eq!(BUILTIN_TYPE_NAMES[SET_SYMBOL as usize], "Set");
        assert_eq!(BUILTIN_TYPE_NAMES[NUMBER_SYMBOL as usize], "Number");
        assert_eq!(BUILTIN_TYPE_NAMES[BOOLEAN_SYMBOL as usize], "Boolean");
        assert_eq!(BUILTIN_TYPE_NAMES[FILE_SYMBOL as usize], "File");
        assert_eq!(BUILTIN_TYPE_NAMES[RANGE_SYMBOL as usize], "Range");
        assert_eq!(
            BUILTIN_TYPE_NAMES[PRIORITY_QUEUE_SYMBOL as usize],
            "PriorityQueue"
        );
    }
}
