use crate::common::stdlib::{create_builtin_objects, BUILTIN_VALUES};

#[test]
fn builtin_values_match_created_objects() {
    let created = create_builtin_objects(Vec::new());
    let declared: Vec<&str> = BUILTIN_VALUES.iter().map(|(name, _)| *name).collect();
    let actual: Vec<&str> = created.keys().map(|name| name.as_str()).collect();
    assert_eq!(declared, actual);
}
