use colored::Colorize;
use std::io::{Read, Write};
use std::process::exit;

use std::fs::File;
use std::{env, io};

use neon::vm::{Result, VirtualMachine};

fn main() {
    setup_logging();

    print_tagline();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        run_repl();
    } else if args.len() >= 2 {
        // Pass all arguments after the script path to the script
        let script_args = args[2..].to_vec();
        run_file(&args[1], script_args);
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
