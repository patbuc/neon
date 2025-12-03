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
