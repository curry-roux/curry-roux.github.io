use wasm_bindgen::prelude::*;
use web_sys::{console, CanvasRenderingContext2d};
use wasm_bindgen::JsValue;

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

    browser::set_canvas_fullscreen().map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?;

    let canvas = browser::canvas().map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?;
    let ctx = canvas
        .get_context("2d")
        .map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?
        .ok_or_else(|| JsValue::from_str("Failed to get 2d context"))?;

    // canvasの背景を黒に設定
    let ctx_2d = ctx
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?;
    ctx_2d.set_fill_style(&JsValue::from_str("black"));
    ctx_2d.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());



    Ok(())
}
