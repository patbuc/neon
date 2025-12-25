use crate::common::{BitsSize, CallFrame, ObjInstance, ObjStruct, Value};
use crate::common::{ObjFunction, Object};
use crate::vm::Result;
use crate::vm::VirtualMachine;
use crate::{as_number, boolean, is_false_like, number, string};
use std::collections::HashMap;
use std::rc::Rc;

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
        let frame = self.current_frame();
        let arg_count = frame.function.chunk.read_u8(frame.ip + 1) as usize;

        // Get the callable from the stack (it's at position -arg_count - 1)
        let callable_value = self.peek(arg_count);

        match &callable_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Function(func) => {
                    if let Some(value) = self.call_function(arg_count, &func) {
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

        let field_count = r#struct.fields.len();
        let mut fields = HashMap::with_capacity(field_count);
        let stack_len = self.stack.len();
        let stack_slice = &self.stack[stack_len - arg_count..stack_len];
        for (field_name, value) in r#struct.fields.iter().zip(stack_slice.iter()) {
            fields.insert(field_name.clone(), value.clone());
        }

        let instance = ObjInstance {
            r#struct: Rc::clone(r#struct),
            fields,
        };

        let n = arg_count + 1;
        let start = self.stack.len().saturating_sub(n);
        self.stack.drain(start..);

        self.push(Value::new_object(instance));

        let current_frame = self.current_frame_mut();
        current_frame.ip += 2;
        None
    }

    fn call_function(&mut self, arg_count: usize, func: &&Rc<ObjFunction>) -> Option<Result> {
        if arg_count != func.arity as usize {
            self.runtime_error(&format!(
                "Expected {} arguments but got {}.",
                func.arity, arg_count
            ));
            return Some(Result::RuntimeError);
        }

        // Calculate slot_start: current stack size - arg_count - 1 (for the function itself)
        let slot_start = (self.stack.len() - arg_count - 1) as isize;
        let new_frame = CallFrame {
            function: Rc::clone(func),
            ip: 0,
            slot_start,
        };

        // Increment the current frame's IP before pushing the new frame
        // to skip both the Call opcode and the argument count byte when we return
        let current_frame = self.current_frame_mut();
        current_frame.ip += 2;

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

        // Clear the stack back to the slot_start (removing arguments and locals)
        self.stack.truncate(slot_start as usize);
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
    pub(in crate::vm) fn fn_call_method(&mut self, bits: BitsSize) -> Option<Result> {
        let index_size = match bits {
            BitsSize::Eight => 1,
            BitsSize::Sixteen => 2,
            BitsSize::ThirtyTwo => 4,
        };
        let ip_increment = 1 + 1 + index_size;

        let frame = self.current_frame();
        let arg_count = frame.function.chunk.read_u8(frame.ip + 1) as usize;
        let method_name_index = match bits {
            BitsSize::Eight => frame.function.chunk.read_u8(frame.ip + 2) as usize,
            BitsSize::Sixteen => frame.function.chunk.read_u16(frame.ip + 2) as usize,
            BitsSize::ThirtyTwo => frame.function.chunk.read_u32(frame.ip + 2) as usize,
        };

        let method_name = {
            let method_value = frame.function.chunk.read_string(method_name_index);
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

        let receiver = self.peek(arg_count);

        // Check if receiver is an instance with the method as a function field
        // This allows Math.abs(x) where abs is a function field in the Math instance
        if let Value::Object(obj) = &receiver {
            if let Object::Instance(instance_ref) = obj.as_ref() {
                let instance = instance_ref.borrow();
                if let Some(field_value) = instance.fields.get(&method_name) {
                    let function_value = field_value.clone();
                    drop(instance);

                    match &function_value {
                        Value::Object(func_obj) => match func_obj.as_ref() {
                            Object::Function(func) => {
                                return if let Some(result) = self.call_function(arg_count, &func) {
                                    Some(result)
                                } else {
                                    let current_frame = self.current_frame_mut();
                                    current_frame.ip += ip_increment;
                                    None
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
                    self.runtime_error(&format!(
                        "Type does not support method calls: {:?}",
                        receiver
                    ));
                    return Some(Result::RuntimeError);
                }
            },
            _ => {
                self.runtime_error(&format!(
                    "Cannot call methods on primitive type: {:?}",
                    receiver
                ));
                return Some(Result::RuntimeError);
            }
        };

        let native_callable = match VirtualMachine::get_native_method(&type_name, &method_name) {
            Some(callable) => callable,
            None => {
                self.runtime_error(&format!(
                    "Undefined method '{}' for type '{}'",
                    method_name, type_name
                ));
                return Some(Result::RuntimeError);
            }
        };

        let stack_len = self.stack.len();
        let receiver_index = stack_len - arg_count - 1;
        let args: Vec<Value> = self.stack[receiver_index..stack_len].to_vec();

        let result = native_callable.function()(&args);

        let n = arg_count + 1;
        let start = self.stack.len().saturating_sub(n);
        self.stack.drain(start..);

        match result {
            Ok(value) => {
                self.push(value);

                let current_frame = self.current_frame_mut();
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

    #[inline(always)]
    pub(in crate::vm) fn fn_call_static_method(&mut self, bits: BitsSize) -> Option<Result> {
        // Read registry index directly from bytecode (O(1) at runtime!)
        let (arg_count, registry_index, ip_increment) = {
            let frame = self.current_frame();
            let arg_count = frame.function.chunk.read_u8(frame.ip + 1) as usize;

            let registry_index = match bits {
                BitsSize::Eight => frame.function.chunk.read_u8(frame.ip + 2) as usize,
                BitsSize::Sixteen => frame.function.chunk.read_u16(frame.ip + 2) as usize,
                BitsSize::ThirtyTwo => frame.function.chunk.read_u32(frame.ip + 2) as usize,
            };

            let ip_increment = 1 + 1 + bits.as_bytes(); // opcode + arg_count + registry_index
            (arg_count, registry_index, ip_increment)
        };

        // Direct array index lookup - O(1)!
        let native_callable =
            match crate::common::method_registry::get_native_method_by_index(registry_index) {
                Some(callable) => callable,
                None => {
                    self.runtime_error(&format!("Invalid registry index: {}", registry_index));
                    return Some(Result::RuntimeError);
                }
            };

        // Pop arguments from stack (no receiver for static methods!)
        let stack_len = self.stack.len();
        let args_start = stack_len - arg_count;
        let args: Vec<Value> = self.stack[args_start..stack_len].to_vec();

        // Special handling for print(function to integrate with VM string buffer)
        let result = if registry_index == 0 {
            // The print(function is at index 0 in the registry - handle specially)
            self.handle_print_function(&args)
        } else {
            // Call the native function normally
            native_callable.function()(&args)
        };

        // Clean up stack
        self.stack.drain(args_start..);

        match result {
            Ok(value) => {
                self.push(value);
                let current_frame = self.current_frame_mut();
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
    pub(in crate::vm) fn fn_call_constructor(&mut self, bits: BitsSize) -> Option<Result> {
        // Read registry index directly from bytecode (O(1) at runtime!)
        let (arg_count, registry_index, ip_increment) = {
            let frame = self.current_frame();
            let arg_count = frame.function.chunk.read_u8(frame.ip + 1) as usize;

            let registry_index = match bits {
                BitsSize::Eight => frame.function.chunk.read_u8(frame.ip + 2) as usize,
                BitsSize::Sixteen => frame.function.chunk.read_u16(frame.ip + 2) as usize,
                BitsSize::ThirtyTwo => frame.function.chunk.read_u32(frame.ip + 2) as usize,
            };

            let ip_increment = 1 + 1 + bits.as_bytes(); // opcode + arg_count + registry_index
            (arg_count, registry_index, ip_increment)
        };

        // Direct array index lookup - O(1)!
        let native_callable =
            match crate::common::method_registry::get_native_method_by_index(registry_index) {
                Some(callable) => callable,
                None => {
                    self.runtime_error(&format!("Invalid registry index: {}", registry_index));
                    return Some(Result::RuntimeError);
                }
            };

        // Pop arguments from stack
        let stack_len = self.stack.len();
        let args_start = stack_len - arg_count;
        let args: Vec<Value> = self.stack[args_start..stack_len].to_vec();

        // Call the constructor function
        let result = native_callable.function()(&args);

        // Clean up stack
        self.stack.drain(args_start..);

        match result {
            Ok(value) => {
                self.push(value);
                let current_frame = self.current_frame_mut();
                current_frame.ip += ip_increment;
                None
            }
            Err(error_msg) => {
                self.runtime_error(&error_msg);
                Some(Result::RuntimeError)
            }
        }
    }

    /// Special handling for print(function to integrate with VM string buffer)
    /// This ensures print(output goes to the correct place in test/debug/WASM contexts)
    fn handle_print_function(&mut self, args: &[Value]) -> std::result::Result<Value, String> {
        if args.is_empty() {
            return Err("print() expects at least 1 argument".to_string());
        }

        // Join all arguments with spaces
        let output = args
            .iter()
            .map(|v: &Value| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        // For test/debug/WASM contexts - append to VM's string buffer
        #[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
        {
            self.string_buffer.push_str(&format!("{}\n", output));
        }

        // For normal execution (non-WASM) - print(to stdout)
        #[cfg(not(target_arch = "wasm32"))]
        {
            println!("{}", output);
        }

        Ok(Value::Nil)
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_slice(&mut self) -> Option<Result> {
        let end_value = self.pop();
        let start_value = self.pop();
        let array_value = self.pop();

        // Ensure we have an array
        let array_ref = match &array_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(arr) => arr,
                _ => {
                    self.runtime_error(&format!(
                        "Slice operation only works on arrays, got {}.",
                        array_value
                    ));
                    return Some(Result::RuntimeError);
                }
            },
            _ => {
                self.runtime_error(&format!(
                    "Slice operation only works on arrays, got {}.",
                    array_value
                ));
                return Some(Result::RuntimeError);
            }
        };

        let array = array_ref.borrow();
        let len = array.len() as i32;

        // Parse start index (nil means 0)
        let start_idx = match start_value {
            Value::Nil => 0,
            Value::Number(n) => {
                let idx = n as i32;
                if idx < 0 {
                    len + idx
                } else {
                    idx
                }
            }
            _ => {
                self.runtime_error(&format!(
                    "Slice start must be a number or nil, got {}.",
                    start_value
                ));
                return Some(Result::RuntimeError);
            }
        };

        // Parse end index (nil means len)
        let end_idx = match end_value {
            Value::Nil => len,
            Value::Number(n) => {
                let idx = n as i32;
                if idx < 0 {
                    len + idx
                } else {
                    idx
                }
            }
            _ => {
                self.runtime_error(&format!(
                    "Slice end must be a number or nil, got {}.",
                    end_value
                ));
                return Some(Result::RuntimeError);
            }
        };

        // Validate bounds
        if start_idx < 0 || start_idx > len {
            self.runtime_error(&format!(
                "Slice start index out of bounds: {} (normalized from original) on array of length {}.",
                start_idx, len
            ));
            return Some(Result::RuntimeError);
        }

        if end_idx < 0 || end_idx > len {
            self.runtime_error(&format!(
                "Slice end index out of bounds: {} (normalized from original) on array of length {}.",
                end_idx, len
            ));
            return Some(Result::RuntimeError);
        }

        if start_idx > end_idx {
            self.runtime_error(&format!(
                "Slice start index ({}) cannot be greater than end index ({}).",
                start_idx, end_idx
            ));
            return Some(Result::RuntimeError);
        }

        // Create the sliced array
        let sliced: Vec<Value> = array[start_idx as usize..end_idx as usize].to_vec();
        let result = Value::new_array(sliced);

        drop(array);
        self.push(result);
        None
    }
}
