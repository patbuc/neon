use colored::Colorize;
use std::io::{Read, Write};
use std::process::exit;

use std::fs::File;
use std::{env, io};

use neon::vm::debug::DebugHandler;
use neon::vm::debugger::CliDebugger;
use neon::vm::{Result, VirtualMachine};

/// Extracts the --debug flag from command line arguments.
///
/// Returns (debug_enabled, remaining_args) where remaining_args excludes all --debug flags.
fn extract_debug_flag(args: &[String]) -> (bool, Vec<String>) {
    let mut debug_enabled = false;
    let mut remaining = Vec::new();

    for arg in args {
        if arg == "--debug" {
            debug_enabled = true;
        } else {
            remaining.push(arg.clone());
        }
    }

    (debug_enabled, remaining)
}

fn main() {
    setup_logging();

    print_tagline();

    let args: Vec<String> = env::args().collect();
    let (debug_enabled, remaining_args) = extract_debug_flag(&args);

    // Create debug handler if --debug flag is present
    let debug_handler: Option<Box<dyn DebugHandler>> = if debug_enabled {
        Some(Box::new(CliDebugger::new()))
    } else {
        None
    };

    if remaining_args.len() == 1 {
        run_repl(debug_handler);
    } else if remaining_args.len() >= 2 {
        run_file(&remaining_args[1], debug_handler);
    } else {
        exit(64);
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

fn run_repl(debug_handler: Option<Box<dyn DebugHandler>>) {
    println!("Type 'exit' or Ctrl+C to quit");

    let mut vm = VirtualMachine::with_args_and_debug(vec![], debug_handler);
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

fn run_file(path: &String, debug_handler: Option<Box<dyn DebugHandler>>) {
    println!("Running file: {} ", path);

    let source = read_file(path);
    let mut vm = VirtualMachine::with_args_and_debug(vec![], debug_handler);

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
