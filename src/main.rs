use colored::Colorize;
use std::io::{Read, Write};
use std::process::exit;

use std::fs::File;
use std::{env, io};

use neon::vm::{Result, VirtualMachine};

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
            _ => {
                // Auto-detection: check if the file has .nbc extension
                let file_path = &args[1];
                // Interpret as source
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
