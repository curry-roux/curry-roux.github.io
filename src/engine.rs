// エンジン、シミュアプリは全部engineから必要なものを呼び出す

use web_sys::CanvasRenderingContext2d;


// ゲーム（あるいはシミュレーター）
pub trait Game {
    fn update(&mut self);
    fn draw(&self, renderer: &Renderer2d);
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