#![cfg(target_arch = "wasm32")]

use neon::wasm::{format_source, interpret_once};
use wasm_bindgen_test::wasm_bindgen_test;

#[derive(serde::Deserialize)]
struct WasmResult {
    success: bool,
    output: Option<String>,
    error: Option<String>,
}

#[wasm_bindgen_test]
fn format_source_formats_valid_code() {
    let result: WasmResult =
        serde_wasm_bindgen::from_value(format_source("val x=1".to_string())).unwrap();

    assert!(result.success);
    assert_eq!(result.output.unwrap(), "val x = 1\n");
    assert!(result.error.is_none());
}

#[wasm_bindgen_test]
fn format_source_reports_syntax_error() {
    let result: WasmResult =
        serde_wasm_bindgen::from_value(format_source("val x =".to_string())).unwrap();

    assert!(!result.success);
    assert!(result.output.is_none());
    assert!(result.error.unwrap().contains("E0006"));
}

#[wasm_bindgen_test]
fn interpret_once_rejects_file_import() {
    let result: WasmResult =
        serde_wasm_bindgen::from_value(interpret_once("import \"b\"\n".to_string())).unwrap();

    assert!(!result.success);
    assert!(result
        .error
        .unwrap()
        .contains("file imports are not available in the browser build"));
}

#[wasm_bindgen_test]
fn interpret_once_does_not_reject_std_import_as_file_import() {
    let result: WasmResult =
        serde_wasm_bindgen::from_value(interpret_once("import \"std/math\"\n".to_string()))
            .unwrap();

    assert!(!result.success);
    let error = result.error.unwrap();
    assert!(error.contains("modules are not supported yet"));
    assert!(!error.contains("file imports are not available in the browser build"));
}
