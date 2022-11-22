use wasm_bindgen::prelude::*;
use workflow_panic_hook::{set_once, Type, show_logs as show_wasm_logs};

#[wasm_bindgen]
pub fn init_console_logger(){
    set_once(Type::Console);
}

#[wasm_bindgen]
pub fn init_popup_logger(){
    set_once(Type::Popup);
}

#[wasm_bindgen]
pub fn show_logs(){
    show_wasm_logs();
}