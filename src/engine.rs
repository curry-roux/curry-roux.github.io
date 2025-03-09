// エンジン、シミュアプリは全部engineから必要なものを呼び出す
use anyhow::{anyhow, Result};
use std::{
    rc::Rc,
    cell::RefCell,
};

use wasm_bindgen::{JsCast};
use web_sys::{CanvasRenderingContext2d, Window};

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

            //browser::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        Ok(())
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
}