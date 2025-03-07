use wasm_bindgen::prelude::*;
use web_sys::console;

#[macro_use]
mod browser;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("ほいお～！"));

    Ok(())
}

#[wasm_bindgen]
pub fn test1() -> Result<(), JsValue> {
    log!("ほいほいお～！");

    Ok(())
}
