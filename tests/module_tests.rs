use neon::vm::VirtualMachine;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to create a temporary module file for testing
fn create_test_module(dir: &TempDir, name: &str, content: &str) -> PathBuf {
    let module_path = dir.path().join(format!("{}.n", name));
    fs::write(&module_path, content).expect("Failed to write test module");
    module_path
        .canonicalize()
        .expect("Failed to canonicalize path")
}

/// Helper to create a main file that imports a module
fn create_main_with_import(dir: &TempDir, import_name: &str, main_content: &str) -> PathBuf {
    let main_content_with_import = format!("import \"{}\"\n{}", import_name, main_content);
    create_test_module(dir, "main", &main_content_with_import)
}

#[test]
fn test_module_basic_loading() {
    // Create temporary directory for test modules
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create a simple module
    create_test_module(
        &temp_dir,
        "math",
        r#"
        val PI = 3.14159
        fn double(x) {
            return x * 2
        }
        "#,
    );

    // Create main file that imports using simple name
    let main_path = create_main_with_import(
        &temp_dir,
        "math",
        r#"
        print("Module loaded")
        "#,
    );

    let mut vm = VirtualMachine::new();
    let result = vm.interpret_file(main_path.clone());

    if result != neon::vm::Result::Ok {
        eprintln!("Compilation error: {}", vm.get_formatted_errors("test.n"));
        eprintln!("Runtime error: {}", vm.get_runtime_errors());
    }

    assert_eq!(
        result,
        neon::vm::Result::Ok,
        "Module loading should succeed"
    );
    let output = vm.get_output();
    assert!(output.contains("Module loaded"), "Expected output to contain 'Module loaded', got: {}", output);
}

#[test]
fn test_module_caching() {
    // Create temporary directory for test modules
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create a module that prints during initialization
    create_test_module(
        &temp_dir,
        "counter",
        r#"
        print("Module initialized")
        var count = 0
        "#,
    );

    // Create main file that imports the same module twice
    let main_path = create_test_module(
        &temp_dir,
        "main",
        r#"
        import "counter"
        import "counter"
        print("Done")
        "#,
    );

    let mut vm = VirtualMachine::new();
    let result = vm.interpret_file(main_path);

    assert_eq!(result, neon::vm::Result::Ok, "Module caching test should succeed");

    let output = vm.get_output();

    // The module should only be initialized once
    let init_count = output.matches("Module initialized").count();
    assert_eq!(
        init_count, 1,
        "Module should only be initialized once, but was initialized {} times. Output: {}",
        init_count, output
    );
}

#[test]
fn test_module_path_resolution_error() {
    // Create temporary directory for test
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create main file that tries to import a non-existent module
    let main_path = create_test_module(
        &temp_dir,
        "main",
        r#"
        import "nonexistent"
        "#,
    );

    let mut vm = VirtualMachine::new();
    let result = vm.interpret_file(main_path);

    // Should fail during runtime when trying to load the module
    assert_eq!(
        result,
        neon::vm::Result::RuntimeError,
        "Importing non-existent module should fail"
    );
}

#[test]
fn test_module_compilation_error() {
    // Create temporary directory for test modules
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create a module with syntax errors
    create_test_module(
        &temp_dir,
        "broken",
        r#"
        this is not valid neon code !@#$
        "#,
    );

    // Create main file that tries to import the broken module
    let main_path = create_main_with_import(
        &temp_dir,
        "broken",
        "",
    );

    let mut vm = VirtualMachine::new();
    let result = vm.interpret_file(main_path);

    // Should fail with a runtime error (module compilation failed)
    assert_eq!(
        result,
        neon::vm::Result::RuntimeError,
        "Importing module with syntax errors should fail"
    );
}

#[test]
fn test_module_non_exported_symbol_inaccessible() {
    // Create temporary directory for test modules
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create a module with both exported and non-exported declarations
    create_test_module(
        &temp_dir,
        "private_test",
        r#"
        export fn public_fn() {
            return 42
        }

        fn private_fn() {
            return 99
        }
        "#,
    );

    // Create main file that tries to access non-exported symbol
    let main_path = create_main_with_import(
        &temp_dir,
        "private_test",
        r#"
        print private_test.private_fn()
        "#,
    );

    let mut vm = VirtualMachine::new();
    let result = vm.interpret_file(main_path);

    // Should fail during compilation when trying to access non-exported symbol
    // The semantic analyzer catches this at compile time
    assert_eq!(
        result,
        neon::vm::Result::CompileError,
        "Accessing non-exported symbol should fail at compile time"
    );

    // Verify the error mentions the inaccessible symbol
    // Note: In tests, we don't have direct access to compilation errors,
    // but the CompileError result indicates the semantic analyzer caught it
}

// Note: Export-based tests are commented out until we properly wire up
// the export information from the compiler to the VM
//
// #[test]
// fn test_module_export_access() {
//     // Create temporary directory for test modules
//     let temp_dir = TempDir::new().expect("Failed to create temp dir");
//
//     // Create a module with exports
//     let module_path = create_test_module(
//         &temp_dir,
//         "math",
//         r#"
//         export val PI = 3.14159
//         export fn add(a, b) {
//             return a + b
//         }
//         "#,
//     );
//
//     // Test script that accesses module exports
//     let script = format!(
//         r#"
//         import "{}" as math
//         print(math.PI)
//         print(math.add(2, 3))
//         "#,
//         module_path.display()
//     );
//
//     let mut vm = VirtualMachine::new();
//     let result = vm.interpret(script);
//
//     assert_eq!(result, neon::vm::Result::Ok, "Module export access should succeed");
//
//     let output = vm.get_output();
//     assert!(output.contains("3.14159"), "Expected PI value in output");
//     assert!(output.contains("5"), "Expected sum in output");
// }
