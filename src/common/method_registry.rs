/// Centralized registry of built-in methods for native types.
///
/// This module provides compile-time validation of method calls by maintaining
/// a complete list of all valid methods for each built-in type in the Neon language.
use crate::common::string_similarity::find_closest_match;
use crate::common::NativeFn;

/// Static registry of all native methods - SINGLE SOURCE OF TRUTH.
///
/// Format: (type_name, method_name, function)
const NATIVE_METHODS: &[(&str, &str, NativeFn)] = &[
    // Array methods
    (
        "Array",
        "push",
        crate::vm::array_functions::native_array_push,
    ),
    ("Array", "pop", crate::vm::array_functions::native_array_pop),
    (
        "Array",
        "length",
        crate::vm::array_functions::native_array_length,
    ),
    (
        "Array",
        "size",
        crate::vm::array_functions::native_array_size,
    ),
    (
        "Array",
        "contains",
        crate::vm::array_functions::native_array_contains,
    ),
    (
        "Array",
        "sort",
        crate::vm::array_functions::native_array_sort,
    ),
    (
        "Array",
        "reverse",
        crate::vm::array_functions::native_array_reverse,
    ),
    (
        "Array",
        "slice",
        crate::vm::array_functions::native_array_slice,
    ),
    (
        "Array",
        "join",
        crate::vm::array_functions::native_array_join,
    ),
    (
        "Array",
        "indexOf",
        crate::vm::array_functions::native_array_index_of,
    ),
    ("Array", "sum", crate::vm::array_functions::native_array_sum),
    ("Array", "min", crate::vm::array_functions::native_array_min),
    ("Array", "max", crate::vm::array_functions::native_array_max),
    // String methods
    (
        "String",
        "len",
        crate::vm::string_functions::native_string_len,
    ),
    (
        "String",
        "substring",
        crate::vm::string_functions::native_string_substring,
    ),
    (
        "String",
        "replace",
        crate::vm::string_functions::native_string_replace,
    ),
    (
        "String",
        "split",
        crate::vm::string_functions::native_string_split,
    ),
    (
        "String",
        "toInt",
        crate::vm::string_functions::native_string_to_int,
    ),
    (
        "String",
        "toFloat",
        crate::vm::string_functions::native_string_to_float,
    ),
    (
        "String",
        "toBool",
        crate::vm::string_functions::native_string_to_bool,
    ),
    (
        "String",
        "trim",
        crate::vm::string_functions::native_string_trim,
    ),
    (
        "String",
        "startsWith",
        crate::vm::string_functions::native_string_starts_with,
    ),
    (
        "String",
        "endsWith",
        crate::vm::string_functions::native_string_ends_with,
    ),
    (
        "String",
        "indexOf",
        crate::vm::string_functions::native_string_index_of,
    ),
    (
        "String",
        "charAt",
        crate::vm::string_functions::native_string_char_at,
    ),
    (
        "String",
        "toUpperCase",
        crate::vm::string_functions::native_string_to_upper_case,
    ),
    (
        "String",
        "toLowerCase",
        crate::vm::string_functions::native_string_to_lower_case,
    ),
    // Number methods
    (
        "Number",
        "toString",
        crate::vm::number_functions::native_number_to_string,
    ),
    // Boolean methods
    (
        "Boolean",
        "toString",
        crate::vm::boolean_functions::native_boolean_to_string,
    ),
    // Map methods
    ("Map", "get", crate::vm::map_functions::native_map_get),
    ("Map", "size", crate::vm::map_functions::native_map_size),
    ("Map", "has", crate::vm::map_functions::native_map_has),
    ("Map", "remove", crate::vm::map_functions::native_map_remove),
    ("Map", "keys", crate::vm::map_functions::native_map_keys),
    ("Map", "values", crate::vm::map_functions::native_map_values),
    (
        "Map",
        "entries",
        crate::vm::map_functions::native_map_entries,
    ),
    // Set methods
    ("Set", "add", crate::vm::set_functions::native_set_add),
    ("Set", "remove", crate::vm::set_functions::native_set_remove),
    ("Set", "has", crate::vm::set_functions::native_set_has),
    ("Set", "size", crate::vm::set_functions::native_set_size),
    ("Set", "clear", crate::vm::set_functions::native_set_clear),
    ("Set", "union", crate::vm::set_functions::native_set_union),
    (
        "Set",
        "intersection",
        crate::vm::set_functions::native_set_intersection,
    ),
    (
        "Set",
        "difference",
        crate::vm::set_functions::native_set_difference,
    ),
    (
        "Set",
        "isSubset",
        crate::vm::set_functions::native_set_is_subset,
    ),
    (
        "Set",
        "toArray",
        crate::vm::set_functions::native_set_to_array,
    ),
    // File methods
    ("File", "read", crate::vm::file_functions::native_file_read),
    (
        "File",
        "readLines",
        crate::vm::file_functions::native_file_read_lines,
    ),
    (
        "File",
        "write",
        crate::vm::file_functions::native_file_write,
    ),
];

pub(crate) fn get_native_method(type_name: &str, method_name: &str) -> Option<NativeFn> {
    NATIVE_METHODS
        .iter()
        .find(|(t, m, _)| *t == type_name && *m == method_name)
        .map(|(_, _, f)| *f)
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
    /// ```
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
    /// ```
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
    /// ```
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
