use crate::common::constants::VARIADIC_ARITY;
use crate::common::stdlib;
use crate::common::string_similarity::find_closest_match;
use crate::common::NativeFn;

/// Classifies native callable functions by their calling convention.
#[derive(Debug, Clone, Copy)]
pub(crate) enum NativeCallable {
    /// Static method (no receiver): Math.abs(x), JSON.parse(s)
    StaticMethod { function: NativeFn, arity: u8 },
    /// Instance method (receiver as first arg): arr.push(x), str.len()
    InstanceMethod { function: NativeFn, arity: u8 },
    /// Constructor (creates new instance): File(path)
    Constructor { function: NativeFn, arity: u8 },
}

impl NativeCallable {
    pub fn function(&self) -> NativeFn {
        match self {
            NativeCallable::StaticMethod { function, .. } => *function,
            NativeCallable::InstanceMethod { function, .. } => *function,
            NativeCallable::Constructor { function, .. } => *function,
        }
    }

    pub fn arity(&self) -> u8 {
        match self {
            NativeCallable::StaticMethod { arity, .. } => *arity,
            NativeCallable::InstanceMethod { arity, .. } => *arity,
            NativeCallable::Constructor { arity, .. } => *arity,
        }
    }
}

/// Static registry of all native methods - SINGLE SOURCE OF TRUTH.
///
/// Format: (type_name, method_name, NativeCallable)
const NATIVE_METHODS: &[(&str, &str, NativeCallable)] = &[
    // Math static methods
    (
        "Math",
        "abs",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_abs,
            arity: 1,
        },
    ),
    (
        "Math",
        "floor",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_floor,
            arity: 1,
        },
    ),
    (
        "Math",
        "ceil",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_ceil,
            arity: 1,
        },
    ),
    (
        "Math",
        "sqrt",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_sqrt,
            arity: 1,
        },
    ),
    (
        "Math",
        "min",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_min,
            arity: VARIADIC_ARITY,
        },
    ),
    (
        "Math",
        "max",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_max,
            arity: VARIADIC_ARITY,
        },
    ),
    // Array instance methods
    (
        "Array",
        "push",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_push,
            arity: 1,
        },
    ),
    (
        "Array",
        "pop",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_pop,
            arity: 0,
        },
    ),
    (
        "Array",
        "length",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_length,
            arity: 0,
        },
    ),
    (
        "Array",
        "size",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_size,
            arity: 0,
        },
    ),
    (
        "Array",
        "contains",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_contains,
            arity: 1,
        },
    ),
    (
        "Array",
        "sort",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_sort,
            arity: 0,
        },
    ),
    (
        "Array",
        "reverse",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_reverse,
            arity: 0,
        },
    ),
    (
        "Array",
        "slice",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_slice,
            arity: 2,
        },
    ),
    (
        "Array",
        "join",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_join,
            arity: 1,
        },
    ),
    (
        "Array",
        "indexOf",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_index_of,
            arity: 1,
        },
    ),
    (
        "Array",
        "sum",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_sum,
            arity: 0,
        },
    ),
    (
        "Array",
        "min",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_min,
            arity: 0,
        },
    ),
    (
        "Array",
        "max",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_max,
            arity: 0,
        },
    ),
    // String instance methods
    (
        "String",
        "len",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_len,
            arity: 0,
        },
    ),
    (
        "String",
        "substring",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_substring,
            arity: 2,
        },
    ),
    (
        "String",
        "replace",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_replace,
            arity: 2,
        },
    ),
    (
        "String",
        "split",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_split,
            arity: 1,
        },
    ),
    (
        "String",
        "toInt",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_to_int,
            arity: 0,
        },
    ),
    (
        "String",
        "toFloat",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_to_float,
            arity: 0,
        },
    ),
    (
        "String",
        "toBool",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_to_bool,
            arity: 0,
        },
    ),
    (
        "String",
        "trim",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_trim,
            arity: 0,
        },
    ),
    (
        "String",
        "startsWith",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_starts_with,
            arity: 1,
        },
    ),
    (
        "String",
        "endsWith",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_ends_with,
            arity: 1,
        },
    ),
    (
        "String",
        "indexOf",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_index_of,
            arity: 1,
        },
    ),
    (
        "String",
        "charAt",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_char_at,
            arity: 1,
        },
    ),
    (
        "String",
        "toUpperCase",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_to_upper_case,
            arity: 0,
        },
    ),
    (
        "String",
        "toLowerCase",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_to_lower_case,
            arity: 0,
        },
    ),
    // Number instance methods
    (
        "Number",
        "toString",
        NativeCallable::InstanceMethod {
            function: stdlib::number_functions::native_number_to_string,
            arity: 0,
        },
    ),
    // Boolean instance methods
    (
        "Boolean",
        "toString",
        NativeCallable::InstanceMethod {
            function: stdlib::boolean_functions::native_boolean_to_string,
            arity: 0,
        },
    ),
    // Map instance methods
    (
        "Map",
        "get",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_get,
            arity: 1,
        },
    ),
    (
        "Map",
        "size",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_size,
            arity: 0,
        },
    ),
    (
        "Map",
        "has",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_has,
            arity: 1,
        },
    ),
    (
        "Map",
        "remove",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_remove,
            arity: 1,
        },
    ),
    (
        "Map",
        "keys",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_keys,
            arity: 0,
        },
    ),
    (
        "Map",
        "values",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_values,
            arity: 0,
        },
    ),
    (
        "Map",
        "entries",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_entries,
            arity: 0,
        },
    ),
    // Set instance methods
    (
        "Set",
        "add",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_add,
            arity: 1,
        },
    ),
    (
        "Set",
        "remove",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_remove,
            arity: 1,
        },
    ),
    (
        "Set",
        "has",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_has,
            arity: 1,
        },
    ),
    (
        "Set",
        "size",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_size,
            arity: 0,
        },
    ),
    (
        "Set",
        "clear",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_clear,
            arity: 0,
        },
    ),
    (
        "Set",
        "union",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_union,
            arity: 1,
        },
    ),
    (
        "Set",
        "intersection",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_intersection,
            arity: 1,
        },
    ),
    (
        "Set",
        "difference",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_difference,
            arity: 1,
        },
    ),
    (
        "Set",
        "isSubset",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_is_subset,
            arity: 1,
        },
    ),
    (
        "Set",
        "toArray",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_to_array,
            arity: 0,
        },
    ),
    // File constructor
    (
        "File",
        "new",
        NativeCallable::Constructor {
            function: stdlib::file_functions::native_file_constructor,
            arity: 1,
        },
    ),
    // File instance methods
    (
        "File",
        "read",
        NativeCallable::InstanceMethod {
            function: stdlib::file_functions::native_file_read,
            arity: 0,
        },
    ),
    (
        "File",
        "readLines",
        NativeCallable::InstanceMethod {
            function: stdlib::file_functions::native_file_read_lines,
            arity: 0,
        },
    ),
    (
        "File",
        "write",
        NativeCallable::InstanceMethod {
            function: stdlib::file_functions::native_file_write,
            arity: 1,
        },
    ),
];

/// Get a native method by type and method name (O(n) - use for runtime fallback only)
pub(crate) fn get_native_method(
    type_name: &str,
    method_name: &str,
) -> Option<&'static NativeCallable> {
    NATIVE_METHODS
        .iter()
        .find(|(t, m, _)| *t == type_name && *m == method_name)
        .map(|(_, _, callable)| callable)
}

/// Get the registry index for a native method (O(n) - but called at compile time)
/// Returns None if the method doesn't exist
pub fn get_native_method_index(type_name: &str, method_name: &str) -> Option<usize> {
    NATIVE_METHODS
        .iter()
        .position(|(t, m, _)| *t == type_name && *m == method_name)
}

/// Get a native method by registry index (O(1) - use at runtime)
pub(crate) fn get_native_method_by_index(index: usize) -> Option<&'static NativeCallable> {
    NATIVE_METHODS.get(index).map(|(_, _, callable)| callable)
}

pub fn get_methods_for_type(type_name: &str) -> Vec<&'static str> {
    NATIVE_METHODS
        .iter()
        .filter(|(t, _, _)| *t == type_name)
        .map(|(_, m, _)| *m)
        .collect()
}

/// Static method registry for validating method calls at compile time.
pub struct MethodRegistry;

impl MethodRegistry {
    /// # Arguments
    /// * `type_name` - The name of the type (e.g., "Array", "String", "Map")
    ///
    /// # Returns
    /// A vector of method names valid for this type, or an empty vector if the type is unknown.
    ///
    /// # Examples
    /// ```ignore
    /// use neon::common::method_registry::MethodRegistry;
    ///
    /// let methods = MethodRegistry::get_methods_for_type("Array");
    /// assert!(methods.contains(&"push"));
    /// ```
    pub fn get_methods_for_type(type_name: &str) -> Vec<&'static str> {
        get_methods_for_type(type_name)
    }

    /// # Arguments
    /// * `type_name` - The name of the type (e.g., "Array", "String", "Map")
    /// * `method_name` - The name of the method to validate
    ///
    /// # Returns
    /// `true` if the method exists for this type, `false` otherwise.
    ///
    /// # Examples
    /// ```ignore
    /// use neon::common::method_registry::MethodRegistry;
    ///
    /// assert!(MethodRegistry::is_valid_method("Array", "push"));
    /// assert!(!MethodRegistry::is_valid_method("Array", "foo"));
    /// ```
    pub fn is_valid_method(type_name: &str, method_name: &str) -> bool {
        get_native_method(type_name, method_name).is_some()
    }

    /// Uses Levenshtein distance to find the closest matching method name
    /// within a threshold of 2 edits.
    ///
    /// # Arguments
    /// * `type_name` - The name of the type (e.g., "Array", "String", "Map")
    /// * `method_name` - The invalid method name to find suggestions for
    ///
    /// # Returns
    /// * `Some(&str)` - A suggested method name if one is found within the threshold
    /// * `None` - If no similar method exists
    ///
    /// # Examples
    /// ```ignore
    /// use neon::common::method_registry::MethodRegistry;
    ///
    /// assert_eq!(MethodRegistry::suggest_method("Array", "psh"), Some("push"));
    /// assert_eq!(MethodRegistry::suggest_method("Array", "lenght"), Some("length"));
    /// assert_eq!(MethodRegistry::suggest_method("Array", "xyz"), None);
    /// ```
    pub fn suggest_method(type_name: &str, method_name: &str) -> Option<&'static str> {
        let methods = Self::get_methods_for_type(type_name);
        find_closest_match(method_name, &methods)
    }
}
