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

        // Collect arguments from the stack
        // Arguments are at stack positions: [stack.len() - arg_count .. stack.len()]
        let stack_len = self.stack.len();
        let args_start = stack_len - arg_count;
        let args: Vec<Value> = self.stack[args_start..stack_len].to_vec();

        // Call the native function
        let result = (native_fn.function)(self, &args);

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

        // Check if this is a built-in global using the sentinel value u32::MAX
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
    pub(in crate::vm) fn fn_call_method(&mut self) -> Option<Result> {
        // Read arg count and method name index
        let frame = self.call_frames.last().unwrap();
        let arg_count = frame.function.bloq.read_u8(frame.ip + 1) as usize;
        let method_name_index = frame.function.bloq.read_u8(frame.ip + 2) as usize;

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
                                    current_frame.ip += 3;
                                    None
                                };
                            }
                            Object::NativeFunction(native_fn) => {
                                // Collect arguments from the stack (receiver + args)
                                // Stack: [receiver, arg1, arg2, ...]
                                let stack_len = self.stack.len();
                                let receiver_index = stack_len - arg_count - 1;
                                let args: Vec<Value> = self.stack[receiver_index + 1..stack_len].to_vec();

                                // Call the native function
                                let result = (native_fn.function)(self, &args);

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
                                        current_frame.ip += 3;

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

        // Collect arguments from the stack (receiver + args)
        // Stack: [receiver, arg1, arg2, ...]
        let stack_len = self.stack.len();
        let receiver_index = stack_len - arg_count - 1;
        let args: Vec<Value> = self.stack[receiver_index..stack_len].to_vec();

        // Call the native method
        let result = native_fn(self, &args);

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
                current_frame.ip += 3;

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

    #[inline(always)]
    pub(in crate::vm) fn fn_get_index(&mut self) {
        let index_value = self.pop();
        let collection_value = self.pop();

        match &collection_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(array_ref) => {
                    // Handle array indexing
                    match index_value {
                        Value::Number(n) => {
                            let index = n as i64;
                            if index < 0 {
                                self.runtime_error(&format!(
                                    "Array index must be non-negative, got {}.",
                                    index
                                ));
                                return;
                            }
                            let array = array_ref.borrow();
                            let index = index as usize;
                            if index >= array.len() {
                                self.runtime_error(&format!(
                                    "Array index out of bounds: index {} but length is {}.",
                                    index,
                                    array.len()
                                ));
                                return;
                            }
                            let result = array[index].clone();
                            self.push(result);
                        }
                        _ => {
                            self.runtime_error(&format!(
                                "Array index must be a number, got {}.",
                                index_value
                            ));
                        }
                    }
                }
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
                Object::Array(array_ref) => {
                    // Handle array assignment
                    match index_value {
                        Value::Number(n) => {
                            let index = n as i64;
                            if index < 0 {
                                self.runtime_error(&format!(
                                    "Array index must be non-negative, got {}.",
                                    index
                                ));
                                return;
                            }
                            let mut array = array_ref.borrow_mut();
                            let index = index as usize;
                            if index >= array.len() {
                                self.runtime_error(&format!(
                                    "Array index out of bounds: index {} but length is {}.",
                                    index,
                                    array.len()
                                ));
                                return;
                            }
                            array[index] = value.clone();
                            // Push the value back (assignment expression returns the value)
                            self.push(value);
                        }
                        _ => {
                            self.runtime_error(&format!(
                                "Array index must be a number, got {}.",
                                index_value
                            ));
                        }
                    }
                }
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
}
