//!
//! Handling of WASM panic hook that allows activation of console-based panic hook
//! as well as a browser-based panic hook.  (the browser-based panic hook activates a full-screen debug 
//! information output in case of a panic - useful on mobile devices or where 
//! the user otherwise has no access to console/developer tools)
//! 

use wasm_bindgen::prelude::*;
use workflow_panic_hook::{set_once, Type, show_logs as show_wasm_logs};

/// Initialize panic hook in console mode
#[wasm_bindgen]
pub fn init_console_panic_hook(){
    set_once(Type::Console);
}

/// Initialize panic hook in browser mode
#[wasm_bindgen]
pub fn init_popup_panic_hook(){
    set_once(Type::Popup);
}

/// Present panic logs to the user
#[wasm_bindgen]
pub fn show_panic_hook_logs(){
    show_wasm_logs();
}
