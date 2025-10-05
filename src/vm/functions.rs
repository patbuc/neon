use crate::common::Object;
use crate::common::{BitsSize, CallFrame, Value};
use crate::vm::Result;
use crate::vm::VirtualMachine;
use crate::{as_number, boolean, is_false_like, number, string};
use std::rc::Rc;

impl VirtualMachine {
    #[inline(always)]
    pub(in crate::vm) fn fn_print(&mut self) {
        let value = self.pop();

        #[cfg(not(test))]
        println!("{}", value);
        #[cfg(test)]
        self.string_buffer.push_str(value.to_string().as_str());
        #[cfg(test)]
        self.string_buffer.push('\n');
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_string4(&mut self) {
        let string = {
            let frame = self.call_frames.last().unwrap();
            let string_index = frame.function.brick.read_u32(frame.ip + 1) as usize;
            frame.function.brick.read_string(string_index)
        };
        self.push(string);
        self.call_frames.last_mut().unwrap().ip += 4;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_string2(&mut self) {
        let string = {
            let frame = self.call_frames.last().unwrap();
            let string_index = frame.function.brick.read_u16(frame.ip + 1) as usize;
            frame.function.brick.read_string(string_index)
        };
        self.push(string);
        self.call_frames.last_mut().unwrap().ip += 2;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_string(&mut self) {
        let string = {
            let frame = self.call_frames.last().unwrap();
            let string_index = frame.function.brick.read_u8(frame.ip + 1) as usize;
            frame.function.brick.read_string(string_index)
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
        let arg_count = frame.function.brick.read_u8(frame.ip + 1) as usize;

        // Get the function from the stack (it's at position -arg_count - 1)
        let function_value = self.peek(arg_count);

        match &function_value {
            Value::Object(obj) => {
                match obj.as_ref() {
                    Object::Function(func) => {
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
                    }
                    _ => {
                        self.runtime_error("Can only call functions.");
                        return Some(Result::RuntimeError);
                    }
                }
            }
            _ => {
                self.runtime_error("Can only call functions.");
                return Some(Result::RuntimeError);
            }
        }

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
            let constant_index = frame.function.brick.read_u32(frame.ip + 1) as usize;
            frame.function.brick.read_constant(constant_index)
        };
        self.push(constant);
        self.call_frames.last_mut().unwrap().ip += 4;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_constant2(&mut self) {
        let constant = {
            let frame = self.call_frames.last().unwrap();
            let constant_index = frame.function.brick.read_u16(frame.ip + 1) as usize;
            frame.function.brick.read_constant(constant_index)
        };
        self.push(constant);
        self.call_frames.last_mut().unwrap().ip += 2;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_constant(&mut self) {
        let constant = {
            let frame = self.call_frames.last().unwrap();
            let constant_index = frame.function.brick.read_u8(frame.ip + 1) as usize;
            frame.function.brick.read_constant(constant_index)
        };
        self.push(constant);
        self.call_frames.last_mut().unwrap().ip += 1;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_value(&mut self, bits: BitsSize) {
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

    #[inline(always)]
    pub(in crate::vm) fn fn_set_variable(&mut self, bits: BitsSize) {
        let index = self.read_bits(&bits);
        let frame = self.call_frames.last().unwrap();
        let absolute_index = (frame.slot_start + 1 + index as isize) as usize;
        self.stack[absolute_index] = self.peek(0);
        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += bits.as_bytes();
    }

    fn read_bits(&mut self, bits: &BitsSize) -> usize {
        let frame = self.call_frames.last().unwrap();
        match bits {
            BitsSize::Eight => frame.function.brick.read_u8(frame.ip + 1) as usize,
            BitsSize::Sixteen => frame.function.brick.read_u16(frame.ip + 1) as usize,
            BitsSize::ThirtyTwo => frame.function.brick.read_u32(frame.ip + 1) as usize,
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_value(&mut self, bits: BitsSize) {
        let index = self.read_bits(&bits);
        let frame = self.call_frames.last().unwrap();
        let absolute_index = (frame.slot_start + 1 + index as isize) as usize;
        self.push(self.stack[absolute_index].clone());
        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += bits.as_bytes()
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_variable(&mut self, bits: BitsSize) {
        let index = self.read_bits(&bits);
        let frame = self.call_frames.last().unwrap();
        let absolute_index = (frame.slot_start + 1 + index as isize) as usize;
        self.push(self.stack[absolute_index].clone());
        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += bits.as_bytes()
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_jump_if_false(&mut self) {
        let offset = self.call_frames.last().unwrap().function.brick.read_u32(
            self.call_frames.last().unwrap().ip + 1
        );
        self.call_frames.last_mut().unwrap().ip += 4;
        if is_false_like!(self.peek(0)) {
            self.pop();
            self.call_frames.last_mut().unwrap().ip += offset as usize;
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_jump(&mut self) {
        let offset = self.call_frames.last().unwrap().function.brick.read_u32(
            self.call_frames.last().unwrap().ip + 1
        );
        let frame = self.call_frames.last_mut().unwrap();
        frame.ip += 4;
        frame.ip += offset as usize;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_loop(&mut self) {
        let offset = self.call_frames.last().unwrap().function.brick.read_u32(
            self.call_frames.last().unwrap().ip + 1
        );
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
}
