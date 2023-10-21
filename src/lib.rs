mod utils;

use std::ops::Deref;
use wasm_bindgen::prelude::*;
use web_sys::{window, console};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    console::log_1(&JsValue::from("Hello World!"))
}

#[wasm_bindgen]
pub fn init() {
    console::time();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    for _ in 0..100000 {
        let div_element = document.create_element("div").unwrap();

        div_element.set_text_content(Option::from("Hello World"));
        body.append_child(div_element.deref()).expect("Hello World div wasn't appended");
    }


    console::time_end();
}
