# Build Feature: Refactor Native Method Registry to Use Declarative Macro

## Objective

Eliminate the triple duplication of native method definitions by creating a single source of truth through a declarative macro. Currently, every native method is manually defined in three places, which violates DRY principles and creates a maintenance burden.

## Background Context

### Current Problem

The native method system has THREE copies of essentially the same data:

1. **`src/common/method_registry.rs:91-143`**: Static arrays for compile-time validation
2. **`src/vm/impl.rs:338-347`**: Debug assertion pattern for consistency checking
3. **`src/vm/impl.rs:357-390`**: Runtime dispatch match arms

This means adding a new method like `Array.filter` requires:
- Adding `"filter"` to `ARRAY_METHODS` array
- Adding `("Array", "filter")` to debug assertion pattern
- Adding `("Array", "filter") => Some(crate::vm::array_functions::native_array_filter)` to dispatch

**This is error-prone and violates DRY.** We need ONE place to define methods.

### Solution: Declarative Macro

Create a `define_native_methods!` macro that generates all three locations from a single declaration:

```rust
define_native_methods! {
    Array => {
        push => crate::vm::array_functions::native_array_push,
        pop => crate::vm::array_functions::native_array_pop,
    },
    String => {
        len => crate::vm::string_functions::native_string_len,
    }
}
```

## Implementation Instructions

### Step 1: Add paste Dependency

**File**: `Cargo.toml`

Add the `paste` crate to dependencies (needed for token concatenation):

```toml
[dependencies]
paste = "1.0"
```

**Rationale**: The `paste` crate enables compile-time string manipulation to generate identifiers like `ARRAY_METHODS` from `Array`.

### Step 2: Create the Macro Module

**File**: `src/common/native_method_registry_macro.rs` (NEW FILE)

**IMPORTANT NOTES**:
- Use `#[macro_export]` to make macro available throughout crate
- The macro must generate FOUR things:
  1. Static arrays (e.g., `ARRAY_METHODS`)
  2. `get_methods_for_type()` function
  3. `get_native_method()` dispatch function with debug assertions
  4. Debug pattern for consistency checking
- Use `paste::paste!` for identifier generation
- Use `stringify!()` to convert identifiers to string literals
- Function paths must be fully qualified (e.g., `crate::vm::array_functions::native_array_push`)

**Macro Structure**:

```rust
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
        // Part 1: Generate static method arrays
        // Example output: static ARRAY_METHODS: &[&str] = &["push", "pop"];
        $(
            paste::paste! {
                static [<$type_name:upper _METHODS>]: &[&str] = &[
                    $(stringify!($method_name)),*
                ];
            }
        )*

        // Part 2: Generate get_methods_for_type() function
        pub fn get_methods_for_type(type_name: &str) -> &'static [&'static str] {
            match type_name {
                $(
                    stringify!($type_name) => paste::paste! { [<$type_name:upper _METHODS>] },
                )*
                _ => &[],
            }
        }

        // Part 3: Generate get_native_method() with debug assertions
        pub fn get_native_method(
            type_name: &str,
            method_name: &str,
        ) -> Option<$crate::common::NativeFn> {
            #[cfg(debug_assertions)]
            {
                // Generate pattern to check if method exists in registry
                let is_valid = matches!(
                    (type_name, method_name),
                    $($(
                        (stringify!($type_name), stringify!($method_name))
                    )|*)|*
                );
                debug_assert!(
                    is_valid == matches!(
                        (type_name, method_name),
                        $($(
                            (stringify!($type_name), stringify!($method_name))
                        )|*)|*
                    ),
                    "Method registry inconsistency detected"
                );
            }

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

**Implementation Details**:
- `$type_name:ident` captures type names like `Array`, `String`
- `$method_name:ident` captures method names like `push`, `len`
- `$function_path:path` captures full paths like `crate::vm::array_functions::native_array_push`
- `$()*` means "repeat for each item"
- `paste::paste! { [<$type_name:upper _METHODS>] }` generates `ARRAY_METHODS` from `Array`

### Step 3: Export Macro from Module

**File**: `src/common/mod.rs`

Add the new module:

```rust
pub mod native_method_registry_macro;
```

### Step 4: Refactor method_registry.rs

**File**: `src/common/method_registry.rs`

**CRITICAL**: This is the single source of truth. ALL method definitions go here.

Replace lines 87-144 (static arrays) with macro invocation:

```rust
use crate::define_native_methods;

// Single source of truth for all native methods
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
```

Keep the `MethodRegistry` struct but simplify it to call macro-generated functions:

```rust
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

**Keep all existing tests unchanged** - they should pass without modification.

### Step 5: Refactor vm/impl.rs

**File**: `src/vm/impl.rs`

Replace the entire `get_native_method()` function (lines 320-391) with a thin wrapper:

```rust
impl VirtualMachine {
    /// Returns the native function implementation for a given type and method name.
    ///
    /// This dispatch table is automatically generated by the define_native_methods! macro
    /// in src/common/method_registry.rs to ensure consistency with compile-time validation.
    pub(in crate::vm) fn get_native_method(
        type_name: &str,
        method_name: &str,
    ) -> Option<crate::common::NativeFn> {
        crate::common::method_registry::get_native_method(type_name, method_name)
    }
}
```

**Delete**:
- Lines 332-355 (debug assertion code)
- Lines 357-390 (match dispatch table)

**Keep**:
- The doc comment explaining the function's purpose
- Update it to mention macro generation

### Step 6: Verify Build

Run cargo check to ensure everything compiles:

```bash
cargo check
```

Expected: No errors, no warnings related to method registry.

### Step 7: Run Tests

Run the full test suite:

```bash
cargo test
```

**Critical tests that MUST pass**:
- `method_registry::tests::test_all_vm_methods_present`
- `method_registry::tests::test_get_methods_for_type_*`
- `method_registry::tests::test_is_valid_method_*`
- `semantic::tests::test_method_call_validation_*`

All existing tests should pass without modification. If any test fails, the macro is generating incorrect output.

### Step 8: Manual Verification

After tests pass, manually verify the macro output by expanding it:

```bash
cargo expand --lib common::method_registry
```

Verify the expanded code contains:
1. Static arrays like `static ARRAY_METHODS: &[&str] = &["push", "pop", ...];`
2. A `get_methods_for_type()` match with all types
3. A `get_native_method()` match with all (type, method) pairs

## Testing Strategy

### Existing Tests (Must Pass Unchanged)

All 27 tests in `src/common/method_registry.rs::tests` must pass:
- Type-specific method queries
- Validation tests
- Suggestion tests
- Consistency tests

All semantic analysis tests in `src/compiler/tests/semantic.rs` must pass.

### New Tests to Add

Add to `src/common/native_method_registry_macro.rs`:

```rust
#[cfg(test)]
mod tests {
    use crate::define_native_methods;

    // Define a minimal test registry
    define_native_methods! {
        TestType => {
            testMethod => crate::vm::array_functions::native_array_push,
        },
    }

    #[test]
    fn test_macro_generates_static_array() {
        assert_eq!(TESTTYPE_METHODS, &["testMethod"]);
    }

    #[test]
    fn test_macro_generates_get_methods() {
        assert_eq!(get_methods_for_type("TestType"), &["testMethod"]);
    }

    #[test]
    fn test_macro_generates_dispatch() {
        assert!(get_native_method("TestType", "testMethod").is_some());
        assert!(get_native_method("TestType", "invalid").is_none());
    }
}
```

## Success Criteria

✅ Cargo build succeeds with no warnings
✅ All existing tests pass without modification
✅ Macro expansion shows correct static arrays, functions, and dispatch
✅ Adding a new method only requires one line in the macro invocation
✅ Debug builds still validate consistency between registry and dispatch
✅ Code is more maintainable and DRY-compliant

## Rollback Plan

If something goes wrong:
1. `git checkout src/common/method_registry.rs` - restore original
2. `git checkout src/vm/impl.rs` - restore original
3. Delete `src/common/native_method_registry_macro.rs`
4. Remove `paste` from `Cargo.toml`
5. Remove module declaration from `src/common/mod.rs`

## Future Enhancements

Once the basic macro works, we can extend it to:
1. Generate method arity validation (argument count checking)
2. Generate type signature strings for better error messages
3. Generate documentation automatically
4. Generate faster lookup tables (e.g., perfect hash maps)

## Notes on Implementation Style

- **Use descriptive variable names** in the macro (e.g., `$type_name`, not `$t`)
- **Add comments** explaining each section of macro expansion
- **Keep macro syntax clean** with trailing commas allowed
- **Preserve existing test coverage** - don't delete or modify tests
- **Update doc comments** to explain macro usage
- **Use fully qualified paths** in macro (e.g., `crate::vm::...`)

## Common Pitfalls to Avoid

❌ Don't use relative paths in macro (e.g., `super::array_functions::...`)
❌ Don't remove existing tests
❌ Don't change function signatures or behavior
❌ Don't forget to export the macro with `#[macro_export]`
❌ Don't assume identifier hygiene - use `$crate::`

✅ Use fully qualified paths starting with `crate::`
✅ Keep all existing tests passing
✅ Maintain exact same runtime behavior
✅ Export macro properly for crate-wide use
✅ Use `$crate::` for hygiene in macro expansion
