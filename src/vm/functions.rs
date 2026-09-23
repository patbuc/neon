use crate::common::constants::{MAX_FRAMES, MAX_NATIVE_CALL_DEPTH};
use crate::common::method_registry::NativeCallable;
use crate::common::{
    BitsSize, CallFrame, NativeCallError, ObjInstance, ObjNativeFunction, ObjStruct, Value,
};
use crate::common::{ObjClosure, Object, Upvalue};
use crate::vm::Result;
use crate::vm::VirtualMachine;
use crate::{as_number, as_string, boolean, is_false_like, number, string};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;

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

impl VirtualMachine {
    #[inline(always)]
    pub(in crate::vm) fn fn_to_string(&mut self) {
        let value = self.pop();
        let string_value = string!(value.to_string());
        self.push(string_value);
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_string4(&mut self) {
        let frame = self.current_frame_mut();
        let string = {
            let string_index = frame.closure.function.chunk.read_u32(frame.ip + 1) as usize;
            frame.closure.function.chunk.read_string(string_index)
        };
        frame.ip += 4;
        self.push(string);
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_string2(&mut self) {
        let frame = self.current_frame_mut();
        let string = {
            let string_index = frame.closure.function.chunk.read_u16(frame.ip + 1) as usize;
            frame.closure.function.chunk.read_string(string_index)
        };
        frame.ip += 2;
        self.push(string);
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_string(&mut self) {
        let frame = self.current_frame_mut();
        let string = {
            let string_index = frame.closure.function.chunk.read_u8(frame.ip + 1) as usize;
            frame.closure.function.chunk.read_string(string_index)
        };
        frame.ip += 1;
        self.push(string);
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_not(&mut self) {
        let value = self.pop();
        self.push(boolean!(is_false_like!(value)));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_call(&mut self) -> Option<Result> {
        if self.frame_limit_reached() {
            return Some(Result::RuntimeError);
        }

        let arg_count = {
            let frame = self.current_frame();
            frame.closure.function.chunk.read_u8(frame.ip + 1) as usize
        };

        let frame = self.current_frame_mut();
        frame.ip += 2; // Skip CALL opcode and arg_count byte

        self.dispatch_call(arg_count)
    }

    /// Records "Stack overflow" and returns true if the call frame stack
    /// is already at its limit. Checked before pushing a new frame, by
    /// both fn_call (before its ip advances, so the error points at the
    /// call site) and call_value's re-entrant native-to-Neon call.
    fn frame_limit_reached(&mut self) -> bool {
        if self.call_frames.len() >= MAX_FRAMES {
            self.runtime_error("Stack overflow");
            true
        } else {
            false
        }
    }

    /// Dispatches a call: the stack must already hold `[args..., callable]`
    /// with the callable on top. Shared by the CALL opcode and by
    /// `call_value`'s re-entrant native-to-Neon calls.
    fn dispatch_call(&mut self, arg_count: usize) -> Option<Result> {
        // Get the callable from the stack
        let callable_value = self.peek(0);

        let result = match &callable_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Closure(closure) => return self.call_closure(arg_count, closure),
                Object::Struct(r#struct) => return self.instantiate_struct(arg_count, r#struct),
                Object::NativeFunction(callable) => {
                    if callable.method_index == u32::MAX {
                        if let Some(outcome) = self.dispatch_user_method_call(arg_count, callable) {
                            return outcome;
                        }
                    }
                    match self.call_native_function(arg_count, callable) {
                        Ok(value) => value,
                        Err(NativeCallError::Message(error)) => {
                            self.runtime_error(&error);
                            return Some(Result::RuntimeError);
                        }
                        Err(NativeCallError::AlreadyReported) => {
                            return Some(Result::RuntimeError);
                        }
                    }
                }
                _ => {
                    self.runtime_error("Value is not callable");
                    return Some(Result::RuntimeError);
                }
            },
            _ => {
                self.runtime_error("Value is not callable");
                return Some(Result::RuntimeError);
            }
        };

        // Pop callable and arguments, then push result
        let stack_len = self.stack.len();
        self.stack.truncate(stack_len - arg_count - 1);
        self.stack.push(result);
        None
    }

    /// Lets a native call a Neon value (closure, lambda, or native) with
    /// the given arguments, running the dispatch loop re-entrantly until
    /// that call returns.
    pub(crate) fn call_value(
        &mut self,
        callee: Value,
        args: &[Value],
    ) -> std::result::Result<Value, NativeCallError> {
        if self.frame_limit_reached() {
            return Err(NativeCallError::AlreadyReported);
        }
        if self.native_call_depth >= MAX_NATIVE_CALL_DEPTH {
            self.runtime_error("Stack overflow");
            return Err(NativeCallError::AlreadyReported);
        }

        let frame_depth = self.call_frames.len();
        self.native_call_depth += 1;

        for arg in args {
            self.push(arg.clone());
        }
        self.push(callee);

        let outcome = match self.dispatch_call(args.len()) {
            Some(result) => result,
            None if self.call_frames.len() > frame_depth => self.run_until(frame_depth),
            None => Result::Ok,
        };

        self.native_call_depth -= 1;

        if outcome == Result::Ok {
            Ok(self.pop())
        } else {
            Err(NativeCallError::AlreadyReported)
        }
    }

    /// Checks the user method table for a by-name call before the native
    /// registry. Returns `None` when no user method matches (fall through to
    /// native dispatch); otherwise dispatches to the method and returns the
    /// outcome to hand straight back to `dispatch_call`'s caller.
    ///
    /// Static calls (`Point.origin()`) receive the struct value itself as
    /// receiver: since the method has no `self` parameter, drop the receiver
    /// slot before invoking the closure. Instance calls keep the receiver as
    /// the closure's first (`self`) argument.
    fn dispatch_user_method_call(
        &mut self,
        arg_count: usize,
        callable: &Rc<ObjNativeFunction>,
    ) -> Option<Option<Result>> {
        let args_start = self.stack.len() - arg_count - 1;
        let receiver = &self.stack[args_start];
        let type_name = self.get_type_name(receiver)?;
        let is_static_call =
            matches!(receiver, Value::Object(obj) if matches!(obj.as_ref(), Object::Struct(_)));

        let closure = self
            .methods
            .get(&(type_name.as_str().to_string(), callable.method_name.clone()))?
            .clone();

        if is_static_call {
            self.stack.remove(args_start);
            Some(self.call_closure(arg_count - 1, &closure))
        } else {
            Some(self.call_closure(arg_count, &closure))
        }
    }

    fn call_native_function(
        &mut self,
        arg_count: usize,
        callable: &Rc<ObjNativeFunction>,
    ) -> std::result::Result<Value, NativeCallError> {
        let native_callable_result = if callable.method_index != u32::MAX {
            self.lookup_native_method_by_index(callable)
        } else {
            self.lookup_native_method_by_name(arg_count, callable)
        };

        let native_callable = native_callable_result.map_err(NativeCallError::Message)?;

        let stack_len = self.stack.len();
        let args_start = stack_len - arg_count - 1;
        let args_end = stack_len - 1;

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
        match *native_callable {
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

    fn instantiate_struct(&mut self, arg_count: usize, r#struct: &Rc<ObjStruct>) -> Option<Result> {
        if arg_count != r#struct.fields.len() {
            self.runtime_error(&format!(
                "Expected {} fields but got {}.",
                r#struct.fields.len(),
                arg_count
            ));
            return Some(Result::RuntimeError);
        }

        let field_count = r#struct.fields.len();
        let mut fields = HashMap::with_capacity(field_count);
        let stack_len = self.stack.len();

        // Unified calling convention: [args..., struct_obj]
        // Extract arguments, excluding the struct object at the top
        let stack_slice = &self.stack[stack_len - arg_count - 1..stack_len - 1];
        for (field_name, value) in r#struct.fields.iter().zip(stack_slice.iter()) {
            fields.insert(field_name.clone(), value.clone());
        }

        let instance = ObjInstance {
            r#struct: Rc::clone(r#struct),
            fields,
        };

        // Pop arguments and struct object from stack
        let n = arg_count + 1;
        let start = self.stack.len().saturating_sub(n);
        self.stack.drain(start..);

        // Push the new instance
        self.push(Value::new_object(instance));

        // IP already incremented by fn_call_unified
        None
    }

    fn call_closure(&mut self, arg_count: usize, closure: &Rc<ObjClosure>) -> Option<Result> {
        let func = &closure.function;
        if arg_count != func.arity as usize {
            self.runtime_error(&format!(
                "Expected {} arguments but got {} for '{}'.",
                func.arity, arg_count, func.name
            ));
            return Some(Result::RuntimeError);
        }

        // Calculate slot_start for unified calling convention [args..., func]
        // The function object is still on the stack at this point
        // Stack layout: [...previous..., arg0, arg1, ..., argN, func_obj]
        // slot_start should point just BEFORE the first argument
        // So: slot_start = current_len - arg_count - 1 (for func) - 1 (to go before first arg)
        let slot_start = (self.stack.len() - arg_count - 1 - 1) as isize;

        let new_frame = CallFrame {
            closure: Rc::clone(closure),
            ip: 0,
            slot_start,
            iterator_depth: self.iterator_stack.len(),
        };

        // NOTE: IP increment is handled by the caller (fn_call_unified)
        // Don't increment here to avoid double increment

        self.call_frames.push(new_frame);
        None
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_return(&mut self) -> Option<Result> {
        let return_value = self.pop();
        let slot_start = self.current_frame().slot_start;
        let iterator_depth = self.current_frame().iterator_depth;
        self.call_frames.pop();
        self.iterator_stack.truncate(iterator_depth);

        // A local captured by a closure that outlives this call must keep
        // its value once this frame's stack slots go away.
        self.close_upvalues_above((slot_start + 1) as usize);

        if self.call_frames.is_empty() {
            self.push(return_value);
            return Some(Result::Ok);
        }

        // Clear the stack back to slot_start + 1 (where first arg was)
        // In unified calling convention [args..., func], we want to replace args+func with result
        self.stack.truncate((slot_start + 1) as usize);
        self.push(return_value);
        None
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_compare(&mut self, wanted: Ordering) -> Option<Result> {
        let b = self.pop();
        let a = self.pop();
        let is_match = match (&a, &b) {
            (Value::Number(x), Value::Number(y)) => Some(x.partial_cmp(y) == Some(wanted)),
            (Value::Object(oa), Value::Object(ob)) => match (oa.as_ref(), ob.as_ref()) {
                (Object::String(sa), Object::String(sb)) => Some(sa.value.cmp(&sb.value) == wanted),
                _ => None,
            },
            _ => None,
        };
        match is_match {
            Some(is_match) => {
                self.push(boolean!(is_match));
                None
            }
            None => {
                self.runtime_error(&format!(
                    "Operands of a comparison must be two numbers or two strings, got {} and {}",
                    a.type_name(),
                    b.type_name()
                ));
                Some(Result::RuntimeError)
            }
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_equal(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(boolean!(a == b));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_divide(&mut self) -> Option<Result> {
        self.binary_number_op("/", |a, b| a / b)
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_modulo(&mut self) -> Option<Result> {
        self.binary_number_op("%", |a, b| a % b)
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_exponent(&mut self) -> Option<Result> {
        self.binary_number_op("**", |a, b| a.powf(b))
    }

    fn binary_number_op(&mut self, op: &str, f: impl Fn(f64, f64) -> f64) -> Option<Result> {
        let b = self.pop();
        let a = self.pop();
        match (&a, &b) {
            (Value::Number(x), Value::Number(y)) => {
                self.push(Value::Number(f(*x, *y)));
                None
            }
            _ => {
                self.runtime_error(&format!(
                    "Operands of '{}' must be numbers, got {} and {}",
                    op,
                    a.type_name(),
                    b.type_name()
                ));
                Some(Result::RuntimeError)
            }
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
    pub(in crate::vm) fn fn_bitwise_and(&mut self) -> Option<Result> {
        self.binary_number_op("&", |a, b| {
            (Self::to_integer(a) & Self::to_integer(b)) as f64
        })
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_bitwise_or(&mut self) -> Option<Result> {
        self.binary_number_op("|", |a, b| {
            (Self::to_integer(a) | Self::to_integer(b)) as f64
        })
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_bitwise_xor(&mut self) -> Option<Result> {
        self.binary_number_op("^", |a, b| {
            (Self::to_integer(a) ^ Self::to_integer(b)) as f64
        })
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_bitwise_not(&mut self) -> Option<Result> {
        if let Value::Number(..) = self.peek(0) {
            let value = self.pop();
            let int_val = Self::to_integer(as_number!(value));
            self.push(Value::Number((!int_val) as f64));
            return None;
        }
        self.runtime_error("Operand must be a number for bitwise NOT");
        Some(Result::RuntimeError)
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_left_shift(&mut self) -> Option<Result> {
        self.binary_number_op("<<", |a, b| {
            let shift_amount = (Self::to_integer(b) & 0x3F) as u32; // mask to 0-63
            (Self::to_integer(a) << shift_amount) as f64
        })
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_right_shift(&mut self) -> Option<Result> {
        self.binary_number_op(">>", |a, b| {
            let shift_amount = (Self::to_integer(b) & 0x3F) as u32; // mask to 0-63
            (Self::to_integer(a) >> shift_amount) as f64
        })
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_multiply(&mut self) -> Option<Result> {
        self.binary_number_op("*", |a, b| a * b)
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_subtract(&mut self) -> Option<Result> {
        self.binary_number_op("-", |a, b| a - b)
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_add(&mut self) -> Option<Result> {
        let b = self.pop();
        let a = self.pop();
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => self.push(Value::Number(a + b)),
            (Value::Object(a), Value::Object(b)) => {
                let obj_a = a.as_ref();
                let obj_b = b.as_ref();
                if let Some(result) = self.fn_add_object(obj_a, obj_b) {
                    return Some(result);
                }
            }
            _ => {
                self.runtime_error("Operands must be two numbers or two strings");
                return Some(Result::RuntimeError);
            }
        }
        None
    }

    fn fn_add_object(&mut self, a: &Object, b: &Object) -> Option<Result> {
        match (a, b) {
            (Object::String(obj_a), Object::String(obj_b)) => {
                let mut combined = String::with_capacity(obj_a.value.len() + obj_b.value.len());
                combined.push_str(&obj_a.value);
                combined.push_str(&obj_b.value);
                self.push(string!(combined));
                None
            }
            _ => {
                self.runtime_error("Operands must be two numbers or two strings");
                Some(Result::RuntimeError)
            }
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_negate(&mut self) -> Option<Result> {
        if let Value::Number(..) = self.peek(0) {
            let value = self.pop();
            self.push(number!(-as_number!(value)));
            return None;
        }
        self.runtime_error("Operand must be a number");
        Some(Result::RuntimeError)
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_constant4(&mut self) {
        let frame = self.current_frame_mut();
        let constant = {
            let constant_index = frame.closure.function.chunk.read_u32(frame.ip + 1) as usize;
            frame.closure.function.chunk.read_constant(constant_index)
        };
        frame.ip += 4;
        self.push(constant);
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_constant2(&mut self) {
        let frame = self.current_frame_mut();
        let constant = {
            let constant_index = frame.closure.function.chunk.read_u16(frame.ip + 1) as usize;
            frame.closure.function.chunk.read_constant(constant_index)
        };
        frame.ip += 2;
        self.push(constant);
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_constant(&mut self) {
        let frame = self.current_frame_mut();
        let constant = {
            let constant_index = frame.closure.function.chunk.read_u8(frame.ip + 1) as usize;
            frame.closure.function.chunk.read_constant(constant_index)
        };
        frame.ip += 1;
        self.push(constant);
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_local(&mut self, bits: BitsSize) -> Option<Result> {
        let index = self.read_bits(&bits);
        let frame = self.current_frame_mut();
        // For functions: slot_start points to function object, args start at slot_start + 1
        // For script: slot_start = -1, so locals start at 0
        // locals (params) are indexed from 0, so param 0 is at slot_start + 1
        let absolute_index = (frame.slot_start + 1 + index as isize) as usize;
        frame.ip += bits.as_bytes();
        if absolute_index >= self.stack.len() {
            self.runtime_error(&format!("Invalid local slot {}", index));
            return Some(Result::RuntimeError);
        }
        self.stack[absolute_index] = self.peek(0);
        None
    }

    fn read_bits(&mut self, bits: &BitsSize) -> usize {
        let frame = self.current_frame();
        match bits {
            BitsSize::Eight => frame.closure.function.chunk.read_u8(frame.ip + 1) as usize,
            BitsSize::Sixteen => frame.closure.function.chunk.read_u16(frame.ip + 1) as usize,
            BitsSize::ThirtyTwo => frame.closure.function.chunk.read_u32(frame.ip + 1) as usize,
        }
    }

    /// Wraps a function constant in a closure, capturing whatever upvalues
    /// its metadata (following the constant index) describes.
    #[inline(always)]
    pub(in crate::vm) fn fn_closure(&mut self, bits: BitsSize) -> Option<Result> {
        let const_index = self.read_bits(&bits);
        let function = {
            let frame = self.current_frame();
            match frame.closure.function.chunk.read_constant(const_index) {
                Value::Object(obj) => match obj.as_ref() {
                    Object::Function(function) => Rc::clone(function),
                    _ => unreachable!("Closure operand must reference a function constant"),
                },
                _ => unreachable!("Closure operand must reference a function constant"),
            }
        };

        let mut offset = bits.as_bytes();
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
                    self.runtime_error(&format!("Invalid upvalue index {}", index));
                    return Some(Result::RuntimeError);
                }
                Rc::clone(&self.current_frame().closure.upvalues[index])
            };
            upvalues.push(upvalue);
        }

        self.push(Value::new_closure(function, upvalues));
        self.current_frame_mut().ip += offset;
        None
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_upvalue(&mut self, bits: BitsSize) -> Option<Result> {
        let index = self.read_bits(&bits);
        if index >= self.current_frame().closure.upvalues.len() {
            self.runtime_error(&format!("Invalid upvalue index {}", index));
            return Some(Result::RuntimeError);
        }
        let upvalue = Rc::clone(&self.current_frame().closure.upvalues[index]);
        let value = match &*upvalue.borrow() {
            Upvalue::Open(stack_index) => self.stack[*stack_index].clone(),
            Upvalue::Closed(value) => value.clone(),
        };
        self.current_frame_mut().ip += bits.as_bytes();
        self.push(value);
        None
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_upvalue(&mut self, bits: BitsSize) -> Option<Result> {
        let index = self.read_bits(&bits);
        if index >= self.current_frame().closure.upvalues.len() {
            self.runtime_error(&format!("Invalid upvalue index {}", index));
            return Some(Result::RuntimeError);
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
        self.current_frame_mut().ip += bits.as_bytes();
        None
    }

    /// Closes the upvalue (if any) pointing at the current top-of-stack
    /// slot, then pops it. Used at block exit for a captured local, in
    /// place of a plain Pop.
    #[inline(always)]
    pub(in crate::vm) fn fn_close_upvalue(&mut self) {
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

    #[inline(always)]
    pub(in crate::vm) fn fn_get_local(&mut self, bits: BitsSize) -> Option<Result> {
        let index = self.read_bits(&bits);
        let frame = self.current_frame_mut();
        let absolute_index = (frame.slot_start + 1 + index as isize) as usize;
        frame.ip += bits.as_bytes();
        if absolute_index >= self.stack.len() {
            self.runtime_error(&format!("Invalid local slot {}", index));
            return Some(Result::RuntimeError);
        }
        self.push(self.stack[absolute_index].clone());
        None
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_jump_if_false(&mut self) {
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
    pub(in crate::vm) fn fn_jump(&mut self) {
        let frame = self.current_frame_mut();
        let offset = frame.closure.function.chunk.read_u32(frame.ip + 1);
        frame.ip += 4;
        frame.ip += offset as usize;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_loop(&mut self) {
        let frame = self.current_frame_mut();
        let offset = frame.closure.function.chunk.read_u32(frame.ip + 1);
        frame.ip += 4;
        frame.ip -= offset as usize;
    }

    pub(in crate::vm) fn fn_get_builtin(&mut self, bits: BitsSize) -> Option<Result> {
        let index = self.read_bits(&bits);
        if let Some(entry) = self.builtin.get_index(index) {
            self.push(entry.1.clone());
        } else {
            self.runtime_error(&format!("Built-in global at index {} not found", index));
            return Some(Result::RuntimeError);
        }
        let frame = self.current_frame_mut();
        frame.ip += bits.as_bytes();
        None
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_global(&mut self, bits: BitsSize) -> Option<Result> {
        let index = self.read_bits(&bits);

        // Regular global variables are in the script frame
        // Script frame has slot_start = -1, so globals start at index 0
        let script_frame = &self.call_frames[0];
        let absolute_index = (script_frame.slot_start + 1 + index as isize) as usize;

        // Make sure we don't go out of bounds
        if absolute_index >= self.stack.len() {
            self.runtime_error(&format!(
                "Global variable index {} out of bounds (stack size: {})",
                absolute_index,
                self.stack.len()
            ));
            return Some(Result::RuntimeError);
        }

        self.push(self.stack[absolute_index].clone());
        let frame = self.current_frame_mut();
        frame.ip += bits.as_bytes();
        None
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_global(&mut self, bits: BitsSize) -> Option<Result> {
        let index = self.read_bits(&bits);
        // Global variables are always in the script frame (first frame)
        // Script frame has slot_start = -1, so globals start at index 0
        let script_frame = &self.call_frames[0];
        let absolute_index = (script_frame.slot_start + 1 + index as isize) as usize;

        if absolute_index >= self.stack.len() {
            self.runtime_error(&format!(
                "Global variable index {} out of bounds (stack size: {})",
                absolute_index,
                self.stack.len()
            ));
            return Some(Result::RuntimeError);
        }

        self.stack[absolute_index] = self.peek(0);
        let frame = self.current_frame_mut();
        frame.ip += bits.as_bytes();
        None
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_field(&mut self, bits: BitsSize) -> Option<Result> {
        let field_name_index = self.read_bits(&bits);
        let instance_value = self.peek(0);

        let field_name = {
            let frame = self.current_frame();
            let field_value = frame.closure.function.chunk.read_string(field_name_index);
            match field_value {
                Value::Object(obj) => match obj.as_ref() {
                    Object::String(s) => s.value.to_string(),
                    _ => {
                        self.runtime_error("Field name must be a string.");
                        return Some(Result::RuntimeError);
                    }
                },
                _ => {
                    self.runtime_error("Field name must be a string.");
                    return Some(Result::RuntimeError);
                }
            }
        };

        match &instance_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Instance(instance_ref) => {
                    let instance = instance_ref.borrow();

                    if let Some(value) = instance.fields.get(&field_name) {
                        let value = value.clone();
                        self.pop();
                        self.push(value);
                    } else {
                        self.runtime_error(&format!("Undefined field '{}'.", field_name));
                        return Some(Result::RuntimeError);
                    }
                }
                _ => {
                    self.runtime_error("Only instances have fields.");
                    return Some(Result::RuntimeError);
                }
            },
            _ => {
                self.runtime_error("Only instances have fields.");
                return Some(Result::RuntimeError);
            }
        }

        let frame = self.current_frame_mut();
        frame.ip += bits.as_bytes();
        None
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_field(&mut self, bits: BitsSize) -> Option<Result> {
        let field_name_index = self.read_bits(&bits);
        let value = self.peek(0);
        let instance_value = self.peek(1);

        let field_name = {
            let frame = self.current_frame();
            let field_value = frame.closure.function.chunk.read_string(field_name_index);
            match field_value {
                Value::Object(obj) => match obj.as_ref() {
                    Object::String(s) => s.value.to_string(),
                    _ => {
                        self.runtime_error("Field name must be a string.");
                        return Some(Result::RuntimeError);
                    }
                },
                _ => {
                    self.runtime_error("Field name must be a string.");
                    return Some(Result::RuntimeError);
                }
            }
        };

        match &instance_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Instance(instance_ref) => {
                    let mut instance = instance_ref.borrow_mut();
                    // Instance fields are always fully populated from the struct's field
                    // list at construction, so this is equivalent to a struct.fields scan.
                    if !instance.fields.contains_key(&field_name) {
                        self.runtime_error(&format!("Undefined field '{}'.", field_name));
                        return Some(Result::RuntimeError);
                    }

                    instance.fields.insert(field_name, value.clone());

                    self.pop();
                    self.pop();
                    self.push(value);
                }
                _ => {
                    self.runtime_error("Only instances have fields.");
                    return Some(Result::RuntimeError);
                }
            },
            _ => {
                self.runtime_error("Only instances have fields.");
                return Some(Result::RuntimeError);
            }
        }

        let frame = self.current_frame_mut();
        frame.ip += bits.as_bytes();
        None
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_create_map(&mut self) -> Option<Result> {
        let count = {
            let frame = self.current_frame();
            frame.closure.function.chunk.read_u16(frame.ip + 1) as usize
        };

        let stack_len = self.stack.len();
        let pairs_start = stack_len - (count * 2);

        let mut map = IndexMap::with_capacity(count);
        for i in 0..count {
            let key_value = &self.stack[pairs_start + i];
            let value = &self.stack[pairs_start + count + i];

            let key = match Self::value_to_map_key(key_value) {
                Some(k) => k,
                None => {
                    self.runtime_error(&format!(
                        "Invalid map key type: {}. Only strings, numbers, and booleans can be used as map keys.",
                        key_value
                    ));
                    return Some(Result::RuntimeError);
                }
            };

            map.insert(key, value.clone());
        }

        self.stack.drain(pairs_start..);

        self.push(Value::new_map(map));

        let frame = self.current_frame_mut();
        frame.ip += 2;
        None
    }

    pub(in crate::vm) fn fn_create_array(&mut self) {
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
    pub(in crate::vm) fn fn_create_set(&mut self) -> Option<Result> {
        let count = {
            let frame = self.current_frame();
            frame.closure.function.chunk.read_u16(frame.ip + 1) as usize
        };

        let stack_len = self.stack.len();
        let elements_start = stack_len - count;

        let mut set = std::collections::BTreeSet::new();
        for i in 0..count {
            let element_value = &self.stack[elements_start + i];

            let key = match Self::value_to_map_key(element_value) {
                Some(k) => k,
                None => {
                    self.runtime_error(&format!(
                        "Invalid set element type: {}. Only strings, numbers, and booleans can be used as set elements.",
                        element_value
                    ));
                    return Some(Result::RuntimeError);
                }
            };

            set.insert(key);
        }

        self.stack.drain(elements_start..);

        self.push(Value::new_set(set));

        let frame = self.current_frame_mut();
        frame.ip += 2;
        None
    }

    pub(in crate::vm) fn fn_create_range(&mut self) -> Option<Result> {
        let inclusive = {
            let frame = self.current_frame();
            frame.closure.function.chunk.read_u8(frame.ip + 1) != 0
        };

        let end_value = self.pop();
        let start_value = self.pop();

        let start = match start_value {
            Value::Number(n) => n,
            _ => {
                self.runtime_error(&format!(
                    "Range start must be a number, got {}",
                    start_value
                ));
                return Some(Result::RuntimeError);
            }
        };

        let end = match end_value {
            Value::Number(n) => n,
            _ => {
                self.runtime_error(&format!("Range end must be a number, got {}", end_value));
                return Some(Result::RuntimeError);
            }
        };

        if start.fract() != 0.0 {
            self.runtime_error(&format!("Range start must be an integer, got {}", start));
            return Some(Result::RuntimeError);
        }

        if end.fract() != 0.0 {
            self.runtime_error(&format!("Range end must be an integer, got {}", end));
            return Some(Result::RuntimeError);
        }

        let start_int = start as i64;
        let end_int = end as i64;

        let elements: Vec<Value> = if inclusive {
            if start_int <= end_int {
                (start_int..=end_int)
                    .map(|i| Value::Number(i as f64))
                    .collect()
            } else {
                Vec::new()
            }
        } else if start_int < end_int {
            (start_int..end_int)
                .map(|i| Value::Number(i as f64))
                .collect()
        } else {
            Vec::new()
        };

        self.push(Value::new_array(elements));

        let frame = self.current_frame_mut();
        frame.ip += 1;

        None
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_index(&mut self) -> Option<Result> {
        let index_value = self.pop();
        let collection_value = self.pop();

        match &collection_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Map(map_ref) => {
                    // Convert index to MapKey
                    let key = match Self::value_to_map_key(&index_value) {
                        Some(k) => k,
                        None => {
                            self.runtime_error(&format!(
                                "Invalid map key type: {}. Only strings, numbers, and booleans can be used as map keys.",
                                index_value
                            ));
                            return Some(Result::RuntimeError);
                        }
                    };

                    let map = map_ref.borrow();
                    let result = map.get(&key).cloned().unwrap_or(Value::Nil);
                    self.push(result);
                    None
                }
                Object::Array(array_ref) => {
                    let index = match index_value {
                        Value::Number(n) => n as i32,
                        _ => {
                            self.runtime_error(&format!(
                                "Array index must be a number, got {}.",
                                index_value
                            ));
                            return Some(Result::RuntimeError);
                        }
                    };

                    let array = array_ref.borrow();
                    let len = array.len() as i32;

                    let actual_index = if index < 0 { len + index } else { index };

                    if actual_index < 0 || actual_index >= len {
                        self.runtime_error(&format!(
                            "Array index out of bounds: index {} (normalized: {}) on array of length {}.",
                            index, actual_index, len
                        ));
                        return Some(Result::RuntimeError);
                    }

                    let result = array[actual_index as usize].clone();
                    self.push(result);
                    None
                }
                _ => {
                    self.runtime_error(&format!(
                        "Only arrays and maps support index access, got {}.",
                        collection_value
                    ));
                    Some(Result::RuntimeError)
                }
            },
            _ => {
                self.runtime_error(&format!(
                    "Only arrays and maps support index access, got {}.",
                    collection_value
                ));
                Some(Result::RuntimeError)
            }
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_index(&mut self) -> Option<Result> {
        let value = self.pop();
        let index_value = self.pop();
        let collection_value = self.pop();

        match &collection_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Map(map_ref) => {
                    // Convert index to MapKey
                    let key = match Self::value_to_map_key(&index_value) {
                        Some(k) => k,
                        None => {
                            self.runtime_error(&format!(
                                "Invalid map key type: {}. Only strings, numbers, and booleans can be used as map keys.",
                                index_value
                            ));
                            return Some(Result::RuntimeError);
                        }
                    };

                    let mut map = map_ref.borrow_mut();
                    map.insert(key, value.clone());

                    self.push(value);
                    None
                }
                Object::Array(array_ref) => {
                    let index = match index_value {
                        Value::Number(n) => n as i32,
                        _ => {
                            self.runtime_error(&format!(
                                "Array index must be a number, got {}.",
                                index_value
                            ));
                            return Some(Result::RuntimeError);
                        }
                    };

                    let mut array = array_ref.borrow_mut();
                    let len = array.len() as i32;

                    let actual_index = if index < 0 { len + index } else { index };

                    if actual_index < 0 || actual_index >= len {
                        self.runtime_error(&format!(
                            "Array index out of bounds: index {} (normalized: {}) on array of length {}.",
                            index, actual_index, len
                        ));
                        return Some(Result::RuntimeError);
                    }

                    array[actual_index as usize] = value.clone();

                    self.push(value);
                    None
                }
                _ => {
                    self.runtime_error(&format!(
                        "Only arrays and maps support index assignment, got {}.",
                        collection_value
                    ));
                    Some(Result::RuntimeError)
                }
            },
            _ => {
                self.runtime_error(&format!(
                    "Only arrays and maps support index assignment, got {}.",
                    collection_value
                ));
                Some(Result::RuntimeError)
            }
        }
    }

    fn value_to_map_key(value: &Value) -> Option<crate::common::MapKey> {
        use crate::common::MapKey;
        use ordered_float::OrderedFloat;

        match value {
            Value::Object(obj) => match obj.as_ref() {
                Object::String(s) => Some(MapKey::String(Rc::clone(&s.value))),
                _ => None,
            },
            Value::Number(n) => Some(MapKey::Number(OrderedFloat(*n))),
            Value::Boolean(b) => Some(MapKey::Boolean(*b)),
            Value::Nil => None,
        }
    }

    /// GetIterator: Convert a collection to an iterator
    /// Pops collection from stack, pushes iterator onto iterator stack
    /// For arrays: iterate over elements directly
    /// For maps: iterate over keys
    /// For sets: convert to array and iterate
    #[inline(always)]
    pub(in crate::vm) fn fn_get_iterator(&mut self) -> Option<Result> {
        let collection = self.pop();

        let iterator_value = match &collection {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(_) => collection,
                Object::Map(map_ref) => {
                    let map = map_ref.borrow();
                    let keys: Vec<Value> = map
                        .keys()
                        .map(|k| match k {
                            crate::common::MapKey::String(s) => {
                                Value::Object(Rc::new(Object::String(crate::common::ObjString {
                                    value: Rc::clone(s),
                                })))
                            }
                            crate::common::MapKey::Number(n) => Value::Number(n.into_inner()),
                            crate::common::MapKey::Boolean(b) => Value::Boolean(*b),
                        })
                        .collect();

                    Value::new_array(keys)
                }
                Object::Set(set_ref) => {
                    let set = set_ref.borrow();
                    let elements: Vec<Value> = set
                        .iter()
                        .map(|k| match k {
                            crate::common::SetKey::String(s) => {
                                Value::Object(Rc::new(Object::String(crate::common::ObjString {
                                    value: Rc::clone(s),
                                })))
                            }
                            crate::common::SetKey::Number(n) => Value::Number(n.into_inner()),
                            crate::common::SetKey::Boolean(b) => Value::Boolean(*b),
                        })
                        .collect();

                    Value::new_array(elements)
                }
                _ => {
                    self.runtime_error(&format!(
                        "Cannot iterate over type: {}. Only arrays, maps, and sets are iterable.",
                        collection
                    ));
                    return Some(Result::RuntimeError);
                }
            },
            _ => {
                self.runtime_error(&format!(
                    "Cannot iterate over type: {}. Only arrays, maps, and sets are iterable.",
                    collection
                ));
                return Some(Result::RuntimeError);
            }
        };

        self.iterator_stack.push((0, iterator_value));
        None
    }

    /// IteratorDone: Check if iteration is complete
    /// Pushes false if done (no more elements), true if not done (more elements remain)
    /// This inverted logic allows JumpIfFalse to exit the loop when done
    #[inline(always)]
    pub(in crate::vm) fn fn_iterator_done(&mut self) -> Option<Result> {
        if let Some((index, collection)) = self.iterator_stack.last() {
            let has_more = match collection {
                Value::Object(obj) => match obj.as_ref() {
                    Object::Array(array_ref) => {
                        let array = array_ref.borrow();
                        *index < array.len()
                    }
                    _ => false,
                },
                _ => false,
            };

            self.push(boolean!(has_more));
            None
        } else {
            self.runtime_error("No iterator initialized");
            Some(Result::RuntimeError)
        }
    }

    /// IteratorNext: Get the next element from the iterator
    /// Pushes the next value onto the stack and advances the iterator
    /// When exiting a loop, pops the iterator from the iterator stack
    #[inline(always)]
    pub(in crate::vm) fn fn_iterator_next(&mut self) -> Option<Result> {
        let (value, new_index) = if let Some((index, collection)) = self.iterator_stack.last() {
            match collection {
                Value::Object(obj) => match obj.as_ref() {
                    Object::Array(array_ref) => {
                        let array = array_ref.borrow();
                        if *index < array.len() {
                            let value = array[*index].clone();
                            (Some(value), Some(*index + 1))
                        } else {
                            (None, None)
                        }
                    }
                    _ => (None, None),
                },
                _ => (None, None),
            }
        } else {
            (None, None)
        };

        match (value, new_index) {
            (Some(v), Some(idx)) => {
                if let Some((index, _)) = self.iterator_stack.last_mut() {
                    *index = idx;
                }
                self.push(v);
                None
            }
            (None, None) if self.iterator_stack.is_empty() => {
                self.runtime_error("No iterator initialized");
                Some(Result::RuntimeError)
            }
            _ => {
                self.runtime_error("Iterator exhausted or invalid state");
                Some(Result::RuntimeError)
            }
        }
    }

    /// Helper: Extract type name from a value for method dispatch
    fn get_type_name(&self, value: &Value) -> Option<TypeName> {
        match value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(_) => Some(TypeName::Static("Array")),
                Object::String(_) => Some(TypeName::Static("String")),
                Object::Map(_) => Some(TypeName::Static("Map")),
                Object::Set(_) => Some(TypeName::Static("Set")),
                Object::File(_) => Some(TypeName::Static("File")),
                Object::Instance(inst) => {
                    Some(TypeName::Struct(Rc::clone(&inst.borrow().r#struct)))
                }
                // The struct value itself (e.g. `Point` in `Point.origin()`)
                // dispatches static methods under the struct's own name.
                Object::Struct(r#struct) => Some(TypeName::Struct(Rc::clone(r#struct))),
                _ => None,
            },
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

    /// Helper: Look up native method by name from receiver type
    fn lookup_native_method_by_name(
        &mut self,
        arg_count: usize,
        callable: &Rc<ObjNativeFunction>,
    ) -> std::result::Result<&'static NativeCallable, String> {
        let args_start = self.stack.len() - arg_count - 1;
        let receiver = &self.stack[args_start];

        let type_name = match self.get_type_name(receiver) {
            None => return Err("Cannot determine type of receiver for method call".to_string()),
            Some(name) => name,
        };

        match crate::common::method_registry::get_native_method_by_name(
            type_name.as_str(),
            &callable.method_name,
        ) {
            None => Err(format!(
                "Unknown method '{}' for type {}",
                callable.method_name, type_name
            )),
            Some(callable) => Ok(callable),
        }
    }

    /// DefineMethod: pops the closure left on top of the stack by a
    /// preceding Closure op and registers it under (type name, method name).
    #[inline(always)]
    pub(in crate::vm) fn fn_define_method(&mut self) {
        let frame = self.current_frame_mut();
        let type_name = {
            let index = frame.closure.function.chunk.read_u32(frame.ip + 1) as usize;
            frame.closure.function.chunk.read_string(index)
        };
        let method_name = {
            let index = frame.closure.function.chunk.read_u32(frame.ip + 5) as usize;
            frame.closure.function.chunk.read_string(index)
        };
        frame.ip += 8;

        let closure_value = self.pop();
        let type_name = as_string!(type_name).value.to_string();
        let method_name = as_string!(method_name).value.to_string();
        match closure_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Closure(closure) => {
                    self.methods
                        .insert((type_name, method_name), Rc::clone(closure));
                }
                _ => unreachable!("DefineMethod expects a closure on top of the stack"),
            },
            _ => unreachable!("DefineMethod expects a closure on top of the stack"),
        }
    }
}
