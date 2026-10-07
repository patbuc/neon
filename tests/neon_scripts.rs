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

/// Fails with `context` and both values unless `expected` equals `actual`.
fn expect_equal<T: PartialEq + std::fmt::Debug>(
    expected: &T,
    actual: &T,
    context: std::fmt::Arguments,
) -> datatest_stable::Result<()> {
    if expected == actual {
        Ok(())
    } else {
        Err(format!("{context}\n  expected: {expected:?}\n    actual: {actual:?}").into())
    }
}

/// Interprets `script` and checks its output and error behavior against
/// its own inline `// Expected:` / `// Expected runtime error:` /
/// `// Expected compile error:` comments. With an `entry` path the script
/// runs as that file, so its imports resolve next to it.
#[allow(clippy::expect_used)]
fn check_script(path: &Path, entry: Option<&Path>, script: &str) -> datatest_stable::Result<()> {
    let expected_compile_error = extract_expected_error(script, COMPILE_ERROR_PREFIX);
    let expected_runtime_error = extract_expected_error(script, RUNTIME_ERROR_PREFIX);
    let expected_output = extract_inline_expectation(script);

    if expected_compile_error.is_some()
        && (expected_runtime_error.is_some() || expected_output.is_some())
    {
        return Err(format!(
            "Conflicting expectations in {}: a '// Expected compile error:' line allows neither \
             a '// Expected runtime error:' line nor '// Expected:' output.",
            path.display()
        )
        .into());
    }

    let expected_result = match expected_output {
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

    let mut vm = VirtualMachine::new();
    let result = match entry {
        Some(entry) => vm.interpret_file(entry, script.to_string()),
        None => vm.interpret(script.to_string()),
    };

    match (expected_compile_error, expected_runtime_error) {
        (Some(expected_message), _) => {
            expect_equal(
                &InterpretResult::CompileError,
                &result,
                format_args!("Expected a compile error for {}", path.display()),
            )?;
            let messages: Vec<&str> = vm
                .get_compile_errors()
                .iter()
                .map(|error| error.message.as_str())
                .collect();
            let [actual_message] = messages.as_slice() else {
                return Err(format!(
                    "Expected exactly one compile error for {}, got {}: {messages:?}",
                    path.display(),
                    messages.len()
                )
                .into());
            };
            expect_equal(
                &expected_message.as_str(),
                actual_message,
                format_args!("Compile error message mismatch for {}", path.display()),
            )?;
        }
        (None, Some(expected_message)) => {
            expect_equal(
                &InterpretResult::RuntimeError,
                &result,
                format_args!("Expected a runtime error for {}", path.display()),
            )?;
            let actual_message = vm
                .get_runtime_error()
                .expect("InterpretResult::RuntimeError but no RuntimeError recorded")
                .message
                .clone();
            expect_equal(
                &expected_message,
                &actual_message,
                format_args!("Runtime error message mismatch for {}", path.display()),
            )?;
        }
        (None, None) => {
            expect_equal(
                &InterpretResult::Ok,
                &result,
                format_args!("VM interpretation failed for {}", path.display()),
            )?;
        }
    }
    expect_equal(
        &expected_result,
        &vm.get_output(),
        format_args!("Output mismatch for {}", path.display()),
    )?;

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

/// Runs a multi-file case that must fail its own expectations, and checks
/// that the failure is a compile error expectation mismatch.
fn run_failing_module_case(path: &Path) -> datatest_stable::Result<()> {
    let script = fs::read_to_string(path)?;
    let failure = match check_script(path, Some(path), &script) {
        Ok(()) => return Err(format!("{} passed but must fail", path.display()).into()),
        Err(error) => error.to_string(),
    };
    if !failure.contains("Compile error message mismatch")
        && !failure.contains("Expected a compile error for")
    {
        return Err(format!("{} failed for another reason: {failure}", path.display()).into());
    }
    Ok(())
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
    // The process id keeps concurrent test runs apart, and the escaping keeps
    // distinct case paths from sharing a name.
    let copy_name = case_dir
        .display()
        .to_string()
        .replace('%', "%25")
        .replace(['/', '\\'], "%2F");
    let copy_dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("formatted-{}-{copy_name}", std::process::id()));
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
    { test = run_failing_module_case, root = "tests/modules_must_fail", pattern = r"^[^/]+/main\.n$" },
    { test = run_module_case, root = "examples", pattern = r"^modules/main\.n$" },
    { test = run_formatted_module_case, root = "examples", pattern = r"^modules/main\.n$" },
}
