use crate::vm::{Result, VirtualMachine};
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
            vm: VirtualMachine::new(Vec::new()),
        }
    }

    pub fn interpret(&mut self, source: String) -> JsValue {
        let result = self.vm.interpret(source);

        match result {
            Result::Ok => {
                let output = self.vm.get_output();
                self.vm.clear_output();
                serde_wasm_bindgen::to_value(&InterpretResult {
                    success: true,
                    output: Some(output),
                    error: None,
                })
                .unwrap()
            }
            Result::CompileError => {
                let errors = self.vm.get_formatted_errors("<input>");
                serde_wasm_bindgen::to_value(&InterpretResult {
                    success: false,
                    output: None,
                    error: Some(errors),
                })
                .unwrap()
            }
            Result::RuntimeError => serde_wasm_bindgen::to_value(&InterpretResult {
                success: false,
                output: None,
                error: Some("Runtime error".to_string()),
            })
            .unwrap(),
        }
    }
}

#[derive(serde::Serialize)]
struct InterpretResult {
    success: bool,
    output: Option<String>,
    error: Option<String>,
}

#[wasm_bindgen]
pub fn interpret_once(source: String) -> JsValue {
    console_error_panic_hook::set_once();
    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(source);

    match result {
        Result::Ok => {
            let output = vm.get_output();
            serde_wasm_bindgen::to_value(&InterpretResult {
                success: true,
                output: Some(output),
                error: None,
            })
            .unwrap()
        }
        Result::CompileError => {
            let errors = vm.get_formatted_errors("<input>");
            serde_wasm_bindgen::to_value(&InterpretResult {
                success: false,
                output: None,
                error: Some(errors),
            })
            .unwrap()
        }
        Result::RuntimeError => serde_wasm_bindgen::to_value(&InterpretResult {
            success: false,
            output: None,
            error: Some("Runtime error".to_string()),
        })
        .unwrap(),
    }
}
