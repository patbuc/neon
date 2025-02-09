use crate::vm::{BitsSize, Block, Result, Value, VirtualMachine};
impl VirtualMachine {
    #[inline(always)]
    pub(super) fn fn_print(&mut self) {
        let value = self.pop();
        println!("{}", value);

        #[cfg(test)]
        self.string_buffer.push_str(value.to_string().as_str());
        #[cfg(test)]
        self.string_buffer.push_str("\n");
    }

    #[inline(always)]
    pub(super) fn fn_string4(&mut self, block: &Block) {
        let string_index = block.read_u32(self.ip + 1) as usize;
        let string = block.read_string(string_index);
        self.push(string);
        self.ip += 4;
    }

    #[inline(always)]
    pub(super) fn fn_string2(&mut self, block: &Block) {
        let string_index = block.read_u16(self.ip + 1) as usize;
        let string = block.read_string(string_index);
        self.push(string);
        self.ip += 2;
    }

    #[inline(always)]
    pub(super) fn fn_string(&mut self, block: &Block) {
        let string_index = block.read_u8(self.ip + 1) as usize;
        let string = block.read_string(string_index);
        self.push(string);
        self.ip += 1;
    }

    #[inline(always)]
    pub(super) fn fn_not(&mut self) {
        let value = self.pop();
        self.push(boolean!(is_false_like!(value)));
    }

    #[inline(always)]
    pub(super) fn fn_less(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(boolean!(as_number!(a) < as_number!(b)));
    }

    #[inline(always)]
    pub(super) fn fn_greater(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(boolean!(as_number!(a) > as_number!(b)));
    }

    #[inline(always)]
    pub(super) fn fn_equal(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(boolean!(a == b));
    }

    #[inline(always)]
    pub(super) fn fn_divide(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::Number(as_number!(a) / as_number!(b)));
    }

    #[inline(always)]
    pub(super) fn fn_multiply(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::Number(as_number!(a) * as_number!(b)));
    }

    #[inline(always)]
    pub(super) fn fn_subtract(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::Number(as_number!(a) - as_number!(b)));
    }

    #[inline(always)]
    pub(super) fn fn_add(&mut self) -> Option<Result> {
        let b = self.pop();
        let a = self.pop();
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => self.push(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => self.push(Value::String(format!("{a}{b}"))),
            _ => {
                self.runtime_error("Operands must be two numbers or two strings");
                return Some(Result::RuntimeError);
            }
        }
        None
    }

    #[inline(always)]
    pub(super) fn fn_negate(&mut self) -> Option<Result> {
        if let Value::Number(..) = self.peek(0) {
            let value = self.pop();
            self.push(number!(-as_number!(value)));
            return None;
        }
        self.runtime_error("Operand must be a number");
        Some(Result::RuntimeError)
    }

    #[inline(always)]
    pub(super) fn fn_constant4(&mut self, block: &Block) {
        let constant_index = block.read_u32(self.ip + 1) as usize;
        let constant = block.read_constant(constant_index);
        self.push(constant);
        self.ip += 4;
    }

    #[inline(always)]
    pub(super) fn fn_constant2(&mut self, block: &Block) {
        let constant_index = block.read_u16(self.ip + 1) as usize;
        let constant = block.read_constant(constant_index);
        self.push(constant);
        self.ip += 2;
    }

    #[inline(always)]
    pub(super) fn fn_constant(&mut self, block: &Block) {
        let constant_index = block.read_u8(self.ip + 1) as usize;
        let constant = block.read_constant(constant_index);
        self.push(constant);
        self.ip += 1;
    }

    #[inline(always)]
    pub(super) fn fn_define_global(&mut self, block: &Block, bits: BitsSize) {
        let index = self.read_bits(block, &bits);
        let name = block.read_global(index);
        self.globals.insert(name, self.peek(0));
        self.pop();
        self.ip += bits.as_bytes()
    }

    fn read_bits(&mut self, block: &Block, bits: &BitsSize) -> usize {
        match bits {
            BitsSize::Eight => block.read_u8(self.ip + 1) as usize,
            BitsSize::Sixteen => block.read_u16(self.ip + 1) as usize,
            BitsSize::ThirtyTwo => block.read_u32(self.ip + 1) as usize,
        }
    }

    #[inline(always)]
    pub(super) fn fn_get_global(&mut self, block: &Block, bits: BitsSize) {
        let index = self.read_bits(block, &bits);
        let name = block.read_constant(index);
        if !self.globals.contains_key(&name.to_string()) {
            self.runtime_error(&format!("Undefined value '{}'", name));
            return;
        }
        self.push(self.globals[&name.to_string()].clone());
        self.ip += bits.as_bytes()
    }

    #[inline(always)]
    pub(super) fn fn_jump_if_false(&mut self, block: &Block) {
        let offset = block.read_u32(self.ip + 1);
        self.ip += 4;
        if is_false_like!(self.peek(0)) {
            self.pop();
            self.ip += offset as usize;
        }
    }

    #[inline(always)]
    pub(super) fn fn_jump(&mut self, block: &Block) {
        let offset = block.read_u32(self.ip + 1);
        self.ip += 4;
        self.ip += offset as usize;
    }
}
