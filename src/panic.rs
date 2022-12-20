//!
//! Handling of WASM panic hook that allows activation of console-based panic hool
//! as well as a UI-based panic hook.  (the UI-base panic hook activates a full-screen debug 
//! information output in case of a panic - useful on mobile devices or there 
//! user otherwise has no access to console/developer tools)
//! 

use wasm_bindgen::prelude::*;
use workflow_panic_hook::{set_once, Type, show_logs as show_wasm_logs};

#[wasm_bindgen]
pub fn init_console_panic_hook(){
    set_once(Type::Console);
}

#[wasm_bindgen]
pub fn init_popup_panic_hook(){
    set_once(Type::Popup);
}

#[wasm_bindgen]
pub fn show_panic_hook_logs(){
    show_wasm_logs();
}
