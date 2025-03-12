// エンジン、シミュアプリは全部engineから必要なものを呼び出す
use anyhow::{anyhow, Result};
use std::{
    rc::Rc,
    cell::RefCell,
};

use wasm_bindgen::{JsValue,};
use web_sys::{CanvasRenderingContext2d,};

use crate::browser::{self, LoopClosure};

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

type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;
impl GameLoop {
    pub async fn start(game: impl Game + 'static) -> Result<()> {
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
            //process_input(&mut keystate, &mut keyevent_receiver);

            let frame_time = perf - game_loop.last_time;
            game_loop.accumulated_delta_time += frame_time as f32;
            //game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;
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

            let _ = browser::request_animation_frame(f.borrow().as_ref().unwrap());
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

// 2D描画用のレンダラー
pub struct Renderer2d{
    pub context: CanvasRenderingContext2d,
}

impl Renderer2d {
    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, self.context.canvas().unwrap().width().into(), self.context.canvas().unwrap().height().into());
    }

    // 以下2Dプリミティブの描画関数
    pub fn circle(&self, x: f64, y: f64, radius: f64) {
        self.context.begin_path();
        self.context.arc(x, y, radius, 0.0, std::f64::consts::PI * 2.0).unwrap();
        // 中を塗りつぶす
        self.context.set_fill_style(&JsValue::from_str("rgba(255, 0, 0, 0.9)"));
        self.context.fill();
        self.context.stroke();
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

    // 以下便利2D描画関数
    pub fn direction_triangle(&self, start_x: f64, start_y: f64) {
        // point struct作ったのでそれに従いましょう！！！
    }

}