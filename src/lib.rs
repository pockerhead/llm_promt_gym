// Экспортируем макрос (он автоматически экспортируется в корне крейта)
#[macro_use]
mod bindings;
mod types;
mod api;
mod ui;
mod chat;

// Экспортируем главную структуру для JavaScript
pub use chat::ChatApp;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    crate::console_log!("WASM module loaded successfully!");
} 