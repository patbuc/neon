pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    strsim::levenshtein(s1, s2)
}

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
