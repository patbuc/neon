use crate::common::stdlib::{create_builtin_objects, BUILTIN_VALUES};
use crate::common::Value;

#[test]
fn builtin_values_match_created_objects() {
    let created = create_builtin_objects(vec!["one".to_string(), "two".to_string()]);
    assert_eq!(created.len(), BUILTIN_VALUES.len());

    let args_index = BUILTIN_VALUES
        .iter()
        .position(|(name, _)| *name == "args")
        .expect("args should be a declared builtin");

    match &created[args_index] {
        Value::Array(elements) => {
            let strings: Vec<String> = elements
                .borrow()
                .iter()
                .map(|value| match value {
                    Value::String(s) => s.to_string(),
                    other => panic!("expected a string element, got {other:?}"),
                })
                .collect();
            assert_eq!(strings, vec!["one", "two"]);
        }
        other => panic!("expected args to be an array, got {other:?}"),
    }
}
