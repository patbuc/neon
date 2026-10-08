use crate::common::constants::VARIADIC_ARITY;
use crate::common::static_type::StaticType;
use crate::common::stdlib;
use crate::common::string_similarity::find_closest_match;
use crate::common::{NativeFn, NativeFnWithVm};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::OnceLock;

/// Classifies native callable functions by their calling convention.
#[derive(Debug, Clone)]
pub(crate) enum NativeCallable {
    /// Static method (no receiver): print(x), String.fromCharCode(c)
    StaticMethod {
        function: NativeFn,
        #[allow(dead_code)]
        arity: u8,
    },
    /// Instance method (receiver as first arg): arr.push(x), str.size()
    InstanceMethod {
        function: NativeFn,
        #[allow(dead_code)]
        arity: u8,
        returns: Option<StaticType>,
    },
    /// Instance method that calls back into Neon code, so it needs the VM:
    /// arr.map(fn), arr.filter(fn), arr.reduce(initial, fn)
    InstanceMethodWithVm {
        function: NativeFnWithVm,
        #[allow(dead_code)]
        arity: u8,
        returns: Option<StaticType>,
    },
    /// Constructor that calls back into Neon code, so it needs the VM:
    /// Array(n, init) calling init(i)
    ConstructorWithVm {
        function: NativeFnWithVm,
        #[allow(dead_code)]
        arity: u8,
    },
}

impl NativeCallable {
    pub fn arity(&self) -> u8 {
        match self {
            NativeCallable::StaticMethod { arity, .. } => *arity,
            NativeCallable::InstanceMethod { arity, .. } => *arity,
            NativeCallable::InstanceMethodWithVm { arity, .. } => *arity,
            NativeCallable::ConstructorWithVm { arity, .. } => *arity,
        }
    }

    fn returns(&self) -> Option<&StaticType> {
        match self {
            NativeCallable::InstanceMethod { returns, .. } => returns.as_ref(),
            NativeCallable::InstanceMethodWithVm { returns, .. } => returns.as_ref(),
            NativeCallable::StaticMethod { .. } | NativeCallable::ConstructorWithVm { .. } => None,
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
    (
        "",
        "sleep",
        NativeCallable::StaticMethod {
            function: stdlib::system_functions::native_system_sleep,
            arity: 1,
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
            returns: Some(StaticType::Boolean),
        },
    ),
    (
        "Array",
        "isEmpty",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_is_empty,
            arity: 0,
            returns: Some(StaticType::Boolean),
        },
    ),
    (
        "Array",
        "sort",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_sort,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "sortBy",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_sort_by,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "minBy",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_min_by,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Array",
        "maxBy",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_max_by,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Array",
        "groupBy",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_group_by,
            arity: 1,
            returns: Some(StaticType::Map),
        },
    ),
    (
        "Array",
        "takeWhile",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_take_while,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "dropWhile",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_drop_while,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "partition",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_partition,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "distinct",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_distinct,
            arity: 0,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "scan",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_scan,
            arity: 2,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "windowed",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_windowed,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "tally",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_tally,
            arity: 0,
            returns: Some(StaticType::Map),
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
    (
        "Array",
        "forEach",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_for_each,
            arity: 1,
            returns: Some(StaticType::Nil),
        },
    ),
    (
        "Array",
        "flatMap",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_flat_map,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "find",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_find,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Array",
        "some",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_some,
            arity: 1,
            returns: Some(StaticType::Boolean),
        },
    ),
    (
        "Array",
        "every",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::array_functions::native_array_every,
            arity: 1,
            returns: Some(StaticType::Boolean),
        },
    ),
    (
        "Array",
        "flat",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_flat,
            arity: 0,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "copy",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_copy,
            arity: 0,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "take",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_take,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "drop",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_drop,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "first",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_first,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Array",
        "last",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_last,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Array",
        "chunked",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_chunked,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "zip",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_zip,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Array",
        "withIndex",
        NativeCallable::InstanceMethod {
            function: stdlib::array_functions::native_array_with_index,
            arity: 0,
            returns: Some(StaticType::Array),
        },
    ),
    // Array constructor
    (
        "Array",
        "new",
        NativeCallable::ConstructorWithVm {
            function: stdlib::array_functions::native_array_constructor,
            arity: 2,
        },
    ),
    // Range instance methods
    (
        "Range",
        "size",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_size,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Range",
        "contains",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_contains,
            arity: 1,
            returns: Some(StaticType::Boolean),
        },
    ),
    (
        "Range",
        "isEmpty",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_is_empty,
            arity: 0,
            returns: Some(StaticType::Boolean),
        },
    ),
    (
        "Range",
        "toArray",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_to_array,
            arity: 0,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "step",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_step,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "slice",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_slice,
            arity: 2,
            returns: None,
        },
    ),
    (
        "Range",
        "join",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_join,
            arity: 1,
            returns: Some(StaticType::String),
        },
    ),
    (
        "Range",
        "indexOf",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_index_of,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Range",
        "sum",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_sum,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Range",
        "min",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_min,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Range",
        "max",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_max,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Range",
        "map",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::range_functions::native_range_map,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "filter",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::range_functions::native_range_filter,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "reduce",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::range_functions::native_range_reduce,
            arity: 2,
            returns: None,
        },
    ),
    (
        "Range",
        "forEach",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::range_functions::native_range_for_each,
            arity: 1,
            returns: Some(StaticType::Nil),
        },
    ),
    (
        "Range",
        "flatMap",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::range_functions::native_range_flat_map,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "sortBy",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::range_functions::native_range_sort_by,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "minBy",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::range_functions::native_range_min_by,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Range",
        "maxBy",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::range_functions::native_range_max_by,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Range",
        "groupBy",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::range_functions::native_range_group_by,
            arity: 1,
            returns: Some(StaticType::Map),
        },
    ),
    (
        "Range",
        "takeWhile",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::range_functions::native_range_take_while,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "dropWhile",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::range_functions::native_range_drop_while,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "partition",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::range_functions::native_range_partition,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "distinct",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_distinct,
            arity: 0,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "scan",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::range_functions::native_range_scan,
            arity: 2,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "windowed",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_windowed,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "tally",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_tally,
            arity: 0,
            returns: Some(StaticType::Map),
        },
    ),
    (
        "Range",
        "take",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_take,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "drop",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_drop,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "first",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_first,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Range",
        "last",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_last,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Range",
        "chunked",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_chunked,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "zip",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_zip,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "withIndex",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_with_index,
            arity: 0,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Range",
        "push",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_push,
            arity: 1,
            returns: None,
        },
    ),
    (
        "Range",
        "pop",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_pop,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Range",
        "sort",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_sort,
            arity: 0,
            returns: None,
        },
    ),
    (
        "Range",
        "reverse",
        NativeCallable::InstanceMethod {
            function: stdlib::range_functions::native_range_reverse,
            arity: 0,
            returns: None,
        },
    ),
    // String instance methods
    (
        "String",
        "size",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_size,
            arity: 0,
            returns: None,
        },
    ),
    (
        "String",
        "isEmpty",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_is_empty,
            arity: 0,
            returns: Some(StaticType::Boolean),
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
        "chars",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_chars,
            arity: 0,
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
        "charCodeAt",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_char_code_at,
            arity: 1,
            returns: Some(StaticType::Number),
        },
    ),
    (
        "String",
        "fromCharCode",
        NativeCallable::StaticMethod {
            function: stdlib::string_functions::native_string_from_char_code,
            arity: 1,
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
    (
        "String",
        "repeat",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_repeat,
            arity: 1,
            returns: Some(StaticType::String),
        },
    ),
    (
        "String",
        "padStart",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_pad_start,
            arity: 2,
            returns: Some(StaticType::String),
        },
    ),
    (
        "String",
        "padEnd",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_pad_end,
            arity: 2,
            returns: Some(StaticType::String),
        },
    ),
    (
        "String",
        "lastIndexOf",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_last_index_of,
            arity: 1,
            returns: None,
        },
    ),
    (
        "String",
        "contains",
        NativeCallable::InstanceMethod {
            function: stdlib::string_functions::native_string_contains,
            arity: 1,
            returns: Some(StaticType::Boolean),
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
    (
        "Number",
        "toInt",
        NativeCallable::InstanceMethod {
            function: stdlib::number_functions::native_number_to_int,
            arity: 0,
            returns: Some(StaticType::Number),
        },
    ),
    (
        "Number",
        "toFloat",
        NativeCallable::InstanceMethod {
            function: stdlib::number_functions::native_number_to_float,
            arity: 0,
            returns: Some(StaticType::Number),
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
        "contains",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_contains,
            arity: 1,
            returns: Some(StaticType::Boolean),
        },
    ),
    (
        "Map",
        "isEmpty",
        NativeCallable::InstanceMethod {
            function: stdlib::map_functions::native_map_is_empty,
            arity: 0,
            returns: Some(StaticType::Boolean),
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
    (
        "Map",
        "forEach",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::map_functions::native_map_for_each,
            arity: 1,
            returns: Some(StaticType::Nil),
        },
    ),
    (
        "Map",
        "map",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::map_functions::native_map_map,
            arity: 1,
            returns: Some(StaticType::Array),
        },
    ),
    (
        "Map",
        "filter",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::map_functions::native_map_filter,
            arity: 1,
            returns: Some(StaticType::Map),
        },
    ),
    (
        "Map",
        "mapValues",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::map_functions::native_map_map_values,
            arity: 1,
            returns: Some(StaticType::Map),
        },
    ),
    (
        "Map",
        "some",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::map_functions::native_map_some,
            arity: 1,
            returns: Some(StaticType::Boolean),
        },
    ),
    (
        "Map",
        "every",
        NativeCallable::InstanceMethodWithVm {
            function: stdlib::map_functions::native_map_every,
            arity: 1,
            returns: Some(StaticType::Boolean),
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
        "contains",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_contains,
            arity: 1,
            returns: Some(StaticType::Boolean),
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
        "isEmpty",
        NativeCallable::InstanceMethod {
            function: stdlib::set_functions::native_set_is_empty,
            arity: 0,
            returns: Some(StaticType::Boolean),
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
    // PriorityQueue instance methods
    (
        "PriorityQueue",
        "push",
        NativeCallable::InstanceMethod {
            function: stdlib::priority_queue_functions::native_priority_queue_push,
            arity: 2,
            returns: None,
        },
    ),
    (
        "PriorityQueue",
        "pop",
        NativeCallable::InstanceMethod {
            function: stdlib::priority_queue_functions::native_priority_queue_pop,
            arity: 0,
            returns: None,
        },
    ),
    (
        "PriorityQueue",
        "peek",
        NativeCallable::InstanceMethod {
            function: stdlib::priority_queue_functions::native_priority_queue_peek,
            arity: 0,
            returns: None,
        },
    ),
    (
        "PriorityQueue",
        "size",
        NativeCallable::InstanceMethod {
            function: stdlib::priority_queue_functions::native_priority_queue_size,
            arity: 0,
            returns: None,
        },
    ),
    (
        "PriorityQueue",
        "isEmpty",
        NativeCallable::InstanceMethod {
            function: stdlib::priority_queue_functions::native_priority_queue_is_empty,
            arity: 0,
            returns: Some(StaticType::Boolean),
        },
    ),
    // Builtin module functions, keyed by module path
    (
        "std/math",
        "abs",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_abs,
            arity: 1,
        },
    ),
    (
        "std/math",
        "floor",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_floor,
            arity: 1,
        },
    ),
    (
        "std/math",
        "ceil",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_ceil,
            arity: 1,
        },
    ),
    (
        "std/math",
        "sqrt",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_sqrt,
            arity: 1,
        },
    ),
    (
        "std/math",
        "min",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_min,
            arity: VARIADIC_ARITY,
        },
    ),
    (
        "std/math",
        "max",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_max,
            arity: VARIADIC_ARITY,
        },
    ),
    (
        "std/math",
        "div",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_div,
            arity: 2,
        },
    ),
    (
        "std/math",
        "round",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_round,
            arity: 1,
        },
    ),
    (
        "std/math",
        "sign",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_sign,
            arity: 1,
        },
    ),
    (
        "std/math",
        "gcd",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_gcd,
            arity: 2,
        },
    ),
    (
        "std/math",
        "lcm",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_lcm,
            arity: 2,
        },
    ),
    (
        "std/math",
        "mod",
        NativeCallable::StaticMethod {
            function: stdlib::math_functions::native_math_mod,
            arity: 2,
        },
    ),
    (
        "std/stdin",
        "read",
        NativeCallable::StaticMethod {
            function: stdlib::stdin_functions::native_stdin_read,
            arity: 0,
        },
    ),
    (
        "std/stdin",
        "readLines",
        NativeCallable::StaticMethod {
            function: stdlib::stdin_functions::native_stdin_read_lines,
            arity: 0,
        },
    ),
    (
        "std/file",
        "open",
        NativeCallable::StaticMethod {
            function: stdlib::file_functions::native_file_constructor,
            arity: 1,
        },
    ),
    (
        "std/pq",
        "new",
        NativeCallable::StaticMethod {
            function: stdlib::priority_queue_functions::native_priority_queue_constructor,
            arity: 0,
        },
    ),
    // Error constructor
    (
        "Error",
        "new",
        NativeCallable::StaticMethod {
            function: stdlib::error_functions::native_error_constructor,
            arity: 1,
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

/// Native methods indexed by method symbol, then by builtin type symbol.
pub(crate) type NativeMethodTable =
    Vec<[Option<&'static NativeCallable>; BUILTIN_TYPE_NAMES.len()]>;

/// Resolves every interned name against every builtin type once, so runtime
/// dispatch can index by symbol ids instead of hashing names.
pub(crate) fn native_method_table(symbols: &[Rc<str>]) -> NativeMethodTable {
    symbols
        .iter()
        .map(|method_name| {
            BUILTIN_TYPE_NAMES.map(|type_name| get_native_method_by_name(type_name, method_name))
        })
        .collect()
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

/// The arity of the native callable at this registry index.
pub fn native_arity(index: usize) -> u8 {
    NATIVE_METHODS[index].2.arity()
}

/// The display label for the native callable at this registry index: the
/// function name for a global, `"{Type}.new"` for a constructor, or the
/// bare method name for a static method.
pub fn native_label(index: usize) -> String {
    let (type_name, method_name, callable) = &NATIVE_METHODS[index];
    match callable {
        NativeCallable::ConstructorWithVm { .. } => {
            format!("{}.new", type_name)
        }
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

/// Runtime type names `get_type_name` (`src/vm/functions.rs`) returns for
/// builtin values. A struct may not be declared under one of these names -
/// the semantic pass infers types by name alone, so a user instance and a
/// builtin value would otherwise be indistinguishable.
pub const BUILTIN_TYPE_NAMES: [&str; 9] = [
    "Array",
    "String",
    "Map",
    "Set",
    "Number",
    "Boolean",
    "File",
    "Range",
    "PriorityQueue",
];

/// The paths of the builtin `std/` modules, sorted.
pub fn builtin_modules() -> Vec<&'static str> {
    let mut paths: Vec<&'static str> = NATIVE_METHODS
        .iter()
        .map(|(type_name, _, _)| *type_name)
        .filter(|type_name| type_name.starts_with("std/"))
        .collect();
    paths.sort_unstable();
    paths.dedup();
    paths
}

/// The registry index of a static method, or None if there is no such
/// static method.
pub fn static_method_index(type_name: &str, method_name: &str) -> Option<usize> {
    NATIVE_METHODS.iter().position(|(t, m, callable)| {
        *t == type_name
            && *m == method_name
            && matches!(callable, NativeCallable::StaticMethod { .. })
    })
}

pub fn has_static_methods(type_name: &str) -> bool {
    NATIVE_METHODS.iter().any(|(t, _, callable)| {
        *t == type_name && matches!(callable, NativeCallable::StaticMethod { .. })
    })
}

pub fn suggest_method(type_name: &str, method_name: &str) -> Option<&'static str> {
    let methods = get_methods_for_type(type_name);
    find_closest_match(method_name, &methods)
}

pub fn is_valid_method(type_name: &str, method_name: &str) -> bool {
    get_native_method_by_name(type_name, method_name).is_some()
}
