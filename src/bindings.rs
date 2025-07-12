use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = window)]
    pub fn processMarkdown(text: &str) -> String;
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::bindings::log(&format_args!($($t)*).to_string()))
} 