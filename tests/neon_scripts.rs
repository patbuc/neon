use neon::vm::{Result, VirtualMachine};
use std::fs;
use std::path::Path;

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
            if trimmed.is_empty() {
                // Empty line ends the expectation block
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

fn run_neon_script(path: &Path) -> datatest_stable::Result<()> {
    let script = fs::read_to_string(path)?;

    // Extract expected output from inline comments
    let expected_result = extract_inline_expectation(&script).ok_or_else(|| {
        format!(
            "No expected output found in {}. Add '// Expected:' block at the top of the file.",
            path.display()
        )
    })?;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(script.to_string());

    assert_eq!(
        Result::Ok,
        result,
        "VM interpretation failed for {}",
        path.display()
    );
    assert_eq!(
        expected_result,
        vm.get_output(),
        "Output mismatch for {}",
        path.display()
    );

    Ok(())
}

datatest_stable::harness! {
    { test = run_neon_script, root = "tests/scripts", pattern = r"^.*\.n$" },
}
