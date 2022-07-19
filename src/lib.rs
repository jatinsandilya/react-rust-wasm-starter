use wasm_bindgen::prelude::*;
use web_sys::console;
// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Code that is defined in JS but to be used in Rust land.
#[wasm_bindgen(module = "/js/defined-in-js.js")]
extern "C" {
    fn name() -> String;
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world! (from wasm)"));
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("Hello from Rust, WebAssembly, and Webpack! Wuhoo!"));
    console::log_1(&JsValue::from_str(&name()));
    let _ = body.append_child(&p);

    Ok(())
}

// Test code in wasm.
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
