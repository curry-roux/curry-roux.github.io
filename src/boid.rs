// ボイドモデルシミュレーター
use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::engine::{
    Game, Renderer2d,
};

struct Boid;

#[async_trait(?Send)]
impl Game for Boid {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        Ok(Box::new(Boid))
    }

    fn update(&mut self) {
        log!("Boid update");
    }

    fn draw(&self, renderer: &Renderer2d) {
        renderer.clear();
        log!("Boid draw");
    }
}