use vedic_core::aadhaar::Aadhaar;
use vedic_core::fetchsource::Sourcecode;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn vedic(str: &str) {
    let mut aadhaar = Aadhaar::new();
    aadhaar.prarambha();
    let sc = Sourcecode {
        code: str.to_owned(),
        path: "script",
    };

    let _ = aadhaar.vyakhyati(sc);
}

// Called when the wasm module is instantiated
// #[wasm_bindgen(start)]
// fn main() -> Result<(), JsValue> {
//     Ok(())
// }
