/// Calculates the Levenshtein distance between two strings.
///
/// The Levenshtein distance is the minimum number of single-character edits
/// (insertions, deletions, or substitutions) required to change one string into another.
///
/// This function uses the `strsim` crate internally for efficient distance calculation.
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
