use crate::vm::{BitsSize, Block, Result, Value, VirtualMachine};

#[inline(always)]
pub(crate) fn fn_print(vm: &mut VirtualMachine) {
    let value = vm.pop();
    println!("{}", value);

    #[cfg(test)]
    vm.string_buffer.push_str(value.to_string().as_str());
}

#[inline(always)]
pub(crate) fn fn_string4(vm: &mut VirtualMachine, block: &Block) {
    let string_index = block.read_u32(vm.ip + 1) as usize;
    let string = block.read_string(string_index);
    vm.push(string);
    vm.ip += 4;
}

#[inline(always)]
pub(crate) fn fn_string2(vm: &mut VirtualMachine, block: &Block) {
    let string_index = block.read_u16(vm.ip + 1) as usize;
    let string = block.read_string(string_index);
    vm.push(string);
    vm.ip += 2;
}

#[inline(always)]
pub(crate) fn fn_string(vm: &mut VirtualMachine, block: &Block) {
    let string_index = block.read_u8(vm.ip + 1) as usize;
    let string = block.read_string(string_index);
    vm.push(string);
    vm.ip += 1;
}

#[inline(always)]
pub(crate) fn fn_not(vm: &mut VirtualMachine) {
    let value = vm.pop();
    vm.push(boolean!(is_false_like!(value)));
}

#[inline(always)]
pub(crate) fn fn_less(vm: &mut VirtualMachine) {
    let b = vm.pop();
    let a = vm.pop();
    vm.push(boolean!(as_number!(a) < as_number!(b)));
}

#[inline(always)]
pub(crate) fn fn_greater(vm: &mut VirtualMachine) {
    let b = vm.pop();
    let a = vm.pop();
    vm.push(boolean!(as_number!(a) > as_number!(b)));
}

#[inline(always)]
pub(crate) fn fn_equal(vm: &mut VirtualMachine) {
    let b = vm.pop();
    let a = vm.pop();
    vm.push(boolean!(a == b));
}

#[inline(always)]
pub(crate) fn fn_divide(vm: &mut VirtualMachine) {
    let b = vm.pop();
    let a = vm.pop();
    vm.push(Value::Number(as_number!(a) / as_number!(b)));
}

#[inline(always)]
pub(crate) fn fn_multiply(vm: &mut VirtualMachine) {
    let b = vm.pop();
    let a = vm.pop();
    vm.push(Value::Number(as_number!(a) * as_number!(b)));
}

#[inline(always)]
pub(crate) fn fn_subtract(vm: &mut VirtualMachine) {
    let b = vm.pop();
    let a = vm.pop();
    vm.push(Value::Number(as_number!(a) - as_number!(b)));
}

#[inline(always)]
pub(crate) fn fn_add(vm: &mut VirtualMachine) -> Option<Result> {
    let b = vm.pop();
    let a = vm.pop();
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => vm.push(Value::Number(a + b)),
        (Value::String(a), Value::String(b)) => vm.push(Value::String(format!("{a}{b}"))),
        _ => {
            vm.runtime_error("Operands must be two numbers or two strings");
            return Some(Result::RuntimeError);
        }
    }
    None
}

#[inline(always)]
pub(crate) fn fn_negate(vm: &mut VirtualMachine) -> Option<Result> {
    if let Value::Number(..) = vm.peek(0) {
        vm.runtime_error("Operand must be a number");
        return Some(crate::vm::Result::RuntimeError);
    }
    let value = vm.pop();
    vm.push(number!(-as_number!(value)));
    None
}

#[inline(always)]
pub(crate) fn fn_constant4(vm: &mut VirtualMachine, block: &Block) {
    let constant_index = block.read_u32(vm.ip + 1) as usize;
    let constant = block.read_constant(constant_index);
    vm.push(constant);
    vm.ip += 4;
}

#[inline(always)]
pub(crate) fn fn_constant2(vm: &mut VirtualMachine, block: &Block) {
    let constant_index = block.read_u16(vm.ip + 1) as usize;
    let constant = block.read_constant(constant_index);
    vm.push(constant);
    vm.ip += 2;
}

#[inline(always)]
pub(crate) fn fn_constant(vm: &mut VirtualMachine, block: &Block) {
    let constant_index = block.read_u8(vm.ip + 1) as usize;
    let constant = block.read_constant(constant_index);
    vm.push(constant);
    vm.ip += 1;
}

#[inline(always)]
pub(crate) fn fn_define_global(vm: &mut VirtualMachine, block: &Block, bits: BitsSize) {
    let index;
    match bits {
        BitsSize::Eight => index = block.read_u8(vm.ip + 1) as usize,
        BitsSize::Sixteen => index = block.read_u16(vm.ip + 1) as usize,
        BitsSize::ThirtyTwo => index = block.read_u32(vm.ip + 1) as usize,
    }
    let name = block.read_global(index);
    vm.globals.insert(name, vm.peek(0));
    vm.pop();
    vm.ip += bits.as_bytes()
}

#[inline(always)]
pub(crate) fn fn_get_global(vm: &mut VirtualMachine, block: &Block, bits: BitsSize) {
    let index;
    match bits {
        BitsSize::Eight => index = block.read_u8(vm.ip + 1) as usize,
        BitsSize::Sixteen => index = block.read_u16(vm.ip + 1) as usize,
        BitsSize::ThirtyTwo => index = block.read_u32(vm.ip + 1) as usize,
    }
    let name = block.read_constant(index);
    vm.push(vm.globals[&name.to_string()].clone());
    vm.ip += bits.as_bytes()
}
