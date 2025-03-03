// ブラウザとやり取りするためのコード
use anyhow::{anyhow, Result};

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Window, Document, HtmlCanvasElement};

macro_rules! log {
    ( $($t:tt)* ) => {
        web_sys::console::log_1(&format!( $($t)* ).into());
    }
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("no window found"))
}

pub fn document() -> Result<Document> {
    window()?.document().ok_or_else(|| anyhow!("no document found"))
}

pub fn canvas() -> Result<HtmlCanvasElement>{
    document()?
        .get_element_by_id("canvas") // とりあえずid="canvas"をハードコーディングしておく
        .ok_or_else(|| anyhow!("No canvas Element found with id 'canvas'"))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
}

// もしかしてキャンバスのサイズを返した方がいいかも？
pub fn set_canvas_fullscreen() -> Result<()> {
    let window = window()?;
    let width = window
        .inner_width()
        .map_err(|err| anyhow!("window.inner_width() failed: {:#?}", err))?
        .as_f64()
        .ok_or_else(|| anyhow!("window.inner_width() is not a f64"))?;
    let height = window
        .inner_height()
        .map_err(|err| anyhow!("window.inner_width() failed: {:#?}", err))?
        .as_f64()
        .ok_or_else(|| anyhow!("window.inner_height() is not a f64"))?;

    let canvas = canvas()?;
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    Ok(())
}