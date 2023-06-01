#[cfg(not(target_arch = "wasm32"))]
pub fn log(message: &str) {
    println!("{message}");
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log_dosa(message: &str) {
    eprintln!("{message}");
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[cfg(target_arch = "wasm32")]
pub fn log(message: &str) {
    let js_value: JsValue = message.into();
    console::log_1(&js_value)
}

#[cfg(target_arch = "wasm32")]
pub fn log_dosa(message: &str) {
    let js_value: JsValue = message.into();
    console::error_1(&js_value)
}
