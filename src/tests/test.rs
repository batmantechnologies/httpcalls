use gloo_console::log;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn pass() {
    log!(" Hello");
    assert_eq!(1, 1);
}

#[wasm_bindgen_test]
fn fail() {
    log!(" Hello");
    assert_eq!(1, 2);
}
