use std::fs;
use std::io::Write;
use std::process::Command;
#[cfg(not(feature = "disassemble"))]
use std::process::Stdio;

#[cfg(feature = "opcode-stats")]
fn assert_has_add_line_after(stderr: &str, expected: &str) {
    let remainder = &stderr[expected.len()..];
    assert!(
        remainder
            .lines()
            .any(|line| line.split_whitespace().next() == Some("Add")),
        "expected an Add line in the opcode report after the error trace:\n{}",
        remainder
    );
}

// The disassemble feature writes chunk traces and log output to stdout by
// design, so it cannot satisfy this test's "only program output" assertion.
#[cfg(not(feature = "disassemble"))]
#[test]
fn run_file_prints_only_program_output() {
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("neon_cli_test_run_file_prints_only_program_output.n");

    let mut file = fs::File::create(&script_path).expect("Failed to create test script");
    file.write_all(b"print(\"hello\")\n")
        .expect("Failed to write test script");
    drop(file);

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&script_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_file(&script_path).ok();

    assert!(output.status.success());
    assert_eq!("hello\n", String::from_utf8_lossy(&output.stdout));
}

#[test]
fn run_file_reports_missing_file_on_stderr() {
    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("/nonexistent_path_neon_cli_test.n")
        .output()
        .expect("Failed to run neon binary");

    assert_eq!(66, output.status.code().unwrap());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("/nonexistent_path_neon_cli_test.n"));
    assert!(!stderr.contains("panic"));
    assert_eq!("", String::from_utf8_lossy(&output.stdout));
}

#[test]
#[cfg(unix)]
fn run_file_reports_unreadable_file_on_stderr() {
    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(std::env::temp_dir())
        .output()
        .expect("Failed to run neon binary");

    assert_eq!(74, output.status.code().unwrap());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("panic"));
    assert_eq!("", String::from_utf8_lossy(&output.stdout));
}

#[test]
fn run_file_reports_runtime_error_on_stderr() {
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("neon_cli_test_run_file_reports_runtime_error_on_stderr.n");

    let mut file = fs::File::create(&script_path).expect("Failed to create test script");
    file.write_all(b"val a = 1\nval b = a + \"x\"\n")
        .expect("Failed to write test script");
    drop(file);

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&script_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_file(&script_path).ok();

    assert_eq!(70, output.status.code().unwrap());
    let expected = "[2:11] Operands must be two numbers or two strings\n  at <script> (line 2)\n";
    let stderr = String::from_utf8_lossy(&output.stderr);
    #[cfg(feature = "opcode-stats")]
    {
        assert!(stderr.starts_with(expected));
        assert_has_add_line_after(&stderr, expected);
    }
    #[cfg(not(feature = "opcode-stats"))]
    assert_eq!(expected, stderr);
}

#[test]
fn run_file_reports_call_trace_on_stderr() {
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("neon_cli_test_run_file_reports_call_trace_on_stderr.n");

    let mut file = fs::File::create(&script_path).expect("Failed to create test script");
    file.write_all(b"fn boom(x) {\n  return x + true\n}\nboom(1)\n")
        .expect("Failed to write test script");
    drop(file);

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&script_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_file(&script_path).ok();

    assert_eq!(70, output.status.code().unwrap());
    let expected =
        "[2:12] Operands must be two numbers or two strings\n  at boom (line 2)\n  at <script> (line 4)\n";
    let stderr = String::from_utf8_lossy(&output.stderr);
    #[cfg(feature = "opcode-stats")]
    {
        assert!(stderr.starts_with(expected));
        assert_has_add_line_after(&stderr, expected);
    }
    #[cfg(not(feature = "opcode-stats"))]
    assert_eq!(expected, stderr);
}

#[test]
fn check_mode_prints_nothing_and_exits_zero_on_success() {
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("neon_cli_test_check_mode_prints_nothing.n");

    let mut file = fs::File::create(&script_path).expect("Failed to create test script");
    file.write_all(b"print(\"hi\")\n")
        .expect("Failed to write test script");
    drop(file);

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("--check")
        .arg(&script_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_file(&script_path).ok();

    assert!(output.status.success());
    assert_eq!("", String::from_utf8_lossy(&output.stdout));
    assert_eq!("", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn check_mode_reports_same_compile_error_as_running() {
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("neon_cli_test_check_mode_reports_compile_error.n");

    let mut file = fs::File::create(&script_path).expect("Failed to create test script");
    file.write_all(b"val x = 1;\n")
        .expect("Failed to write test script");
    drop(file);

    let check_output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("--check")
        .arg(&script_path)
        .output()
        .expect("Failed to run neon binary");

    let run_output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&script_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_file(&script_path).ok();

    assert_eq!(65, check_output.status.code().unwrap());
    assert_eq!("", String::from_utf8_lossy(&check_output.stdout));

    let check_stderr = String::from_utf8_lossy(&check_output.stderr);
    let run_stderr = String::from_utf8_lossy(&run_output.stderr);
    assert!(!check_stderr.is_empty());
    assert_eq!(check_stderr, run_stderr);
}

#[test]
fn check_mode_without_file_prints_usage_on_stderr() {
    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("--check")
        .output()
        .expect("Failed to run neon binary");

    assert_eq!(64, output.status.code().unwrap());
    assert_eq!(
        "Usage: neon --check <file>\n",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!("", String::from_utf8_lossy(&output.stdout));
}

#[test]
fn check_mode_reports_missing_file_on_stderr() {
    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("--check")
        .arg("/nonexistent_path_neon_cli_test.n")
        .output()
        .expect("Failed to run neon binary");

    assert_eq!(66, output.status.code().unwrap());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("/nonexistent_path_neon_cli_test.n"));
    assert!(!stderr.contains("panic"));
    assert_eq!("", String::from_utf8_lossy(&output.stdout));
}

#[test]
fn help_lists_check_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("--help")
        .output()
        .expect("Failed to run neon binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--check"));
}

#[cfg(feature = "opcode-stats")]
#[test]
fn run_file_prints_opcode_stats_to_stderr_only() {
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("neon_cli_test_run_file_prints_opcode_stats.n");

    let mut file = fs::File::create(&script_path).expect("Failed to create test script");
    file.write_all(b"print(\"hello\")\n")
        .expect("Failed to write test script");
    drop(file);

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&script_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_file(&script_path).ok();

    assert!(output.status.success());
    assert_eq!("hello\n", String::from_utf8_lossy(&output.stdout));

    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut constant_count = None;
    let mut saw_pair_key = false;
    for line in stderr.lines().filter(|line| !line.is_empty()) {
        let mut parts = line.split_whitespace();
        let name = parts.next().expect("line has an opcode name");
        let count: u64 = parts
            .next()
            .expect("line has a count")
            .parse()
            .expect("count is a number");
        assert!(parts.next().is_none(), "line has extra fields: {}", line);
        if name == "Constant" {
            constant_count = Some(count);
        }
        if name.contains("->") {
            saw_pair_key = true;
        }
    }
    assert_eq!(Some(2), constant_count);
    assert!(saw_pair_key, "expected a pair key in report:\n{}", stderr);
}

#[cfg(not(feature = "disassemble"))]
fn run_script_with_stdin(script: &str, stdin_input: &[u8]) -> std::process::Output {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join(format!(
        "neon_cli_test_stdin_{}_{}.n",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));

    let mut file = fs::File::create(&script_path).expect("Failed to create test script");
    file.write_all(script.as_bytes())
        .expect("Failed to write test script");
    drop(file);

    let mut child = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&script_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn neon binary");

    child
        .stdin
        .take()
        .expect("child stdin")
        .write_all(stdin_input)
        .expect("Failed to write stdin");

    let output = child.wait_with_output().expect("Failed to run neon binary");

    fs::remove_file(&script_path).ok();

    output
}

#[cfg(not(feature = "disassemble"))]
#[test]
fn stdin_read_lines_splits_like_file() {
    let output = run_script_with_stdin("print(Stdin.readLines())\n", b"a\nb\n");

    assert!(output.status.success());
    assert_eq!("[a, b]\n", String::from_utf8_lossy(&output.stdout));
}

#[cfg(not(feature = "disassemble"))]
#[test]
fn stdin_read_returns_full_piped_text() {
    let output = run_script_with_stdin("print(Stdin.read())\n", b"a\nb\n");

    assert!(output.status.success());
    assert_eq!("a\nb\n\n", String::from_utf8_lossy(&output.stdout));
}

#[cfg(not(feature = "disassemble"))]
#[test]
fn stdin_read_on_empty_input_returns_empty_string() {
    let output = run_script_with_stdin("print(\"[\" + Stdin.read() + \"]\")\n", b"");

    assert!(output.status.success());
    assert_eq!("[]\n", String::from_utf8_lossy(&output.stdout));
}

#[cfg(not(feature = "disassemble"))]
#[test]
fn stdin_second_read_after_eof_returns_empty_string() {
    let script = "val first = Stdin.read()\nprint(\"[\" + Stdin.read() + \"]\")\n";
    let output = run_script_with_stdin(script, b"hi");

    assert!(output.status.success());
    assert_eq!("[]\n", String::from_utf8_lossy(&output.stdout));
}
