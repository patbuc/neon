use crate::common::stdlib::math_functions::{
    native_math_gcd, native_math_lcm, native_math_mod, native_math_round, native_math_sign,
};
use crate::common::Value;
use crate::vm::{InterpretResult, VirtualMachine};

// ============================================================================
// math.abs(), math.floor(), math.ceil() - Keep ints exact
// ============================================================================

#[test]
fn test_math_abs_keeps_int() {
    let program = r#"
        import "std/math"
        print(math.abs(-9007199254740993))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("9007199254740993", vm.get_output());
}

#[test]
fn test_math_floor_ceil_keep_int() {
    let program = r#"
        import "std/math"
        print(math.floor(9007199254740993))
        print(math.ceil(9007199254740993))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("9007199254740993\n9007199254740993", vm.get_output());
}

#[test]
fn test_math_floor_ceil_int() {
    let program = r#"
        import "std/math"
        print(math.div(math.floor(2.7), 1))
        print(math.div(math.ceil(2.1), 1))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("2\n3", vm.get_output());
}

// ============================================================================
// math.min() / math.max() - Keep the winning variant, compare exactly
// ============================================================================

#[test]
fn test_math_min_max_keep_variant() {
    let program = r#"
        import "std/math"
        print(math.max(9007199254740993, 1).toString())
        print(math.min(-9007199254740993, -1).toString())
        print(math.min(9007199254740993, 1).toString())
        print(math.max(1, 2.5).toString())
        print(math.min(1, 2.5).toString())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!(
        "9007199254740993\n-9007199254740993\n1\n2.5\n1",
        vm.get_output()
    );
}

// ============================================================================
// math.round() - Success Cases
// ============================================================================

#[test]
fn test_math_round() {
    assert!(matches!(
        native_math_round(&[Value::Number(2.5)]).unwrap(),
        Value::Int(3)
    ));
    assert!(matches!(
        native_math_round(&[Value::Number(-2.5)]).unwrap(),
        Value::Int(-3)
    ));
    assert!(matches!(
        native_math_round(&[Value::Number(2.4)]).unwrap(),
        Value::Int(2)
    ));
    assert!(matches!(
        native_math_round(&[Value::Number(2.6)]).unwrap(),
        Value::Int(3)
    ));
}

#[test]
fn test_math_round_int() {
    assert!(matches!(
        native_math_round(&[Value::Int(9007199254740993)]).unwrap(),
        Value::Int(9007199254740993)
    ));
}

#[test]
fn test_math_round_out_of_range() {
    let result = native_math_round(&[Value::Number(1e300)]);
    assert_eq!(result, Err("round() result is out of range".to_string()));
}

// ============================================================================
// math.sign() - Success Cases
// ============================================================================

#[test]
fn test_math_sign() {
    assert!(matches!(
        native_math_sign(&[Value::Int(-4)]).unwrap(),
        Value::Int(-1)
    ));
    assert!(matches!(
        native_math_sign(&[Value::Int(0)]).unwrap(),
        Value::Int(0)
    ));
    assert!(matches!(
        native_math_sign(&[Value::Int(4)]).unwrap(),
        Value::Int(1)
    ));
    assert!(matches!(
        native_math_sign(&[Value::Number(-0.5)]).unwrap(),
        Value::Int(-1)
    ));
    assert!(matches!(
        native_math_sign(&[Value::Number(0.5)]).unwrap(),
        Value::Int(1)
    ));
    assert!(matches!(
        native_math_sign(&[Value::Number(-0.0)]).unwrap(),
        Value::Int(0)
    ));
}

#[test]
fn test_math_sign_nan() {
    let result = native_math_sign(&[Value::Number(f64::NAN)]);
    assert_eq!(result, Err("sign() argument is NaN".to_string()));
}

// ============================================================================
// math.gcd() - Success Cases
// ============================================================================

#[test]
fn test_math_gcd() {
    assert!(matches!(
        native_math_gcd(&[Value::Int(12), Value::Int(18)]).unwrap(),
        Value::Int(6)
    ));
    assert!(matches!(
        native_math_gcd(&[Value::Int(-4), Value::Int(6)]).unwrap(),
        Value::Int(2)
    ));
    assert!(matches!(
        native_math_gcd(&[Value::Int(0), Value::Int(0)]).unwrap(),
        Value::Int(0)
    ));
    assert!(matches!(
        native_math_gcd(&[Value::Int(0), Value::Int(5)]).unwrap(),
        Value::Int(5)
    ));
}

#[test]
fn test_math_gcd_float_arg() {
    let result = native_math_gcd(&[Value::Number(4.0), Value::Int(6)]);
    assert_eq!(
        result,
        Err("gcd() expects two integers, got float".to_string())
    );
}

#[test]
fn test_math_gcd_overflow() {
    let result = native_math_gcd(&[Value::Int(i64::MIN), Value::Int(0)]);
    assert_eq!(result, Err("integer overflow in gcd()".to_string()));
}

// ============================================================================
// math.lcm() - Success Cases
// ============================================================================

#[test]
fn test_math_lcm() {
    assert!(matches!(
        native_math_lcm(&[Value::Int(4), Value::Int(6)]).unwrap(),
        Value::Int(12)
    ));
    assert!(matches!(
        native_math_lcm(&[Value::Int(0), Value::Int(5)]).unwrap(),
        Value::Int(0)
    ));
    assert!(matches!(
        native_math_lcm(&[Value::Int(3000000019), Value::Int(3000000021)]).unwrap(),
        Value::Int(9000000120000000399)
    ));
}

#[test]
fn test_math_lcm_float_arg() {
    let result = native_math_lcm(&[Value::Int(4), Value::Number(6.0)]);
    assert_eq!(
        result,
        Err("lcm() expects two integers, got float".to_string())
    );
}

#[test]
fn test_math_lcm_overflow() {
    let result = native_math_lcm(&[Value::Int(1i64 << 62), Value::Int(3)]);
    assert_eq!(result, Err("integer overflow in lcm()".to_string()));
}

// ============================================================================
// math.mod() - Success Cases
// ============================================================================

#[test]
fn test_math_mod_ints() {
    assert!(matches!(
        native_math_mod(&[Value::Int(-7), Value::Int(3)]).unwrap(),
        Value::Int(2)
    ));
    assert!(matches!(
        native_math_mod(&[Value::Int(7), Value::Int(-3)]).unwrap(),
        Value::Int(1)
    ));
    assert!(matches!(
        native_math_mod(&[Value::Int(-9007199254740993), Value::Int(2)]).unwrap(),
        Value::Int(1)
    ));
}

#[test]
fn test_math_mod_int_min_by_negative_one() {
    assert!(matches!(
        native_math_mod(&[Value::Int(i64::MIN), Value::Int(-1)]).unwrap(),
        Value::Int(0)
    ));
}

#[test]
fn test_math_mod_floats() {
    assert!(matches!(
        native_math_mod(&[Value::Number(-7.5), Value::Int(2)]).unwrap(),
        Value::Number(n) if n == 0.5
    ));
    assert!(matches!(
        native_math_mod(&[Value::Int(7), Value::Number(2.5)]).unwrap(),
        Value::Number(n) if n == 2.0
    ));
}

#[test]
fn test_math_mod_normalizes_bounds() {
    // Rounding can push rem_euclid to exactly |b| or to -0.0; both must
    // normalize into the documented [0, |b|) range.
    assert!(matches!(
        native_math_mod(&[Value::Number(-1e-20), Value::Int(1)]).unwrap(),
        Value::Number(n) if n == 0.0 && n.is_sign_positive()
    ));
    assert!(matches!(
        native_math_mod(&[Value::Number(-0.0), Value::Int(3)]).unwrap(),
        Value::Number(n) if n == 0.0 && n.is_sign_positive()
    ));
}

#[test]
fn test_math_mod_division_by_zero() {
    let result = native_math_mod(&[Value::Int(1), Value::Int(0)]);
    assert_eq!(result, Err("mod() division by zero".to_string()));
}

#[test]
fn test_math_mod_float_division_by_zero() {
    let result = native_math_mod(&[Value::Int(1), Value::Number(0.0)]);
    assert_eq!(result, Err("mod() division by zero".to_string()));
}
