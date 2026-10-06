use neon::vm::{InterpretResult, VirtualMachine};
use std::fs;
use std::path::Path;

const RUNTIME_ERROR_PREFIX: &str = "// Expected runtime error:";

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
            if trimmed.is_empty() || trimmed.starts_with(RUNTIME_ERROR_PREFIX) {
                // Empty line, or the runtime-error line, ends the expectation block
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
/// its own inline `// Expected:` / `// Expected runtime error:` comments.
#[allow(clippy::expect_used)]
fn check_script(path: &Path, script: &str) -> datatest_stable::Result<()> {
    // Extract expected output from inline comments
    let expected_result = extract_inline_expectation(script).ok_or_else(|| {
        format!(
            "No expected output found in {}. Add '// Expected:' block at the top of the file.",
            path.display()
        )
    })?;

    let expected_runtime_error = extract_expected_error(script, RUNTIME_ERROR_PREFIX);

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(script.to_string());

    match expected_runtime_error {
        Some(expected_message) => {
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
        None => {
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
    check_script(path, &script)
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
    check_script(path, &formatted)
}

datatest_stable::harness! {
    { test = run_neon_script, root = "tests/scripts", pattern = r"^.*\.n$" },
    { test = run_neon_script, root = "benches", pattern = r"^.*\.n$" },
    { test = run_formatted_neon_script, root = "tests/scripts", pattern = r"^.*\.n$" },
    { test = run_formatted_neon_script, root = "benches", pattern = r"^.*\.n$" },
}
