use crate::vm::{InterpretResult, VirtualMachine};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct NeonVM {
    vm: VirtualMachine,
}

#[wasm_bindgen]
impl NeonVM {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        NeonVM {
            vm: VirtualMachine::new(),
        }
    }

    pub fn interpret(&mut self, source: String) -> JsValue {
        let result = self.vm.interpret(source);

        match result {
            InterpretResult::Ok => {
                let output = self.vm.get_output();
                self.vm.clear_output();
                serde_wasm_bindgen::to_value(&WasmResult {
                    success: true,
                    output: Some(output),
                    error: None,
                })
                .unwrap()
            }
            InterpretResult::CompileError => {
                let errors = self.vm.get_formatted_errors("<input>");
                serde_wasm_bindgen::to_value(&WasmResult {
                    success: false,
                    output: None,
                    error: Some(errors),
                })
                .unwrap()
            }
            InterpretResult::RuntimeError => {
                let errors = self
                    .vm
                    .get_runtime_error()
                    .map(|e| e.report())
                    .unwrap_or_default();
                serde_wasm_bindgen::to_value(&WasmResult {
                    success: false,
                    output: None,
                    error: Some(errors),
                })
                .unwrap()
            }
        }
    }
}

#[derive(serde::Serialize)]
struct WasmResult {
    success: bool,
    output: Option<String>,
    error: Option<String>,
}

#[wasm_bindgen]
pub fn interpret_once(source: String) -> JsValue {
    console_error_panic_hook::set_once();
    let mut vm = VirtualMachine::new();
    let result = vm.interpret(source);

    match result {
        InterpretResult::Ok => {
            let output = vm.get_output();
            serde_wasm_bindgen::to_value(&WasmResult {
                success: true,
                output: Some(output),
                error: None,
            })
            .unwrap()
        }
        InterpretResult::CompileError => {
            let errors = vm.get_formatted_errors("<input>");
            serde_wasm_bindgen::to_value(&WasmResult {
                success: false,
                output: None,
                error: Some(errors),
            })
            .unwrap()
        }
        InterpretResult::RuntimeError => {
            let errors = vm
                .get_runtime_error()
                .map(|e| e.report())
                .unwrap_or_default();
            serde_wasm_bindgen::to_value(&WasmResult {
                success: false,
                output: None,
                error: Some(errors),
            })
            .unwrap()
        }
    }
}
