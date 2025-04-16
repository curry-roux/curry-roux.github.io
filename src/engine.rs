// エンジン、シミュアプリは全部engineから必要なものを呼び出す
use wasm_bindgen::prelude::*;
use anyhow::{anyhow, Ok, Result};
use std::{
    cell::{RefCell}, rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

use wasm_bindgen::{JsValue,};
use web_sys::{CanvasRenderingContext2d,};

use crate::browser::{self, LoopClosure};

// グローバルな停止フラグ
static LOOP_RUNNING: AtomicBool = AtomicBool::new(true);

#[wasm_bindgen]
pub fn stop_loop() {
    LOOP_RUNNING.store(false, Ordering::Relaxed);
}

// ゲーム（あるいはシミュレーター）
#[async_trait::async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self);
    fn draw(&self, renderer: &Renderer2d);
}

const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0; // 60fps
pub struct GameLoop {
    last_time: f64,
    accumulated_delta_time: f32,
}

pub type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;
impl GameLoop {
    pub async fn start(game: impl Game + 'static) -> Result<()>{
        LOOP_RUNNING.store(true, Ordering::Relaxed);

        let mut game = game.initialize().await?;

        let mut game_loop = GameLoop {
            last_time: browser::now()?,
            accumulated_delta_time: 0.0,
        };

        let renderer = Renderer2d {
            context: browser::context2d()?,
        };

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(browser::create_ref_closure(move |perf:f64|{
            if !LOOP_RUNNING.load(Ordering::Relaxed) {
                log!("Game Loop: Stopped");
                renderer.clear();
                return;
            }
            //process_input(&mut keystate, &mut keyevent_receiver);

            let frame_time = perf - game_loop.last_time;
            game_loop.accumulated_delta_time += frame_time as f32;

            while game_loop.accumulated_delta_time >= FRAME_SIZE{
                game.update();
                game_loop.accumulated_delta_time -= FRAME_SIZE;
            }
            game_loop.last_time = perf;
            game.draw(&renderer);

            // if cfg!(debug_assertions){
            //     unsafe {
            //         draw_frame_rate(&renderer, frame_time);
            //     }
            // }
            
            // let _ = browser::request_animation_frame(f.borrow().as_ref().unwrap());

            if f.borrow().is_some() {
                let _ = browser::request_animation_frame(f.borrow().as_ref().unwrap()).unwrap();
            } else {
                log!("Game Loop: Loop is None");
            }
        }));

        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("Game Loop: Loop is None"))?,
        )?;

        Ok(())
    }
}

// math utilsみたいなものになるんかなぁ
#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }

    pub fn new_fron_uint(x: u32, y: u32) -> Self {
        Self {x: x as f64, y: y as f64}
    }
}

// 2D描画用のレンダラー
pub struct Renderer2d{
    pub context: CanvasRenderingContext2d,
}

impl Renderer2d {
    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, self.context.canvas().unwrap().width().into(), self.context.canvas().unwrap().height().into());
    }

    // 以下2Dプリミティブの描画関数
    pub fn circle(&self, center: Point, radius: f64, color: &str) {
        let color_str = get_color(color);
        self.context.begin_path();
        self.context.arc(center.x, center.y, radius, 0.0, std::f64::consts::PI * 2.0).unwrap();
        // 中を塗りつぶす
        self.context.set_fill_style(&JsValue::from_str(color_str.as_str()));
        self.context.fill();
        // self.context.stroke(); 淵の線が描画される
        self.context.close_path();
    }

    pub fn line(&self, start: Point, end: Point, size: f64, color: &str) {
        let color_str = get_color(color);
        self.context.begin_path();
        self.context.move_to(start.x, start.y);
        self.context.line_to(end.x, end.y);
        self.context.set_stroke_style(&JsValue::from_str(color_str.as_str()));
        self.context.set_line_width(size);
        self.context.stroke();
        self.context.close_path();
    }

    pub fn triangle(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        self.context.begin_path();
        self.context.move_to(x1, y1);
        self.context.line_to(x2, y2);
        self.context.line_to(x3, y3);
        self.context.close_path();
        // 中を塗りつぶす
        self.context.set_fill_style(&JsValue::from_str("rgba(0, 255, 0, 0.9)"));
        self.context.fill();
        self.context.stroke();
    }

}

fn get_color(color: &str) -> String {
    match color {
        "red" => "rgba(255, 0, 0, 0.9)".to_string(),
        "green" => "rgba(0, 255, 0, 0.9)".to_string(),
        "blue" => "rgba(0, 0, 255, 0.9)".to_string(),
        _ => "rgba(10, 10, 10, 0.9)".to_string(), // default to black
    }
}