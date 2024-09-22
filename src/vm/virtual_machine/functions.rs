use crate::vm::{Block, Result, Value, VirtualMachine};

#[inline(always)]
pub(crate) fn fn_get_variable4(mut vm: &mut VirtualMachine, block: &Block) {
    let constant_index = block.read_u32(vm.ip + 1) as usize;
    let name = block.read_constant(constant_index);
    vm.push(vm.variables[&name.to_string()].clone());
    vm.ip += 4;
}

#[inline(always)]
pub(crate) fn fn_get_variable2(mut vm: &mut VirtualMachine, block: &Block) {
    let constant_index = block.read_u16(vm.ip + 1) as usize;
    let name = block.read_constant(constant_index);
    vm.push(vm.variables[&name.to_string()].clone());
    vm.ip += 2;
}

#[inline(always)]
pub(crate) fn fn_get_variable(mut vm: &mut VirtualMachine, block: &Block) {
    let constant_index = block.read_u8(vm.ip + 1) as usize;
    let name = block.read_constant(constant_index);
    vm.push(vm.variables[&name.to_string()].clone());
    vm.ip += 1;
}

#[inline(always)]
pub(crate) fn fn_get_value4(mut vm: &mut VirtualMachine, block: &Block) {
    let constant_index = block.read_u32(vm.ip + 1) as usize;
    let name = block.read_constant(constant_index);
    vm.push(vm.values[&name.to_string()].clone());
    vm.ip += 4;
}

#[inline(always)]
pub(crate) fn fn_get_value2(mut vm: &mut VirtualMachine, block: &Block) {
    let constant_index = block.read_u16(vm.ip + 1) as usize;
    let name = block.read_constant(constant_index);
    vm.push(vm.values[&name.to_string()].clone());
    vm.ip += 2;
}

#[inline(always)]
pub(crate) fn fn_get_value(mut vm: &mut VirtualMachine, block: &Block) {
    let constant_index = block.read_u8(vm.ip + 1) as usize;
    let name = block.read_constant(constant_index);
    vm.push(vm.values[&name.to_string()].clone());
    vm.ip += 1;
}

#[inline(always)]
pub(crate) fn fn_set_variable4(mut vm: &mut VirtualMachine, block: &Block) {
    let value_index = block.read_u32(vm.ip + 1) as usize;
    let value_name = block.read_variable(value_index);
    vm.variables.insert(value_name, vm.peek(0));
    vm.pop();
    vm.ip += 4;
}

#[inline(always)]
pub(crate) fn fn_set_variable2(mut vm: &mut VirtualMachine, block: &Block) {
    let value_index = block.read_u16(vm.ip + 1) as usize;
    let value_name = block.read_variable(value_index);
    vm.variables.insert(value_name, vm.peek(0));
    vm.pop();
    vm.ip += 2;
}

#[inline(always)]
pub(crate) fn fn_set_variable(mut vm: &mut VirtualMachine, block: &Block) {
    let value_index = block.read_u8(vm.ip + 1) as usize;
    let value_name = block.read_variable(value_index);
    vm.variables.insert(value_name, vm.peek(0));
    vm.pop();
    vm.ip += 1;
}

#[inline(always)]
pub(crate) fn fn_set_value4(mut vm: &mut VirtualMachine, block: &Block) {
    let value_index = block.read_u32(vm.ip + 1) as usize;
    let value_name = block.read_value(value_index);
    vm.values.insert(value_name, vm.peek(0));
    vm.pop();
    vm.ip += 4;
}

#[inline(always)]
pub(crate) fn fn_set_value2(mut vm: &mut VirtualMachine, block: &Block) {
    let value_index = block.read_u16(vm.ip + 1) as usize;
    let value_name = block.read_value(value_index);
    vm.values.insert(value_name, vm.peek(0));
    vm.pop();
    vm.ip += 2;
}

#[inline(always)]
pub(crate) fn fn_set_value(mut vm: &mut VirtualMachine, block: &Block) {
    let value_index = block.read_u8(vm.ip + 1) as usize;
    let value_name = block.read_value(value_index);
    vm.values.insert(value_name, vm.peek(0));
    vm.pop();
    vm.ip += 1;
}

#[inline(always)]
pub(crate) fn fn_print(mut vm: &mut VirtualMachine) {
    let value = vm.pop();
    vm.output_handler.println(value);
}

#[inline(always)]
pub(crate) fn fn_string4(mut vm: &mut VirtualMachine, block: &Block) {
    let string_index = block.read_u32(vm.ip + 1) as usize;
    let string = block.read_string(string_index);
    vm.push(string);
    vm.ip += 4;
}

#[inline(always)]
pub(crate) fn fn_string2(mut vm: &mut VirtualMachine, block: &Block) {
    let string_index = block.read_u16(vm.ip + 1) as usize;
    let string = block.read_string(string_index);
    vm.push(string);
    vm.ip += 2;
}

#[inline(always)]
pub(crate) fn fn_string(mut vm: &mut VirtualMachine, block: &Block) {
    let string_index = block.read_u8(vm.ip + 1) as usize;
    let string = block.read_string(string_index);
    vm.push(string);
    vm.ip += 1;
}

#[inline(always)]
pub(crate) fn fn_not(mut vm: &mut VirtualMachine) {
    let value = vm.pop();
    vm.push(boolean!(is_falsey!(value)));
}

#[inline(always)]
pub(crate) fn fn_less(mut vm: &mut VirtualMachine) {
    let b = vm.pop();
    let a = vm.pop();
    vm.push(boolean!(as_number!(a) < as_number!(b)));
}

#[inline(always)]
pub(crate) fn fn_greater(mut vm: &mut VirtualMachine) {
    let b = vm.pop();
    let a = vm.pop();
    vm.push(boolean!(as_number!(a) > as_number!(b)));
}

#[inline(always)]
pub(crate) fn fn_equal(mut vm: &mut VirtualMachine) {
    let b = vm.pop();
    let a = vm.pop();
    vm.push(boolean!(a == b));
}

#[inline(always)]
pub(crate) fn fn_divide(mut vm: &mut VirtualMachine) {
    let b = vm.pop();
    let a = vm.pop();
    vm.push(Value::Number(as_number!(a) / as_number!(b)));
}

#[inline(always)]
pub(crate) fn fn_multiply(mut vm: &mut VirtualMachine) {
    let b = vm.pop();
    let a = vm.pop();
    vm.push(Value::Number(as_number!(a) * as_number!(b)));
}

#[inline(always)]
pub(crate) fn fn_subtract(mut vm: &mut VirtualMachine) {
    let b = vm.pop();
    let a = vm.pop();
    vm.push(Value::Number(as_number!(a) - as_number!(b)));
}

#[inline(always)]
pub(crate) fn fn_add(mut vm: &mut VirtualMachine) -> Option<Result> {
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
pub(crate) fn fn_negate(mut vm: &mut VirtualMachine) -> Option<Result> {
    if let Value::Number(..) = vm.peek(0) {
        vm.runtime_error("Operand must be a number");
        return Some(crate::vm::Result::RuntimeError);
    }
    let value = vm.pop();
    vm.push(number!(-as_number!(value)));
    None
}

#[inline(always)]
pub(crate) fn fn_constant4(mut vm: &mut VirtualMachine, block: &Block) {
    let constant_index = block.read_u32(vm.ip + 1) as usize;
    let constant = block.read_constant(constant_index);
    vm.push(constant);
    vm.ip += 4;
}

#[inline(always)]
pub(crate) fn fn_constant2(mut vm: &mut VirtualMachine, block: &Block) {
    let constant_index = block.read_u16(vm.ip + 1) as usize;
    let constant = block.read_constant(constant_index);
    vm.push(constant);
    vm.ip += 2;
}

#[inline(always)]
pub(crate) fn fn_constant(mut vm: &mut VirtualMachine, block: &Block) {
    let constant_index = block.read_u8(vm.ip + 1) as usize;
    let constant = block.read_constant(constant_index);
    vm.push(constant);
    vm.ip += 1;
}
