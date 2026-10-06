use neon::vm::{InterpretResult, VirtualMachine};
use std::fs;
use std::path::Path;

const RUNTIME_ERROR_PREFIX: &str = "// Expected runtime error:";
const COMPILE_ERROR_PREFIX: &str = "// Expected compile error:";

/// Extracts the message from a `<prefix> <message>` line anywhere in the
/// script, if present.
fn extract_expected_error(script: &str, prefix: &str) -> Option<String> {
    script.lines().find_map(|line| {
        line.trim()
            .strip_prefix(prefix)
            .map(|rest| rest.trim().to_string())
    })
}

/// Extracts expected output from inline comments in the script.
/// Looks for lines starting with "// Expected:" followed by the expected output lines.
/// Each subsequent comment line (starting with "//") is treated as a line of expected output.
///
/// Example:
/// ```neon
/// // Expected:
/// // Hello World
/// // 42
/// print "Hello World"
/// print 42
/// ```
fn extract_inline_expectation(script: &str) -> Option<String> {
    let lines = script.lines();
    let mut in_expectation_block = false;
    let mut expected_lines = Vec::new();

    for line in lines {
        let trimmed = line.trim();

        if trimmed.starts_with("// Expected:") {
            in_expectation_block = true;
            continue;
        }

        if in_expectation_block {
            if trimmed.is_empty()
                || trimmed.starts_with(RUNTIME_ERROR_PREFIX)
                || trimmed.starts_with(COMPILE_ERROR_PREFIX)
            {
                // Empty line, or an expected-error line, ends the expectation block
                break;
            } else if trimmed.starts_with("//") {
                // Remove the comment prefix and trim
                let content = trimmed.trim_start_matches("//").trim_start();
                expected_lines.push(content.to_string());
            } else {
                // Non-comment line ends the expectation block
                break;
            }
        }
    }

    if expected_lines.is_empty() {
        None
    } else {
        Some(expected_lines.join("\n"))
    }
}

/// Interprets `script` and checks its output and error behavior against
/// its own inline `// Expected:` / `// Expected runtime error:` /
/// `// Expected compile error:` comments. With an `entry` path the script
/// runs as that file, so its imports resolve next to it.
#[allow(clippy::expect_used)]
fn check_script(path: &Path, entry: Option<&Path>, script: &str) -> datatest_stable::Result<()> {
    let expected_compile_error = extract_expected_error(script, COMPILE_ERROR_PREFIX);

    // Extract expected output from inline comments
    let expected_result = match extract_inline_expectation(script) {
        Some(expected) => expected,
        None if expected_compile_error.is_some() => String::new(),
        None => {
            return Err(format!(
                "No expected output found in {}. Add '// Expected:' block at the top of the file.",
                path.display()
            )
            .into())
        }
    };

    let expected_runtime_error = extract_expected_error(script, RUNTIME_ERROR_PREFIX);

    let mut vm = VirtualMachine::new();
    let result = match entry {
        Some(entry) => vm.interpret_file(entry, script.to_string()),
        None => vm.interpret(script.to_string()),
    };

    match (expected_compile_error, expected_runtime_error) {
        (Some(expected_message), _) => {
            assert_eq!(
                InterpretResult::CompileError,
                result,
                "Expected a compile error for {}",
                path.display()
            );
            let actual_message = &vm
                .get_compile_errors()
                .first()
                .expect("InterpretResult::CompileError but no CompilationError recorded")
                .message;
            assert_eq!(
                &expected_message,
                actual_message,
                "Compile error message mismatch for {}",
                path.display()
            );
        }
        (None, Some(expected_message)) => {
            assert_eq!(
                InterpretResult::RuntimeError,
                result,
                "Expected a runtime error for {}",
                path.display()
            );
            let actual_message = vm
                .get_runtime_error()
                .expect("InterpretResult::RuntimeError but no RuntimeError recorded")
                .message
                .clone();
            assert_eq!(
                expected_message,
                actual_message,
                "Runtime error message mismatch for {}",
                path.display()
            );
        }
        (None, None) => {
            assert_eq!(
                InterpretResult::Ok,
                result,
                "VM interpretation failed for {}",
                path.display()
            );
        }
    }
    assert_eq!(
        expected_result,
        vm.get_output(),
        "Output mismatch for {}",
        path.display()
    );

    Ok(())
}

fn run_neon_script(path: &Path) -> datatest_stable::Result<()> {
    let script = fs::read_to_string(path)?;
    check_script(path, None, &script)
}

/// Formats `script`, checking that formatting kept its `// Expected:` block
/// intact.
fn format_script(path: &Path, script: &str) -> datatest_stable::Result<String> {
    let formatted = neon::compiler::format(script)
        .map_err(|errors| format!("format({}) failed: {errors:?}", path.display()))?;

    let original_expectation = extract_inline_expectation(script);
    let formatted_expectation = extract_inline_expectation(&formatted);
    if original_expectation != formatted_expectation {
        return Err(format!(
            "formatting {} changed its `// Expected:` block",
            path.display()
        )
        .into());
    }

    Ok(formatted)
}

/// Formats the script first, then runs the formatted source through the same
/// checks as `run_neon_script`.
fn run_formatted_neon_script(path: &Path) -> datatest_stable::Result<()> {
    let script = fs::read_to_string(path)?;
    let formatted = format_script(path, &script)?;
    check_script(path, None, &formatted)
}

/// Runs a multi-file case: `path` is its `main.n`, run as the entry file
/// with its modules beside or below it.
fn run_module_case(path: &Path) -> datatest_stable::Result<()> {
    let script = fs::read_to_string(path)?;
    check_script(path, Some(path), &script)
}

/// Copies the directory `from` to `to`, formatting every `.n` file on the way.
fn copy_formatted(from: &Path, to: &Path) -> datatest_stable::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let source = entry?.path();
        let target = to.join(source.file_name().ok_or("directory entry without a name")?);
        if source.is_dir() {
            copy_formatted(&source, &target)?;
        } else if source.extension().is_some_and(|ext| ext == "n") {
            let formatted = format_script(&source, &fs::read_to_string(&source)?)?;
            fs::write(&target, formatted)?;
        } else {
            fs::copy(&source, &target)?;
        }
    }
    Ok(())
}

/// Formats a multi-file case, `main.n` and every module beside or below it,
/// into a temporary copy of its directory and runs the copy's `main.n`.
fn run_formatted_module_case(path: &Path) -> datatest_stable::Result<()> {
    let case_dir = path.parent().ok_or("main.n without a case directory")?;
    let copy_name = case_dir.display().to_string().replace(['/', '\\'], "_");
    let copy_dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("formatted_{copy_name}"));
    if copy_dir.exists() {
        fs::remove_dir_all(&copy_dir)?;
    }
    copy_formatted(case_dir, &copy_dir)?;

    let entry = copy_dir.join("main.n");
    let formatted = fs::read_to_string(&entry)?;
    let result = check_script(path, Some(&entry), &formatted);
    fs::remove_dir_all(&copy_dir)?;
    result
}

datatest_stable::harness! {
    { test = run_neon_script, root = "tests/scripts", pattern = r"^.*\.n$" },
    { test = run_neon_script, root = "benches", pattern = r"^.*\.n$" },
    { test = run_formatted_neon_script, root = "tests/scripts", pattern = r"^.*\.n$" },
    { test = run_formatted_neon_script, root = "benches", pattern = r"^.*\.n$" },
    { test = run_module_case, root = "tests/modules", pattern = r"^[^/]+/main\.n$" },
    { test = run_formatted_module_case, root = "tests/modules", pattern = r"^[^/]+/main\.n$" },
    { test = run_module_case, root = "examples", pattern = r"^modules/main\.n$" },
    { test = run_formatted_module_case, root = "examples", pattern = r"^modules/main\.n$" },
}
