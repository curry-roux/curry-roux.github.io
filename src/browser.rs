// ブラウザとやり取りするためのコード
use anyhow::{anyhow, Result};
use std::{
    future::Future,
};

use wasm_bindgen::{
    prelude::{Closure, JsCast},
    closure::{WasmClosure,WasmClosureFnOnce,},
};
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement, Window, HtmlInputElement
};

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

pub fn context2d() -> Result<CanvasRenderingContext2d> {
    canvas()?
        .get_context("2d")
        .map_err(|js_value| anyhow!("Error getting 2d context {:#?}", js_value))?
        .ok_or_else(|| anyhow!("2d context not found"))?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|element|{
            anyhow!("Error converting {:#?} to CanvasRenderingContext2d", element)
        })
}

pub fn canvas() -> Result<HtmlCanvasElement>{
    document()?
        .get_element_by_id("canvas") // とりあえずid="canvas"をハードコーディングしておく
        .ok_or_else(|| anyhow!("No canvas Element found with id 'canvas'"))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
}

// キャンバスサイズ関連
pub fn set_canvas_fullscreen() -> Result<(u32, u32)> {
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

    log!("width: {}, height: {}", width, height);
    let canvas = canvas()?;
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    Ok((width as u32, height as u32))
}

pub fn set_canvas_left_top(width: u32, height: u32) -> Result<(u32, u32)> {
    // ブラウザの左側、正方形にcanvasを配置する
    let canvas = canvas()?;
    canvas.set_width(width);
    canvas.set_height(height);
    let html_canvas = canvas
        .dyn_into::<HtmlElement>()
        .map_err(|err: HtmlCanvasElement| anyhow!("Failed to convert canvas to HtmlElement: {:#?}", err))?;

    let style = html_canvas.style();
    let _ = style.set_property("position", "static");
    let _ = style.set_property("margin-top", "24px");

    Ok((width, height))
}

pub fn spawn_local<F>(future: F)
where 
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

pub type LoopClosure = Closure<dyn FnMut(f64)>;

pub fn request_animation_frame(callback: &LoopClosure) -> Result<i32>{
    window()?
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .map_err(|err| anyhow!("Failed to request animation frame {:#?}", err))
}

pub fn closure_once<F, A, R>(fn_once: F) -> Closure<F::FnMut>
where
    F: 'static + WasmClosureFnOnce<A, R>,
{
    Closure::once(fn_once)
}

pub fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
    Closure::wrap(data)
}

pub fn create_ref_closure(f: impl FnMut(f64) + 'static) -> LoopClosure {
    closure_wrap(Box::new(f))
}

// utils
pub fn now() -> Result<f64>{
    Ok(window()?
        .performance()
        .ok_or_else(|| anyhow!("Performance object not found"))?
        .now())
}

// パラメータUI回り
pub fn check_parameter_ui() -> Result<bool> {
    // パラメータuiが存在するかどうかを確認する
    let document = document()?;
    let exists = document
        .get_element_by_id("parameter")
        .is_some();

    if exists {Ok(true)} else {Ok(false)}
}

pub fn set_fps(fps: f64) -> Result<()> {
    let document = document()?;
    let fps_element = document
        .get_element_by_id("draw-fps")
        .ok_or_else(|| anyhow!("No element found with id 'draw-fps'"))?
        .dyn_into::<HtmlElement>()
        .map_err(|err| anyhow!("Failed to convert element to HtmlElement: {:#?}", err))?;

    fps_element.set_inner_text(&format!("FPS: {:.2}", fps));

    Ok(())
}

pub fn set_parameter_ui(name: &str) -> Result<()> {
    log!("init parameter ui slider: {}", name);
    let document = document()?;

    let slider = document
        .get_element_by_id(format!("param-{}", name).as_str())
        .ok_or_else(|| anyhow!("No element found with id 'param-{}'", name))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|err| anyhow!("Failed to convert element to HtmlInputElement: {:#?}", err))?;

    let number_input = document
        .get_element_by_id(format!("param-{}-input", name).as_str())
        .ok_or_else(|| anyhow!("No element found with id 'param-{}'", name))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|err| anyhow!("Failed to convert element to HtmlInputElement: {:#?}", err))?;

    Ok(())
}