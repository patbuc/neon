use crate::vm::{Result, VirtualMachine};
use std::fs;
use std::path::Path;

#[test]
fn test_scripts() {
    let folder_path = "resources/tests/scripts"; // Change this to your folder path
    let entries = fs::read_dir(folder_path).expect("Could not read folder");
    for result in entries {
        let dir_entry = result.expect("Could not read entry");
        let path = dir_entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "n") {
            run_file(&path);
        }
    }
}

#[cfg(test)]
fn run_file(path: &Path) {
    print!("Running file: {:<34} - ", path.display());

    let script = fs::read_to_string(&path).expect("Could not read file");
    let mut result_path_buf = path.to_path_buf();
    result_path_buf.set_extension("result");
    let expected_result = fs::read_to_string(&result_path_buf).expect("Could not read file");

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(script.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!(expected_result, vm.get_output());

    println!("Passed");
}
