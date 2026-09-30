use crate::common::method_registry::NATIVE_METHODS;
use std::collections::HashSet;

/// Range's `push`/`pop`/`sort`/`reverse` only exist to raise "ranges are
/// immutable" runtime errors; they're deliberately undocumented as methods.
const UNDOCUMENTED: &[(&str, &str)] = &[
    ("Range", "push"),
    ("Range", "pop"),
    ("Range", "sort"),
    ("Range", "reverse"),
];

/// Splits `text` on backticks and returns the odd-indexed (quoted) spans.
fn backtick_tokens(text: &str) -> Vec<&str> {
    text.split('`').skip(1).step_by(2).collect()
}

/// Parses one backtick token into the (type, method) pairs it documents,
/// given the type(s) the surrounding doc section is about. Handles
/// `.method(...)` and bare `method(...)` (section type), `Type.method(...)`
/// and `lowercase.method(...)` (explicit/section type), and `Type(...)`
/// constructors (reported as `Type.new`).
fn parse_token(token: &str, section_types: &[String]) -> Vec<(String, String)> {
    let token = token.trim();
    let name_part = match token.find('(') {
        Some(i) => &token[..i],
        None => token,
    };

    if let Some(dot_pos) = name_part.find('.') {
        let left = &name_part[..dot_pos];
        let right = &name_part[dot_pos + 1..];
        return if left.starts_with(|c: char| c.is_uppercase()) {
            vec![(left.to_string(), right.to_string())]
        } else {
            section_types
                .iter()
                .map(|t| (t.clone(), right.to_string()))
                .collect()
        };
    }

    if let Some(method) = name_part.strip_prefix('.') {
        return section_types
            .iter()
            .map(|t| (t.clone(), method.to_string()))
            .collect();
    }

    if token.contains('(') && name_part.starts_with(|c: char| c.is_uppercase()) {
        return vec![(name_part.to_string(), "new".to_string())];
    }

    if name_part.starts_with(|c: char| c.is_alphabetic()) {
        return section_types
            .iter()
            .map(|t| (t.clone(), name_part.to_string()))
            .collect();
    }

    vec![]
}

/// Parses the skill's `## Native methods that exist` section: one bullet per
/// type, `- **Type:** \`method\`, \`method(args)\`, ...`, wrapping onto
/// indented continuation lines. `Global` isn't a registry type and is
/// skipped.
fn parse_skill_methods(text: &str) -> HashSet<(String, String)> {
    let section = section_body(text, "## Native methods that exist", "## ");
    let mut methods = HashSet::new();
    let mut section_types: Vec<String> = Vec::new();
    let mut in_bullet = false;

    for line in section.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("- **") {
            let (header, body) = rest.split_once("**").expect("bullet missing closing **");
            let header = header.trim_end_matches(':');
            section_types = header.split('/').map(|t| t.trim().to_string()).collect();
            in_bullet = section_types != vec!["Global".to_string()];
            if in_bullet {
                for token in backtick_tokens(body) {
                    methods.extend(parse_token(token, &section_types));
                }
            }
        } else if in_bullet && !trimmed.is_empty() {
            for token in backtick_tokens(trimmed) {
                methods.extend(parse_token(token, &section_types));
            }
        } else if trimmed.is_empty() {
            in_bullet = false;
        }
    }
    methods
}

/// Maps a `### `/bold-bullet heading in README's Standard Library section to
/// the registry type it documents, or `None` for headings that aren't a
/// registry type (`Global Functions`, the `Type Conversions` heading itself).
fn heading_to_type(heading: &str) -> Option<String> {
    match heading {
        "Global Functions" | "Type Conversions" => None,
        "File" => Some("File".to_string()),
        "Math (Static Methods)" => Some("Math".to_string()),
        _ => heading.strip_suffix(" Methods").map(|t| t.to_string()),
    }
}

/// Parses README's `## Standard Library` section. Subsections are `### `
/// headings, plus `**Number Methods:**` / `**Boolean Methods:**` bold
/// sub-headers under `### Type Conversions`. Method bullets are `- \`...\` -
/// description`; only the backtick tokens before the ` - ` separator count.
fn parse_readme_methods(text: &str) -> HashSet<(String, String)> {
    let section = section_body(text, "## Standard Library", "## ");
    let mut methods = HashSet::new();
    let mut section_types: Vec<String> = Vec::new();
    let mut in_code_block = false;

    for line in section.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            continue;
        }
        if let Some(heading) = trimmed.strip_prefix("### ") {
            section_types = heading_to_type(heading).into_iter().collect();
            continue;
        }
        if let Some(bold) = trimmed
            .strip_prefix("**")
            .and_then(|s| s.strip_suffix("**"))
        {
            if let Some(t) = bold.strip_suffix(" Methods:") {
                section_types = vec![t.to_string()];
            }
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("- `") {
            let rest = format!("`{rest}");
            let tokens_region = rest.split(" - ").next().unwrap_or(&rest);
            for token in backtick_tokens(tokens_region) {
                methods.extend(parse_token(token, &section_types));
            }
        }
    }
    methods
}

/// Returns the lines between `start_heading` and the next line beginning
/// with `next_prefix` (exclusive of both), or to the end of `text`.
fn section_body<'a>(text: &'a str, start_heading: &str, next_prefix: &str) -> &'a str {
    let start = text
        .find(start_heading)
        .unwrap_or_else(|| panic!("missing heading {start_heading:?}"));
    let after_heading = start + start_heading.len();
    let rest = &text[after_heading..];
    let end = rest.find('\n').map(|i| i + 1).unwrap_or(rest.len());
    let body = &rest[end..];
    match body[..].find(&format!("\n{next_prefix}")) {
        Some(i) => &body[..i],
        None => body,
    }
}

fn registry_methods() -> HashSet<(String, String)> {
    NATIVE_METHODS
        .iter()
        .filter(|(type_name, _, _)| !type_name.is_empty())
        .map(|(type_name, method_name, _)| (type_name.to_string(), method_name.to_string()))
        .filter(|entry| {
            !UNDOCUMENTED
                .iter()
                .any(|(t, m)| entry == &(t.to_string(), m.to_string()))
        })
        .collect()
}

fn diff(
    doc: &str,
    doc_methods: &HashSet<(String, String)>,
    registry: &HashSet<(String, String)>,
) -> Vec<String> {
    let mut mismatches = Vec::new();
    for (t, m) in registry {
        if !doc_methods.contains(&(t.clone(), m.clone())) {
            mismatches.push(format!("{doc}: {t}.{m} missing"));
        }
    }
    for (t, m) in doc_methods {
        if !registry.contains(&(t.clone(), m.clone())) {
            mismatches.push(format!("{doc}: {t}.{m} not in registry"));
        }
    }
    mismatches
}

#[test]
fn native_method_docs_match_registry() {
    let registry = registry_methods();

    let skill = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/.claude/skills/writing-neon/SKILL.md"
    ))
    .expect("failed to read SKILL.md");
    let readme = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))
        .expect("failed to read README.md");

    let mut mismatches = diff("SKILL.md", &parse_skill_methods(&skill), &registry);
    mismatches.extend(diff("README.md", &parse_readme_methods(&readme), &registry));

    assert!(
        mismatches.is_empty(),
        "native-method docs drifted from the registry:\n{}",
        mismatches.join("\n")
    );
}
