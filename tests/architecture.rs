//! Enforces the layer boundaries between the crate's top-level modules.
//!
//! Allowed edges: compiler -> common, vm -> common, vm -> compiler (plus any
//! layer referencing itself). `src/main.rs`, `src/lib.rs`, `src/wasm.rs` and
//! `src/macros.rs` sit outside the layers and are unrestricted.

use std::fs;
use std::path::{Path, PathBuf};

const LAYERS: [&str; 3] = ["common", "compiler", "vm"];

fn allowed(from: &str, to: &str) -> bool {
    from == to
        || matches!(
            (from, to),
            ("compiler", "common") | ("vm", "common") | ("vm", "compiler")
        )
}

/// The layer a source file belongs to, or `None` for files outside the
/// layered tree (crate-root files, and anything under a `tests` directory).
fn layer_of(path: &str) -> Option<&'static str> {
    if path.contains("/tests/") {
        return None;
    }
    let mut parts = path.split('/');
    while let Some(part) = parts.next() {
        if part == "src" {
            let next = parts.next()?;
            return LAYERS.iter().find(|&&layer| layer == next).copied();
        }
    }
    None
}

fn line_of(source: &str, byte_pos: usize) -> usize {
    source[..byte_pos].matches('\n').count() + 1
}

/// Byte index (relative to `s`) of the `}` matching the `{` at `open_pos`.
fn matching_brace(s: &str, open_pos: usize) -> usize {
    let bytes = s.as_bytes();
    let mut depth = 0usize;
    for (i, &b) in bytes.iter().enumerate().skip(open_pos) {
        match b {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return i;
                }
            }
            _ => {}
        }
    }
    bytes.len().saturating_sub(1)
}

/// Byte ranges (inclusive) covered by `#[cfg(test)]`-gated items, which are
/// dropped before scanning for layer references: a module block up to its
/// matching brace, or a single item (`use ...;`, `mod ...;`, ...) up to its
/// terminating `;`.
fn cfg_test_ranges(source: &str) -> Vec<(usize, usize)> {
    const ATTR: &str = "#[cfg(test)]";
    let mut ranges = Vec::new();
    let mut from = 0;
    while let Some(rel) = source[from..].find(ATTR) {
        let start = from + rel;
        let after = start + ATTR.len();
        let rest = &source[after..];
        let brace_pos = rest.find('{');
        let semi_pos = rest.find(';');
        let end = match (brace_pos, semi_pos) {
            (Some(b), Some(s)) if s < b => after + s,
            (Some(b), _) => after + matching_brace(rest, b),
            (None, Some(s)) => after + s,
            (None, None) => source.len().saturating_sub(1),
        };
        ranges.push((start, end));
        from = end + 1;
        if from >= source.len() {
            break;
        }
    }
    ranges
}

fn in_ranges(pos: usize, ranges: &[(usize, usize)]) -> bool {
    ranges.iter().any(|&(s, e)| pos >= s && pos <= e)
}

fn ident_prefix(s: &str) -> &str {
    let end = s
        .find(|c: char| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(s.len());
    &s[..end]
}

/// Splits the content of a `crate::{...}` group into its top-level,
/// comma-separated items (ignoring commas inside nested groups).
fn split_top_level(content: &str) -> Vec<&str> {
    let mut items = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    for (i, c) in content.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => depth -= 1,
            ',' if depth == 0 => {
                items.push(content[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    let last = content[start..].trim();
    if !last.is_empty() {
        items.push(last);
    }
    items
}

/// For each `crate::<layer>` reference in `source`, returns the layer name
/// and the byte position of the `crate::` token.
fn layer_refs(source: &str) -> Vec<(&str, usize)> {
    const PREFIX: &str = "crate::";
    let mut refs = Vec::new();
    for (rel, _) in source.match_indices(PREFIX) {
        let after = rel + PREFIX.len();
        let rest = &source[after..];
        if rest.starts_with('{') {
            let close = matching_brace(rest, 0);
            let content = &rest[1..close];
            for item in split_top_level(content) {
                let layer = ident_prefix(item);
                if !layer.is_empty() {
                    refs.push((layer, rel));
                }
            }
        } else {
            let layer = ident_prefix(rest);
            if !layer.is_empty() {
                refs.push((layer, rel));
            }
        }
    }
    refs
}

/// Reports every `crate::<layer>` reference in `source` (a file at `path`,
/// relative to the repo root) whose edge crosses a forbidden layer boundary.
fn violations(path: &str, source: &str) -> Vec<String> {
    let Some(from_layer) = layer_of(path) else {
        return Vec::new();
    };
    let excluded = cfg_test_ranges(source);
    let mut found = Vec::new();
    for (layer, pos) in layer_refs(source) {
        if in_ranges(pos, &excluded) {
            continue;
        }
        if !LAYERS.contains(&layer) {
            continue;
        }
        if !allowed(from_layer, layer) {
            found.push(format!(
                "{}:{}: {} \u{2192} {}",
                path,
                line_of(source, pos),
                from_layer,
                layer
            ));
        }
    }
    found
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries =
        fs::read_dir(dir).unwrap_or_else(|e| panic!("failed to read dir {}: {}", dir.display(), e));
    for entry in entries {
        let entry = entry.expect("failed to read dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn current_tree_has_no_layer_violations() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);

    let mut all_violations = Vec::new();
    for file in files {
        let relative = file
            .strip_prefix(manifest_dir)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        let source = fs::read_to_string(&file).expect("failed to read source file");
        all_violations.extend(violations(&relative, &source));
    }

    assert!(
        all_violations.is_empty(),
        "found layer violations:\n{}",
        all_violations.join("\n")
    );
}

#[test]
fn use_crate_vm_in_compiler_is_reported_with_file_line_and_edge() {
    let source = "use crate::vm::VirtualMachine;\n";

    let found = violations("src/compiler/foo.rs", source);

    assert_eq!(found, vec!["src/compiler/foo.rs:1: compiler \u{2192} vm"]);
}

#[test]
fn inline_crate_vm_path_without_a_use_is_reported() {
    let source = "fn f() {\n    let r = crate::vm::InterpretResult::Ok;\n}\n";

    let found = violations("src/compiler/foo.rs", source);

    assert_eq!(found, vec!["src/compiler/foo.rs:2: compiler \u{2192} vm"]);
}

#[test]
fn grouped_use_crate_common_vm_in_compiler_is_reported() {
    let source = "use crate::{common, vm};\n";

    let found = violations("src/compiler/foo.rs", source);

    assert_eq!(found, vec!["src/compiler/foo.rs:1: compiler \u{2192} vm"]);
}

#[test]
fn crate_vm_inside_a_cfg_test_module_is_ignored() {
    let source = "#[cfg(test)]\nmod tests {\n    use crate::vm::VirtualMachine;\n}\n";

    let found = violations("src/compiler/foo.rs", source);

    assert!(found.is_empty(), "expected no violations, got {:?}", found);
}

#[test]
fn files_under_a_tests_directory_are_skipped() {
    let source = "use crate::vm::VirtualMachine;\n";

    let found = violations("src/compiler/tests/foo.rs", source);

    assert!(found.is_empty(), "expected no violations, got {:?}", found);
}

#[test]
fn use_crate_compiler_in_vm_is_allowed() {
    let source = "use crate::compiler::Compiler;\n";

    let found = violations("src/vm/foo.rs", source);

    assert!(found.is_empty(), "expected no violations, got {:?}", found);
}
