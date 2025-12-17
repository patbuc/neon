use crate::common::method_registry::MethodRegistry;

#[test]
fn test_get_methods_for_type_array() {
    let methods = MethodRegistry::get_methods_for_type("Array");
    assert_eq!(methods.len(), 13);
    assert!(methods.contains(&"push"));
    assert!(methods.contains(&"pop"));
    assert!(methods.contains(&"length"));
    assert!(methods.contains(&"size"));
    assert!(methods.contains(&"contains"));
    assert!(methods.contains(&"sort"));
    assert!(methods.contains(&"reverse"));
    assert!(methods.contains(&"slice"));
    assert!(methods.contains(&"join"));
    assert!(methods.contains(&"indexOf"));
    assert!(methods.contains(&"sum"));
    assert!(methods.contains(&"min"));
    assert!(methods.contains(&"max"));
}

#[test]
fn test_get_methods_for_type_string() {
    let methods = MethodRegistry::get_methods_for_type("String");
    assert_eq!(methods.len(), 14);
    assert!(methods.contains(&"len"));
    assert!(methods.contains(&"substring"));
    assert!(methods.contains(&"replace"));
    assert!(methods.contains(&"split"));
    assert!(methods.contains(&"toInt"));
    assert!(methods.contains(&"toFloat"));
    assert!(methods.contains(&"toBool"));
    assert!(methods.contains(&"trim"));
    assert!(methods.contains(&"startsWith"));
    assert!(methods.contains(&"endsWith"));
    assert!(methods.contains(&"indexOf"));
    assert!(methods.contains(&"charAt"));
    assert!(methods.contains(&"toUpperCase"));
    assert!(methods.contains(&"toLowerCase"));
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
    assert!(MethodRegistry::is_valid_method("Array", "sort"));
    assert!(MethodRegistry::is_valid_method("Array", "reverse"));
    assert!(MethodRegistry::is_valid_method("Array", "slice"));
    assert!(MethodRegistry::is_valid_method("Array", "join"));
    assert!(MethodRegistry::is_valid_method("Array", "indexOf"));
    assert!(MethodRegistry::is_valid_method("Array", "sum"));
    assert!(MethodRegistry::is_valid_method("Array", "min"));
    assert!(MethodRegistry::is_valid_method("Array", "max"));
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
    assert_eq!(methods.len(), 4);
    assert!(methods.contains(&"new"));
    assert!(methods.contains(&"read"));
    assert!(methods.contains(&"readLines"));
    assert!(methods.contains(&"write"));
}

#[test]
fn test_is_valid_method_file() {
    assert!(MethodRegistry::is_valid_method("File", "read"));
    assert!(MethodRegistry::is_valid_method("File", "readLines"));
    assert!(MethodRegistry::is_valid_method("File", "write"));
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
    assert!(MethodRegistry::is_valid_method("Array", "sort"));
    assert!(MethodRegistry::is_valid_method("Array", "reverse"));
    assert!(MethodRegistry::is_valid_method("Array", "slice"));
    assert!(MethodRegistry::is_valid_method("Array", "join"));
    assert!(MethodRegistry::is_valid_method("Array", "indexOf"));
    assert!(MethodRegistry::is_valid_method("Array", "sum"));
    assert!(MethodRegistry::is_valid_method("Array", "min"));
    assert!(MethodRegistry::is_valid_method("Array", "max"));

    // String methods from vm/impl.rs
    assert!(MethodRegistry::is_valid_method("String", "len"));
    assert!(MethodRegistry::is_valid_method("String", "substring"));
    assert!(MethodRegistry::is_valid_method("String", "replace"));
    assert!(MethodRegistry::is_valid_method("String", "split"));
    assert!(MethodRegistry::is_valid_method("String", "toInt"));
    assert!(MethodRegistry::is_valid_method("String", "toFloat"));
    assert!(MethodRegistry::is_valid_method("String", "toBool"));
    assert!(MethodRegistry::is_valid_method("String", "trim"));
    assert!(MethodRegistry::is_valid_method("String", "startsWith"));
    assert!(MethodRegistry::is_valid_method("String", "endsWith"));
    assert!(MethodRegistry::is_valid_method("String", "indexOf"));
    assert!(MethodRegistry::is_valid_method("String", "charAt"));
    assert!(MethodRegistry::is_valid_method("String", "toUpperCase"));
    assert!(MethodRegistry::is_valid_method("String", "toLowerCase"));

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
