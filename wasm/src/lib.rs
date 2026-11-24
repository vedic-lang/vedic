use vedic_core::aadhaar::Aadhaar;
use vedic_core::fetchsource::Sourcecode;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn vedic(source: &str) {
    vedic_with_path(source, "script");
}

#[wasm_bindgen]
pub fn vedic_with_path(source: &str, path: &str) {
    let mut aadhaar = Aadhaar::new();
    aadhaar.prarambha();
    let sc = Sourcecode {
        code: source.to_owned(),
        path,
    };

    let _ = aadhaar.vyakhyati(sc);
}

// Called when the wasm module is instantiated
// #[wasm_bindgen(start)]
// fn main() -> Result<(), JsValue> {
//     Ok(())
// }
