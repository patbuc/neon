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

fn unique_temp_dir(name: &str) -> std::path::PathBuf {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    let dir = std::env::temp_dir().join(format!(
        "neon_cli_fmt_test_{}_{}_{}",
        name,
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&dir).expect("Failed to create temp dir");
    dir
}

#[test]
fn fmt_rewrites_file_in_place_and_exits_zero() {
    let dir = unique_temp_dir("rewrite");
    let path = dir.join("a.n");
    fs::write(&path, "val x = 1").expect("Failed to write test script");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg(&path)
        .output()
        .expect("Failed to run neon binary");

    let contents = fs::read_to_string(&path).unwrap_or_default();
    fs::remove_dir_all(&dir).ok();

    assert!(output.status.success());
    assert_eq!("", String::from_utf8_lossy(&output.stdout));
    assert_eq!("val x = 1\n", contents);
}

#[test]
fn fmt_leaves_already_formatted_file_mtime_unchanged() {
    let dir = unique_temp_dir("mtime");
    let path = dir.join("a.n");
    fs::write(&path, "val x = 1\n").expect("Failed to write test script");

    let mtime_before = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1);
    fs::File::options()
        .write(true)
        .open(&path)
        .expect("Failed to open test script")
        .set_modified(mtime_before)
        .expect("Failed to set mtime");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg(&path)
        .output()
        .expect("Failed to run neon binary");

    let mtime_after = fs::metadata(&path).unwrap().modified().unwrap();
    fs::remove_dir_all(&dir).ok();

    assert!(output.status.success());
    assert_eq!(mtime_before, mtime_after);
}

#[cfg(unix)]
#[test]
fn fmt_check_directory_skips_symlinks_and_does_not_loop() {
    let dir = unique_temp_dir("symlink_loop");
    let a = dir.join("a.n");
    fs::write(&a, "val x = 1").expect("Failed to write test script");
    std::os::unix::fs::symlink(&dir, dir.join("loop")).expect("Failed to create symlink");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg("--check")
        .arg(&dir)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert_eq!(1, output.status.code().unwrap());
    assert_eq!(
        format!("{}\n", a.display()),
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn fmt_directory_walks_recursively_and_ignores_other_files() {
    let dir = unique_temp_dir("walk");
    let sub = dir.join("sub");
    fs::create_dir_all(&sub).expect("Failed to create nested dir");

    let top = dir.join("a.n");
    let nested = sub.join("b.n");
    let other = dir.join("note.txt");
    fs::write(&top, "val x = 1").expect("Failed to write test script");
    fs::write(&nested, "val y = 2").expect("Failed to write test script");
    fs::write(&other, "val x = 1").expect("Failed to write test file");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg(&dir)
        .output()
        .expect("Failed to run neon binary");

    let top_contents = fs::read_to_string(&top).unwrap_or_default();
    let nested_contents = fs::read_to_string(&nested).unwrap_or_default();
    let other_contents = fs::read_to_string(&other).unwrap_or_default();
    fs::remove_dir_all(&dir).ok();

    assert!(output.status.success());
    assert_eq!("val x = 1\n", top_contents);
    assert_eq!("val y = 2\n", nested_contents);
    assert_eq!("val x = 1", other_contents);
}

#[test]
fn fmt_check_directory_lists_files_in_sorted_order() {
    let dir = unique_temp_dir("walk_order");

    let b = dir.join("b.n");
    fs::write(&b, "val y = 2").expect("Failed to write test script");
    let a = dir.join("a.n");
    fs::write(&a, "val x = 1").expect("Failed to write test script");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg("--check")
        .arg(&dir)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert_eq!(1, output.status.code().unwrap());
    assert_eq!(
        format!("{}\n{}\n", a.display(), b.display()),
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn fmt_check_prints_changed_paths_exits_one_and_writes_nothing() {
    let dir = unique_temp_dir("check_dirty");
    let path = dir.join("a.n");
    fs::write(&path, "val x = 1").expect("Failed to write test script");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg("--check")
        .arg(&path)
        .output()
        .expect("Failed to run neon binary");

    let contents = fs::read_to_string(&path).unwrap_or_default();
    fs::remove_dir_all(&dir).ok();

    assert_eq!(1, output.status.code().unwrap());
    assert_eq!(
        format!("{}\n", path.display()),
        String::from_utf8_lossy(&output.stdout)
    );
    assert_eq!("val x = 1", contents);
}

#[test]
fn fmt_check_exits_zero_when_clean() {
    let dir = unique_temp_dir("check_clean");
    let path = dir.join("a.n");
    fs::write(&path, "val x = 1\n").expect("Failed to write test script");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg("--check")
        .arg(&path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert!(output.status.success());
    assert_eq!("", String::from_utf8_lossy(&output.stdout));
}

#[test]
fn fmt_dash_reads_stdin_and_writes_stdout() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn neon binary");

    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"val x = 1")
        .expect("Failed to write stdin");

    let output = child.wait_with_output().expect("Failed to wait on child");

    assert!(output.status.success());
    assert_eq!("val x = 1\n", String::from_utf8_lossy(&output.stdout));
}

#[test]
fn fmt_check_dash_prints_dash_when_stdin_would_change() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg("--check")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn neon binary");

    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"val x = 1")
        .expect("Failed to write stdin");

    let output = child.wait_with_output().expect("Failed to wait on child");

    assert_eq!(1, output.status.code().unwrap());
    assert_eq!("-\n", String::from_utf8_lossy(&output.stdout));
}

#[test]
fn fmt_syntax_error_leaves_file_unchanged_formats_rest_and_exits_65() {
    let dir = unique_temp_dir("syntax_error");
    let bad = dir.join("bad.n");
    let good = dir.join("good.n");
    fs::write(&bad, "val = 1\n").expect("Failed to write test script");
    fs::write(&good, "val x = 1").expect("Failed to write test script");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg(&bad)
        .arg(&good)
        .output()
        .expect("Failed to run neon binary");

    let bad_contents = fs::read_to_string(&bad).unwrap_or_default();
    let good_contents = fs::read_to_string(&good).unwrap_or_default();
    fs::remove_dir_all(&dir).ok();

    assert_eq!(65, output.status.code().unwrap());
    assert_eq!("val = 1\n", bad_contents);
    assert_eq!("val x = 1\n", good_contents);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(&bad.display().to_string()));
    assert!(stderr.contains("expecting variable name"));
    assert!(stderr.contains(":1:5"));
}

#[test]
fn fmt_unreadable_file_is_skipped_rest_still_format_and_exits_74() {
    let dir = unique_temp_dir("unreadable");
    let a = dir.join("a.n");
    let b = dir.join("b.n");
    let c = dir.join("c.n");
    fs::write(&a, "val x = 1").expect("Failed to write test script");
    fs::write(&b, [0xff, 0xfe]).expect("Failed to write test script");
    fs::write(&c, "val y = 2").expect("Failed to write test script");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg(&a)
        .arg(&b)
        .arg(&c)
        .output()
        .expect("Failed to run neon binary");

    let a_contents = fs::read_to_string(&a).unwrap_or_default();
    let b_contents = fs::read(&b).unwrap_or_default();
    let c_contents = fs::read_to_string(&c).unwrap_or_default();
    fs::remove_dir_all(&dir).ok();

    assert_eq!(74, output.status.code().unwrap());
    assert_eq!("val x = 1\n", a_contents);
    assert_eq!(vec![0xff, 0xfe], b_contents);
    assert_eq!("val y = 2\n", c_contents);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(&b.display().to_string()));
}

#[test]
fn fmt_io_error_takes_precedence_over_syntax_error_exits_74() {
    let dir = unique_temp_dir("io_over_syntax");
    let bad = dir.join("bad.n");
    let unreadable = dir.join("unreadable.n");
    fs::write(&bad, "val = 1\n").expect("Failed to write test script");
    fs::write(&unreadable, [0xff, 0xfe]).expect("Failed to write test script");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg(&dir)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert_eq!(74, output.status.code().unwrap());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(&bad.display().to_string()));
    assert!(stderr.contains(&unreadable.display().to_string()));
}

#[cfg(unix)]
#[test]
fn fmt_write_failure_is_skipped_rest_still_format_and_exits_74() {
    use std::os::unix::fs::PermissionsExt;

    let dir = unique_temp_dir("write_failure");
    let readonly = dir.join("a.n");
    let writable = dir.join("b.n");
    fs::write(&readonly, "val x = 1").expect("Failed to write test script");
    fs::write(&writable, "val y = 2").expect("Failed to write test script");

    let mut perms = fs::metadata(&readonly).unwrap().permissions();
    perms.set_mode(0o444);
    fs::set_permissions(&readonly, perms).expect("Failed to set permissions");

    // Root ignores the read-only bit, which would turn this into a false negative.
    if fs::write(&readonly, "val x = 1").is_ok() {
        fs::remove_dir_all(&dir).ok();
        eprintln!("skipping: running as a user that bypasses file permissions");
        return;
    }

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg(&readonly)
        .arg(&writable)
        .output()
        .expect("Failed to run neon binary");

    let mut perms = fs::metadata(&readonly).unwrap().permissions();
    perms.set_mode(0o644);
    fs::set_permissions(&readonly, perms).ok();

    let writable_contents = fs::read_to_string(&writable).unwrap_or_default();
    fs::remove_dir_all(&dir).ok();

    assert_eq!(74, output.status.code().unwrap());
    assert_eq!("val y = 2\n", writable_contents);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(&readonly.display().to_string()));
}

#[test]
fn fmt_without_paths_prints_usage_and_exits_64() {
    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .output()
        .expect("Failed to run neon binary");

    assert_eq!(64, output.status.code().unwrap());
    assert_eq!("", String::from_utf8_lossy(&output.stdout));
    assert!(!String::from_utf8_lossy(&output.stderr).is_empty());
}

#[test]
fn fmt_check_without_paths_prints_usage_and_exits_64() {
    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg("--check")
        .output()
        .expect("Failed to run neon binary");

    assert_eq!(64, output.status.code().unwrap());
    assert_eq!("", String::from_utf8_lossy(&output.stdout));
    assert!(!String::from_utf8_lossy(&output.stderr).is_empty());
}

#[test]
fn fmt_missing_path_exits_66_and_modifies_nothing() {
    let dir = unique_temp_dir("missing");
    let existing = dir.join("a.n");
    let missing = dir.join("does_not_exist.n");
    fs::write(&existing, "val x = 1").expect("Failed to write test script");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg(&existing)
        .arg(&missing)
        .output()
        .expect("Failed to run neon binary");

    let existing_contents = fs::read_to_string(&existing).unwrap_or_default();
    fs::remove_dir_all(&dir).ok();

    assert_eq!(66, output.status.code().unwrap());
    assert_eq!("val x = 1", existing_contents);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(&missing.display().to_string()));
}

#[test]
fn help_lists_fmt_command() {
    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("help")
        .output()
        .expect("Failed to run neon binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("fmt"));
    assert!(stdout.contains("--check"));
}
