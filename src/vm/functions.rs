use crate::common::constants::{MAX_FRAMES, MAX_NATIVE_CALL_DEPTH};
use crate::common::method_registry::NativeCallable;
use crate::common::{
    CallFrame, MapKey, NativeCallError, ObjInstance, ObjNativeFunction, ObjStruct, Value,
};
use crate::common::{ObjClosure, Upvalue};
use crate::vm::RuntimeError;
use crate::vm::VirtualMachine;
use crate::{as_number, as_string, boolean, is_false_like, number, string};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::rc::Rc;

pub(in crate::vm) type OpResult = std::result::Result<(), RuntimeError>;

pub(in crate::vm) enum Comparison {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

/// Registry index for the print() function (always at index 0)
const PRINT_METHOD_INDEX: u32 = 0;

/// A receiver's type name for method dispatch: a fixed name for builtin
/// types, or the struct definition for an instance (cloning the `Rc` is a
/// refcount bump, not a string allocation).
enum TypeName {
    Static(&'static str),
    Struct(Rc<ObjStruct>),
}

impl TypeName {
    fn as_str(&self) -> &str {
        match self {
            TypeName::Static(name) => name,
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
    pub(in crate::vm) fn op_string(&mut self) {
        let frame = self.current_frame_mut();
        let string = {
            let string_index = frame.closure.function.chunk.read_u16(frame.ip + 1) as usize;
            frame.closure.function.chunk.read_string(string_index)
        };
        frame.ip += 2;
        self.push(string);
    }

    #[inline(always)]
    pub(in crate::vm) fn op_not(&mut self) {
        let value = self.pop();
        self.push(boolean!(is_false_like!(value)));
    }

    #[inline(always)]
    pub(in crate::vm) fn op_call(&mut self) -> OpResult {
        let arg_count = {
            let frame = self.current_frame();
            frame.closure.function.chunk.read_u8(frame.ip + 1) as usize
        };

        let frame = self.current_frame_mut();
        frame.ip += 2; // Skip CALL opcode and arg_count byte

        self.check_frame_limit()?;

        self.dispatch_call(arg_count)
    }

    /// Invoke: a method call dispatched by name at runtime. Stack before:
    /// `[receiver, args...]`, argc excluding the receiver.
    #[inline(always)]
    pub(in crate::vm) fn op_invoke(&mut self) -> OpResult {
        let (method_name, arg_count) = {
            let frame = self.current_frame();
            let name_index = frame.closure.function.chunk.read_u16(frame.ip + 1) as usize;
            let name = frame.closure.function.chunk.read_string(name_index);
            let arg_count = frame.closure.function.chunk.read_u8(frame.ip + 3) as usize;
            (name, arg_count)
        };

        let frame = self.current_frame_mut();
        frame.ip += 4; // Skip Invoke opcode, name_index and arg_count byte

        self.check_frame_limit()?;

        self.dispatch_invoke(as_string!(method_name), arg_count)
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
    /// Shared by the CALL opcode, `call_value`'s re-entrant native-to-Neon
    /// calls, and `Invoke` falling through to a callable instance field.
    fn dispatch_call(&mut self, arg_count: usize) -> OpResult {
        // Get the callable from the stack
        let callable_value = self.peek(arg_count);

        let result = match &callable_value {
            Value::Closure(closure) => return self.call_closure(arg_count, closure),
            Value::Struct(r#struct) => return self.instantiate_struct(arg_count, r#struct),
            Value::NativeFunction(callable) => {
                match self.call_native_function(arg_count, callable) {
                    Ok(value) => value,
                    Err(NativeCallError::Message(error)) => return Err(self.call_error(error)),
                    Err(NativeCallError::Runtime(e)) => return Err(e),
                }
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

    /// Dispatches a method call by name: the stack holds
    /// `[receiver, args...]`, `arg_count` excluding the receiver. Tries, in
    /// order, a user-defined method, a native method, and a callable
    /// instance field; otherwise reports an unknown method.
    fn dispatch_invoke(&mut self, method_name: &str, arg_count: usize) -> OpResult {
        let receiver_index = self.stack.len() - arg_count - 1;
        let receiver = self.stack[receiver_index].clone();
        let type_name = self.get_type_name(&receiver);

        if let Some(type_name) = &type_name {
            let is_static_call = matches!(receiver, Value::Struct(_));
            match self.dispatch_method_by_name(
                type_name.as_str(),
                is_static_call,
                receiver_index,
                arg_count,
                method_name,
            ) {
                MethodDispatch::Found(closure, arg_count, exclude_self) => {
                    return self.call_closure_with(arg_count, &closure, exclude_self);
                }
                MethodDispatch::Mismatch(e) => return Err(e),
                MethodDispatch::NotFound => {}
            }

            if let Some(native) = crate::common::method_registry::get_native_method_by_name(
                type_name.as_str(),
                method_name,
            ) {
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
                return Ok(());
            }
        }

        if let Value::Instance(inst) = &receiver {
            let field_value = inst.borrow().field(method_name).cloned();
            if let Some(field_value) = field_value {
                self.stack[receiver_index] = field_value;
                return self.dispatch_call(arg_count);
            }
        }

        Err(self.call_error(match type_name {
            Some(type_name) => format!("Unknown method '{}' for type {}", method_name, type_name),
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
            NativeCallable::InstanceMethodWithVm { function, .. } => {
                let args: Vec<Value> = self.stack[args_start..args_end].to_vec();
                function(self, &args)
            }
            NativeCallable::StaticMethod { function, .. }
            | NativeCallable::InstanceMethod { function, .. }
            | NativeCallable::Constructor { function, .. } => {
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

    /// Looks up `method_name` in the user method table under `type_name`
    /// and checks the call form (static vs. instance) against the method's
    /// definition. For an instance call, inserts a `Nil` callee slot below
    /// the receiver so the receiver becomes `self`.
    fn dispatch_method_by_name(
        &mut self,
        type_name: &str,
        is_static_call: bool,
        receiver_index: usize,
        arg_count: usize,
        method_name: &str,
    ) -> MethodDispatch {
        let Some((closure, takes_self)) = self
            .methods
            .get(type_name)
            .and_then(|methods| methods.get(method_name))
            .cloned()
        else {
            return MethodDispatch::NotFound;
        };

        match (is_static_call, takes_self) {
            (true, true) => MethodDispatch::Mismatch(self.call_error(format!(
                "Method '{}' needs an instance; call it on a {} value",
                method_name, type_name
            ))),
            (false, false) => MethodDispatch::Mismatch(self.call_error(format!(
                "Method '{}' is static; call it as {}.{}()",
                method_name, type_name, method_name
            ))),
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

    fn instantiate_struct(&mut self, arg_count: usize, r#struct: &Rc<ObjStruct>) -> OpResult {
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

        let instance = ObjInstance {
            r#struct: Rc::clone(r#struct),
            fields,
        };

        // Pop arguments and struct object from stack
        let n = arg_count + 1;
        let start = self.stack.len().saturating_sub(n);
        self.stack.drain(start..);

        // Push the new instance
        self.push(Value::new_instance(instance));

        // IP already incremented by fn_call_unified
        Ok(())
    }

    fn call_closure(&mut self, arg_count: usize, closure: &Rc<ObjClosure>) -> OpResult {
        self.call_closure_with(arg_count, closure, false)
    }

    /// Calls a closure; `exclude_self` leaves the leading `self` argument
    /// out of an arity-mismatch message.
    fn call_closure_with(
        &mut self,
        arg_count: usize,
        closure: &Rc<ObjClosure>,
        exclude_self: bool,
    ) -> OpResult {
        let func = &closure.function;
        if arg_count != func.arity as usize {
            let (expected, got) = if exclude_self {
                (func.arity - 1, arg_count - 1)
            } else {
                (func.arity, arg_count)
            };
            return Err(self.call_error(format!(
                "Expected {} arguments but got {} for '{}'.",
                expected, got, func.name
            )));
        }

        let slot_start = self.stack.len() as isize - arg_count as isize - 1;

        let new_frame = CallFrame {
            closure: Rc::clone(closure),
            ip: 0,
            slot_start,
        };

        // NOTE: IP increment is handled by the caller (fn_call_unified)
        // Don't increment here to avoid double increment

        self.call_frames.push(new_frame);
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_return(&mut self) {
        let return_value = self.pop();
        let slot_start = self.current_frame().slot_start;
        self.call_frames.pop();

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

    #[inline(always)]
    pub(in crate::vm) fn op_compare(&mut self, wanted: Comparison) -> OpResult {
        let b = self.pop();
        let a = self.pop();
        let is_match = match (&a, &b) {
            (Value::Number(x), Value::Number(y)) => Some(match wanted {
                Comparison::Greater => x > y,
                Comparison::GreaterEqual => x >= y,
                Comparison::Less => x < y,
                Comparison::LessEqual => x <= y,
            }),
            (Value::String(sa), Value::String(sb)) => Some(match wanted {
                Comparison::Greater => sa > sb,
                Comparison::GreaterEqual => sa >= sb,
                Comparison::Less => sa < sb,
                Comparison::LessEqual => sa <= sb,
            }),
            _ => None,
        };
        match is_match {
            Some(is_match) => {
                self.push(boolean!(is_match));
                Ok(())
            }
            None => Err(self.runtime_error(format!(
                "Operands of a comparison must be two numbers or two strings, got {} and {}",
                a.type_name(),
                b.type_name()
            ))),
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn op_equal(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(boolean!(a == b));
    }

    #[inline(always)]
    pub(in crate::vm) fn op_divide(&mut self) -> OpResult {
        self.binary_number_op("/", |a, b| a / b)
    }

    #[inline(always)]
    pub(in crate::vm) fn op_modulo(&mut self) -> OpResult {
        self.binary_number_op("%", |a, b| a % b)
    }

    #[inline(always)]
    pub(in crate::vm) fn op_exponent(&mut self) -> OpResult {
        self.binary_number_op("**", |a, b| a.powf(b))
    }

    fn binary_number_op(&mut self, op: &str, f: impl Fn(f64, f64) -> f64) -> OpResult {
        let b = self.pop();
        let a = self.pop();
        match (&a, &b) {
            (Value::Number(x), Value::Number(y)) => {
                self.push(Value::Number(f(*x, *y)));
                Ok(())
            }
            _ => Err(self.runtime_error(format!(
                "Operands of '{}' must be numbers, got {} and {}",
                op,
                a.type_name(),
                b.type_name()
            ))),
        }
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

    #[inline(always)]
    pub(in crate::vm) fn op_bitwise_and(&mut self) -> OpResult {
        self.binary_number_op("&", |a, b| {
            (Self::to_integer(a) & Self::to_integer(b)) as f64
        })
    }

    #[inline(always)]
    pub(in crate::vm) fn op_bitwise_or(&mut self) -> OpResult {
        self.binary_number_op("|", |a, b| {
            (Self::to_integer(a) | Self::to_integer(b)) as f64
        })
    }

    #[inline(always)]
    pub(in crate::vm) fn op_bitwise_xor(&mut self) -> OpResult {
        self.binary_number_op("^", |a, b| {
            (Self::to_integer(a) ^ Self::to_integer(b)) as f64
        })
    }

    #[inline(always)]
    pub(in crate::vm) fn op_bitwise_not(&mut self) -> OpResult {
        if let Value::Number(..) = self.peek(0) {
            let value = self.pop();
            let int_val = Self::to_integer(as_number!(value));
            self.push(Value::Number((!int_val) as f64));
            return Ok(());
        }
        Err(self.runtime_error("Operand must be a number for bitwise NOT"))
    }

    #[inline(always)]
    pub(in crate::vm) fn op_left_shift(&mut self) -> OpResult {
        self.binary_number_op("<<", |a, b| {
            let shift_amount = (Self::to_integer(b) & 0x3F) as u32; // mask to 0-63
            (Self::to_integer(a) << shift_amount) as f64
        })
    }

    #[inline(always)]
    pub(in crate::vm) fn op_right_shift(&mut self) -> OpResult {
        self.binary_number_op(">>", |a, b| {
            let shift_amount = (Self::to_integer(b) & 0x3F) as u32; // mask to 0-63
            (Self::to_integer(a) >> shift_amount) as f64
        })
    }

    #[inline(always)]
    pub(in crate::vm) fn op_multiply(&mut self) -> OpResult {
        self.binary_number_op("*", |a, b| a * b)
    }

    #[inline(always)]
    pub(in crate::vm) fn op_subtract(&mut self) -> OpResult {
        self.binary_number_op("-", |a, b| a - b)
    }

    #[inline(always)]
    pub(in crate::vm) fn op_add(&mut self) -> OpResult {
        let b = self.pop();
        let a = self.pop();
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => self.push(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => {
                let mut combined = String::with_capacity(a.len() + b.len());
                combined.push_str(&a);
                combined.push_str(&b);
                self.push(string!(combined));
            }
            _ => {
                return Err(self.runtime_error("Operands must be two numbers or two strings"));
            }
        }
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_negate(&mut self) -> OpResult {
        if let Value::Number(..) = self.peek(0) {
            let value = self.pop();
            self.push(number!(-as_number!(value)));
            return Ok(());
        }
        Err(self.runtime_error("Operand must be a number"))
    }

    #[inline(always)]
    pub(in crate::vm) fn op_constant(&mut self) {
        let frame = self.current_frame_mut();
        let constant = {
            let constant_index = frame.closure.function.chunk.read_u16(frame.ip + 1) as usize;
            frame.closure.function.chunk.read_constant(constant_index)
        };
        frame.ip += 2;
        self.push(constant);
    }

    #[inline(always)]
    pub(in crate::vm) fn op_set_local(&mut self) -> OpResult {
        let (index, absolute_index) = self.read_local_slot();
        if absolute_index >= self.stack.len() {
            return Err(self.runtime_error(format!("Invalid local slot {}", index)));
        }
        self.stack[absolute_index] = self.peek(0);
        Ok(())
    }

    fn read_index(&self) -> usize {
        let frame = self.current_frame();
        frame.closure.function.chunk.read_u16(frame.ip + 1) as usize
    }

    /// Wraps a function constant in a closure, capturing whatever upvalues
    /// its metadata (following the constant index) describes.
    #[inline(always)]
    pub(in crate::vm) fn op_closure(&mut self) -> OpResult {
        let const_index = self.read_index();
        let function = {
            let frame = self.current_frame();
            match frame.closure.function.chunk.read_constant(const_index) {
                Value::Function(function) => function,
                _ => unreachable!("Closure operand must reference a function constant"),
            }
        };

        let mut offset = 2;
        let upvalue_count = {
            let frame = self.current_frame();
            frame.closure.function.chunk.read_u8(frame.ip + 1 + offset) as usize
        };
        offset += 1;

        let mut upvalues = Vec::with_capacity(upvalue_count);
        for _ in 0..upvalue_count {
            let (is_local, index) = {
                let frame = self.current_frame();
                let is_local = frame.closure.function.chunk.read_u8(frame.ip + 1 + offset) != 0;
                let index = frame
                    .closure
                    .function
                    .chunk
                    .read_u16(frame.ip + 1 + offset + 1) as usize;
                (is_local, index)
            };
            offset += 3;

            let upvalue = if is_local {
                let absolute_index =
                    (self.current_frame().slot_start + 1 + index as isize) as usize;
                self.capture_upvalue(absolute_index)
            } else {
                if index >= self.current_frame().closure.upvalues.len() {
                    return Err(self.runtime_error(format!("Invalid upvalue index {}", index)));
                }
                Rc::clone(&self.current_frame().closure.upvalues[index])
            };
            upvalues.push(upvalue);
        }

        self.push(Value::new_closure(function, upvalues));
        self.current_frame_mut().ip += offset;
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_get_upvalue(&mut self) -> OpResult {
        let index = self.read_index();
        if index >= self.current_frame().closure.upvalues.len() {
            return Err(self.runtime_error(format!("Invalid upvalue index {}", index)));
        }
        let upvalue = Rc::clone(&self.current_frame().closure.upvalues[index]);
        let value = match &*upvalue.borrow() {
            Upvalue::Open(stack_index) => self.stack[*stack_index].clone(),
            Upvalue::Closed(value) => value.clone(),
        };
        self.current_frame_mut().ip += 2;
        self.push(value);
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_set_upvalue(&mut self) -> OpResult {
        let index = self.read_index();
        if index >= self.current_frame().closure.upvalues.len() {
            return Err(self.runtime_error(format!("Invalid upvalue index {}", index)));
        }
        let value = self.peek(0);
        let upvalue = Rc::clone(&self.current_frame().closure.upvalues[index]);
        let stack_index = match &*upvalue.borrow() {
            Upvalue::Open(stack_index) => Some(*stack_index),
            Upvalue::Closed(_) => None,
        };
        match stack_index {
            Some(stack_index) => self.stack[stack_index] = value,
            None => *upvalue.borrow_mut() = Upvalue::Closed(value),
        }
        self.current_frame_mut().ip += 2;
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

    /// Closes the upvalue on the top-of-stack slot without popping it.
    #[inline(always)]
    pub(in crate::vm) fn op_close_upvalue_in_place(&mut self) {
        let top_index = self.stack.len() - 1;
        self.close_upvalues_above(top_index);
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

    /// Reads a u16 local-slot operand, advancing `ip` past it, and returns
    /// it with its absolute stack index. Locals start at `slot_start + 1`,
    /// which is 0 for the script frame (`slot_start` is -1).
    #[inline(always)]
    fn read_local_slot(&mut self) -> (usize, usize) {
        let index = self.read_index();
        let frame = self.current_frame_mut();
        let absolute_index = (frame.slot_start + 1 + index as isize) as usize;
        frame.ip += 2;
        (index, absolute_index)
    }

    #[inline(always)]
    pub(in crate::vm) fn op_get_local(&mut self) -> OpResult {
        let (index, absolute_index) = self.read_local_slot();
        if absolute_index >= self.stack.len() {
            return Err(self.runtime_error(format!("Invalid local slot {}", index)));
        }
        self.push(self.stack[absolute_index].clone());
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_jump_if_false(&mut self) {
        let peeked_value = self.peek(0);
        let frame = self.current_frame_mut();
        let offset = frame.closure.function.chunk.read_u32(frame.ip + 1);
        frame.ip += 4;
        if is_false_like!(peeked_value) {
            // Don't pop! Leave the value on the stack for logical operators
            // The caller is responsible for popping if needed (e.g., in if statements)
            frame.ip += offset as usize;
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn op_jump(&mut self) {
        let frame = self.current_frame_mut();
        let offset = frame.closure.function.chunk.read_u32(frame.ip + 1);
        frame.ip += 4;
        frame.ip += offset as usize;
    }

    #[inline(always)]
    pub(in crate::vm) fn op_loop(&mut self) {
        let frame = self.current_frame_mut();
        let offset = frame.closure.function.chunk.read_u32(frame.ip + 1);
        frame.ip += 4;
        frame.ip -= offset as usize;
    }

    pub(in crate::vm) fn op_get_builtin(&mut self) -> OpResult {
        let index = self.read_index();
        if let Some(value) = self.builtin.get(index) {
            self.push(value.clone());
        } else {
            return Err(self.runtime_error(format!("Built-in global at index {} not found", index)));
        }
        let frame = self.current_frame_mut();
        frame.ip += 2;
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_get_global(&mut self) -> OpResult {
        let index = self.read_index();

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
        let frame = self.current_frame_mut();
        frame.ip += 2;
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_set_global(&mut self) -> OpResult {
        let index = self.read_index();
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

        self.stack[absolute_index] = self.peek(0);
        let frame = self.current_frame_mut();
        frame.ip += 2;
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_check_initialized(&mut self) -> OpResult {
        if let Some(message) = self.stack.last().and_then(Self::uninitialized_error) {
            return Err(self.runtime_error(message));
        }
        Ok(())
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
    pub(in crate::vm) fn op_get_field(&mut self) -> OpResult {
        let field_name_index = self.read_index();
        let instance_value = self.peek(0);

        let field_name_value = {
            let frame = self.current_frame();
            frame.closure.function.chunk.read_string(field_name_index)
        };
        let field_name = match &field_name_value {
            Value::String(s) => s.as_str(),
            _ => return Err(self.runtime_error("Field name must be a string.")),
        };

        match &instance_value {
            Value::Instance(instance_ref) => {
                let instance = instance_ref.borrow();

                if let Some(value) = instance.field(field_name).cloned() {
                    self.pop();
                    self.push(value);
                } else {
                    return Err(self.runtime_error(format!("Undefined field '{}'.", field_name)));
                }
            }
            _ => return Err(self.runtime_error("Only instances have fields.")),
        }

        let frame = self.current_frame_mut();
        frame.ip += 2;
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_set_field(&mut self) -> OpResult {
        let field_name_index = self.read_index();
        let value = self.peek(0);
        let instance_value = self.peek(1);

        let field_name_value = {
            let frame = self.current_frame();
            frame.closure.function.chunk.read_string(field_name_index)
        };
        let field_name = match &field_name_value {
            Value::String(s) => s.as_str(),
            _ => return Err(self.runtime_error("Field name must be a string.")),
        };

        match &instance_value {
            Value::Instance(instance_ref) => {
                let mut instance = instance_ref.borrow_mut();
                let Some(index) = instance.r#struct.field_index(field_name) else {
                    return Err(self.runtime_error(format!("Undefined field '{}'.", field_name)));
                };

                instance.fields[index] = value.clone();

                self.pop();
                self.pop();
                self.push(value);
            }
            _ => return Err(self.runtime_error("Only instances have fields.")),
        }

        let frame = self.current_frame_mut();
        frame.ip += 2;
        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_create_map(&mut self) -> OpResult {
        let count = {
            let frame = self.current_frame();
            frame.closure.function.chunk.read_u16(frame.ip + 1) as usize
        };

        let stack_len = self.stack.len();
        let pairs_start = stack_len - (count * 2);

        let mut map = IndexMap::with_capacity(count);
        for i in 0..count {
            let key_value = &self.stack[pairs_start + 2 * i];
            let value = &self.stack[pairs_start + 2 * i + 1];

            let key = match MapKey::from_value(key_value) {
                Some(k) => k,
                None => {
                    return Err(self.runtime_error(format!(
                        "Invalid map key type: {}. Only strings, numbers, and booleans can be used as map keys.",
                        key_value
                    )));
                }
            };

            map.insert(key, value.clone());
        }

        self.stack.drain(pairs_start..);

        self.push(Value::new_map(map));

        let frame = self.current_frame_mut();
        frame.ip += 2;
        Ok(())
    }

    pub(in crate::vm) fn op_create_array(&mut self) {
        let count = {
            let frame = self.current_frame();
            frame.closure.function.chunk.read_u16(frame.ip + 1) as usize
        };

        let stack_len = self.stack.len();
        let elements_start = stack_len - count;

        let elements: Vec<Value> = self.stack[elements_start..stack_len].to_vec();

        self.stack.drain(elements_start..);

        self.push(Value::new_array(elements));

        let frame = self.current_frame_mut();
        frame.ip += 2;
    }

    #[inline(always)]
    pub(in crate::vm) fn op_create_set(&mut self) -> OpResult {
        let count = {
            let frame = self.current_frame();
            frame.closure.function.chunk.read_u16(frame.ip + 1) as usize
        };

        let stack_len = self.stack.len();
        let elements_start = stack_len - count;

        let mut set = std::collections::BTreeSet::new();
        for i in 0..count {
            let element_value = &self.stack[elements_start + i];

            let key = match MapKey::from_value(element_value) {
                Some(k) => k,
                None => {
                    return Err(self.runtime_error(format!(
                        "Invalid set element type: {}. Only strings, numbers, and booleans can be used as set elements.",
                        element_value
                    )));
                }
            };

            set.insert(key);
        }

        self.stack.drain(elements_start..);

        self.push(Value::new_set(set));

        let frame = self.current_frame_mut();
        frame.ip += 2;
        Ok(())
    }

    pub(in crate::vm) fn op_create_range(&mut self) -> OpResult {
        let inclusive = {
            let frame = self.current_frame();
            frame.closure.function.chunk.read_u8(frame.ip + 1) != 0
        };

        let end_value = self.pop();
        let start_value = self.pop();

        let start = match start_value {
            Value::Number(n) => n,
            _ => {
                return Err(self
                    .runtime_error(format!("Range start must be a number, got {}", start_value)));
            }
        };

        let end = match end_value {
            Value::Number(n) => n,
            _ => {
                return Err(
                    self.runtime_error(format!("Range end must be a number, got {}", end_value))
                );
            }
        };

        if start.fract() != 0.0 {
            return Err(
                self.runtime_error(format!("Range start must be an integer, got {}", start))
            );
        }

        if end.fract() != 0.0 {
            return Err(self.runtime_error(format!("Range end must be an integer, got {}", end)));
        }

        const MAX_RANGE_BOUND: f64 = 9007199254740992.0; // 2^53, the largest integer an f64 represents exactly

        if start.abs() > MAX_RANGE_BOUND {
            return Err(self.runtime_error(format!(
                "Range start must be between -2^53 and 2^53, got {}",
                start
            )));
        }

        if end.abs() > MAX_RANGE_BOUND {
            return Err(self.runtime_error(format!(
                "Range end must be between -2^53 and 2^53, got {}",
                end
            )));
        }

        let range = Value::new_range(start as i64, end as i64, inclusive);
        if let Value::Range(r) = &range {
            if r.len() > MAX_RANGE_BOUND as i64 {
                return Err(self.runtime_error(format!(
                    "Range must have at most 2^53 elements, got {}",
                    range
                )));
            }
        }

        self.push(range);

        let frame = self.current_frame_mut();
        frame.ip += 1;

        Ok(())
    }

    #[inline(always)]
    pub(in crate::vm) fn op_get_index(&mut self) -> OpResult {
        let index_value = self.pop();
        let collection_value = self.pop();

        match &collection_value {
            Value::Map(map_ref) => {
                // Convert index to MapKey
                let key = match MapKey::from_value(&index_value) {
                    Some(k) => k,
                    None => {
                        return Err(self.runtime_error(format!(
                            "Invalid map key type: {}. Only strings, numbers, and booleans can be used as map keys.",
                            index_value
                        )));
                    }
                };

                let map = map_ref.borrow();
                let result = map.get(&key).cloned().unwrap_or(Value::Nil);
                self.push(result);
                Ok(())
            }
            Value::Array(array_ref) => {
                let index = match index_value {
                    Value::Number(n) => n as i32,
                    _ => {
                        return Err(self.runtime_error(format!(
                            "Array index must be a number, got {}.",
                            index_value
                        )));
                    }
                };

                let array = array_ref.borrow();
                let len = array.len() as i32;

                let actual_index = if index < 0 { len + index } else { index };

                if actual_index < 0 || actual_index >= len {
                    return Err(self.runtime_error(format!(
                        "Array index out of bounds: index {} (normalized: {}) on array of length {}.",
                        index, actual_index, len
                    )));
                }

                let result = array[actual_index as usize].clone();
                self.push(result);
                Ok(())
            }
            Value::Range(range) => {
                let index = match index_value {
                    Value::Number(n) => n as i64,
                    _ => {
                        return Err(self.runtime_error(format!(
                            "Range index must be a number, got {}.",
                            index_value
                        )));
                    }
                };

                let len = range.len();
                let actual_index = if index < 0 { len + index } else { index };

                if actual_index < 0 || actual_index >= len {
                    return Err(self.runtime_error(format!(
                        "Range index out of bounds: index {} (normalized: {}) on range of length {}.",
                        index, actual_index, len
                    )));
                }

                self.push(Value::Number(range.get(actual_index) as f64));
                Ok(())
            }
            _ => Err(self.runtime_error(format!(
                "Only arrays, maps, and ranges support index access, got {}.",
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
                // Convert index to MapKey
                let key = match MapKey::from_value(&index_value) {
                    Some(k) => k,
                    None => {
                        return Err(self.runtime_error(format!(
                            "Invalid map key type: {}. Only strings, numbers, and booleans can be used as map keys.",
                            index_value
                        )));
                    }
                };

                let mut map = map_ref.borrow_mut();
                map.insert(key, value.clone());

                self.push(value);
                Ok(())
            }
            Value::Array(array_ref) => {
                let index = match index_value {
                    Value::Number(n) => n as i32,
                    _ => {
                        return Err(self.runtime_error(format!(
                            "Array index must be a number, got {}.",
                            index_value
                        )));
                    }
                };

                let mut array = array_ref.borrow_mut();
                let len = array.len() as i32;

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
    pub(in crate::vm) fn op_get_iterator(&mut self) -> OpResult {
        let collection = self.pop();

        let iterator_value = match &collection {
            Value::Array(_) => collection,
            Value::Range(_) => collection,
            Value::Map(map_ref) => {
                let map = map_ref.borrow();
                let keys: Vec<Value> = map.keys().map(MapKey::to_value).collect();

                Value::new_array(keys)
            }
            Value::Set(set_ref) => {
                let set = set_ref.borrow();
                let elements: Vec<Value> = set.iter().map(MapKey::to_value).collect();

                Value::new_array(elements)
            }
            _ => {
                return Err(self.runtime_error(format!(
                    "Cannot iterate over type: {}. Only arrays, maps, sets, and ranges are iterable.",
                    collection
                )));
            }
        };

        self.push(iterator_value);
        self.push(number!(0.0));
        Ok(())
    }

    /// Resolves the u16 slot operand for an iterator opcode to the absolute
    /// index of its first hidden slot (collection), checking that both it and
    /// the index slot right after it are in range.
    #[inline(always)]
    fn read_iterator_slot(&mut self) -> std::result::Result<usize, RuntimeError> {
        let (index, slot) = self.read_local_slot();
        if slot + 1 >= self.stack.len() {
            return Err(self.runtime_error(format!("Invalid local slot {}", index)));
        }
        Ok(slot)
    }

    /// IteratorDone: Check if iteration is complete for the hidden iterator
    /// slots starting at the given local slot (collection, then index).
    /// Pushes false if done (no more elements), true if not done (more elements remain)
    /// This inverted logic allows JumpIfFalse to exit the loop when done
    #[inline(always)]
    pub(in crate::vm) fn op_iterator_done(&mut self) -> OpResult {
        let slot = self.read_iterator_slot()?;
        let index = match &self.stack[slot + 1] {
            Value::Number(n) => *n as usize,
            other => {
                return Err(self.runtime_error(format!(
                    "Invalid iterator index, got {}.",
                    other.type_name()
                )));
            }
        };
        let has_more = match &self.stack[slot] {
            Value::Array(array_ref) => index < array_ref.borrow().len(),
            Value::Range(range) => (index as i64) < range.len(),
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
    pub(in crate::vm) fn op_iterator_next(&mut self) -> OpResult {
        let slot = self.read_iterator_slot()?;
        let index = match &self.stack[slot + 1] {
            Value::Number(n) => *n as usize,
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
                if index >= array.len() {
                    return Err(self.runtime_error("Iterator exhausted"));
                }
                array[index].clone()
            }
            Value::Range(range) => {
                if (index as i64) >= range.len() {
                    return Err(self.runtime_error("Iterator exhausted"));
                }
                Value::Number(range.get(index as i64) as f64)
            }
            other => {
                return Err(self.runtime_error(format!(
                    "Invalid iterator collection, got {}.",
                    other.type_name()
                )));
            }
        };

        self.stack[slot + 1] = number!((index + 1) as f64);
        self.push(value);
        Ok(())
    }

    /// Helper: Extract type name from a value for method dispatch
    fn get_type_name(&self, value: &Value) -> Option<TypeName> {
        match value {
            Value::Array(_) => Some(TypeName::Static("Array")),
            Value::String(_) => Some(TypeName::Static("String")),
            Value::Map(_) => Some(TypeName::Static("Map")),
            Value::Set(_) => Some(TypeName::Static("Set")),
            Value::File(_) => Some(TypeName::Static("File")),
            Value::Range(_) => Some(TypeName::Static("Range")),
            Value::Instance(inst) => Some(TypeName::Struct(Rc::clone(&inst.borrow().r#struct))),
            // The struct value itself (e.g. `Point` in `Point.origin()`)
            // dispatches static methods under the struct's own name.
            Value::Struct(r#struct) => Some(TypeName::Struct(Rc::clone(r#struct))),
            Value::Number(_) => Some(TypeName::Static("Number")),
            Value::Boolean(_) => Some(TypeName::Static("Boolean")),
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
    /// preceding Closure op and registers it under (type name, method name),
    /// along with whether the method takes `self`.
    #[inline(always)]
    pub(in crate::vm) fn op_define_method(&mut self) {
        let frame = self.current_frame_mut();
        let type_name = {
            let index = frame.closure.function.chunk.read_u16(frame.ip + 1) as usize;
            frame.closure.function.chunk.read_string(index)
        };
        let method_name = {
            let index = frame.closure.function.chunk.read_u16(frame.ip + 3) as usize;
            frame.closure.function.chunk.read_string(index)
        };
        let takes_self = frame.closure.function.chunk.read_u8(frame.ip + 5) != 0;
        frame.ip += 5;

        let closure_value = self.pop();
        let type_name = as_string!(type_name).to_string();
        let method_name = as_string!(method_name).to_string();
        let Value::Closure(closure) = &closure_value else {
            unreachable!("DefineMethod expects a closure on top of the stack")
        };
        self.methods
            .entry(type_name)
            .or_default()
            .insert(method_name, (Rc::clone(closure), takes_self));
    }
}
