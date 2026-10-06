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
    let path = script_path.display();
    let expected = format!(
        "[{path}:2:11] Operands must be two numbers or two strings\n  at <script> ({path}:2)\n"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    #[cfg(feature = "opcode-stats")]
    {
        assert!(stderr.starts_with(&expected));
        assert_has_add_line_after(&stderr, &expected);
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
    let path = script_path.display();
    let expected = format!(
        "[{path}:2:12] Operands must be two numbers or two strings\n  at boom ({path}:2)\n  at <script> ({path}:4)\n"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    #[cfg(feature = "opcode-stats")]
    {
        assert!(stderr.starts_with(&expected));
        assert_has_add_line_after(&stderr, &expected);
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
fn check_mode_reports_missing_import_with_path_tried() {
    let dir = std::env::temp_dir().join("neon_cli_test_check_missing_import");
    fs::create_dir_all(&dir).expect("Failed to create test dir");
    let main_path = dir.join("main.n");
    fs::write(&main_path, "import \"missing\"\n").expect("Failed to write test script");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("--check")
        .arg(&main_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert_eq!(65, output.status.code().unwrap());
    assert_eq!("", String::from_utf8_lossy(&output.stdout));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("cannot find module 'missing'"), "{stderr}");
    assert!(
        stderr.contains(&dir.join("missing.n").display().to_string()),
        "{stderr}"
    );
}

#[test]
fn check_mode_renders_imported_file_error_against_that_file() {
    let dir = std::env::temp_dir().join("neon_cli_test_check_imported_file_error");
    fs::create_dir_all(&dir).expect("Failed to create test dir");
    let main_path = dir.join("main.n");
    let module_path = dir.join("b.n");
    fs::write(&main_path, "import \"b\"\n").expect("Failed to write test script");
    fs::write(&module_path, "val x = 1\nval = 1\n").expect("Failed to write test module");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("--check")
        .arg(&main_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert_eq!(65, output.status.code().unwrap());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(&format!("{}:2:", module_path.display())),
        "{stderr}"
    );
    assert!(stderr.contains("val = 1"), "{stderr}");
}

#[test]
fn run_file_runs_an_imported_module_function() {
    let dir = std::env::temp_dir().join("neon_cli_test_run_imported_module_function");
    fs::create_dir_all(&dir).expect("Failed to create test dir");
    let main_path = dir.join("main.n");
    fs::write(&main_path, "import \"utils\"\nprint(utils.double(21))\n")
        .expect("Failed to write test script");
    fs::write(
        dir.join("utils.n"),
        "export fn double(x) { return x * 2 }\n",
    )
    .expect("Failed to write test module");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&main_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert!(output.status.success());
    assert_eq!("42\n", String::from_utf8_lossy(&output.stdout));
    #[cfg(not(feature = "opcode-stats"))]
    assert_eq!("", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn run_file_initializes_a_module_imported_by_two_importers_once() {
    let dir = std::env::temp_dir().join("neon_cli_test_run_module_initialized_once");
    fs::create_dir_all(&dir).expect("Failed to create test dir");
    let main_path = dir.join("main.n");
    fs::write(
        &main_path,
        "import \"a\"\nimport \"b\"\nprint(\"main\")\nprint(a.fa() + b.fb())\n",
    )
    .expect("Failed to write test script");
    fs::write(dir.join("c.n"), "print(\"c loaded\")\nexport val one = 1\n")
        .expect("Failed to write test module");
    fs::write(
        dir.join("a.n"),
        "import \"c\"\nprint(\"a loaded\")\nexport fn fa() { return c.one + 1 }\n",
    )
    .expect("Failed to write test module");
    fs::write(
        dir.join("b.n"),
        "import \"c\"\nprint(\"b loaded\")\nexport fn fb() { return c.one + 2 }\n",
    )
    .expect("Failed to write test module");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&main_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert!(output.status.success());
    assert_eq!(
        "c loaded\na loaded\nb loaded\nmain\n5\n",
        String::from_utf8_lossy(&output.stdout)
    );
    #[cfg(not(feature = "opcode-stats"))]
    assert_eq!("", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn run_file_reads_a_module_variable_after_it_changes() {
    let dir = std::env::temp_dir().join("neon_cli_test_run_module_live_binding");
    fs::create_dir_all(&dir).expect("Failed to create test dir");
    let main_path = dir.join("main.n");
    fs::write(
        &main_path,
        "import \"utils\"\nprint(utils.counter)\nutils.bump()\nprint(utils.counter)\n",
    )
    .expect("Failed to write test script");
    fs::write(
        dir.join("utils.n"),
        "export var counter = 0\nexport fn bump() { counter = counter + 1 }\n",
    )
    .expect("Failed to write test module");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&main_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert!(output.status.success());
    assert_eq!("0\n1\n", String::from_utf8_lossy(&output.stdout));
    #[cfg(not(feature = "opcode-stats"))]
    assert_eq!("", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn run_file_updates_module_globals_from_a_returned_closure() {
    let dir = std::env::temp_dir().join("neon_cli_test_run_module_closure_globals");
    fs::create_dir_all(&dir).expect("Failed to create test dir");
    let main_path = dir.join("main.n");
    fs::write(
        &main_path,
        "import \"utils\"\nval next = utils.make()\nprint(next())\nprint(next())\nprint(utils.counter)\n",
    )
    .expect("Failed to write test script");
    fs::write(
        dir.join("utils.n"),
        "export var counter = 0\nexport fn make() {\n    return fn() {\n        counter = counter + 1\n        return counter\n    }\n}\n",
    )
    .expect("Failed to write test module");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&main_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert!(output.status.success());
    assert_eq!("1\n2\n2\n", String::from_utf8_lossy(&output.stdout));
    #[cfg(not(feature = "opcode-stats"))]
    assert_eq!("", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn run_file_runs_an_aliased_module_import() {
    let dir = std::env::temp_dir().join("neon_cli_test_run_aliased_module_import");
    fs::create_dir_all(dir.join("lib")).expect("Failed to create test dir");
    let main_path = dir.join("main.n");
    fs::write(
        &main_path,
        "import \"lib/utils\" as u\nprint(u.double(21))\n",
    )
    .expect("Failed to write test script");
    fs::write(
        dir.join("lib").join("utils.n"),
        "export fn double(x) { return x * 2 }\n",
    )
    .expect("Failed to write test module");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&main_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert!(output.status.success());
    assert_eq!("42\n", String::from_utf8_lossy(&output.stdout));
    #[cfg(not(feature = "opcode-stats"))]
    assert_eq!("", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn run_file_keeps_entry_stack_slots_after_modules() {
    let dir = std::env::temp_dir().join("neon_cli_test_run_entry_stack_slots_after_modules");
    fs::create_dir_all(&dir).expect("Failed to create test dir");
    let main_path = dir.join("main.n");
    fs::write(
        &main_path,
        "import \"a\"\nimport \"b\"\nval m1 = 10\nval m2 = 20\nif true {\n    val q = m1 + m2\n    print(q)\n}\nfor i in 0..2 {\n    print(i + a.x)\n}\nprint(m1 + m2 + a.x + a.y + b.z)\n",
    )
    .expect("Failed to write test script");
    fs::write(
        dir.join("a.n"),
        "export val x = 1\nval hidden = 2\nexport val y = 3\n",
    )
    .expect("Failed to write test module");
    fs::write(dir.join("b.n"), "val hidden = 4\nexport val z = 5\n")
        .expect("Failed to write test module");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&main_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!("30\n1\n2\n39\n", String::from_utf8_lossy(&output.stdout));
}

#[test]
fn run_file_reports_a_runtime_error_inside_a_module_body_with_that_module_file_name_and_line() {
    let dir = unique_temp_dir("neon_cli_test_run_module_body_runtime_error");
    let main_path = dir.join("main.n");
    fs::write(&main_path, "import \"utils\"\n").expect("Failed to write test script");
    fs::write(dir.join("utils.n"), "val a = 1\nval x = [1][5]\n")
        .expect("Failed to write test module");
    let utils = dir.canonicalize().unwrap().join("utils.n");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&main_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert_eq!(70, output.status.code().unwrap());
    let stderr = String::from_utf8_lossy(&output.stderr);
    let utils = utils.display();
    assert!(stderr.starts_with(&format!("[{utils}:2:")), "{stderr}");
    assert!(
        stderr.contains(&format!("\n  at <script> ({utils}:2)\n")),
        "{stderr}"
    );
}

#[test]
fn run_file_names_both_files_in_the_call_trace_when_the_entry_file_calls_into_a_module_that_errors()
{
    let dir = unique_temp_dir("neon_cli_test_run_module_call_trace");
    let main_path = dir.join("main.n");
    fs::write(&main_path, "import \"utils\"\nutils.boom()\n").expect("Failed to write test script");
    fs::write(
        dir.join("utils.n"),
        "export fn boom() {\n  return [1][5]\n}\n",
    )
    .expect("Failed to write test module");
    let utils = dir.canonicalize().unwrap().join("utils.n");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&main_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert_eq!(70, output.status.code().unwrap());
    let stderr = String::from_utf8_lossy(&output.stderr);
    let utils = utils.display();
    let main = main_path.display();
    assert!(stderr.starts_with(&format!("[{utils}:2:")), "{stderr}");
    assert!(
        stderr.contains(&format!(
            "\n  at boom ({utils}:2)\n  at <script> ({main}:2)\n"
        )),
        "{stderr}"
    );
}

#[test]
fn run_file_reports_a_compile_error_in_an_imported_module_with_that_module_file_name() {
    let dir = unique_temp_dir("neon_cli_test_run_module_compile_error");
    let main_path = dir.join("main.n");
    fs::write(&main_path, "import \"b\"\n").expect("Failed to write test script");
    fs::write(dir.join("b.n"), "val x = 1\nval = 1\n").expect("Failed to write test module");
    let module = dir.canonicalize().unwrap().join("b.n");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg(&main_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert_eq!(65, output.status.code().unwrap());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(&format!("--> {}:2:", module.display())),
        "{stderr}"
    );
}

#[test]
fn check_mode_accepts_a_program_with_imports() {
    let dir = std::env::temp_dir().join("neon_cli_test_check_accepts_imports");
    fs::create_dir_all(&dir).expect("Failed to create test dir");
    let main_path = dir.join("main.n");
    fs::write(&main_path, "import \"utils\"\nprint(utils.double(21))\n")
        .expect("Failed to write test script");
    fs::write(
        dir.join("utils.n"),
        "export fn double(x) { return x * 2 }\n",
    )
    .expect("Failed to write test module");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("--check")
        .arg(&main_path)
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    assert!(output.status.success());
    assert_eq!("", String::from_utf8_lossy(&output.stdout));
    assert_eq!("", String::from_utf8_lossy(&output.stderr));
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
#[allow(clippy::expect_used)]
fn spawn_neon_with_stdin(
    configure: impl FnOnce(&mut Command),
    stdin_input: &[u8],
) -> std::process::Child {
    let mut command = Command::new(env!("CARGO_BIN_EXE_neon"));
    configure(&mut command);
    let mut child = command
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

    child
}

// Guards against a regression reintroducing an infinite REPL loop: a hung
// child is killed after the deadline instead of letting the test suite hang.
#[cfg(not(feature = "disassemble"))]
#[allow(clippy::expect_used)]
fn wait_with_timeout(
    mut child: std::process::Child,
    timeout: std::time::Duration,
) -> std::process::Output {
    let deadline = std::time::Instant::now() + timeout;
    loop {
        if child
            .try_wait()
            .expect("Failed to poll neon binary")
            .is_some()
        {
            break;
        }
        if std::time::Instant::now() >= deadline {
            child.kill().ok();
            child.wait().ok();
            panic!("neon process did not exit within {:?}", timeout);
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    child.wait_with_output().expect("Failed to run neon binary")
}

#[cfg(not(feature = "disassemble"))]
#[allow(clippy::expect_used)]
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

    let child = spawn_neon_with_stdin(
        |command| {
            command.arg(&script_path);
        },
        stdin_input,
    );
    let output = child.wait_with_output().expect("Failed to run neon binary");

    fs::remove_file(&script_path).ok();

    output
}

#[cfg(not(feature = "disassemble"))]
fn run_repl_with_stdin(stdin_input: &[u8]) -> std::process::Output {
    let child = spawn_neon_with_stdin(|_| {}, stdin_input);
    wait_with_timeout(child, std::time::Duration::from_secs(5))
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

#[allow(clippy::expect_used)]
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

#[cfg(unix)]
#[test]
fn fmt_unreadable_directory_is_skipped_rest_still_format_and_exits_74() {
    use std::os::unix::fs::PermissionsExt;

    let dir = unique_temp_dir("unreadable_dir");
    let a = dir.join("a.n");
    let locked = dir.join("locked");
    fs::write(&a, "val x = 1").expect("Failed to write test script");
    fs::create_dir(&locked).expect("Failed to create locked dir");

    let mut perms = fs::metadata(&locked).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&locked, perms).expect("Failed to set permissions");

    // Root ignores directory permissions, which would turn this into a false negative.
    if fs::read_dir(&locked).is_ok() {
        let mut perms = fs::metadata(&locked).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&locked, perms).ok();
        fs::remove_dir_all(&dir).ok();
        eprintln!("skipping: running as a user that bypasses directory permissions");
        return;
    }

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .arg("fmt")
        .arg(&dir)
        .output()
        .expect("Failed to run neon binary");

    let mut perms = fs::metadata(&locked).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&locked, perms).ok();

    let a_contents = fs::read_to_string(&a).unwrap_or_default();
    fs::remove_dir_all(&dir).ok();

    assert_eq!(74, output.status.code().unwrap());
    assert_eq!("val x = 1\n", a_contents);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(&locked.display().to_string()));
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

#[cfg(not(feature = "disassemble"))]
#[test]
fn repl_ends_at_eof_without_exit() {
    let output = run_repl_with_stdin(b"print(1)\n");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains('1'),
        "expected stdout to contain 1:\n{}",
        stdout
    );
}

#[cfg(not(feature = "disassemble"))]
#[test]
fn repl_exit_stops_before_eof() {
    let output = run_repl_with_stdin(b"print(2)\nexit\nprint(3)\n");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains('2'),
        "expected stdout to contain 2:\n{}",
        stdout
    );
    assert!(
        !stdout.contains('3'),
        "expected stdout not to contain 3:\n{}",
        stdout
    );
}

#[cfg(not(feature = "disassemble"))]
#[test]
fn repl_keeps_state_across_lines() {
    let output = run_repl_with_stdin(b"var x = 1\nprint(x)\nfn f() { return x + 1 }\nprint(f())\n");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(">> 1"),
        "expected stdout to contain the printed 1:\n{}",
        stdout
    );
    assert!(
        stdout.contains(">> 2"),
        "expected stdout to contain the printed 2:\n{}",
        stdout
    );
}

#[cfg(not(feature = "disassemble"))]
#[allow(clippy::expect_used)]
fn run_repl_in_dir_with_stdin(dir: &std::path::Path, stdin_input: &[u8]) -> std::process::Output {
    let child = spawn_neon_with_stdin(
        |command| {
            command.current_dir(dir);
        },
        stdin_input,
    );
    wait_with_timeout(child, std::time::Duration::from_secs(5))
}

#[cfg(not(feature = "disassemble"))]
#[test]
#[allow(clippy::expect_used)]
fn repl_resolves_an_import_relative_to_the_current_directory() {
    let dir = std::env::temp_dir().join("neon_cli_test_repl_import_relative_to_cwd");
    fs::create_dir_all(&dir).expect("Failed to create test dir");
    fs::write(dir.join("b.n"), "print(\"loaded b\")\n").expect("Failed to write test module");

    let output = run_repl_in_dir_with_stdin(&dir, b"import \"b\"\n");

    fs::remove_dir_all(&dir).ok();

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(combined.contains("loaded b"), "{combined}");
    assert!(
        !combined.contains("file imports are not available"),
        "{combined}"
    );
    assert!(!combined.contains("cannot find module"), "{combined}");
}

#[cfg(not(feature = "disassemble"))]
#[test]
#[allow(clippy::expect_used)]
fn repl_reports_missing_import_with_path_tried_in_the_current_directory() {
    let dir = std::env::temp_dir().join("neon_cli_test_repl_missing_import_in_cwd");
    fs::create_dir_all(&dir).expect("Failed to create test dir");
    let dir = dir.canonicalize().expect("Failed to canonicalize test dir");

    let output = run_repl_in_dir_with_stdin(&dir, b"import \"missing\"\n");

    fs::remove_dir_all(&dir).ok();

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        combined.contains("cannot find module 'missing'"),
        "{combined}"
    );
    assert!(
        combined.contains(&dir.join("missing.n").display().to_string()),
        "{combined}"
    );
    assert!(combined.contains("--> <repl>:1:"), "{combined}");
    assert!(combined.contains("| import \"missing\""), "{combined}");
}

#[test]
fn run_renders_an_entry_parse_error_with_the_relative_path_typed() {
    let dir = std::env::temp_dir().join("neon_cli_test_entry_parse_error_relative");
    fs::create_dir_all(&dir).expect("Failed to create test dir");
    fs::write(dir.join("syn.n"), "val x = 1\nval = 1\n").expect("Failed to write test script");

    let output = Command::new(env!("CARGO_BIN_EXE_neon"))
        .current_dir(&dir)
        .arg("syn.n")
        .output()
        .expect("Failed to run neon binary");

    fs::remove_dir_all(&dir).ok();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--> syn.n:2:"), "{stderr}");
}
