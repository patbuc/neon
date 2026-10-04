#![cfg(target_arch = "wasm32")]

use neon::wasm::format_source;
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
