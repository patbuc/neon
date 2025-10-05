use crate::common::Object;
use crate::common::{BitsSize, Brick, Value};
use crate::vm::Result;
use crate::vm::VirtualMachine;
use crate::{as_number, boolean, is_false_like, number, string};

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
    pub(in crate::vm) fn fn_string4(&mut self, brick: &Brick) {
        let string_index = brick.read_u32(self.ip + 1) as usize;
        let string = brick.read_string(string_index);
        self.push(string);
        self.ip += 4;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_string2(&mut self, brick: &Brick) {
        let string_index = brick.read_u16(self.ip + 1) as usize;
        let string = brick.read_string(string_index);
        self.push(string);
        self.ip += 2;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_string(&mut self, brick: &Brick) {
        let string_index = brick.read_u8(self.ip + 1) as usize;
        let string = brick.read_string(string_index);
        self.push(string);
        self.ip += 1;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_not(&mut self) {
        let value = self.pop();
        self.push(boolean!(is_false_like!(value)));
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_call(&mut self, brick: &Brick) -> Option<Result> {
        let arg_count = brick.read_u8(self.ip + 1) as usize;

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

                        // For now, we'll execute the function by recursively calling run
                        // This is a simplified approach - a full implementation would use call frames
                        let saved_ip = self.ip;

                        self.ip = 0; // Start at the beginning of the function brick
                        let result = self.run(func.brick.as_ref());
                        // Restore the main VM state
                        self.ip = saved_ip;

                        // Push the result (for now, functions implicitly return nil)
                        self.push(crate::nil!());

                        // Skip the argument count byte
                        self.ip += 1;

                        if result != Result::Ok {
                            return Some(result);
                        }
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
    pub(in crate::vm) fn fn_constant4(&mut self, brick: &Brick) {
        let constant_index = brick.read_u32(self.ip + 1) as usize;
        let constant = brick.read_constant(constant_index);
        self.push(constant);
        self.ip += 4;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_constant2(&mut self, brick: &Brick) {
        let constant_index = brick.read_u16(self.ip + 1) as usize;
        let constant = brick.read_constant(constant_index);
        self.push(constant);
        self.ip += 2;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_constant(&mut self, brick: &Brick) {
        let constant_index = brick.read_u8(self.ip + 1) as usize;
        let constant = brick.read_constant(constant_index);
        self.push(constant);
        self.ip += 1;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_value(&mut self, brick: &Brick, bits: BitsSize) {
        let index = self.read_bits(brick, &bits);
        self.stack[index] = self.peek(0);
        self.ip += bits.as_bytes()
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_set_variable(&mut self, brick: &Brick, bits: BitsSize) {
        let index = self.read_bits(brick, &bits);
        self.stack[index] = self.peek(0);
        self.ip += bits.as_bytes();
    }

    fn read_bits(&mut self, brick: &Brick, bits: &BitsSize) -> usize {
        match bits {
            BitsSize::Eight => brick.read_u8(self.ip + 1) as usize,
            BitsSize::Sixteen => brick.read_u16(self.ip + 1) as usize,
            BitsSize::ThirtyTwo => brick.read_u32(self.ip + 1) as usize,
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_value(&mut self, brick: &Brick, bits: BitsSize) {
        let index = self.read_bits(brick, &bits);
        self.push(self.stack[index].clone());
        self.ip += bits.as_bytes()
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_get_variable(&mut self, brick: &Brick, bits: BitsSize) {
        let index = self.read_bits(brick, &bits);
        self.push(self.stack[index].clone());
        self.ip += bits.as_bytes()
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_jump_if_false(&mut self, brick: &Brick) {
        let offset = brick.read_u32(self.ip + 1);
        self.ip += 4;
        if is_false_like!(self.peek(0)) {
            self.pop();
            self.ip += offset as usize;
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_jump(&mut self, brick: &Brick) {
        let offset = brick.read_u32(self.ip + 1);
        self.ip += 4;
        self.ip += offset as usize;
    }

    #[inline(always)]
    pub(in crate::vm) fn fn_loop(&mut self, brick: &Brick) {
        let offset = brick.read_u32(self.ip + 1);
        self.ip += 4;
        self.ip -= offset as usize;
    }
}
