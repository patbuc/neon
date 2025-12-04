# Alternative Approach: Simple Static Registry

## Problem with Current Macro

The `define_native_methods!` macro is hard to read because:
- Nested repetition syntax `$($(...))*)*`
- `paste::paste!` token manipulation
- Multiple levels of macro expansion
- Debug assertions duplicated with identical patterns

## Better Solution: Const Static Arrays

Use simple, readable const arrays with a struct-based registry.

### Approach 1: Struct-Based Registry (RECOMMENDED)

**File**: `src/common/method_registry.rs`

```rust
use crate::common::NativeFn;
use crate::common::string_similarity::find_closest_match;

/// A single method entry in the registry
pub struct MethodEntry {
    pub type_name: &'static str,
    pub method_name: &'static str,
    pub function: NativeFn,
}

/// Static registry of all native methods - SINGLE SOURCE OF TRUTH
const NATIVE_METHODS: &[MethodEntry] = &[
    // Array methods
    MethodEntry {
        type_name: "Array",
        method_name: "push",
        function: crate::vm::array_functions::native_array_push
    },
    MethodEntry {
        type_name: "Array",
        method_name: "pop",
        function: crate::vm::array_functions::native_array_pop
    },
    MethodEntry {
        type_name: "Array",
        method_name: "length",
        function: crate::vm::array_functions::native_array_length
    },
    MethodEntry {
        type_name: "Array",
        method_name: "size",
        function: crate::vm::array_functions::native_array_size
    },
    MethodEntry {
        type_name: "Array",
        method_name: "contains",
        function: crate::vm::array_functions::native_array_contains
    },

    // String methods
    MethodEntry {
        type_name: "String",
        method_name: "len",
        function: crate::vm::string_functions::native_string_len
    },
    MethodEntry {
        type_name: "String",
        method_name: "substring",
        function: crate::vm::string_functions::native_string_substring
    },
    MethodEntry {
        type_name: "String",
        method_name: "replace",
        function: crate::vm::string_functions::native_string_replace
    },
    MethodEntry {
        type_name: "String",
        method_name: "split",
        function: crate::vm::string_functions::native_string_split
    },
    MethodEntry {
        type_name: "String",
        method_name: "toInt",
        function: crate::vm::string_functions::native_string_to_int
    },
    MethodEntry {
        type_name: "String",
        method_name: "toFloat",
        function: crate::vm::string_functions::native_string_to_float
    },
    MethodEntry {
        type_name: "String",
        method_name: "toBool",
        function: crate::vm::string_functions::native_string_to_bool
    },

    // Number methods
    MethodEntry {
        type_name: "Number",
        method_name: "toString",
        function: crate::vm::number_functions::native_number_to_string
    },

    // Boolean methods
    MethodEntry {
        type_name: "Boolean",
        method_name: "toString",
        function: crate::vm::boolean_functions::native_boolean_to_string
    },

    // Map methods
    MethodEntry {
        type_name: "Map",
        method_name: "get",
        function: crate::vm::map_functions::native_map_get
    },
    MethodEntry {
        type_name: "Map",
        method_name: "size",
        function: crate::vm::map_functions::native_map_size
    },
    MethodEntry {
        type_name: "Map",
        method_name: "has",
        function: crate::vm::map_functions::native_map_has
    },
    MethodEntry {
        type_name: "Map",
        method_name: "remove",
        function: crate::vm::map_functions::native_map_remove
    },
    MethodEntry {
        type_name: "Map",
        method_name: "keys",
        function: crate::vm::map_functions::native_map_keys
    },
    MethodEntry {
        type_name: "Map",
        method_name: "values",
        function: crate::vm::map_functions::native_map_values
    },
    MethodEntry {
        type_name: "Map",
        method_name: "entries",
        function: crate::vm::map_functions::native_map_entries
    },

    // Set methods
    MethodEntry {
        type_name: "Set",
        method_name: "add",
        function: crate::vm::set_functions::native_set_add
    },
    MethodEntry {
        type_name: "Set",
        method_name: "remove",
        function: crate::vm::set_functions::native_set_remove
    },
    MethodEntry {
        type_name: "Set",
        method_name: "has",
        function: crate::vm::set_functions::native_set_has
    },
    MethodEntry {
        type_name: "Set",
        method_name: "size",
        function: crate::vm::set_functions::native_set_size
    },
    MethodEntry {
        type_name: "Set",
        method_name: "clear",
        function: crate::vm::set_functions::native_set_clear
    },
    MethodEntry {
        type_name: "Set",
        method_name: "union",
        function: crate::vm::set_functions::native_set_union
    },
    MethodEntry {
        type_name: "Set",
        method_name: "intersection",
        function: crate::vm::set_functions::native_set_intersection
    },
    MethodEntry {
        type_name: "Set",
        method_name: "difference",
        function: crate::vm::set_functions::native_set_difference
    },
    MethodEntry {
        type_name: "Set",
        method_name: "isSubset",
        function: crate::vm::set_functions::native_set_is_subset
    },
    MethodEntry {
        type_name: "Set",
        method_name: "toArray",
        function: crate::vm::set_functions::native_set_to_array
    },
];

/// Get native function implementation for a type and method
pub fn get_native_method(type_name: &str, method_name: &str) -> Option<NativeFn> {
    NATIVE_METHODS
        .iter()
        .find(|entry| entry.type_name == type_name && entry.method_name == method_name)
        .map(|entry| entry.function)
}

/// Get all method names for a given type
pub fn get_methods_for_type(type_name: &str) -> Vec<&'static str> {
    NATIVE_METHODS
        .iter()
        .filter(|entry| entry.type_name == type_name)
        .map(|entry| entry.method_name)
        .collect()
}

/// Public API struct
pub struct MethodRegistry;

impl MethodRegistry {
    pub fn get_methods_for_type(type_name: &str) -> Vec<&'static str> {
        get_methods_for_type(type_name)
    }

    pub fn is_valid_method(type_name: &str, method_name: &str) -> bool {
        get_native_method(type_name, method_name).is_some()
    }

    pub fn suggest_method(type_name: &str, method_name: &str) -> Option<&'static str> {
        let methods = Self::get_methods_for_type(type_name);
        find_closest_match(method_name, &methods)
    }
}
```

### Approach 2: Even Simpler with Grouped Arrays

```rust
/// Array methods with their functions
const ARRAY_METHODS: &[(&str, NativeFn)] = &[
    ("push", crate::vm::array_functions::native_array_push),
    ("pop", crate::vm::array_functions::native_array_pop),
    ("length", crate::vm::array_functions::native_array_length),
    ("size", crate::vm::array_functions::native_array_size),
    ("contains", crate::vm::array_functions::native_array_contains),
];

const STRING_METHODS: &[(&str, NativeFn)] = &[
    ("len", crate::vm::string_functions::native_string_len),
    ("substring", crate::vm::string_functions::native_string_substring),
    ("replace", crate::vm::string_functions::native_string_replace),
    ("split", crate::vm::string_functions::native_string_split),
    ("toInt", crate::vm::string_functions::native_string_to_int),
    ("toFloat", crate::vm::string_functions::native_string_to_float),
    ("toBool", crate::vm::string_functions::native_string_to_bool),
];

// ... other types ...

pub fn get_native_method(type_name: &str, method_name: &str) -> Option<NativeFn> {
    let methods = match type_name {
        "Array" => ARRAY_METHODS,
        "String" => STRING_METHODS,
        "Number" => NUMBER_METHODS,
        "Boolean" => BOOLEAN_METHODS,
        "Map" => MAP_METHODS,
        "Set" => SET_METHODS,
        _ => return None,
    };

    methods
        .iter()
        .find(|(name, _)| *name == method_name)
        .map(|(_, func)| *func)
}
```

## Comparison

### Macro Approach
❌ Hard to read (nested repetitions, paste macro)
❌ Difficult to debug (macro expansion errors)
❌ Requires external dependency (`paste` crate)
✅ Zero runtime cost (compile-time generation)
✅ Type-safe (compiler checks paths)

### Struct-Based Approach (Approach 1)
✅ **Extremely readable** - just a simple array of structs
✅ Easy to debug - it's just normal Rust code
✅ No external dependencies
✅ Single source of truth
✅ Type-safe (compiler checks all function pointers)
⚠️ Tiny runtime cost (linear search through const array)

### Grouped Arrays Approach (Approach 2)
✅ Very readable
✅ Slightly faster (smaller arrays per type)
⚠️ Still some duplication (type names in match)
⚠️ Two places to add methods (array + match)

## Performance Analysis

For 31 methods:
- **Linear search**: ~15-31 comparisons worst case
- **Impact**: Negligible (method calls are expensive anyway)
- **Reality**: This is a non-issue for a language interpreter

## Recommendation

**Use Approach 1 (Struct-Based Registry)** because:
1. Most readable - anyone can understand it immediately
2. True single source of truth - one array, no duplication
3. Zero dependencies (drop `paste` crate)
4. Trivial to add methods - just add one struct to the array
5. Performance cost is negligible for an interpreter

## Migration Path

1. Replace `native_method_registry_macro.rs` with simple const array
2. Remove `paste` dependency from Cargo.toml
3. Update tests (should pass without changes)
4. Much simpler codebase!

## Adding a New Method

**With macro** (hard to read):
```rust
define_native_methods! {
    Array => {
        filter => crate::vm::array_functions::native_array_filter,
    }
}
```

**With struct approach** (crystal clear):
```rust
MethodEntry {
    type_name: "Array",
    method_name: "filter",
    function: crate::vm::array_functions::native_array_filter
},
```

The struct approach is more verbose per-method, but **far more readable** and maintainable.
