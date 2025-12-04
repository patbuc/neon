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
    ("Array", "push", crate::vm::array_functions::native_array_push),
    ("Array", "pop", crate::vm::array_functions::native_array_pop),
    ("Array", "length", crate::vm::array_functions::native_array_length),
    ("Array", "size", crate::vm::array_functions::native_array_size),
    ("Array", "contains", crate::vm::array_functions::native_array_contains),
    // String methods
    ("String", "len", crate::vm::string_functions::native_string_len),
    ("String", "substring", crate::vm::string_functions::native_string_substring),
    ("String", "replace", crate::vm::string_functions::native_string_replace),
    ("String", "split", crate::vm::string_functions::native_string_split),
    ("String", "toInt", crate::vm::string_functions::native_string_to_int),
    ("String", "toFloat", crate::vm::string_functions::native_string_to_float),
    ("String", "toBool", crate::vm::string_functions::native_string_to_bool),
    // Number methods
    ("Number", "toString", crate::vm::number_functions::native_number_to_string),
    // Boolean methods
    ("Boolean", "toString", crate::vm::boolean_functions::native_boolean_to_string),
    // Map methods
    ("Map", "get", crate::vm::map_functions::native_map_get),
    ("Map", "size", crate::vm::map_functions::native_map_size),
    ("Map", "has", crate::vm::map_functions::native_map_has),
    ("Map", "remove", crate::vm::map_functions::native_map_remove),
    ("Map", "keys", crate::vm::map_functions::native_map_keys),
    ("Map", "values", crate::vm::map_functions::native_map_values),
    ("Map", "entries", crate::vm::map_functions::native_map_entries),
    // Set methods
    ("Set", "add", crate::vm::set_functions::native_set_add),
    ("Set", "remove", crate::vm::set_functions::native_set_remove),
    ("Set", "has", crate::vm::set_functions::native_set_has),
    ("Set", "size", crate::vm::set_functions::native_set_size),
    ("Set", "clear", crate::vm::set_functions::native_set_clear),
    ("Set", "union", crate::vm::set_functions::native_set_union),
    ("Set", "intersection", crate::vm::set_functions::native_set_intersection),
    ("Set", "difference", crate::vm::set_functions::native_set_difference),
    ("Set", "isSubset", crate::vm::set_functions::native_set_is_subset),
    ("Set", "toArray", crate::vm::set_functions::native_set_to_array),
    // File methods
    ("File", "read", crate::vm::file_functions::native_file_read),
    ("File", "readLines", crate::vm::file_functions::native_file_read_lines),
];

/// Get native function implementation for a type and method.
pub fn get_native_method(type_name: &str, method_name: &str) -> Option<NativeFn> {
    NATIVE_METHODS
        .iter()
        .find(|(t, m, _)| *t == type_name && *m == method_name)
        .map(|(_, _, f)| *f)
}

/// Get all method names for a given type.
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
    /// Returns a slice of valid method names for the given type.
    ///
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

    /// Checks if a method is valid for the given type.
    ///
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

    /// Suggests a similar method name if the given method is invalid.
    ///
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_methods_for_type_array() {
        let methods = MethodRegistry::get_methods_for_type("Array");
        assert_eq!(methods.len(), 5);
        assert!(methods.contains(&"push"));
        assert!(methods.contains(&"pop"));
        assert!(methods.contains(&"length"));
        assert!(methods.contains(&"size"));
        assert!(methods.contains(&"contains"));
    }

    #[test]
    fn test_get_methods_for_type_string() {
        let methods = MethodRegistry::get_methods_for_type("String");
        assert_eq!(methods.len(), 7);
        assert!(methods.contains(&"len"));
        assert!(methods.contains(&"substring"));
        assert!(methods.contains(&"replace"));
        assert!(methods.contains(&"split"));
        assert!(methods.contains(&"toInt"));
        assert!(methods.contains(&"toFloat"));
        assert!(methods.contains(&"toBool"));
    }

    #[test]
    fn test_get_methods_for_type_number() {
        let methods = MethodRegistry::get_methods_for_type("Number");
        assert_eq!(methods.len(), 1);
        assert!(methods.contains(&"toString"));
    }

    #[test]
    fn test_get_methods_for_type_boolean() {
        let methods = MethodRegistry::get_methods_for_type("Boolean");
        assert_eq!(methods.len(), 1);
        assert!(methods.contains(&"toString"));
    }

    #[test]
    fn test_get_methods_for_type_map() {
        let methods = MethodRegistry::get_methods_for_type("Map");
        assert_eq!(methods.len(), 7);
        assert!(methods.contains(&"get"));
        assert!(methods.contains(&"size"));
        assert!(methods.contains(&"has"));
        assert!(methods.contains(&"remove"));
        assert!(methods.contains(&"keys"));
        assert!(methods.contains(&"values"));
        assert!(methods.contains(&"entries"));
    }

    #[test]
    fn test_get_methods_for_type_set() {
        let methods = MethodRegistry::get_methods_for_type("Set");
        assert_eq!(methods.len(), 10);
        assert!(methods.contains(&"add"));
        assert!(methods.contains(&"remove"));
        assert!(methods.contains(&"has"));
        assert!(methods.contains(&"size"));
        assert!(methods.contains(&"clear"));
        assert!(methods.contains(&"union"));
        assert!(methods.contains(&"intersection"));
        assert!(methods.contains(&"difference"));
        assert!(methods.contains(&"isSubset"));
        assert!(methods.contains(&"toArray"));
    }

    #[test]
    fn test_get_methods_for_type_unknown() {
        let methods = MethodRegistry::get_methods_for_type("UnknownType");
        assert_eq!(methods.len(), 0);
    }

    #[test]
    fn test_is_valid_method_array() {
        assert!(MethodRegistry::is_valid_method("Array", "push"));
        assert!(MethodRegistry::is_valid_method("Array", "pop"));
        assert!(MethodRegistry::is_valid_method("Array", "length"));
        assert!(!MethodRegistry::is_valid_method("Array", "invalid"));
    }

    #[test]
    fn test_is_valid_method_string() {
        assert!(MethodRegistry::is_valid_method("String", "len"));
        assert!(MethodRegistry::is_valid_method("String", "substring"));
        assert!(!MethodRegistry::is_valid_method("String", "length"));
    }

    #[test]
    fn test_is_valid_method_map() {
        assert!(MethodRegistry::is_valid_method("Map", "get"));
        assert!(MethodRegistry::is_valid_method("Map", "has"));
        assert!(!MethodRegistry::is_valid_method("Map", "push"));
    }

    #[test]
    fn test_is_valid_method_set() {
        assert!(MethodRegistry::is_valid_method("Set", "add"));
        assert!(MethodRegistry::is_valid_method("Set", "union"));
        assert!(!MethodRegistry::is_valid_method("Set", "get"));
    }

    #[test]
    fn test_suggest_method_array() {
        // Typo in "push"
        assert_eq!(MethodRegistry::suggest_method("Array", "psh"), Some("push"));
        assert_eq!(MethodRegistry::suggest_method("Array", "puh"), Some("push"));

        // Typo in "pop"
        assert_eq!(MethodRegistry::suggest_method("Array", "poop"), Some("pop"));

        // Typo in "length"
        assert_eq!(
            MethodRegistry::suggest_method("Array", "lenght"),
            Some("length")
        );
    }

    #[test]
    fn test_suggest_method_string() {
        // Typo in "len"
        assert_eq!(MethodRegistry::suggest_method("String", "lenn"), Some("len"));

        // Typo in "split"
        assert_eq!(
            MethodRegistry::suggest_method("String", "splt"),
            Some("split")
        );
    }

    #[test]
    fn test_suggest_method_map() {
        // Typo in "remove"
        assert_eq!(
            MethodRegistry::suggest_method("Map", "remov"),
            Some("remove")
        );

        // Typo in "keys"
        assert_eq!(MethodRegistry::suggest_method("Map", "key"), Some("keys"));
    }

    #[test]
    fn test_suggest_method_set() {
        // Typo in "union"
        assert_eq!(MethodRegistry::suggest_method("Set", "uniom"), Some("union"));

        // Typo in "isSubset"
        assert_eq!(
            MethodRegistry::suggest_method("Set", "isSubet"),
            Some("isSubset")
        );
    }

    #[test]
    fn test_suggest_method_no_match() {
        // Completely wrong method name (too different)
        assert_eq!(MethodRegistry::suggest_method("Array", "xyz"), None);
        assert_eq!(
            MethodRegistry::suggest_method("String", "completely_wrong"),
            None
        );
    }

    #[test]
    fn test_suggest_method_unknown_type() {
        // Unknown type returns None (no methods to suggest from)
        assert_eq!(MethodRegistry::suggest_method("UnknownType", "method"), None);
    }

    #[test]
    fn test_suggest_method_exact_match() {
        // Even exact matches are returned as suggestions
        assert_eq!(
            MethodRegistry::suggest_method("Array", "push"),
            Some("push")
        );
    }

    #[test]
    fn test_case_sensitive() {
        // Method names are case-sensitive
        assert!(!MethodRegistry::is_valid_method("Array", "Push"));
        assert!(!MethodRegistry::is_valid_method("String", "LEN"));
    }

    #[test]
    fn test_get_methods_for_type_file() {
        let methods = MethodRegistry::get_methods_for_type("File");
        assert_eq!(methods.len(), 2);
        assert!(methods.contains(&"read"));
        assert!(methods.contains(&"readLines"));
    }

    #[test]
    fn test_is_valid_method_file() {
        assert!(MethodRegistry::is_valid_method("File", "read"));
        assert!(MethodRegistry::is_valid_method("File", "readLines"));
        assert!(!MethodRegistry::is_valid_method("File", "write"));
    }

    #[test]
    fn test_all_vm_methods_present() {
        // Verify that all methods from vm/impl.rs are present

        // Array methods from vm/impl.rs
        assert!(MethodRegistry::is_valid_method("Array", "push"));
        assert!(MethodRegistry::is_valid_method("Array", "pop"));
        assert!(MethodRegistry::is_valid_method("Array", "length"));
        assert!(MethodRegistry::is_valid_method("Array", "size"));
        assert!(MethodRegistry::is_valid_method("Array", "contains"));

        // String methods from vm/impl.rs
        assert!(MethodRegistry::is_valid_method("String", "len"));
        assert!(MethodRegistry::is_valid_method("String", "substring"));
        assert!(MethodRegistry::is_valid_method("String", "replace"));
        assert!(MethodRegistry::is_valid_method("String", "split"));
        assert!(MethodRegistry::is_valid_method("String", "toInt"));
        assert!(MethodRegistry::is_valid_method("String", "toFloat"));
        assert!(MethodRegistry::is_valid_method("String", "toBool"));

        // Number methods from vm/impl.rs
        assert!(MethodRegistry::is_valid_method("Number", "toString"));

        // Boolean methods from vm/impl.rs
        assert!(MethodRegistry::is_valid_method("Boolean", "toString"));

        // Map methods from vm/impl.rs
        assert!(MethodRegistry::is_valid_method("Map", "get"));
        assert!(MethodRegistry::is_valid_method("Map", "size"));
        assert!(MethodRegistry::is_valid_method("Map", "has"));
        assert!(MethodRegistry::is_valid_method("Map", "remove"));
        assert!(MethodRegistry::is_valid_method("Map", "keys"));
        assert!(MethodRegistry::is_valid_method("Map", "values"));
        assert!(MethodRegistry::is_valid_method("Map", "entries"));

        // Set methods from vm/impl.rs
        assert!(MethodRegistry::is_valid_method("Set", "add"));
        assert!(MethodRegistry::is_valid_method("Set", "remove"));
        assert!(MethodRegistry::is_valid_method("Set", "has"));
        assert!(MethodRegistry::is_valid_method("Set", "size"));
        assert!(MethodRegistry::is_valid_method("Set", "clear"));
        assert!(MethodRegistry::is_valid_method("Set", "union"));
        assert!(MethodRegistry::is_valid_method("Set", "intersection"));
        assert!(MethodRegistry::is_valid_method("Set", "difference"));
        assert!(MethodRegistry::is_valid_method("Set", "isSubset"));
        assert!(MethodRegistry::is_valid_method("Set", "toArray"));
    }
}
