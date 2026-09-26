use crate::common::method_registry::BUILTIN_TYPE_NAMES;
use crate::compiler::resolutions::Resolutions;

#[test]
fn builtin_type_names_get_ids_in_registry_order() {
    let resolutions = Resolutions::default();
    let names = resolutions.symbol_names();

    let builtin_names: Vec<&str> = names[..BUILTIN_TYPE_NAMES.len()]
        .iter()
        .map(|n| n.as_ref())
        .collect();
    assert_eq!(builtin_names, BUILTIN_TYPE_NAMES);
}

#[test]
fn interning_an_existing_name_returns_its_id() {
    let mut resolutions = Resolutions::default();
    let first = resolutions.intern_symbol("Point").unwrap();
    let second = resolutions.intern_symbol("Point").unwrap();
    assert_eq!(first, second);

    // A builtin re-interned keeps the id it was seeded with.
    let array_id = resolutions.intern_symbol("Array").unwrap();
    assert_eq!(array_id, 0);
}

#[test]
fn interning_beyond_65536_names_fails_but_existing_names_still_succeed() {
    let mut resolutions = Resolutions::default();

    let already = BUILTIN_TYPE_NAMES.len();
    for i in 0..(u16::MAX as usize + 1 - already) {
        let name = format!("sym{}", i);
        assert!(
            resolutions.intern_symbol(&name).is_some(),
            "expected symbol {} to fit within the 65536 limit",
            i
        );
    }

    // The table now holds exactly 65536 names; one more distinct name fails.
    assert!(resolutions.intern_symbol("one_too_many").is_none());

    // An already-interned name still resolves.
    assert!(resolutions.intern_symbol("sym0").is_some());
    assert!(resolutions.intern_symbol("Array").is_some());
}
