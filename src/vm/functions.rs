use crate::common::{BitsSize, CallFrame, ObjInstance, ObjStruct, Value};
use crate::common::{ObjFunction, Object};
use crate::vm::Result;
use crate::vm::VirtualMachine;
use crate::{as_number, boolean, is_false_like, nil, number, string};
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
            self.pop();
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
        // Global variables are always in the script frame (first frame)
        // Script frame has slot_start = -1, so globals start at index 0
        let script_frame = &self.call_frames[0];
        let absolute_index = (script_frame.slot_start + 1 + index as isize) as usize;
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

    // ===== Map Operations =====

    #[inline(always)]
    pub(in crate::vm) fn fn_map(&mut self) {
        // Create empty map and push to stack
        self.push(Value::new_map());
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_map_set(&mut self, bits: BitsSize) {
        let key_index = self.read_bits(&bits);
        let value = self.peek(0); // Value to insert
        let map_value = self.peek(1); // Map

        // Read the key from strings
        let key = {
            let frame = self.call_frames.last().unwrap();
            let key_value = frame.function.bloq.read_string(key_index);
            match key_value {
                Value::Object(obj) => match obj.as_ref() {
                    Object::String(s) => s.value.to_string(),
                    _ => {
                        self.runtime_error("Map key must be a string.");
                        return;
                    }
                },
                _ => {
                    self.runtime_error("Map key must be a string.");
                    return;
                }
            }
        };

        match &map_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Map(map_ref) => {
                    let mut map = map_ref.borrow_mut();
                    map.entries.insert(key, value.clone());

                    // Pop value, keep map on stack
                    self.pop(); // Pop value
                    // Map stays on stack (at position 0 after pop)
                }
                _ => {
                    self.runtime_error("Expected map for set operation.");
                    return;
                }
            },
            _ => {
                self.runtime_error("Expected map for set operation.");
                return;
            }
        }

        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += bits.as_bytes();
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_map_get(&mut self, bits: BitsSize) {
        let key_index = self.read_bits(&bits);
        let map_value = self.peek(0);

        // Read the key from strings
        let key = {
            let frame = self.call_frames.last().unwrap();
            let key_value = frame.function.bloq.read_string(key_index);
            match key_value {
                Value::Object(obj) => match obj.as_ref() {
                    Object::String(s) => s.value.to_string(),
                    _ => {
                        self.runtime_error("Map key must be a string.");
                        return;
                    }
                },
                _ => {
                    self.runtime_error("Map key must be a string.");
                    return;
                }
            }
        };

        match &map_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Map(map_ref) => {
                    let map = map_ref.borrow();
                    let value = map.entries.get(&key).cloned().unwrap_or(nil!());

                    self.pop(); // Pop map
                    self.push(value); // Push value (or nil if not found)
                }
                _ => {
                    self.runtime_error("Expected map for get operation.");
                    return;
                }
            },
            _ => {
                self.runtime_error("Expected map for get operation.");
                return;
            }
        }

        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += bits.as_bytes();
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_map_has(&mut self, bits: BitsSize) {
        let key_index = self.read_bits(&bits);
        let map_value = self.peek(0);

        // Read the key from strings
        let key = {
            let frame = self.call_frames.last().unwrap();
            let key_value = frame.function.bloq.read_string(key_index);
            match key_value {
                Value::Object(obj) => match obj.as_ref() {
                    Object::String(s) => s.value.to_string(),
                    _ => {
                        self.runtime_error("Map key must be a string.");
                        return;
                    }
                },
                _ => {
                    self.runtime_error("Map key must be a string.");
                    return;
                }
            }
        };

        match &map_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Map(map_ref) => {
                    let map = map_ref.borrow();
                    let has_key = map.entries.contains_key(&key);

                    self.pop(); // Pop map
                    self.push(boolean!(has_key)); // Push boolean result
                }
                _ => {
                    self.runtime_error("Expected map for has operation.");
                    return;
                }
            },
            _ => {
                self.runtime_error("Expected map for has operation.");
                return;
            }
        }

        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += bits.as_bytes();
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_map_remove(&mut self, bits: BitsSize) {
        let key_index = self.read_bits(&bits);
        let map_value = self.peek(0);

        // Read the key from strings
        let key = {
            let frame = self.call_frames.last().unwrap();
            let key_value = frame.function.bloq.read_string(key_index);
            match key_value {
                Value::Object(obj) => match obj.as_ref() {
                    Object::String(s) => s.value.to_string(),
                    _ => {
                        self.runtime_error("Map key must be a string.");
                        return;
                    }
                },
                _ => {
                    self.runtime_error("Map key must be a string.");
                    return;
                }
            }
        };

        match &map_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Map(map_ref) => {
                    let mut map = map_ref.borrow_mut();
                    map.entries.remove(&key);
                    // Map stays on stack
                }
                _ => {
                    self.runtime_error("Expected map for remove operation.");
                    return;
                }
            },
            _ => {
                self.runtime_error("Expected map for remove operation.");
                return;
            }
        }

        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += bits.as_bytes();
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_map_keys(&mut self) {
        let map_value = self.peek(0);

        match &map_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Map(map_ref) => {
                    let map = map_ref.borrow();
                    // Create an array-like representation (for now, just return nil)
                    // TODO: Implement proper array type and return array of keys
                    self.pop(); // Pop map
                    self.push(nil!()); // Placeholder for array of keys
                }
                _ => {
                    self.runtime_error("Expected map for keys operation.");
                    return;
                }
            },
            _ => {
                self.runtime_error("Expected map for keys operation.");
                return;
            }
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_map_values(&mut self) {
        let map_value = self.peek(0);

        match &map_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Map(map_ref) => {
                    let map = map_ref.borrow();
                    // Create an array-like representation (for now, just return nil)
                    // TODO: Implement proper array type and return array of values
                    self.pop(); // Pop map
                    self.push(nil!()); // Placeholder for array of values
                }
                Object::Set(set_ref) => {
                    // Handle SetValues through this same opcode
                    let set = set_ref.borrow();
                    // TODO: Implement proper array type and return array of values
                    self.pop(); // Pop set
                    self.push(nil!()); // Placeholder for array of values
                }
                _ => {
                    self.runtime_error("Expected map or set for values operation.");
                    return;
                }
            },
            _ => {
                self.runtime_error("Expected map or set for values operation.");
                return;
            }
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_map_size(&mut self) {
        let obj_value = self.peek(0);

        match &obj_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Map(map_ref) => {
                    let map = map_ref.borrow();
                    let size = map.entries.len() as f64;

                    self.pop(); // Pop map
                    self.push(number!(size)); // Push size as number
                }
                Object::Set(set_ref) => {
                    // Handle SetSize through this same opcode
                    let set = set_ref.borrow();
                    let size = set.elements.len() as f64;

                    self.pop(); // Pop set
                    self.push(number!(size)); // Push size as number
                }
                _ => {
                    self.runtime_error("Expected map or set for size operation.");
                    return;
                }
            },
            _ => {
                self.runtime_error("Expected map or set for size operation.");
                return;
            }
        }
    }

    // ===== Set Operations =====

    #[inline(always)]
    pub(in crate::vm) fn fn_set(&mut self) {
        // Create empty set and push to stack
        self.push(Value::new_set());
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_add(&mut self) {
        let value = self.peek(0); // Value to add
        let set_value = self.peek(1); // Set

        // Convert value to string key for storage (sets use hash for uniqueness)
        let key = format!("{}", value);

        match &set_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(set_ref) => {
                    let mut set = set_ref.borrow_mut();
                    set.elements.insert(key, value.clone());

                    // Pop value, keep set on stack
                    self.pop(); // Pop value
                    // Set stays on stack
                }
                _ => {
                    self.runtime_error("Expected set for add operation.");
                    return;
                }
            },
            _ => {
                self.runtime_error("Expected set for add operation.");
                return;
            }
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_has(&mut self) {
        let value = self.peek(0); // Value to check
        let set_value = self.peek(1); // Set

        // Convert value to string key for lookup
        let key = format!("{}", value);

        match &set_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(set_ref) => {
                    let set = set_ref.borrow();
                    let has_value = set.elements.contains_key(&key);

                    self.pop(); // Pop value
                    self.pop(); // Pop set
                    self.push(boolean!(has_value)); // Push boolean result
                }
                _ => {
                    self.runtime_error("Expected set for has operation.");
                    return;
                }
            },
            _ => {
                self.runtime_error("Expected set for has operation.");
                return;
            }
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_remove(&mut self) {
        let value = self.peek(0); // Value to remove
        let set_value = self.peek(1); // Set

        // Convert value to string key for removal
        let key = format!("{}", value);

        match &set_value {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(set_ref) => {
                    let mut set = set_ref.borrow_mut();
                    set.elements.remove(&key);

                    self.pop(); // Pop value
                    // Set stays on stack
                }
                _ => {
                    self.runtime_error("Expected set for remove operation.");
                    return;
                }
            },
            _ => {
                self.runtime_error("Expected set for remove operation.");
                return;
            }
        }
    }
}
