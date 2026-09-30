use crate::common::method_registry::NATIVE_METHODS;
use std::collections::BTreeSet;

const UNDOCUMENTED: &[(&str, &str)] = &[
    ("Range", "push"),
    ("Range", "pop"),
    ("Range", "sort"),
    ("Range", "reverse"),
];

fn backtick_tokens(text: &str) -> Vec<&str> {
    text.split('`').skip(1).step_by(2).collect()
}

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

fn parse_skill_methods(text: &str) -> BTreeSet<(String, String)> {
    let section = section_body(text, "## Native methods that exist");
    let mut methods = BTreeSet::new();
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

enum Section {
    None,
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

fn process_bullet(lines: &[String], section: &Section, methods: &mut BTreeSet<(String, String)>) {
    let joined = lines.join(" ");
    let tokens_region = match joined.split_once(" - ") {
        Some((region, _)) => region,
        None => joined.as_str(),
    };
    match section {
        Section::None => {
            panic!("README.md: method bullet with no current section type: {joined}")
        }
        Section::Global => {
            for token in backtick_tokens(tokens_region) {
                let name = match token.split_once('(') {
                    Some((name, _)) => name,
                    None => token,
                }
                .trim();
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
    pending: &mut Option<Vec<String>>,
    section: &Section,
    methods: &mut BTreeSet<(String, String)>,
) {
    if let Some(lines) = pending.take() {
        process_bullet(&lines, section, methods);
    }
}

fn parse_readme_methods(text: &str) -> BTreeSet<(String, String)> {
    let section = section_body(text, "## Standard Library");
    let mut methods = BTreeSet::new();
    let mut current = Section::None;
    let mut in_code_block = false;
    let mut pending: Option<Vec<String>> = None;

    for line in section.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("```") {
            flush_pending(&mut pending, &current, &mut methods);
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            continue;
        }

        if let Some(heading) = trimmed.strip_prefix("### ") {
            flush_pending(&mut pending, &current, &mut methods);
            current = heading_section(heading);
        } else if let Some(bold) = trimmed
            .strip_prefix("**")
            .and_then(|s| s.strip_suffix("**"))
        {
            flush_pending(&mut pending, &current, &mut methods);
            if let Some(t) = bold.strip_suffix(" Methods:") {
                current = Section::Types(vec![t.to_string()]);
            }
        } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            flush_pending(&mut pending, &current, &mut methods);
            pending = Some(vec![trimmed.to_string()]);
        } else if trimmed.is_empty() {
            flush_pending(&mut pending, &current, &mut methods);
        } else if let Some(lines) = pending.as_mut() {
            lines.push(trimmed.to_string());
        }
    }
    flush_pending(&mut pending, &current, &mut methods);

    methods
}

fn section_body<'a>(text: &'a str, start_heading: &str) -> &'a str {
    let needle = format!("\n{start_heading}\n");
    let start = text
        .find(&needle)
        .unwrap_or_else(|| panic!("missing heading {start_heading:?}"));
    let body = &text[start + needle.len()..];
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
