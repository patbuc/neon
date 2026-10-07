use colored::Colorize;
use std::io::{Read, Write};
use std::process::exit;

use std::fs::{self, File};
use std::path::Path;
use std::{env, io};

use neon::vm::{InterpretResult, VirtualMachine};

#[allow(clippy::print_stderr)]
fn main() {
    setup_logging();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_tagline();
        run_repl();
    } else if args.len() >= 2 {
        match args[1].as_str() {
            "help" | "--help" | "-h" => {
                print_help();
            }
            "--version" | "-V" => {
                print_version();
            }
            "--tokens" => {
                if args.len() < 3 {
                    eprintln!("Usage: neon --tokens <file>");
                    exit(64);
                }
                print_tokens(&args[2]);
            }
            "--check" => {
                if args.len() < 3 {
                    eprintln!("Usage: neon --check <file>");
                    exit(64);
                }
                check_file(&args[2]);
            }
            "-e" | "--eval" => {
                if args.len() < 3 {
                    eprintln!("Usage: neon -e <code> [args...]");
                    exit(64);
                }
                run_source(args[2].clone(), args[3..].to_vec(), "<eval>");
            }
            "-" => {
                run_source(read_stdin(), args[2..].to_vec(), "<stdin>");
            }
            "fmt" => {
                fmt_command(&args[2..]);
            }
            _ => {
                let file_path = &args[1];
                let script_args = args[2..].to_vec();
                run_file(file_path, script_args);
            }
        }
    }
}

fn setup_logging() {
    env_logger::init();
}

#[allow(clippy::print_stdout)]
fn print_tagline() {
    println!(
        "✨ neon {} - a toy language you didn't wait for",
        env!("NEON_VERSION")
    );
}

#[allow(clippy::print_stdout, clippy::print_stderr)]
fn run_repl() {
    println!("Type 'exit' or Ctrl+D to quit");

    // REPL has no command-line arguments
    let dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!("Failed to get the current directory: {}", err);
            exit(74);
        }
    };
    let mut vm = VirtualMachine::new();
    loop {
        print_prompt();
        let line = match read_line() {
            Some(line) => line,
            None => break,
        };
        if line == "exit" {
            println!("Ciao 👋 - May your coffee be strong");
            break;
        }
        let result = vm.interpret_line_in(line, &dir);
        match result {
            InterpretResult::Ok => {}
            InterpretResult::CompileError => {
                let formatted_errors = vm.get_formatted_errors("<repl>");
                eprintln!("{}", formatted_errors);
            }
            InterpretResult::RuntimeError => {
                if let Some(error) = vm.get_runtime_error() {
                    eprintln!("{}", error.report());
                }
                eprintln!("{}", "Runtime error.".red());
            }
        }
        println!();
    }
}

#[allow(clippy::print_stderr)]
fn read_line() -> Option<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => None,
        Ok(_) => Some(String::from(input.trim())),
        Err(err) => {
            eprintln!("Failed to read line: {}", err);
            exit(74);
        }
    }
}

#[allow(clippy::print_stdout, clippy::print_stderr)]
fn print_prompt() {
    print!(">> ");
    if let Err(err) = io::stdout().flush() {
        eprintln!("Failed to flush stdout: {}", err);
        exit(74);
    }
}

fn run_file(path: &str, args: Vec<String>) {
    let source = read_file(path);
    let mut vm = VirtualMachine::with_args(args);

    let result: InterpretResult = vm.interpret_file(Path::new(path), source);
    finish_run(&vm, result, path);
}

fn run_source(source: String, args: Vec<String>, name: &str) {
    let mut vm = VirtualMachine::with_args(args);
    let result = vm.interpret(source);
    finish_run(&vm, result, name);
}

#[allow(clippy::print_stderr)]
fn read_stdin() -> String {
    let mut source = String::new();
    match io::stdin().read_to_string(&mut source) {
        Ok(_) => source,
        Err(err) => {
            eprintln!("Failed to read stdin: {}", err);
            exit(74);
        }
    }
}

#[allow(clippy::print_stderr)]
fn finish_run(vm: &VirtualMachine, result: InterpretResult, name: &str) {
    let exit_code = match result {
        InterpretResult::Ok => None,
        InterpretResult::CompileError => {
            // Print formatted compilation errors
            let formatted_errors = vm.get_formatted_errors(name);
            eprintln!("{}", formatted_errors);
            Some(65)
        }
        InterpretResult::RuntimeError => {
            if let Some(error) = vm.get_runtime_error() {
                eprintln!("{}", error.report());
            }
            Some(70)
        }
    };

    #[cfg(feature = "opcode-stats")]
    {
        let report = vm.opcode_stats_report();
        if !report.is_empty() {
            eprintln!("{}", report);
        }
    }

    if let Some(code) = exit_code {
        exit(code);
    }
}

#[allow(clippy::print_stdout)]
fn print_tokens(path: &str) {
    let source = read_file(path);
    println!("{}", neon::compiler::tokens_to_json(&source));
}

#[allow(clippy::print_stderr)]
fn check_file(path: &str) {
    let source = read_file(path);
    let mut vm = VirtualMachine::new();

    if vm.check_file(Path::new(path), source) == InterpretResult::CompileError {
        let formatted_errors = vm.get_formatted_errors(path);
        eprintln!("{}", formatted_errors);
        exit(65);
    }
}

#[allow(clippy::print_stdout, clippy::print_stderr)]
fn fmt_command(args: &[String]) {
    let check_mode = args.first().map(|arg| arg == "--check").unwrap_or(false);
    let paths = if check_mode { &args[1..] } else { args };

    if paths.is_empty() {
        eprintln!("Usage: neon fmt [--check] <paths...>");
        exit(64);
    }

    for path in paths {
        if path != "-" && !Path::new(path).exists() {
            eprintln!("Path not found: {}", path);
            exit(66);
        }
    }

    let mut had_io_error = false;
    let mut files = Vec::new();
    for path in paths {
        if path == "-" {
            files.push(path.clone());
        } else if Path::new(path).is_dir() {
            collect_n_files(Path::new(path), &mut files, &mut had_io_error);
        } else {
            files.push(path.clone());
        }
    }

    let mut had_syntax_error = false;
    let mut would_change = false;

    for file in &files {
        let source = if file == "-" {
            let mut source = String::new();
            match io::stdin().read_to_string(&mut source) {
                Ok(_) => source,
                Err(err) => {
                    eprintln!("Failed to read stdin: {}", err);
                    had_io_error = true;
                    continue;
                }
            }
        } else {
            match read_file_for_fmt(file) {
                Some(source) => source,
                None => {
                    had_io_error = true;
                    continue;
                }
            }
        };

        match neon::compiler::format(&source) {
            Ok(formatted) => {
                let changed = formatted != source;
                would_change |= changed;
                if check_mode {
                    if changed {
                        println!("{}", file);
                    }
                } else if file == "-" {
                    print!("{}", formatted);
                } else if changed {
                    if let Err(err) = fs::write(file, &formatted) {
                        eprintln!("Failed to write the file {}: {}", file, err);
                        had_io_error = true;
                    }
                }
            }
            Err(errors) => {
                had_syntax_error = true;
                let rendered = neon::common::error_renderer::ErrorRenderer::default()
                    .render_errors(&errors, &source, file);
                eprintln!("{}", rendered);
            }
        }
    }

    if had_io_error {
        exit(74);
    }
    if had_syntax_error {
        exit(65);
    }
    if check_mode && would_change {
        exit(1);
    }
}

#[allow(clippy::print_stderr)]
fn read_file_for_fmt(path: &str) -> Option<String> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open the file {}: {}", path, err);
            return None;
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Some(contents),
        Err(err) => {
            eprintln!("Failed to read the file {}: {}", path, err);
            None
        }
    }
}

#[allow(clippy::print_stderr)]
fn collect_n_files(dir: &Path, out: &mut Vec<String>, had_io_error: &mut bool) {
    let read_dir = match fs::read_dir(dir) {
        Ok(read_dir) => read_dir,
        Err(err) => {
            eprintln!("Failed to read directory {}: {}", dir.display(), err);
            *had_io_error = true;
            return;
        }
    };

    let mut entries = Vec::new();
    for entry in read_dir {
        match entry {
            Ok(entry) => entries.push(entry),
            Err(err) => {
                eprintln!(
                    "Failed to read entry in directory {}: {}",
                    dir.display(),
                    err
                );
                *had_io_error = true;
            }
        }
    }
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(err) => {
                eprintln!(
                    "Failed to read file type of {}: {}",
                    entry.path().display(),
                    err
                );
                *had_io_error = true;
                continue;
            }
        };
        if file_type.is_symlink() {
            continue;
        }

        let path = entry.path();
        if file_type.is_dir() {
            collect_n_files(&path, out, had_io_error);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("n") {
            out.push(path.to_string_lossy().into_owned());
        }
    }
}

#[allow(clippy::print_stderr)]
fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap_or_else(|err| {
        eprintln!("Failed to open the file {}: {}", path, err);
        exit(66);
    });

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_else(|err| {
        eprintln!("Failed to read the file {}: {}", path, err);
        exit(74);
    });
    contents
}

#[allow(clippy::print_stdout)]
fn print_version() {
    println!("neon {}", env!("NEON_VERSION"));
}

#[allow(clippy::print_stdout)]
fn print_help() {
    println!(
        "Neon {} - a toy language you didn't wait for",
        env!("NEON_VERSION")
    );
    println!();
    println!("Usage:");
    println!("  neon                     Start interactive REPL");
    println!("  neon <file.n> [args...]  Interpret source file");
    println!("  neon --check <file.n>    Compile without executing");
    println!("  neon -e <code> [args...]  Run a snippet");
    println!("  neon fmt [--check] <paths...>  Format .n files in place");
    println!("  neon --version, -V       Print the version");
    println!("  neon help                Show this help message");
    println!();
    println!("Examples:");
    println!("  neon script.n            # Interpret script.n");
    println!("  neon script.n arg1 arg2  # Interpret script.n with arguments");
    println!("  neon fmt src/            # Format every .n file under src/");
    println!("  neon fmt --check src/    # Print files that would change, exit 1 if any");
}
