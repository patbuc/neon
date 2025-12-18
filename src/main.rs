use colored::Colorize;
use std::io::{Read, Write};
use std::process::exit;

use std::fs::File;
use std::path::Path;
use std::{env, io};

use neon::common::chunk::binary::{read_binary_file, write_binary_file, BinaryError};
use neon::common::stdlib::create_builtin_objects;
use neon::compiler::Compiler;
use neon::vm::{Result, VirtualMachine};

fn main() {
    setup_logging();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_tagline();
        run_repl();
    } else if args.len() >= 2 {
        match args[1].as_str() {
            "compile" => {
                run_compile(&args[2..]);
            }
            "run" => {
                run_binary(&args[2..]);
            }
            "help" | "--help" | "-h" => {
                print_help();
            }
            _ => {
                // Auto-detection: check if the file has .nbc extension
                let file_path = &args[1];
                if file_path.ends_with(".nbc") {
                    // Run as binary
                    run_binary(&args[1..]);
                } else {
                    // Interpret as source
                    let script_args = args[2..].to_vec();
                    run_file(file_path, script_args);
                }
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
            Result::Ok => {}
            Result::CompileError => {
                let formatted_errors = vm.get_formatted_errors("<repl>");
                eprintln!("{}", formatted_errors);
            }
            Result::RuntimeError => eprintln!("{}", "Runtime error.".red()),
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

fn run_file(path: &String, args: Vec<String>) {
    println!("Running file: {} ", path);

    let source = read_file(path);
    let mut vm = VirtualMachine::with_args(args);

    let result: Result = vm.interpret(source);
    match result {
        Result::Ok => (),
        Result::CompileError => {
            // Print formatted compilation errors
            let formatted_errors = vm.get_formatted_errors(path);
            eprintln!("{}", formatted_errors);
            exit(65);
        }
        Result::RuntimeError => exit(70),
    }
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap_or_else(|_| panic!("Failed to open the file {}", path));

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("Failed to read the file {}", path));
    contents
}

fn run_compile(args: &[String]) {
    if args.is_empty() {
        eprintln!("{}", "Error: Missing input file".red());
        print_compile_help();
        exit(64);
    }

    let input_path = &args[0];

    // Determine output path
    let output_path = if args.len() >= 3 && (args[1] == "-o" || args[1] == "--output") {
        args[2].clone()
    } else {
        // Default: replace extension with .nbc
        let input_path_obj = Path::new(input_path);
        let stem = input_path_obj
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        format!("{}.nbc", stem)
    };

    // Read source file
    let source = match std::fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}: {}", "Error reading file".red(), input_path, e);
            exit(66);
        }
    };

    // Compile source to chunk
    let builtin = create_builtin_objects(vec![]);
    let mut compiler = Compiler::new(builtin);
    let chunk = match compiler.compile(&source) {
        Some(chunk) => chunk,
        None => {
            let formatted_errors =
                format_compilation_errors(&compiler.get_structured_errors(), input_path, &source);
            eprintln!("{}", formatted_errors);
            exit(65);
        }
    };

    // Write compiled chunk to binary file
    let output_path_obj = Path::new(&output_path);
    if let Err(e) = write_binary_file(output_path_obj, &chunk) {
        eprintln!(
            "{} {}: {}",
            "Error writing binary file".red(),
            output_path,
            format_binary_error(&e)
        );
        exit(66);
    }

    println!("{} Compiled {} -> {}", "✓".green(), input_path, output_path);
}

fn run_binary(args: &[String]) {
    if args.is_empty() {
        eprintln!("{}", "Error: Missing binary file".red());
        print_run_help();
        exit(64);
    }

    let binary_path = &args[0];
    let script_args = args[1..].to_vec();

    // Read compiled binary
    let binary_path_obj = Path::new(binary_path);
    let chunk = match read_binary_file(binary_path_obj) {
        Ok(chunk) => chunk,
        Err(e) => {
            eprintln!(
                "{} {}: {}",
                "Error reading binary file".red(),
                binary_path,
                format_binary_error(&e)
            );
            exit(66);
        }
    };

    // Execute chunk in VM
    let mut vm = VirtualMachine::with_args(script_args);
    let result: Result = vm.execute_chunk(chunk);

    match result {
        Result::Ok => (),
        Result::CompileError => {
            // This shouldn't happen when executing pre-compiled bytecode
            eprintln!("{}", "Unexpected compilation error during execution".red());
            exit(65);
        }
        Result::RuntimeError => exit(70),
    }
}

fn format_binary_error(error: &BinaryError) -> String {
    match error {
        BinaryError::IoError(e) => format!("{}", e),
        BinaryError::SerializationError(msg) => format!("Serialization error: {}", msg),
        BinaryError::DeserializationError(msg) => format!("Deserialization error: {}", msg),
        BinaryError::InvalidFormat(msg) => format!("Invalid format: {}", msg),
        BinaryError::UnsupportedVersion { found, current } => {
            format!(
                "Unsupported format version: {} (current version is {})",
                found, current
            )
        }
    }
}

fn format_compilation_errors(
    errors: &[neon::common::errors::CompilationError],
    filename: &str,
    source: &str,
) -> String {
    use neon::common::error_renderer::ErrorRenderer;
    let renderer = ErrorRenderer::default();
    renderer.render_errors(errors, source, filename)
}

fn print_help() {
    println!(
        "Neon {} - a toy language you didn't wait for",
        env!("CARGO_PKG_VERSION")
    );
    println!();
    println!("Usage:");
    println!("  neon                     Start interactive REPL");
    println!("  neon <file.n>           Interpret source file");
    println!("  neon <file.nbc>         Execute compiled binary");
    println!("  neon compile <input.n> [-o <output.nbc>]");
    println!("                          Compile source to binary");
    println!("  neon run <file.nbc> [args...]");
    println!("                          Execute compiled binary");
    println!("  neon help               Show this help message");
    println!();
    println!("Examples:");
    println!("  neon script.n           # Interpret script.n");
    println!("  neon compile script.n   # Compile to script.nbc");
    println!("  neon script.nbc         # Execute compiled binary");
    println!("  neon run script.nbc arg1 arg2  # Execute with arguments");
}

fn print_compile_help() {
    println!("Usage: neon compile <input.n> [-o <output.nbc>]");
    println!();
    println!("Compile a Neon source file to bytecode.");
    println!();
    println!("Options:");
    println!("  -o, --output <file>  Specify output file (default: <input>.nbc)");
    println!();
    println!("Examples:");
    println!("  neon compile script.n              # Creates script.nbc");
    println!("  neon compile script.n -o out.nbc   # Creates out.nbc");
}

fn print_run_help() {
    println!("Usage: neon run <file.nbc> [args...]");
    println!();
    println!("Execute a compiled Neon binary file.");
    println!();
    println!("Arguments after the binary file are passed to the script.");
    println!();
    println!("Examples:");
    println!("  neon run script.nbc");
    println!("  neon run script.nbc arg1 arg2");
}
