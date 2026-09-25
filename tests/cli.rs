use std::fs;
use std::io::Write;
use std::process::Command;

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
    assert_eq!(
        "[2:11] Operands must be two numbers or two strings\n  at <script> (line 2)\n",
        String::from_utf8_lossy(&output.stderr)
    );
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
    assert_eq!(
        "[2:12] Operands must be two numbers or two strings\n  at boom (line 2)\n  at <script> (line 4)\n",
        String::from_utf8_lossy(&output.stderr)
    );
}
