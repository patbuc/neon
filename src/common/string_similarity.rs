/// Calculates the Levenshtein distance between two strings.
///
/// The Levenshtein distance is the minimum number of single-character edits
/// (insertions, deletions, or substitutions) required to change one string into another.
///
/// # Arguments
/// * `s1` - The first string
/// * `s2` - The second string
///
/// # Returns
/// The edit distance between the two strings
///
/// # Examples
/// ```
/// use neon::common::string_similarity::levenshtein_distance;
///
/// assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
/// assert_eq!(levenshtein_distance("length", "lenght"), 2);
/// ```
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    strsim::levenshtein(s1, s2)
}

/// Finds the closest matching string from a list of candidates.
///
/// Returns the candidate with the smallest Levenshtein distance to the target,
/// but only if the distance is within the threshold (default: 2).
///
/// # Arguments
/// * `target` - The string to find a match for
/// * `candidates` - A slice of candidate strings to search through
///
/// # Returns
/// * `Some(&str)` - The closest match if one exists within the threshold
/// * `None` - If no match exists within the threshold
///
/// # Examples
/// ```
/// use neon::common::string_similarity::find_closest_match;
///
/// let candidates = vec!["length", "width", "height"];
/// assert_eq!(find_closest_match("lenght", &candidates), Some("length"));
/// assert_eq!(find_closest_match("xyz", &candidates), None);
/// ```
pub fn find_closest_match<'a>(target: &str, candidates: &[&'a str]) -> Option<&'a str> {
    const THRESHOLD: usize = 2;

    let mut best_match: Option<&'a str> = None;
    let mut best_distance = THRESHOLD + 1;

    for &candidate in candidates {
        let distance = levenshtein_distance(target, candidate);

        if distance <= THRESHOLD && distance < best_distance {
            best_distance = distance;
            best_match = Some(candidate);
        }
    }

    best_match
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(find_closest_match("completely_different", &candidates), None);
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
}
