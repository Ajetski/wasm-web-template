mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log_u32(num: u32);

    #[wasm_bindgen(js_namespace = console)]
    fn log_i32(num: i32);
}

#[wasm_bindgen]
pub fn greet() {
    log("Hello, World!");
}
