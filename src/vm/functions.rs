use crate::common::constants::VARIADIC_ARITY;
use crate::common::{BitsSize, CallFrame, ObjInstance, ObjStruct, Value, ObjNativeFunction};
use crate::common::{ObjFunction, Object};
use crate::vm::Result;
use crate::vm::VirtualMachine;
use crate::{as_number, boolean, is_false_like, number, string};
use std::collections::HashMap;
use std::rc::Rc;

impl VirtualMachine {
    #[inline(always)]
    pub(in crate::vm) fn fn_print(&mut self) {
        let value = self.pop();

        #[cfg(not(target_arch = "wasm32"))]
        println!("{}", value);

        #[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
        {
            self.string_buffer.push_str(value.to_string().as_str());
            self.string_buffer.push('\n');
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_to_string(&mut self) {
        let value = self.pop();
        let string_value = string!(value.to_string());
        self.push(string_value);
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_string4(&mut self) {
        let string = {
            let frame = self.call_frames.last().unwrap();
            let string_index = frame.function.bloq.read_u32(frame.ip + 1) as usize;
            frame.function.bloq.read_string(string_index)
        };
        self.push(string);
        self.call_frames.last_mut().unwrap().ip += 4;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_string2(&mut self) {
        let string = {
            let frame = self.call_frames.last().unwrap();
            let string_index = frame.function.bloq.read_u16(frame.ip + 1) as usize;
            frame.function.bloq.read_string(string_index)
        };
        self.push(string);
        self.call_frames.last_mut().unwrap().ip += 2;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_string(&mut self) {
        let string = {
            let frame = self.call_frames.last().unwrap();
            let string_index = frame.function.bloq.read_u8(frame.ip + 1) as usize;
            frame.function.bloq.read_string(string_index)
        };
        self.push(string);
        self.call_frames.last_mut().unwrap().ip += 1;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_not(&mut self) {
        let value = self.pop();
        self.push(boolean!(is_false_like!(value)));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_call(&mut self) -> Option<Result> {
        let frame = self.call_frames.last().unwrap();
        let arg_count = frame.function.bloq.read_u8(frame.ip + 1) as usize;

        // Get the callable from the stack (it's at position -arg_count - 1)
        let callable_value = self.peek(arg_count);

        match &callable_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Function(func) => {
                    if let Some(value) = self.call_function(arg_count, &func) {
                        return Some(value);
                    }
                }
                Object::NativeFunction(native_fn) => {
                    if let Some(value) = self.call_native_function(arg_count, native_fn) {
                        return Some(value);
                    }
                }
                Object::Struct(instance) => {
                    if let Some(value) = self.instantiate_struct(arg_count, instance) {
                        return Some(value);
                    }
                }
                _ => {
                    self.runtime_error("Can only call functions and structs.");
                    return Some(Result::RuntimeError);
                }
            },
            _ => {
                self.runtime_error("Can only call functions and structs.");
                return Some(Result::RuntimeError);
            }
        }
        None
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

        // Create instance with fields
        let field_count = r#struct.fields.len();
        let mut fields = HashMap::with_capacity(field_count);
        let stack_len = self.stack.len();
        let stack_slice = &self.stack[stack_len - arg_count..stack_len];
        for (field_name, value) in r#struct.fields.iter().zip(stack_slice.iter()) {
            fields.insert(field_name.clone(), value.clone()); // If Value is not Copy
        }

        let instance = ObjInstance {
            r#struct: Rc::clone(r#struct),
            fields,
        };

        // Pop arguments and struct definition
        let n = arg_count + 1;
        let start = self.stack.len().saturating_sub(n);
        self.stack.drain(start..);

        // Push instance
        self.push(Value::new_object(instance));

        // Increment IP to skip Call opcode and arg count
        let current_frame = self.call_frames.last_mut().unwrap();
        current_frame.ip += 2;
        None
    }

    fn call_function(&mut self, arg_count: usize, func: &&Rc<ObjFunction>) -> Option<Result> {
        // Check arity
        if arg_count != func.arity as usize {
            self.runtime_error(&format!(
                "Expected {} arguments but got {}.",
                func.arity, arg_count
            ));
            return Some(Result::RuntimeError);
        }

        // Calculate slot_start: current stack size - arg_count - 1 (for the function itself)
        let slot_start = (self.stack.len() - arg_count - 1) as isize;

        // Create a new call frame
        let new_frame = CallFrame {
            function: Rc::clone(func),
            ip: 0,
            slot_start,
        };

        // Increment the current frame's IP before pushing the new frame
        // to skip both the Call opcode and the argument count byte when we return
        let current_frame = self.call_frames.last_mut().unwrap();
        current_frame.ip += 2;

        self.call_frames.push(new_frame);
        None
    }

    fn call_native_function(&mut self, arg_count: usize, native_fn: &ObjNativeFunction) -> Option<Result> {
        // Check arity (VARIADIC_ARITY means variadic function - any number of args allowed)
        if native_fn.arity != VARIADIC_ARITY && arg_count != native_fn.arity as usize {
            self.runtime_error(&format!(
                "Expected {} arguments but got {}.",
                native_fn.arity, arg_count
            ));
            return Some(Result::RuntimeError);
        }

        // Get arguments from the stack without copying
        // Arguments are at stack positions: [stack.len() - arg_count .. stack.len()]
        let stack_len = self.stack.len();
        let args_start = stack_len - arg_count;

        // SAFETY: We create a slice from raw parts to avoid Vec allocation.
        // This is safe because:
        // 1. The pointer is valid for the duration of the slice
        // 2. Native functions only read from args, they don't modify these stack positions
        // 3. The slice lifetime is limited to this function call
        // 4. We pop these arguments immediately after the call
        let args: &[Value] = unsafe {
            std::slice::from_raw_parts(
                self.stack.as_ptr().add(args_start),
                arg_count
            )
        };

        // Call the native function
        let result = (native_fn.function)(self, args);

        // Pop arguments and the native function object from the stack
        let n = arg_count + 1;
        let start = self.stack.len().saturating_sub(n);
        self.stack.drain(start..);

        // Handle the result
        match result {
            Ok(value) => {
                // Push the return value onto the stack
                self.push(value);

                // Increment IP to skip Call opcode and arg count
                let current_frame = self.call_frames.last_mut().unwrap();
                current_frame.ip += 2;

                None
            }
            Err(error_msg) => {
                self.runtime_error(&error_msg);
                Some(Result::RuntimeError)
            }
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_return(&mut self) -> Option<Result> {
        // Get the return value (top of stack)
        let return_value = self.pop();

        // Pop the current call frame
        let frame = self.call_frames.pop().unwrap();

        // If this was the last frame, we're done
        if self.call_frames.is_empty() {
            // Push the return value back for the script/test to access
            self.push(return_value);
            return Some(Result::Ok);
        }

        // Clear the stack back to the slot_start (removing arguments and locals)
        self.stack.truncate(frame.slot_start as usize);

        // Push the return value
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
        // match on ObjString
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
        let constant = {
            let frame = self.call_frames.last().unwrap();
            let constant_index = frame.function.bloq.read_u32(frame.ip + 1) as usize;
            frame.function.bloq.read_constant(constant_index)
        };
        self.push(constant);
        self.call_frames.last_mut().unwrap().ip += 4;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_constant2(&mut self) {
        let constant = {
            let frame = self.call_frames.last().unwrap();
            let constant_index = frame.function.bloq.read_u16(frame.ip + 1) as usize;
            frame.function.bloq.read_constant(constant_index)
        };
        self.push(constant);
        self.call_frames.last_mut().unwrap().ip += 2;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_constant(&mut self) {
        let constant = {
            let frame = self.call_frames.last().unwrap();
            let constant_index = frame.function.bloq.read_u8(frame.ip + 1) as usize;
            frame.function.bloq.read_constant(constant_index)
        };
        self.push(constant);
        self.call_frames.last_mut().unwrap().ip += 1;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_local(&mut self, bits: BitsSize) {
        let index = self.read_bits(&bits);
        let frame = self.call_frames.last().unwrap();
        // For functions: slot_start points to function object, args start at slot_start + 1
        // For script: slot_start = -1, so locals start at 0
        // locals (params) are indexed from 0, so param 0 is at slot_start + 1
        let absolute_index = (frame.slot_start + 1 + index as isize) as usize;
        self.stack[absolute_index] = self.peek(0);
        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += bits.as_bytes()
    }

    fn read_bits(&mut self, bits: &BitsSize) -> usize {
        let frame = self.call_frames.last().unwrap();
        match bits {
            BitsSize::Eight => frame.function.bloq.read_u8(frame.ip + 1) as usize,
            BitsSize::Sixteen => frame.function.bloq.read_u16(frame.ip + 1) as usize,
            BitsSize::ThirtyTwo => frame.function.bloq.read_u32(frame.ip + 1) as usize,
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_local(&mut self, bits: BitsSize) {
        let index = self.read_bits(&bits);
        let frame = self.call_frames.last().unwrap();
        let absolute_index = (frame.slot_start + 1 + index as isize) as usize;
        self.push(self.stack[absolute_index].clone());
        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += bits.as_bytes()
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_jump_if_false(&mut self) {
        let offset = self
            .call_frames
            .last()
            .unwrap()
            .function
            .bloq
            .read_u32(self.call_frames.last().unwrap().ip + 1);
        self.call_frames.last_mut().unwrap().ip += 4;
        if is_false_like!(self.peek(0)) {
            // Don't pop! Leave the value on the stack for logical operators
            // The caller is responsible for popping if needed (e.g., in if statements)
            self.call_frames.last_mut().unwrap().ip += offset as usize;
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_jump(&mut self) {
        let offset = self
            .call_frames
            .last()
            .unwrap()
            .function
            .bloq
            .read_u32(self.call_frames.last().unwrap().ip + 1);
        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += 4;
        frame.ip += offset as usize;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_loop(&mut self) {
        let offset = self
            .call_frames
            .last()
            .unwrap()
            .function
            .bloq
            .read_u32(self.call_frames.last().unwrap().ip + 1);
        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += 4;
        frame.ip -= offset as usize;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_global(&mut self, bits: BitsSize) {
        let index = self.read_bits(&bits);

        // Check if this is a built-in global using sentinel values
        if index == u32::MAX as usize {
            // This is a request for Math from the globals HashMap
            if let Some(value) = self.globals.get("Math") {
                self.push(value.clone());
                let frame = self.call_frames.last_mut().unwrap();
                frame.ip += bits.as_bytes();
                return;
            }
            // If Math is not found, this is an internal error
            self.runtime_error("Built-in global 'Math' not found");
            return;
        }
        if index == (u32::MAX - 1) as usize {
            // This is a request for File from the globals HashMap
            if let Some(value) = self.globals.get("File") {
                self.push(value.clone());
                let frame = self.call_frames.last_mut().unwrap();
                frame.ip += bits.as_bytes();
                return;
            }
            // If File is not found, this is an internal error
            self.runtime_error("Built-in global 'File' not found");
            return;
        }
        if index == (u32::MAX - 2) as usize {
            // This is a request for args from the globals HashMap
            if let Some(value) = self.globals.get("args") {
                self.push(value.clone());
                let frame = self.call_frames.last_mut().unwrap();
                frame.ip += bits.as_bytes();
                return;
            }
            // If args is not found, this is an internal error
            self.runtime_error("Built-in global 'args' not found");
            return;
        }

        // Regular global variables are in the script frame
        // Script frame has slot_start = -1, so globals start at index 0
        let script_frame = &self.call_frames[0];
        let absolute_index = (script_frame.slot_start + 1 + index as isize) as usize;

        // Make sure we don't go out of bounds
        if absolute_index >= self.stack.len() {
            self.runtime_error(&format!("Global variable index {} out of bounds (stack size: {})", absolute_index, self.stack.len()));
            return;
        }

        self.push(self.stack[absolute_index].clone());
        let frame = self.call_frames.last_mut().unwrap();
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
        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += bits.as_bytes();
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_field(&mut self, bits: BitsSize) {
        let field_name_index = self.read_bits(&bits);
        let instance_value = self.peek(0);

        // Read the field name from strings
        let field_name = {
            let frame = self.call_frames.last().unwrap();
            let field_value = frame.function.bloq.read_string(field_name_index);
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
                        self.pop(); // Pop instance
                        self.push(value); // Push field value
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

        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += bits.as_bytes();
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_call_method(&mut self, bits: BitsSize) -> Option<Result> {
        // Calculate index size and IP increment based on variant
        let index_size = match bits {
            BitsSize::Eight => 1,
            BitsSize::Sixteen => 2,
            BitsSize::ThirtyTwo => 4,
        };
        let ip_increment = 1 + 1 + index_size; // opcode + arg_count + method_index

        // Read arg count and method name index
        let frame = self.call_frames.last().unwrap();
        let arg_count = frame.function.bloq.read_u8(frame.ip + 1) as usize;
        let method_name_index = match bits {
            BitsSize::Eight => frame.function.bloq.read_u8(frame.ip + 2) as usize,
            BitsSize::Sixteen => frame.function.bloq.read_u16(frame.ip + 2) as usize,
            BitsSize::ThirtyTwo => frame.function.bloq.read_u32(frame.ip + 2) as usize,
        };

        // Read the method name from strings
        let method_name = {
            let frame = self.call_frames.last().unwrap();
            let method_value = frame.function.bloq.read_string(method_name_index);
            match method_value {
                Value::Object(obj) => match obj.as_ref() {
                    Object::String(s) => s.value.to_string(),
                    _ => {
                        self.runtime_error("Method name must be a string.");
                        return Some(Result::RuntimeError);
                    }
                },
                _ => {
                    self.runtime_error("Method name must be a string.");
                    return Some(Result::RuntimeError);
                }
            }
        };

        // Get the receiver (object) from the stack
        // Stack layout: [receiver, arg1, arg2, ...]
        let receiver = self.peek(arg_count);

        // Check if receiver is an instance with the method as a function field
        // This allows Math.abs(x) where abs is a function field in the Math instance
        if let Value::Object(obj) = &receiver {
            if let Object::Instance(instance_ref) = obj.as_ref() {
                let instance = instance_ref.borrow();
                if let Some(field_value) = instance.fields.get(&method_name) {
                    // The "method" is actually a function field - call it
                    let function_value = field_value.clone();
                    drop(instance); // Drop the borrow before calling

                    // Handle different function types
                    match &function_value {
                        Value::Object(func_obj) => match func_obj.as_ref() {
                            Object::Function(func) => {
                                // Call user-defined function
                                return if let Some(result) = self.call_function(arg_count, &func) {
                                    Some(result)
                                } else {
                                    // Increment IP for CallMethod (opcode + arg_count + method_name_index)
                                    let current_frame = self.call_frames.last_mut().unwrap();
                                    current_frame.ip += ip_increment;
                                    None
                                };
                            }
                            Object::NativeFunction(native_fn) => {
                                // Get arguments from the stack without copying (receiver + args)
                                // Stack: [receiver, arg1, arg2, ...]
                                let stack_len = self.stack.len();
                                let receiver_index = stack_len - arg_count - 1;
                                let args_start = receiver_index + 1;

                                // SAFETY: Create slice from raw parts to avoid Vec allocation.
                                // Safe because native functions only read args, not modify stack.
                                let args: &[Value] = unsafe {
                                    std::slice::from_raw_parts(
                                        self.stack.as_ptr().add(args_start),
                                        arg_count
                                    )
                                };

                                // Call the native function
                                let result = (native_fn.function)(self, args);

                                // Pop receiver and arguments from the stack
                                let n = arg_count + 1;
                                let start = self.stack.len().saturating_sub(n);
                                self.stack.drain(start..);

                                // Handle the result
                                return match result {
                                    Ok(value) => {
                                        // Push the return value onto the stack
                                        self.push(value);

                                        // Increment IP for CallMethod (opcode + arg_count + method_name_index)
                                        let current_frame = self.call_frames.last_mut().unwrap();
                                        current_frame.ip += ip_increment;

                                        None
                                    }
                                    Err(error_msg) => {
                                        self.runtime_error(&error_msg);
                                        Some(Result::RuntimeError)
                                    }
                                };
                            }
                            _ => {
                                self.runtime_error("Field is not a callable function");
                                return Some(Result::RuntimeError);
                            }
                        },
                        _ => {
                            self.runtime_error("Field is not a callable function");
                            return Some(Result::RuntimeError);
                        }
                    }
                }
            }
        }

        // Determine the type of the receiver for native method lookup
        let type_name = match &receiver {
            Value::Number(_) => "Number".to_string(),
            Value::Boolean(_) => "Boolean".to_string(),
            Value::Object(obj) => match obj.as_ref() {
                Object::String(_) => "String".to_string(),
                Object::Array(_) => "Array".to_string(),
                Object::Map(_) => "Map".to_string(),
                Object::Set(_) => "Set".to_string(),
                Object::File(_) => "File".to_string(),
                Object::Instance(instance_ref) => {
                    let instance = instance_ref.borrow();
                    instance.r#struct.name.clone()
                }
                _ => {
                    self.runtime_error(&format!("Type does not support method calls: {:?}", receiver));
                    return Some(Result::RuntimeError);
                }
            },
            _ => {
                self.runtime_error(&format!("Cannot call methods on primitive type: {:?}", receiver));
                return Some(Result::RuntimeError);
            }
        };

        // Look up the native method
        let native_fn = match VirtualMachine::get_native_method(&type_name, &method_name) {
            Some(f) => f,
            None => {
                self.runtime_error(&format!(
                    "Undefined method '{}' for type '{}'",
                    method_name, type_name
                ));
                return Some(Result::RuntimeError);
            }
        };

        // Get arguments from the stack without copying (receiver + args)
        // Stack: [receiver, arg1, arg2, ...]
        let stack_len = self.stack.len();
        let receiver_index = stack_len - arg_count - 1;
        let args_count = arg_count + 1; // Include receiver as args[0]

        // SAFETY: Create slice from raw parts to avoid Vec allocation.
        // Safe because native methods only read args, not modify stack.
        let args: &[Value] = unsafe {
            std::slice::from_raw_parts(
                self.stack.as_ptr().add(receiver_index),
                args_count
            )
        };

        // Call the native method
        let result = native_fn(self, args);

        // Pop receiver and arguments from the stack
        let n = arg_count + 1;
        let start = self.stack.len().saturating_sub(n);
        self.stack.drain(start..);

        // Handle the result
        match result {
            Ok(value) => {
                // Push the return value onto the stack
                self.push(value);

                // Increment IP to skip CallMethod opcode, arg count, and method name index
                let current_frame = self.call_frames.last_mut().unwrap();
                current_frame.ip += ip_increment;

                None
            }
            Err(error_msg) => {
                self.runtime_error(&error_msg);
                Some(Result::RuntimeError)
            }
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_field(&mut self, bits: BitsSize) {
        let field_name_index = self.read_bits(&bits);
        let value = self.peek(0); // Value to set
        let instance_value = self.peek(1); // Instance

        // Read the field name from strings
        let field_name = {
            let frame = self.call_frames.last().unwrap();
            let field_value = frame.function.bloq.read_string(field_name_index);
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
                    // Verify field exists in struct definition
                    if !instance.r#struct.fields.contains(&field_name) {
                        self.runtime_error(&format!("Undefined field '{}'.", field_name));
                        return;
                    }

                    instance.fields.insert(field_name, value.clone());

                    // Pop value and instance, push value back (assignment expression returns the value)
                    self.pop(); // Pop value
                    self.pop(); // Pop instance
                    self.push(value); // Push value back
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

        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += bits.as_bytes();
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_create_map(&mut self) {
        // Read the count of entries from bytecode
        let count = {
            let frame = self.call_frames.last().unwrap();
            frame.function.bloq.read_u8(frame.ip + 1) as usize
        };

        // Pop key-value pairs from stack and build the map
        // Stack layout from compiler: [key1, key2, ..., keyN, value1, value2, ..., valueN]
        let stack_len = self.stack.len();
        let pairs_start = stack_len - (count * 2);

        let mut map = HashMap::with_capacity(count);
        for i in 0..count {
            // Keys are in the first half, values in the second half
            let key_value = &self.stack[pairs_start + i];
            let value = &self.stack[pairs_start + count + i];

            // Convert Value to MapKey
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

        // Pop all key-value pairs from stack
        self.stack.drain(pairs_start..);

        // Push the new map onto the stack
        self.push(Value::new_map(map));

        // Increment IP to skip the count byte
        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += 1;
    }

    pub(in crate::vm) fn fn_create_array(&mut self) {
        // Read the count of elements from bytecode
        let count = {
            let frame = self.call_frames.last().unwrap();
            frame.function.bloq.read_u16(frame.ip + 1) as usize
        };

        // Pop elements from stack and build the array
        // Stack layout: [elem0, elem1, ..., elemN]
        let stack_len = self.stack.len();
        let elements_start = stack_len - count;

        // Collect elements into a Vec
        let elements: Vec<Value> = self.stack[elements_start..stack_len].to_vec();

        // Pop all elements from stack
        self.stack.drain(elements_start..);

        // Push the new array onto the stack
        self.push(Value::new_array(elements));

        // Increment IP to skip the count bytes (u16 = 2 bytes)
        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += 2;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_create_set(&mut self) {
        // Read the count of elements from bytecode
        let count = {
            let frame = self.call_frames.last().unwrap();
            frame.function.bloq.read_u8(frame.ip + 1) as usize
        };

        // Pop elements from stack and build the set
        // Stack layout from compiler: [element1, element2, ..., elementN]
        let stack_len = self.stack.len();
        let elements_start = stack_len - count;

        let mut set = std::collections::BTreeSet::new();
        for i in 0..count {
            let element_value = &self.stack[elements_start + i];

            // Convert Value to SetKey
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

            // Insert into set (duplicates are automatically handled by HashSet)
            set.insert(key);
        }

        // Pop all elements from stack
        self.stack.drain(elements_start..);

        // Push the new set onto the stack
        self.push(Value::new_set(set));

        // Increment IP to skip the count byte
        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += 1;
    }

    pub(in crate::vm) fn fn_create_range(&mut self) -> Option<Result> {
        // Read the inclusive flag from bytecode
        let inclusive = {
            let frame = self.call_frames.last().unwrap();
            frame.function.bloq.read_u8(frame.ip + 1) != 0
        };

        // Pop end and start values from stack
        let end_value = self.pop();
        let start_value = self.pop();

        // Extract numeric values
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
                self.runtime_error(&format!(
                    "Range end must be a number, got {}",
                    end_value
                ));
                return Some(Result::RuntimeError);
            }
        };

        // Check if both are integers
        if start.fract() != 0.0 {
            self.runtime_error(&format!(
                "Range start must be an integer, got {}",
                start
            ));
            return Some(Result::RuntimeError);
        }

        if end.fract() != 0.0 {
            self.runtime_error(&format!(
                "Range end must be an integer, got {}",
                end
            ));
            return Some(Result::RuntimeError);
        }

        let start_int = start as i64;
        let end_int = end as i64;

        // Build the range array
        let elements: Vec<Value> = if inclusive {
            if start_int <= end_int {
                (start_int..=end_int).map(|i| Value::Number(i as f64)).collect()
            } else {
                // Empty range for reverse inclusive ranges
                Vec::new()
            }
        } else {
            if start_int < end_int {
                (start_int..end_int).map(|i| Value::Number(i as f64)).collect()
            } else {
                // Empty range for reverse exclusive ranges
                Vec::new()
            }
        };

        // Push the new array onto the stack
        self.push(Value::new_array(elements));

        // Increment IP to skip the inclusive flag byte
        let frame = self.call_frames.last_mut().unwrap();
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
                    // Extract index as number
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

                    // Normalize negative indices
                    let actual_index = if index < 0 {
                        len + index
                    } else {
                        index
                    };

                    // Bounds check
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

                    // Push the value back (assignment expression returns the value)
                    self.push(value);
                }
                Object::Array(array_ref) => {
                    // Extract index as number
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

                    // Normalize negative indices
                    let actual_index = if index < 0 {
                        len + index
                    } else {
                        index
                    };

                    // Bounds check
                    if actual_index < 0 || actual_index >= len {
                        self.runtime_error(&format!(
                            "Array index out of bounds: index {} (normalized: {}) on array of length {}.",
                            index, actual_index, len
                        ));
                        return;
                    }

                    array[actual_index as usize] = value.clone();

                    // Push the value back (assignment expression returns the value)
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

    /// Convert a Value to a MapKey if possible
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

        // Validate collection type and prepare for iteration
        let iterator_value = match &collection {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(_) => {
                    // Arrays can be iterated directly
                    collection
                }
                Object::Map(map_ref) => {
                    // For maps, we need to iterate over keys
                    // Convert keys to an array for consistent iteration
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
                    // For sets, convert to array for iteration
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

        // Push new iterator onto the stack
        self.iterator_stack.push((0, iterator_value));
        None
    }

    /// IteratorDone: Check if iteration is complete
    /// Pushes false if done (no more elements), true if not done (more elements remain)
    /// This inverted logic allows JumpIfFalse to exit the loop when done
    #[inline(always)]
    pub(in crate::vm) fn fn_iterator_done(&mut self) {
        if let Some((index, collection)) = self.iterator_stack.last() {
            // Check if we have more elements (NOT done)
            let has_more = match collection {
                Value::Object(obj) => match obj.as_ref() {
                    Object::Array(array_ref) => {
                        let array = array_ref.borrow();
                        *index < array.len()
                    }
                    _ => false, // Should not happen since GetIterator normalizes to arrays
                },
                _ => false,
            };

            self.push(boolean!(has_more));
        } else {
            // No iterator initialized - this is an error
            self.runtime_error("No iterator initialized");
            self.push(boolean!(false)); // Return false (done/error)
        }
    }

    /// IteratorNext: Get the next element from the iterator
    /// Pushes the next value onto the stack and advances the iterator
    /// When exiting a loop, pops the iterator from the iterator stack
    #[inline(always)]
    pub(in crate::vm) fn fn_iterator_next(&mut self) -> Option<Result> {
        // Extract values from iterator state to avoid multiple mutable borrows
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

        // Now handle the results without holding iterator borrow
        match (value, new_index) {
            (Some(v), Some(idx)) => {
                // Update the iterator index
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
}
