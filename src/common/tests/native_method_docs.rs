use crate::common::method_registry::NATIVE_METHODS;
use std::collections::BTreeSet;

// Range's push/pop/sort/reverse only raise "immutable" runtime errors and aren't documented.
const UNDOCUMENTED: &[(&str, &str)] = &[
    ("Range", "push"),
    ("Range", "pop"),
    ("Range", "sort"),
    ("Range", "reverse"),
];

fn backtick_tokens(text: &str) -> Vec<&str> {
    text.split('`').skip(1).step_by(2).collect()
}

/// Removes plain-text parenthesized asides that sit outside any backtick
/// span, so a doc note like "(creates the file; errors if it exists)" can't
/// smuggle a stray method-looking backtick token past `backtick_tokens`.
fn strip_parenthetical(text: &str) -> String {
    let mut result = String::new();
    let mut in_backtick = false;
    let mut depth = 0i32;
    for c in text.chars() {
        match c {
            '`' => {
                in_backtick = !in_backtick;
                result.push(c);
            }
            '(' if !in_backtick => depth += 1,
            ')' if !in_backtick => depth -= 1,
            _ if !in_backtick && depth > 0 => {}
            _ => result.push(c),
        }
    }
    result
}

/// Recognizes `.method(...)`/bare `method(...)` (current section type),
/// `Type.method(...)`/`lowercase.method(...)` (explicit or section type),
/// and `Type(...)` constructors, reported as `Type.new`.
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

/// Parses `## Native methods that exist`: one bullet per type, `- **Type:**
/// \`method\`, ...`, wrapping onto indented continuation lines. `Global`
/// isn't a registry type and is skipped.
fn parse_skill_methods(text: &str) -> BTreeSet<(String, String)> {
    let section = section_body(text, "## Native methods that exist");
    let mut methods = BTreeSet::new();
    let mut section_types: Vec<String> = Vec::new();
    let mut in_bullet = false;

    for line in section.lines() {
        let trimmed = strip_parenthetical(line.trim());
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
            for token in backtick_tokens(&trimmed) {
                methods.extend(parse_token(token, &section_types));
            }
        } else if trimmed.is_empty() {
            in_bullet = false;
        }
    }
    methods
}

/// What a `### `/bold heading in README's Standard Library section documents.
enum Section {
    /// No registry type is active (e.g. straight after `### Type Conversions`).
    None,
    /// `### Global Functions`: out of scope, but only `print` may appear here.
    Global,
    Types(Vec<String>),
}

fn heading_section(heading: &str) -> Section {
    match heading {
        "Global Functions" => Section::Global,
        "Type Conversions" => Section::None,
        "File" => Section::Types(vec!["File".to_string()]),
        "Math (Static Methods)" => Section::Types(vec!["Math".to_string()]),
        other => match other.strip_suffix(" Methods") {
            Some(t) => Section::Types(vec![t.to_string()]),
            None => {
                panic!("README.md: unrecognized heading '### {other}' in Standard Library section")
            }
        },
    }
}

fn process_bullet(bullet: &str, section: &Section, methods: &mut BTreeSet<(String, String)>) {
    let tokens_region = match bullet.split_once(" - ") {
        Some((region, _)) => region,
        None => bullet,
    };
    match section {
        Section::None => panic!("README.md: method bullet with no current section type: {bullet}"),
        Section::Global => {
            for token in backtick_tokens(tokens_region) {
                let name = token.split('(').next().unwrap_or(token).trim();
                assert_eq!(
                    name, "print",
                    "README.md: unexpected Global Functions entry `{token}`"
                );
            }
        }
        Section::Types(types) => {
            for token in backtick_tokens(tokens_region) {
                methods.extend(parse_token(token, types));
            }
        }
    }
}

fn flush_pending(
    pending: &mut Option<String>,
    section: &Section,
    methods: &mut BTreeSet<(String, String)>,
) {
    if let Some(bullet) = pending.take() {
        process_bullet(&bullet, section, methods);
    }
}

/// Parses README's `## Standard Library` section: `### ` headings plus
/// `**Number Methods:**` / `**Boolean Methods:**` bold sub-headers under
/// `### Type Conversions`. Method bullets are `- \`...\` - description`;
/// only the backtick tokens before the ` - ` separator count, and a bullet
/// may wrap onto indented continuation lines.
fn parse_readme_methods(text: &str) -> BTreeSet<(String, String)> {
    let section = section_body(text, "## Standard Library");
    let mut methods = BTreeSet::new();
    let mut current = Section::None;
    let mut in_code_block = false;
    let mut pending: Option<String> = None;

    for line in section.lines() {
        let trimmed = line.trim();

        if in_code_block {
            if trimmed.starts_with("```") {
                in_code_block = false;
            }
            continue;
        }
        if trimmed.starts_with("```") {
            flush_pending(&mut pending, &current, &mut methods);
            in_code_block = true;
            continue;
        }

        let is_continuation =
            pending.is_some() && !line.is_empty() && line.starts_with(char::is_whitespace);
        if is_continuation {
            let bullet = pending.as_mut().unwrap();
            bullet.push(' ');
            bullet.push_str(trimmed);
            continue;
        }

        flush_pending(&mut pending, &current, &mut methods);

        if let Some(heading) = trimmed.strip_prefix("### ") {
            current = heading_section(heading);
        } else if let Some(bold) = trimmed
            .strip_prefix("**")
            .and_then(|s| s.strip_suffix("**"))
        {
            if let Some(t) = bold.strip_suffix(" Methods:") {
                current = Section::Types(vec![t.to_string()]);
            }
        } else if trimmed.starts_with("- `") {
            pending = Some(trimmed.to_string());
        }
        // else: blank line or prose paragraph, ignored
    }
    flush_pending(&mut pending, &current, &mut methods);

    methods
}

fn section_body<'a>(text: &'a str, start_heading: &str) -> &'a str {
    let start = text
        .find(start_heading)
        .unwrap_or_else(|| panic!("missing heading {start_heading:?}"));
    let rest = &text[start + start_heading.len()..];
    let after_heading_line = rest.find('\n').map(|i| i + 1).unwrap_or(rest.len());
    let body = &rest[after_heading_line..];
    match body.find("\n## ") {
        Some(i) => &body[..i],
        None => body,
    }
}

fn registry_methods() -> BTreeSet<(String, String)> {
    NATIVE_METHODS
        .iter()
        .filter(|(type_name, method_name, _)| {
            !type_name.is_empty() && !UNDOCUMENTED.contains(&(*type_name, *method_name))
        })
        .map(|(type_name, method_name, _)| (type_name.to_string(), method_name.to_string()))
        .collect()
}

fn diff(
    doc: &str,
    doc_methods: &BTreeSet<(String, String)>,
    registry: &BTreeSet<(String, String)>,
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
