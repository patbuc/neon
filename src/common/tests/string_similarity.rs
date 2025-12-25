use crate::common::string_similarity::*;

#[test]
fn test_levenshtein_distance_identical_strings() {
    assert_eq!(levenshtein_distance("hello", "hello"), 0);
    assert_eq!(levenshtein_distance("", ""), 0);
}

#[test]
fn test_levenshtein_distance_empty_strings() {
    assert_eq!(levenshtein_distance("", "hello"), 5);
    assert_eq!(levenshtein_distance("hello", ""), 5);
}

#[test]
fn test_levenshtein_distance_single_char_difference() {
    assert_eq!(levenshtein_distance("cat", "bat"), 1);
    assert_eq!(levenshtein_distance("cat", "cut"), 1);
}

#[test]
fn test_levenshtein_distance_common_typos() {
    // Common typo: swapped adjacent characters
    assert_eq!(levenshtein_distance("length", "lenght"), 2);

    // Missing character
    assert_eq!(levenshtein_distance("push", "psh"), 1);

    // Extra character
    assert_eq!(levenshtein_distance("pop", "popp"), 1);

    // Wrong character
    assert_eq!(levenshtein_distance("split", "spilt"), 2);
}

#[test]
fn test_levenshtein_distance_classic_example() {
    assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
}

#[test]
fn test_find_closest_match_exact_match() {
    let candidates = vec!["push", "pop", "length"];
    assert_eq!(find_closest_match("push", &candidates), Some("push"));
}

#[test]
fn test_find_closest_match_within_threshold() {
    let candidates = vec!["push", "pop", "length"];

    // One character off
    assert_eq!(find_closest_match("psh", &candidates), Some("push"));

    // Two characters off (swapped)
    assert_eq!(find_closest_match("lenght", &candidates), Some("length"));
}

#[test]
fn test_find_closest_match_multiple_candidates() {
    let candidates = vec!["push", "pop", "put"];

    // Should return closest match when multiple are within threshold
    assert_eq!(find_closest_match("pus", &candidates), Some("push"));
}

#[test]
fn test_find_closest_match_no_match() {
    let candidates = vec!["push", "pop", "length"];

    // More than 2 edits away from any candidate
    assert_eq!(find_closest_match("xyz", &candidates), None);
    assert_eq!(
        find_closest_match("completely_different", &candidates),
        None
    );
}

#[test]
fn test_find_closest_match_empty_candidates() {
    let candidates: Vec<&str> = vec![];
    assert_eq!(find_closest_match("push", &candidates), None);
}

#[test]
fn test_find_closest_match_prefers_smaller_distance() {
    let candidates = vec!["length", "lengthy"];

    // "lenght" is distance 2 from "length" and distance 3 from "lengthy"
    assert_eq!(find_closest_match("lenght", &candidates), Some("length"));
}

#[test]
fn test_levenshtein_distance_unicode() {
    assert_eq!(levenshtein_distance("café", "cafe"), 1);
    assert_eq!(levenshtein_distance("hello", "hëllo"), 1);
}

#[test]
fn test_find_closest_match_method_names() {
    let methods = vec!["push", "pop", "insert", "remove", "clear", "len"];

    // Common typos in method names
    assert_eq!(find_closest_match("psuh", &methods), Some("push"));
    assert_eq!(find_closest_match("pup", &methods), Some("pop"));
    assert_eq!(find_closest_match("insrt", &methods), Some("insert"));
    assert_eq!(find_closest_match("remov", &methods), Some("remove"));
}
