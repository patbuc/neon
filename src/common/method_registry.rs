use crate::common::constants::VARIADIC_ARITY;
use crate::common::static_type::StaticType;
use crate::common::stdlib;
use crate::common::string_similarity::find_closest_match;
use crate::common::{NativeFn, NativeFnWithVm};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Classifies native callable functions by their calling convention.
#[derive(Debug, Clone)]
pub(crate) enum NativeCallable {
    /// Static method (no receiver): Math.abs(x), JSON.parse(s)
    StaticMethod {
        function: NativeFn,
        #[allow(dead_code)]
        arity: u8,
    },
    /// Instance method (receiver as first arg): arr.push(x), str.len()
    InstanceMethod {
        function: NativeFn,
        #[allow(dead_code)]
        arity: u8,
        returns: Option<StaticType>,
    },
    /// Instance method that calls back into Neon code, so it needs the VM:
    /// arr.map(fn), arr.filter(fn), arr.reduce(fn, initial)
    InstanceMethodWithVm {
        function: NativeFnWithVm,
        #[allow(dead_code)]
        arity: u8,
        returns: Option<StaticType>,
    },
    /// Constructor (creates new instance): File(path)
    Constructor {
        function: NativeFn,
        #[allow(dead_code)]
        arity: u8,
    },
}

impl NativeCallable {
    #[allow(dead_code)]
    pub fn arity(&self) -> u8 {
        match self {
            NativeCallable::StaticMethod { arity, .. } => *arity,
            NativeCallable::InstanceMethod { arity, .. } => *arity,
            NativeCallable::InstanceMethodWithVm { arity, .. } => *arity,
            NativeCallable::Constructor { arity, .. } => *arity,
        }
    }

    fn returns(&self) -> Option<&StaticType> {
        match self {
            NativeCallable::InstanceMethod { returns, .. } => returns.as_ref(),
            NativeCallable::InstanceMethodWithVm { returns, .. } => returns.as_ref(),
            NativeCallable::StaticMethod { .. } | NativeCallable::Constructor { .. } => None,
        }
    }
}

/// Static registry of all native methods - SINGLE SOURCE OF TRUTH.
///
/// Format: (type_name, method_name, NativeCallable)
pub(crate) const NATIVE_METHODS: &[(&str, &str, NativeCallable)] = &[
    // Global functions
    (
        "",
        "print",
        NativeCallable::StaticMethod {
            function: stdlib::system_functions::native_system_print,
            arity: VARIADIC_ARITY,
        },
    ),
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
            returns: None,
        },
    ),
    (
        "Array",
        "pop",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_pop,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Array",
        "length",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_length,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Array",
        "size",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_size,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Array",
        "contains",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_contains,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Array",
        "sort",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_sort,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Array",
        "reverse",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_reverse,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Array",
        "slice",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_slice,
            arity: 2,
            returns: None,
        },
    ),
    (
        "Array",
        "join",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_join,
            arity: 1,
            returns: Some(StaticType::String),
        },
    ),
    (
        "Array",
        "indexOf",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_index_of,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Array",
        "sum",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_sum,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Array",
        "min",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_min,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Array",
        "max",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_max,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Array",
        "map",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_map,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "filter",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_filter,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "reduce",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_reduce,
            arity: 2,
            returns: None,
        },
    ),
    // String instance methods
    (
        "String",
        "len",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_len,
            arity: 0,
            returns: None,
        },
    ),
    (
        "String",
        "substring",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_substring,
            arity: 2,
            returns: None,
        },
    ),
    (
        "String",
        "replace",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_replace,
            arity: 2,
            returns: None,
        },
    ),
    (
        "String",
        "split",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_split,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "String",
        "toInt",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_to_int,
            arity: 0,
            returns: Some(StaticType::Number),
        },
    ),
    (
        "String",
        "toFloat",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_to_float,
            arity: 0,
            returns: Some(StaticType::Number),
        },
    ),
    (
        "String",
        "toBool",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_to_bool,
            arity: 0,
            returns: None,
        },
    ),
    (
        "String",
        "trim",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_trim,
            arity: 0,
            returns: Some(StaticType::String),
        },
    ),
    (
        "String",
        "startsWith",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_starts_with,
            arity: 1,
            returns: None,
        },
    ),
    (
        "String",
        "endsWith",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_ends_with,
            arity: 1,
            returns: None,
        },
    ),
    (
        "String",
        "indexOf",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_index_of,
            arity: 1,
            returns: None,
        },
    ),
    (
        "String",
        "charAt",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_char_at,
            arity: 1,
            returns: Some(StaticType::String),
        },
    ),
    (
        "String",
        "toUpperCase",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_to_upper_case,
            arity: 0,
            returns: Some(StaticType::String),
        },
    ),
    (
        "String",
        "toLowerCase",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_to_lower_case,
            arity: 0,
            returns: Some(StaticType::String),
        },
    ),
    // Number instance methods
    (
        "Number",
        "toString",
        NativeCallable::InstanceMethod {
            function: stdlib::number_functions::native_number_to_string,
            arity: 0,
            returns: Some(StaticType::String),
        },
    ),
    // Boolean instance methods
    (
        "Boolean",
        "toString",
        NativeCallable::InstanceMethod {
            function: stdlib::boolean_functions::native_boolean_to_string,
            arity: 0,
            returns: None,
        },
    ),
    // Map instance methods
    (
        "Map",
        "get",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_get,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Map",
        "size",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_size,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Map",
        "has",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_has,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Map",
        "remove",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_remove,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Map",
        "keys",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_keys,
            arity: 0,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Map",
        "values",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_values,
            arity: 0,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Map",
        "entries",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_entries,
            arity: 0,
            returns: None,
        },
    ),
    // Set instance methods
    (
        "Set",
        "add",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_add,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Set",
        "remove",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_remove,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Set",
        "has",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_has,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Set",
        "size",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_size,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Set",
        "clear",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_clear,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Set",
        "union",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_union,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Set",
        "intersection",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_intersection,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Set",
        "difference",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_difference,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Set",
        "isSubset",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_is_subset,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Set",
        "toArray",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_to_array,
            arity: 0,
            returns: Some(StaticType::Array),
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
            returns: None,
        },
    ),
    (
        "File",
        "readLines",
        NativeCallable::InstanceMethod {
            function: stdlib::file_functions::native_file_read_lines,
            arity: 0,
            returns: None,
        },
    ),
    (
        "File",
        "write",
        NativeCallable::InstanceMethod {
            function: stdlib::file_functions::native_file_write,
            arity: 1,
            returns: None,
        },
    ),
];

/// HashMap for O(1) method lookups at runtime
static METHOD_MAP: OnceLock<HashMap<(&'static str, &'static str), &'static NativeCallable>> =
    OnceLock::new();

/// Initialize the method lookup HashMap (called lazily on first access)
fn init_method_map() -> HashMap<(&'static str, &'static str), &'static NativeCallable> {
    NATIVE_METHODS
        .iter()
        .map(|(type_name, method_name, callable)| ((*type_name, *method_name), callable))
        .collect()
}

/// Get a native method by type and method name (O(1) - HashMap lookup)
pub(crate) fn get_native_method_by_name(
    type_name: &str,
    method_name: &str,
) -> Option<&'static NativeCallable> {
    METHOD_MAP
        .get_or_init(init_method_map)
        .get(&(type_name, method_name))
        .copied()
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

/// The display label for the native callable at this registry index: the
/// function name for a global, `"{Type}.new"` for a constructor, or the
/// bare method name for a static method.
pub fn native_label(index: usize) -> String {
    let (type_name, method_name, callable) = &NATIVE_METHODS[index];
    match callable {
        NativeCallable::Constructor { .. } => format!("{}.new", type_name),
        _ => method_name.to_string(),
    }
}

/// The statically known return type of a builtin instance method, if the
/// registry tracks one - consulted by the semantic analyzer to type the
/// result of a method call for further chaining.
pub fn instance_return_type(type_name: &str, method_name: &str) -> Option<StaticType> {
    get_native_method_by_name(type_name, method_name)
        .and_then(|callable| callable.returns())
        .cloned()
}

pub fn get_methods_for_type(type_name: &str) -> Vec<&'static str> {
    NATIVE_METHODS
        .iter()
        .filter(|(t, _, _)| *t == type_name)
        .map(|(_, m, _)| *m)
        .collect()
}

pub fn get_static_methods_for_type(type_name: &str) -> Vec<&'static str> {
    NATIVE_METHODS
        .iter()
        .filter_map(|(t, m, callable)| {
            if *t == type_name {
                match callable {
                    NativeCallable::StaticMethod { .. } => Some(*m),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect()
}

pub fn is_static_namespace(name: &str) -> bool {
    !get_static_methods_for_type(name).is_empty()
}

/// Runtime type names `get_type_name` (`src/vm/functions.rs`) returns for
/// builtin values. A struct may not be declared under one of these names -
/// the semantic pass infers types by name alone, so a user instance and a
/// builtin value would otherwise be indistinguishable.
pub const BUILTIN_TYPE_NAMES: [&str; 8] = [
    "Array", "String", "Map", "Set", "Number", "Boolean", "File", "Range",
];

/// Names of registry types that are namespaces rather than instance types:
/// callable as `Name.method(...)` (has static methods) or constructible as
/// `Name(...)` (has a constructor). This is the single source of truth the
/// semantic analyzer uses to pre-define Math, File, etc.
pub fn namespaces() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = NATIVE_METHODS
        .iter()
        .filter(|(type_name, _, callable)| {
            !type_name.is_empty()
                && matches!(
                    callable,
                    NativeCallable::StaticMethod { .. } | NativeCallable::Constructor { .. }
                )
        })
        .map(|(type_name, _, _)| *type_name)
        .collect();
    names.sort_unstable();
    names.dedup();
    names
}

/// The arity of a namespace's constructor (e.g. `File.new`), if it has one.
pub fn constructor_arity(type_name: &str) -> Option<u8> {
    match get_native_method_by_name(type_name, "new") {
        Some(NativeCallable::Constructor { arity, .. }) => Some(*arity),
        _ => None,
    }
}

pub fn is_static_method(type_name: &str, method_name: &str) -> bool {
    matches!(
        get_native_method_by_name(type_name, method_name),
        Some(NativeCallable::StaticMethod { .. })
    )
}

pub fn suggest_method(type_name: &str, method_name: &str) -> Option<&'static str> {
    let methods = get_methods_for_type(type_name);
    find_closest_match(method_name, &methods)
}

pub fn is_valid_method(type_name: &str, method_name: &str) -> bool {
    get_native_method_by_name(type_name, method_name).is_some()
}
