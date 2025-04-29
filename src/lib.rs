use wasm_bindgen::prelude::*;
//use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::JsValue;

#[macro_use]
mod browser;
mod engine;
mod boid;
mod analog_clock;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub fn test1() -> Result<(), JsValue> {
    log!("ほいほいお～！");

    //browser::set_canvas_fullscreen().map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?;
    browser::set_canvas_left_top(600, 600).map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?;

    let canvas = browser::canvas().map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?;
    let ctx = canvas
        .get_context("2d")
        .map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?
        .ok_or_else(|| JsValue::from_str("Failed to get 2d context"))?;

    // canvasの背景を黒に設定 後で消す
    let ctx_2d = ctx
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?;
    ctx_2d.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.2)"));
    ctx_2d.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

    // // windows resizeイベントを登録
    // let window = browser::window().map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?;
    // let closure = Closure::<dyn FnMut()>::wrap(Box::new(move|| {
    //     log!("Window resized!");
    //     browser::set_canvas_fullscreen().map_err(|err| JsValue::from_str(&format!("{:#?}", err))).unwrap();
    // }));

    // window
    //     .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
    //     .map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?;

    // closure.forget();

    // debug
    log!("デバッグ！");
    let renderer = engine::Renderer2d{context: ctx_2d};
    renderer.clear();
    let width = canvas.width();
    let height = canvas.height();

    browser::spawn_local(async move{
        // let game = boid::Boid::new(width, height);
        let game = analog_clock::AnalogClock::new(width, height);

        engine::GameLoop::start(game)
            .await
            .expect("Failed to start game");
    });

    Ok(())
}


#[wasm_bindgen]
pub fn test2() -> Result<(), JsValue> {
    log!("test2 called!");
    Ok(())
}

#[wasm_bindgen]
pub fn boid() -> Result<(), JsValue> {
    log!("boid called!");

    browser::set_canvas_fullscreen().map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?;

    let canvas = browser::canvas().map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?;
    let ctx = canvas
        .get_context("2d")
        .map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?
        .ok_or_else(|| JsValue::from_str("Failed to get 2d context"))?;

    // canvasの背景を黒に設定 後で消す
    let ctx_2d = ctx
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|err| JsValue::from_str(&format!("{:#?}", err)))?;
    ctx_2d.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.2)"));
    ctx_2d.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

    let width = canvas.width();
    let height = canvas.height();

    browser::spawn_local(async move{
        let game = boid::Boid::new(width, height);

        engine::GameLoop::start(game)
            .await
            .expect("Failed to start game");
    });

    Ok(())
}


// 以下テスト&デバッグ用