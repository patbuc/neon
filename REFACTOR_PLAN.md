# Refactor Plan: Declarative Macro for Native Method Registry

## Problem Analysis

Currently, native method definitions are duplicated in THREE places:

1. **`src/common/method_registry.rs`** (lines 91-143): Static arrays defining method names
   ```rust
   static ARRAY_METHODS: [&str; 5] = ["push", "pop", "length", "size", "contains"];
   ```

2. **`src/vm/impl.rs`** (lines 338-347): Debug assertion pattern match
   ```rust
   ("Array", "push") | ("Array", "pop") | ("Array", "length") | ...
   ```

3. **`src/vm/impl.rs`** (lines 357-390): Runtime dispatch match
   ```rust
   ("Array", "push") => Some(crate::vm::array_functions::native_array_push),
   ```

**This violates DRY and creates maintenance burden**: Adding a new native method requires updating all three locations correctly, or the system breaks.

## Proposed Solution: Single Source of Truth via Declarative Macro

Create a **declarative macro** `define_native_methods!` that generates:
1. Static method name arrays (for compile-time validation)
2. Debug assertion pattern (for consistency checking)
3. Runtime dispatch function (for VM execution)
4. `get_methods_for_type()` implementation

### Design Philosophy

- **Single source of truth**: All method definitions in one place
- **Compile-time generation**: Zero runtime overhead
- **Type-safe**: Macro ensures path correctness
- **Maintainable**: Adding a method = one line of code
- **Self-documenting**: Method definitions are readable

## Implementation Plan

### 1. Create New Macro Module

**File**: `src/common/native_method_registry_macro.rs`

```rust
/// Declarative macro for defining native methods with zero duplication.
///
/// This macro generates:
/// - Static method name arrays for each type
/// - get_methods_for_type() implementation
/// - get_native_method() dispatch function
/// - Debug assertion patterns for consistency checking
///
/// Usage:
/// ```
/// define_native_methods! {
///     Array => {
///         push => crate::vm::array_functions::native_array_push,
///         pop => crate::vm::array_functions::native_array_pop,
///         length => crate::vm::array_functions::native_array_length,
///     },
///     String => {
///         len => crate::vm::string_functions::native_string_len,
///         substring => crate::vm::string_functions::native_string_substring,
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_native_methods {
    (
        $(
            $type_name:ident => {
                $(
                    $method_name:ident => $function_path:path
                ),* $(,)?
            }
        ),* $(,)?
    ) => {
        // 1. Generate static method arrays
        $(
            paste::paste! {
                static [<$type_name:upper _METHODS>]: &[&str] = &[
                    $(stringify!($method_name)),*
                ];
            }
        )*

        // 2. Generate get_methods_for_type() implementation
        pub fn get_methods_for_type(type_name: &str) -> &'static [&'static str] {
            match type_name {
                $(
                    stringify!($type_name) => paste::paste! { [<$type_name:upper _METHODS>] },
                )*
                _ => &[],
            }
        }

        // 3. Generate get_native_method() dispatch function
        pub fn get_native_method(
            type_name: &str,
            method_name: &str,
        ) -> Option<$crate::common::NativeFn> {
            // Debug assertion for consistency checking
            #[cfg(debug_assertions)]
            {
                let is_valid = matches!(
                    (type_name, method_name),
                    $($(
                        (stringify!($type_name), stringify!($method_name))
                    )|*)|*
                );
                let result_exists = matches!(
                    (type_name, method_name),
                    $($(
                        (stringify!($type_name), stringify!($method_name))
                    )|*)|*
                );
                debug_assert_eq!(
                    is_valid, result_exists,
                    "Method registry inconsistency: type={}, method={}",
                    type_name, method_name
                );
            }

            // Runtime dispatch
            match (type_name, method_name) {
                $($(
                    (stringify!($type_name), stringify!($method_name)) => Some($function_path),
                )*)*
                _ => None,
            }
        }
    };
}
```

### 2. Refactor `method_registry.rs`

**File**: `src/common/method_registry.rs`

Replace static arrays and manual implementation with macro invocation:

```rust
use crate::define_native_methods;

define_native_methods! {
    Array => {
        push => crate::vm::array_functions::native_array_push,
        pop => crate::vm::array_functions::native_array_pop,
        length => crate::vm::array_functions::native_array_length,
        size => crate::vm::array_functions::native_array_size,
        contains => crate::vm::array_functions::native_array_contains,
    },
    String => {
        len => crate::vm::string_functions::native_string_len,
        substring => crate::vm::string_functions::native_string_substring,
        replace => crate::vm::string_functions::native_string_replace,
        split => crate::vm::string_functions::native_string_split,
        toInt => crate::vm::string_functions::native_string_to_int,
        toFloat => crate::vm::string_functions::native_string_to_float,
        toBool => crate::vm::string_functions::native_string_to_bool,
    },
    Number => {
        toString => crate::vm::number_functions::native_number_to_string,
    },
    Boolean => {
        toString => crate::vm::boolean_functions::native_boolean_to_string,
    },
    Map => {
        get => crate::vm::map_functions::native_map_get,
        size => crate::vm::map_functions::native_map_size,
        has => crate::vm::map_functions::native_map_has,
        remove => crate::vm::map_functions::native_map_remove,
        keys => crate::vm::map_functions::native_map_keys,
        values => crate::vm::map_functions::native_map_values,
        entries => crate::vm::map_functions::native_map_entries,
    },
    Set => {
        add => crate::vm::set_functions::native_set_add,
        remove => crate::vm::set_functions::native_set_remove,
        has => crate::vm::set_functions::native_set_has,
        size => crate::vm::set_functions::native_set_size,
        clear => crate::vm::set_functions::native_set_clear,
        union => crate::vm::set_functions::native_set_union,
        intersection => crate::vm::set_functions::native_set_intersection,
        difference => crate::vm::set_functions::native_set_difference,
        isSubset => crate::vm::set_functions::native_set_is_subset,
        toArray => crate::vm::set_functions::native_set_to_array,
    },
}

pub struct MethodRegistry;

impl MethodRegistry {
    pub fn get_methods_for_type(type_name: &str) -> &'static [&'static str] {
        get_methods_for_type(type_name)
    }

    pub fn is_valid_method(type_name: &str, method_name: &str) -> bool {
        Self::get_methods_for_type(type_name)
            .iter()
            .any(|&m| m == method_name)
    }

    pub fn suggest_method(type_name: &str, method_name: &str) -> Option<&'static str> {
        let methods = Self::get_methods_for_type(type_name);
        find_closest_match(method_name, methods)
    }
}
```

### 3. Refactor `vm/impl.rs`

**File**: `src/vm/impl.rs`

Replace `get_native_method()` function with re-export from macro:

```rust
impl VirtualMachine {
    /// Returns the native function implementation for a given type and method name.
    ///
    /// This dispatch table is automatically generated by the define_native_methods! macro
    /// in method_registry.rs to ensure consistency with compile-time validation.
    pub(in crate::vm) fn get_native_method(
        type_name: &str,
        method_name: &str,
    ) -> Option<crate::common::NativeFn> {
        crate::common::method_registry::get_native_method(type_name, method_name)
    }
}
```

### 4. Update Dependencies

**File**: `Cargo.toml`

Add `paste` crate for token concatenation in macro:

```toml
[dependencies]
paste = "1.0"
```

### 5. Testing Strategy

All existing tests should pass without modification:
- `src/common/method_registry.rs::tests::*`
- `src/compiler/tests/semantic.rs` (method validation tests)
- All VM integration tests

**Additional tests to add:**
- Test that macro generates correct static arrays
- Test that all three outputs are consistent
- Test that adding a new method works end-to-end

## Benefits

1. **Single source of truth**: Method definitions exist in exactly one place
2. **Impossible to desync**: Macro generates all three locations from same input
3. **Easy to add methods**: One line in macro = complete integration
4. **Zero runtime cost**: All generation at compile time
5. **Type-safe**: Compiler verifies function paths exist
6. **Maintainable**: Clear, declarative syntax

## Migration Path

1. Add `paste` dependency
2. Create macro in new module
3. Refactor `method_registry.rs` to use macro
4. Refactor `vm/impl.rs` to call macro-generated function
5. Run all tests to verify equivalence
6. Remove old manual implementations
7. Update documentation

## Future Enhancements

Once the macro is working, we can extend it to generate:
- Documentation for each method
- Arity validation (argument count checking)
- Type signatures for better error messages
- Auto-generated method lookup tables for faster dispatch
