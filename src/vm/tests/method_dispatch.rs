// Tests for user-defined method dispatch (impl blocks)

use crate::vm::{Result, VirtualMachine};

// =============================================================================
// Instance Method Dispatch Tests
// =============================================================================

#[test]
fn test_instance_method_call() {
    let program = r#"
        struct Point { x y }
        
        impl Point {
            fn get_x(self) {
                return self.x
            }
        }
        
        val p = Point(3, 4)
        print(p.get_x())
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("3", vm.get_output());
}

#[test]
fn test_instance_method_with_field_access() {
    let program = r#"
        struct Point { x y }
        
        impl Point {
            fn sum(self) {
                return self.x + self.y
            }
        }
        
        val p = Point(10, 20)
        print(p.sum())
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("30", vm.get_output());
}

#[test]
fn test_instance_method_with_argument() {
    let program = r#"
        struct Point { x y }
        
        impl Point {
            fn add_to_x(self, amount) {
                return self.x + amount
            }
        }
        
        val p = Point(5, 10)
        print(p.add_to_x(7))
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("12", vm.get_output());
}

#[test]
fn test_instance_method_with_multiple_arguments() {
    let program = r#"
        struct Rect { width height }
        
        impl Rect {
            fn scale(self, sx, sy) {
                return self.width * sx + self.height * sy
            }
        }
        
        val r = Rect(10, 20)
        print(r.scale(2, 3))
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("80", vm.get_output()); // 10*2 + 20*3 = 20 + 60 = 80
}

// =============================================================================
// Mutating Method Dispatch Tests
// =============================================================================

#[test]
fn test_mutating_method_modifies_instance() {
    let program = r#"
        struct Counter { value }
        
        impl Counter {
            fn increment(mut self) {
                self.value = self.value + 1
            }
        }
        
        val c = Counter(0)
        c.increment()
        print(c.value)
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1", vm.get_output());
}

#[test]
fn test_mutating_method_multiple_calls() {
    let program = r#"
        struct Counter { value }
        
        impl Counter {
            fn increment(mut self) {
                self.value = self.value + 1
            }
        }
        
        val c = Counter(0)
        c.increment()
        c.increment()
        c.increment()
        print(c.value)
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("3", vm.get_output());
}

#[test]
fn test_mutating_method_with_argument() {
    let program = r#"
        struct Counter { value }
        
        impl Counter {
            fn add(mut self, amount) {
                self.value = self.value + amount
            }
        }
        
        val c = Counter(10)
        c.add(5)
        print(c.value)
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("15", vm.get_output());
}

#[test]
fn test_mutating_method_modifies_multiple_fields() {
    let program = r#"
        struct Point { x y }
        
        impl Point {
            fn translate(mut self, dx, dy) {
                self.x = self.x + dx
                self.y = self.y + dy
            }
        }
        
        val p = Point(1, 2)
        p.translate(10, 20)
        print(p.x)
        print(p.y)
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("11\n22", vm.get_output());
}

// =============================================================================
// Static Method Dispatch Tests
// =============================================================================

#[test]
fn test_static_method_call() {
    let program = r#"
        struct Point { x y }
        
        impl Point {
            fn origin() {
                return Point(0, 0)
            }
        }
        
        val p = Point.origin()
        print(p.x)
        print(p.y)
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("0\n0", vm.get_output());
}

#[test]
fn test_static_method_with_argument() {
    let program = r#"
        struct Point { x y }
        
        impl Point {
            fn unit(scale) {
                return Point(scale, scale)
            }
        }
        
        val p = Point.unit(5)
        print(p.x)
        print(p.y)
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("5\n5", vm.get_output());
}

#[test]
fn test_static_method_with_multiple_arguments() {
    let program = r#"
        struct Rect { width height }
        
        impl Rect {
            fn square(size) {
                return Rect(size, size)
            }
            
            fn from_dimensions(w, h) {
                return Rect(w, h)
            }
        }
        
        val r1 = Rect.square(10)
        val r2 = Rect.from_dimensions(3, 4)
        print(r1.width)
        print(r2.width)
        print(r2.height)
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("10\n3\n4", vm.get_output());
}

// =============================================================================
// Error Handling Tests
// =============================================================================

#[test]
fn test_undefined_method_error() {
    let program = r#"
        struct Point { x y }
        
        val p = Point(3, 4)
        p.undefined_method()
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("Unknown method") || errors.contains("Undefined method"),
        "Expected error about unknown/undefined method, got: {}",
        errors
    );
}

#[test]
fn test_undefined_static_method_error() {
    let program = r#"
        struct Point { x y }
        
        val p = Point.nonexistent()
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("Unknown method")
            || errors.contains("Undefined")
            || errors.contains("Cannot determine type"),
        "Expected error about unknown/undefined method, got: {}",
        errors
    );
}

// =============================================================================
// Combined Method Types Tests
// =============================================================================

#[test]
fn test_combined_method_types() {
    let program = r#"
        struct Counter { value }
        
        impl Counter {
            fn new(start) {
                return Counter(start)
            }
            
            fn get(self) {
                return self.value
            }
            
            fn increment(mut self) {
                self.value = self.value + 1
            }
        }
        
        val c = Counter.new(10)
        print(c.get())
        c.increment()
        print(c.get())
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("10\n11", vm.get_output());
}

#[test]
fn test_method_returning_self_type() {
    let program = r#"
        struct Point { x y }
        
        impl Point {
            fn add(self, other) {
                return Point(self.x + other.x, self.y + other.y)
            }
        }
        
        val p1 = Point(1, 2)
        val p2 = Point(3, 4)
        val p3 = p1.add(p2)
        print(p3.x)
        print(p3.y)
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("4\n6", vm.get_output());
}
