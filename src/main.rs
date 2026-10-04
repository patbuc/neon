use colored::Colorize;
use std::io::{Read, Write};
use std::process::exit;

use std::fs::File;
use std::{env, io};

use neon::vm::{InterpretResult, VirtualMachine};

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
    #[cfg(not(feature = "disassemble"))]
    env_logger::init();
    #[cfg(feature = "disassemble")]
    setup_tracing();
}

#[cfg(feature = "disassemble")]
fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::ENTER
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        )
        .init()
}

fn print_tagline() {
    println!(
        "✨ neon {} - a toy language you didn't wait for",
        env!("CARGO_PKG_VERSION")
    );
}

fn run_repl() {
    println!("Type 'exit' or Ctrl+C to quit");

    // REPL has no command-line arguments
    let mut vm = VirtualMachine::new();
    loop {
        print_prompt();
        let line = read_line();
        if line == "exit" {
            println!("Ciao 👋 - May your coffee be strong");
            break;
        }
        let result = vm.interpret(line);
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

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    String::from(input.trim())
}

fn print_prompt() {
    print!(">> ");
    io::stdout().flush().unwrap();
}

fn run_file(path: &str, args: Vec<String>) {
    let source = read_file(path);
    let mut vm = VirtualMachine::with_args(args);

    let result: InterpretResult = vm.interpret(source);
    let exit_code = match result {
        InterpretResult::Ok => None,
        InterpretResult::CompileError => {
            // Print formatted compilation errors
            let formatted_errors = vm.get_formatted_errors(path);
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

fn print_tokens(path: &str) {
    let source = read_file(path);
    println!("{}", neon::compiler::tokens_to_json(&source));
}

fn check_file(path: &str) {
    let source = read_file(path);
    let mut vm = VirtualMachine::new();

    if vm.check(source) == InterpretResult::CompileError {
        let formatted_errors = vm.get_formatted_errors(path);
        eprintln!("{}", formatted_errors);
        exit(65);
    }
}

fn fmt_command(args: &[String]) {
    let check_mode = args.first().map(|arg| arg == "--check").unwrap_or(false);
    let paths = if check_mode { &args[1..] } else { args };

    if paths.is_empty() {
        eprintln!("Usage: neon fmt [--check] <paths...>");
        exit(64);
    }

    for path in paths {
        if path != "-" && !std::path::Path::new(path).exists() {
            eprintln!("Path not found: {}", path);
            exit(66);
        }
    }

    let mut files = Vec::new();
    for path in paths {
        if path == "-" {
            files.push(path.clone());
        } else if std::path::Path::new(path).is_dir() {
            collect_n_files(std::path::Path::new(path), &mut files);
        } else {
            files.push(path.clone());
        }
    }

    let mut had_error = false;
    let mut would_change = false;

    for file in &files {
        let source = if file == "-" {
            let mut source = String::new();
            io::stdin()
                .read_to_string(&mut source)
                .unwrap_or_else(|err| {
                    eprintln!("Failed to read stdin: {}", err);
                    exit(74);
                });
            source
        } else {
            read_file(file)
        };

        match neon::compiler::format(&source) {
            Ok(formatted) => {
                let changed = formatted != source;
                would_change |= changed;
                if file == "-" {
                    if check_mode {
                        if changed {
                            println!("-");
                        }
                    } else {
                        print!("{}", formatted);
                    }
                } else if changed {
                    if check_mode {
                        println!("{}", file);
                    } else {
                        std::fs::write(file, &formatted).unwrap_or_else(|err| {
                            eprintln!("Failed to write the file {}: {}", file, err);
                            exit(74);
                        });
                    }
                }
            }
            Err(errors) => {
                had_error = true;
                let rendered = neon::common::error_renderer::ErrorRenderer::default()
                    .render_errors(&errors, &source, file);
                eprintln!("{}", rendered);
            }
        }
    }

    if had_error {
        exit(65);
    }
    if check_mode && would_change {
        exit(1);
    }
}

/// Recursively collects `.n` files under `dir`, sorted within each directory
/// for deterministic ordering.
fn collect_n_files(dir: &std::path::Path, out: &mut Vec<String>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .unwrap_or_else(|err| {
            eprintln!("Failed to read directory {}: {}", dir.display(), err);
            exit(74);
        })
        .filter_map(|entry| entry.ok())
        .collect();
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let file_type = entry.file_type().unwrap_or_else(|err| {
            eprintln!(
                "Failed to read file type of {}: {}",
                entry.path().display(),
                err
            );
            exit(74);
        });
        if file_type.is_symlink() {
            continue;
        }

        let path = entry.path();
        if file_type.is_dir() {
            collect_n_files(&path, out);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("n") {
            out.push(path.to_string_lossy().into_owned());
        }
    }
}

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

fn print_help() {
    println!(
        "Neon {} - a toy language you didn't wait for",
        env!("CARGO_PKG_VERSION")
    );
    println!();
    println!("Usage:");
    println!("  neon                     Start interactive REPL");
    println!("  neon <file.n> [args...]  Interpret source file");
    println!("  neon --check <file.n>    Compile without executing");
    println!("  neon fmt [--check] <paths...>  Format .n files in place");
    println!("  neon help                Show this help message");
    println!();
    println!("Examples:");
    println!("  neon script.n            # Interpret script.n");
    println!("  neon script.n arg1 arg2  # Interpret script.n with arguments");
    println!("  neon fmt src/            # Format every .n file under src/");
    println!("  neon fmt --check src/    # Print files that would change, exit 1 if any");
}
