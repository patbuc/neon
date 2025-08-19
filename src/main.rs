mod compiler;
mod vm;

use colored::Colorize;
use std::io::{Read, Write};
use std::process::exit;

use std::fs::File;
use std::{env, io};

use crate::vm::VirtualMachine;

fn main() {
    setup_logging();

    print_tagline();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        run_repl();
    } else if args.len() >= 2 {
        run_file(&args[1]);
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
        "🟣 Neon {} - a toy language you didn't wait for",
        env!("CARGO_PKG_VERSION")
    );
}

fn run_repl() {
    println!("Type 'exit' or Ctrl+C to quit");

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
            vm::Result::Ok => {}
            vm::Result::CompileError => eprintln!("{}", "Compile error.".red()),
            vm::Result::RuntimeError => eprintln!("{}", "Runtime error.".red()),
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

fn run_file(path: &String) {
    println!("Running file: {} ", path);
    println!();

    let source = read_file(path);
    let mut vm = VirtualMachine::new();

    let result: vm::Result = vm.interpret(source);
    match result {
        vm::Result::Ok => return,
        vm::Result::CompileError => exit(65),
        vm::Result::RuntimeError => exit(70),
    }
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect(format!("Failed to open the file {}", path).as_str());

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(format!("Failed to read the file {}", path).as_str());
    contents
}
