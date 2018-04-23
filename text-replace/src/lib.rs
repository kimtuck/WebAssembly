#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "./index")]
extern {
    fn update_text(s: &str);
}

#[wasm_bindgen]
pub fn text_replace(from: &str, to: &str, text: &str) {
    let stext = text.to_string();
    let newtext = stext.replace(from,to);
    update_text(&newtext);
}