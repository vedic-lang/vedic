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

// Tabs on both lines start at the same gutter. Measure each intervening run in terminal cells.
pub(crate) fn log_insertion(heading: &str, line: &str, prefix: &str, delimiter: char) {
    use unicode_width::UnicodeWidthStr;

    let padding = prefix
        .split('\t')
        .map(|run| " ".repeat(run.width()))
        .collect::<Vec<_>>()
        .join("\t");
    let hint = format!("^ insert '{delimiter}' here");
    let message = format!("{heading}\n    {line}\n    {padding}{hint}");

    #[cfg(not(target_arch = "wasm32"))]
    log_dosa(&message);

    #[cfg(target_arch = "wasm32")]
    {
        // Browser fonts shape Devanagari clusters proportionally. Supply the actual prefix
        // alongside the plain-text diagnostic so the IDE can align the marker to the glyphs.
        let details = js_sys::Object::new();
        for (key, value) in [
            ("heading", heading),
            ("line", line),
            ("prefix", prefix),
            ("hint", &hint),
        ] {
            js_sys::Reflect::set(&details, &key.into(), &value.into()).unwrap();
        }
        console::error_2(&message.into(), &details);
    }
}
