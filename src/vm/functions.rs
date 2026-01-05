use crate::common::method_registry::NativeCallable;
use crate::common::{BitsSize, CallFrame, ObjInstance, ObjNativeFunction, ObjStruct, Value};
use crate::common::{ObjFunction, Object};
use crate::vm::Result;
use crate::vm::VirtualMachine;
use crate::{as_number, boolean, is_false_like, number, string};
use std::collections::HashMap;
use std::rc::Rc;

/// Registry index for the print() function (always at index 0)
const PRINT_METHOD_INDEX: u32 = 0;

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
            let string_index = frame.function.chunk.read_u32(frame.ip + 1) as usize;
            frame.function.chunk.read_string(string_index)
        };
        frame.ip += 4;
        self.push(string);
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_string2(&mut self) {
        let frame = self.current_frame_mut();
        let string = {
            let string_index = frame.function.chunk.read_u16(frame.ip + 1) as usize;
            frame.function.chunk.read_string(string_index)
        };
        frame.ip += 2;
        self.push(string);
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_string(&mut self) {
        let frame = self.current_frame_mut();
        let string = {
            let string_index = frame.function.chunk.read_u8(frame.ip + 1) as usize;
            frame.function.chunk.read_string(string_index)
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
        let arg_count = {
            let frame = self.current_frame();
            frame.function.chunk.read_u8(frame.ip + 1) as usize
        };

        let frame = self.current_frame_mut();
        frame.ip += 2; // Skip CALL opcode and arg_count byte

        // Get the callable from the stack
        let callable_value = self.peek(0);

        let result = match &callable_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Function(callable) => return self.call_function(arg_count, &callable),
                Object::Struct(r#struct) => return self.instantiate_struct(arg_count, r#struct),
                Object::NativeFunction(callable) => {
                    match self.call_native_function(arg_count, callable) {
                        Ok(value) => value,
                        Err(error) => {
                            self.runtime_error(&error);
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

    fn call_native_function(
        &mut self,
        arg_count: usize,
        callable: &Rc<ObjNativeFunction>,
    ) -> std::result::Result<Value, String> {
        let native_callable_result = if callable.method_index != u32::MAX {
            self.lookup_native_method_by_index(callable)
        } else {
            self.lookup_native_method_by_name(arg_count, callable)
        };

        let native_callable = native_callable_result?;

        let stack_len = self.stack.len();
        let args_start = stack_len - arg_count - 1;
        let args_end = stack_len - 1;
        let args: Vec<Value> = self.stack[args_start..args_end].to_vec();

        #[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
        {
            if callable.method_index == PRINT_METHOD_INDEX {
                self.print_to_vm_buffer(arg_count);
            }
        }
        native_callable.function()(&args)
    }

    #[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
    fn print_to_vm_buffer(&mut self, arg_count: usize) {
        let stack_len = self.stack.len();
        let args_start = stack_len - arg_count - 1;
        let args_end = stack_len - 1;
        let args = &self.stack[args_start..args_end];
        if !args.is_empty() {
            use std::fmt::Write;
            writeln!(self.string_buffer, "{}", args[0]).ok();
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

    fn call_function(&mut self, arg_count: usize, func: &&Rc<ObjFunction>) -> Option<Result> {
        // Validate argument count with default parameters support
        let min_arity = func.min_arity as usize;
        let max_arity = func.arity as usize;

        if arg_count < min_arity || arg_count > max_arity {
            if min_arity == max_arity {
                self.runtime_error(&format!(
                    "Expected {} arguments but got {}.",
                    max_arity, arg_count
                ));
            } else {
                self.runtime_error(&format!(
                    "Expected {}-{} arguments but got {}.",
                    min_arity, max_arity, arg_count
                ));
            }
            return Some(Result::RuntimeError);
        }

        // Push default values for missing arguments
        // The function object is at the top of the stack, so we need to insert defaults before it
        let missing_count = max_arity - arg_count;
        if missing_count > 0 {
            // Remove function object temporarily
            let func_obj = self.pop();

            // Push default values for missing parameters
            for i in arg_count..max_arity {
                if let Some(Some(default_value)) = func.defaults.get(i) {
                    self.push(default_value.clone());
                } else {
                    // This shouldn't happen if codegen is correct, but fallback to nil
                    self.push(Value::Nil);
                }
            }

            // Push function object back
            self.push(func_obj);
        }

        // Calculate slot_start for unified calling convention [args..., func]
        // After pushing defaults, we now have max_arity arguments on the stack
        // Stack layout: [...previous..., arg0, arg1, ..., argN, defaults..., func_obj]
        // slot_start should point just BEFORE the first argument
        // So: slot_start = current_len - max_arity - 1 (for func) - 1 (to go before first arg)
        let slot_start = (self.stack.len() - max_arity - 1 - 1) as isize;

        let new_frame = CallFrame {
            function: Rc::clone(func),
            ip: 0,
            slot_start,
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
        self.call_frames.pop();

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
    pub(in crate::vm) fn fn_less(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(boolean!(as_number!(a) < as_number!(b)));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_greater(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(boolean!(as_number!(a) > as_number!(b)));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_equal(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(boolean!(a == b));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_divide(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::Number(as_number!(a) / as_number!(b)));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_floor_divide(&mut self) {
        let b = self.pop();
        let a = self.pop();
        let result = (as_number!(a) / as_number!(b)).floor();
        self.push(Value::Number(result));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_modulo(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::Number(as_number!(a) % as_number!(b)));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_exponent(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::Number(as_number!(a).powf(as_number!(b))));
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
    pub(in crate::vm) fn fn_bitwise_and(&mut self) {
        let b = self.pop();
        let a = self.pop();
        let result = Self::to_integer(as_number!(a)) & Self::to_integer(as_number!(b));
        self.push(Value::Number(result as f64));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_bitwise_or(&mut self) {
        let b = self.pop();
        let a = self.pop();
        let result = Self::to_integer(as_number!(a)) | Self::to_integer(as_number!(b));
        self.push(Value::Number(result as f64));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_bitwise_xor(&mut self) {
        let b = self.pop();
        let a = self.pop();
        let result = Self::to_integer(as_number!(a)) ^ Self::to_integer(as_number!(b));
        self.push(Value::Number(result as f64));
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
    pub(in crate::vm) fn fn_left_shift(&mut self) {
        let b = self.pop();
        let a = self.pop();
        let shift_amount = (Self::to_integer(as_number!(b)) & 0x3F) as u32; // Mask to 6 bits (0-63)
        let result = Self::to_integer(as_number!(a)) << shift_amount;
        self.push(Value::Number(result as f64));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_right_shift(&mut self) {
        let b = self.pop();
        let a = self.pop();
        let shift_amount = (Self::to_integer(as_number!(b)) & 0x3F) as u32; // Mask to 6 bits (0-63)
        let result = Self::to_integer(as_number!(a)) >> shift_amount; // Arithmetic right shift
        self.push(Value::Number(result as f64));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_multiply(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::Number(as_number!(a) * as_number!(b)));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_subtract(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::Number(as_number!(a) - as_number!(b)));
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
            let constant_index = frame.function.chunk.read_u32(frame.ip + 1) as usize;
            frame.function.chunk.read_constant(constant_index)
        };
        frame.ip += 4;
        self.push(constant);
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_constant2(&mut self) {
        let frame = self.current_frame_mut();
        let constant = {
            let constant_index = frame.function.chunk.read_u16(frame.ip + 1) as usize;
            frame.function.chunk.read_constant(constant_index)
        };
        frame.ip += 2;
        self.push(constant);
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_constant(&mut self) {
        let frame = self.current_frame_mut();
        let constant = {
            let constant_index = frame.function.chunk.read_u8(frame.ip + 1) as usize;
            frame.function.chunk.read_constant(constant_index)
        };
        frame.ip += 1;
        self.push(constant);
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_local(&mut self, bits: BitsSize) {
        let index = self.read_bits(&bits);
        let frame = self.current_frame_mut();
        // For functions: slot_start points to function object, args start at slot_start + 1
        // For script: slot_start = -1, so locals start at 0
        // locals (params) are indexed from 0, so param 0 is at slot_start + 1
        let absolute_index = (frame.slot_start + 1 + index as isize) as usize;
        frame.ip += bits.as_bytes();
        self.stack[absolute_index] = self.peek(0);
    }

    fn read_bits(&mut self, bits: &BitsSize) -> usize {
        let frame = self.current_frame();
        match bits {
            BitsSize::Eight => frame.function.chunk.read_u8(frame.ip + 1) as usize,
            BitsSize::Sixteen => frame.function.chunk.read_u16(frame.ip + 1) as usize,
            BitsSize::ThirtyTwo => frame.function.chunk.read_u32(frame.ip + 1) as usize,
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_local(&mut self, bits: BitsSize) {
        let index = self.read_bits(&bits);
        let frame = self.current_frame_mut();
        let absolute_index = (frame.slot_start + 1 + index as isize) as usize;
        frame.ip += bits.as_bytes();
        self.push(self.stack[absolute_index].clone());
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_jump_if_false(&mut self) {
        let peeked_value = self.peek(0);
        let frame = self.current_frame_mut();
        let offset = frame.function.chunk.read_u32(frame.ip + 1);
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
        let offset = frame.function.chunk.read_u32(frame.ip + 1);
        frame.ip += 4;
        frame.ip += offset as usize;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_loop(&mut self) {
        let frame = self.current_frame_mut();
        let offset = frame.function.chunk.read_u32(frame.ip + 1);
        frame.ip += 4;
        frame.ip -= offset as usize;
    }

    pub(in crate::vm) fn fn_get_builtin(&mut self, bits: BitsSize) {
        let index = self.read_bits(&bits);
        if let Some(entry) = self.builtin.get_index(index) {
            self.push(entry.1.clone());
        } else {
            self.runtime_error(&format!("Built-in global at index {} not found", index));
        }
        let frame = self.current_frame_mut();
        frame.ip += bits.as_bytes();
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_global(&mut self, bits: BitsSize) {
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
            return;
        }

        self.push(self.stack[absolute_index].clone());
        let frame = self.current_frame_mut();
        frame.ip += bits.as_bytes()
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_global(&mut self, bits: BitsSize) {
        let index = self.read_bits(&bits);
        // Global variables are always in the script frame (first frame)
        // Script frame has slot_start = -1, so globals start at index 0
        let script_frame = &self.call_frames[0];
        let absolute_index = (script_frame.slot_start + 1 + index as isize) as usize;
        self.stack[absolute_index] = self.peek(0);
        let frame = self.current_frame_mut();
        frame.ip += bits.as_bytes();
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_field(&mut self, bits: BitsSize) {
        let field_name_index = self.read_bits(&bits);
        let instance_value = self.peek(0);

        let field_name = {
            let frame = self.current_frame();
            let field_value = frame.function.chunk.read_string(field_name_index);
            match field_value {
                Value::Object(obj) => match obj.as_ref() {
                    Object::String(s) => s.value.to_string(),
                    _ => {
                        self.runtime_error("Field name must be a string.");
                        return;
                    }
                },
                _ => {
                    self.runtime_error("Field name must be a string.");
                    return;
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
                        return;
                    }
                }
                _ => {
                    self.runtime_error("Only instances have fields.");
                    return;
                }
            },
            _ => {
                self.runtime_error("Only instances have fields.");
                return;
            }
        }

        let frame = self.current_frame_mut();
        frame.ip += bits.as_bytes();
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_field(&mut self, bits: BitsSize) {
        let field_name_index = self.read_bits(&bits);
        let value = self.peek(0);
        let instance_value = self.peek(1);

        let field_name = {
            let frame = self.current_frame();
            let field_value = frame.function.chunk.read_string(field_name_index);
            match field_value {
                Value::Object(obj) => match obj.as_ref() {
                    Object::String(s) => s.value.to_string(),
                    _ => {
                        self.runtime_error("Field name must be a string.");
                        return;
                    }
                },
                _ => {
                    self.runtime_error("Field name must be a string.");
                    return;
                }
            }
        };

        match &instance_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Instance(instance_ref) => {
                    let mut instance = instance_ref.borrow_mut();
                    if !instance.r#struct.fields.contains(&field_name) {
                        self.runtime_error(&format!("Undefined field '{}'.", field_name));
                        return;
                    }

                    instance.fields.insert(field_name, value.clone());

                    self.pop();
                    self.pop();
                    self.push(value);
                }
                _ => {
                    self.runtime_error("Only instances have fields.");
                    return;
                }
            },
            _ => {
                self.runtime_error("Only instances have fields.");
                return;
            }
        }

        let frame = self.current_frame_mut();
        frame.ip += bits.as_bytes();
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_create_map(&mut self) {
        let count = {
            let frame = self.current_frame();
            frame.function.chunk.read_u8(frame.ip + 1) as usize
        };

        let stack_len = self.stack.len();
        let pairs_start = stack_len - (count * 2);

        let mut map = HashMap::with_capacity(count);
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
                    return;
                }
            };

            map.insert(key, value.clone());
        }

        self.stack.drain(pairs_start..);

        self.push(Value::new_map(map));

        let frame = self.current_frame_mut();
        frame.ip += 1;
    }

    pub(in crate::vm) fn fn_create_array(&mut self) {
        let count = {
            let frame = self.current_frame();
            frame.function.chunk.read_u16(frame.ip + 1) as usize
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
    pub(in crate::vm) fn fn_create_set(&mut self) {
        let count = {
            let frame = self.current_frame();
            frame.function.chunk.read_u8(frame.ip + 1) as usize
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
                    return;
                }
            };

            set.insert(key);
        }

        self.stack.drain(elements_start..);

        self.push(Value::new_set(set));

        let frame = self.current_frame_mut();
        frame.ip += 1;
    }

    pub(in crate::vm) fn fn_create_range(&mut self) -> Option<Result> {
        let inclusive = {
            let frame = self.current_frame();
            frame.function.chunk.read_u8(frame.ip + 1) != 0
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
    pub(in crate::vm) fn fn_get_index(&mut self) {
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
                            return;
                        }
                    };

                    let map = map_ref.borrow();
                    let result = map.get(&key).cloned().unwrap_or(Value::Nil);
                    self.push(result);
                }
                Object::Array(array_ref) => {
                    let index = match index_value {
                        Value::Number(n) => n as i32,
                        _ => {
                            self.runtime_error(&format!(
                                "Array index must be a number, got {}.",
                                index_value
                            ));
                            return;
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
                        return;
                    }

                    let result = array[actual_index as usize].clone();
                    self.push(result);
                }
                _ => {
                    self.runtime_error(&format!(
                        "Only arrays and maps support index access, got {}.",
                        collection_value
                    ));
                }
            },
            _ => {
                self.runtime_error(&format!(
                    "Only arrays and maps support index access, got {}.",
                    collection_value
                ));
            }
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_index(&mut self) {
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
                            return;
                        }
                    };

                    let mut map = map_ref.borrow_mut();
                    map.insert(key, value.clone());

                    self.push(value);
                }
                Object::Array(array_ref) => {
                    let index = match index_value {
                        Value::Number(n) => n as i32,
                        _ => {
                            self.runtime_error(&format!(
                                "Array index must be a number, got {}.",
                                index_value
                            ));
                            return;
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
                        return;
                    }

                    array[actual_index as usize] = value.clone();

                    self.push(value);
                }
                _ => {
                    self.runtime_error(&format!(
                        "Only arrays and maps support index assignment, got {}.",
                        collection_value
                    ));
                }
            },
            _ => {
                self.runtime_error(&format!(
                    "Only arrays and maps support index assignment, got {}.",
                    collection_value
                ));
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
    pub(in crate::vm) fn fn_iterator_done(&mut self) {
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
        } else {
            self.runtime_error("No iterator initialized");
            self.push(boolean!(false));
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
    fn get_type_name(&self, value: &Value) -> Option<String> {
        match value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(_) => Some("Array".to_string()),
                Object::String(_) => Some("String".to_string()),
                Object::Map(_) => Some("Map".to_string()),
                Object::Set(_) => Some("Set".to_string()),
                Object::File(_) => Some("File".to_string()),
                Object::Instance(inst) => Some(inst.borrow().r#struct.name.clone()),
                _ => None,
            },
            Value::Number(_) => Some("Number".to_string()),
            Value::Boolean(_) => Some("Boolean".to_string()),
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
                &callable.method_name, type_name
            )),
            Some(callable) => Ok(callable),
        }
    }
}
