// ボイドモデルシミュレーター
use anyhow::{anyhow, Result};
use rand::Rng;
use async_trait::async_trait;

use crate::engine::{
    Game, Renderer2d, Point,
};

// Config
const BOID_SIZE: f32 = 10.0;
const BOID_COUNT: usize = 100;

// ボイドモデルシミュレータ
struct Boid {
    agents: Vec<BoidAgent>,
}

impl Boid {
    pub fn new() -> Self {
        Boid {
            agents: Vec::new(),
        }
    }
}


struct BoidAgent {
    position: Point,
    velocity: Point,
    acceleration: Point,
}

impl BoidAgent {
    // 位置と向きから描画用の三角形の頂点を返す
    pub fn triangle(&self) -> [Point; 3] {
        let angle = self.velocity.y.atan2(self.velocity.x) as f32;
        let angle1 = angle + 2.0 * std::f32::consts::PI / 3.0;
        let angle2 = angle - 2.0 * std::f32::consts::PI / 3.0;
        let x1 = self.position.x as f32 + BOID_SIZE * angle1.cos();
        let y1 = self.position.y as f32 + BOID_SIZE * angle1.sin();
        let x2 = self.position.x as f32 + BOID_SIZE * angle2.cos();
        let y2 = self.position.y as f32 + BOID_SIZE * angle2.sin();
        [self.position, Point { x: x1 as f64, y: y1 as f64 }, Point { x: x2 as f64, y: y2 as f64 }]
    }
}

#[async_trait(?Send)]
impl Game for Boid {
    async fn initialize(&self) -> Result<Box<dyn Game>> {

        let mut agents: Vec<BoidAgent> = vec![];
        let mut rng = rand::thread_rng();
        for i in 0..BOID_COUNT{
            let agent = BoidAgent {
                position: Point {
                    x: rng.gen_range(0.0..800.0),
                    y: rng.gen_range(0.0..600.0),
                },
                velocity: Point {
                    x: rng.gen_range(-1.0..1.0),
                    y: rng.gen_range(-1.0..1.0),
                },
                acceleration: Point {
                    x: 0.0,
                    y: 0.0,
                },
            };
            agents.push(agent);
        }

        Ok(Box::new(Boid {
            agents: agents,
        }))
    }

    fn update(&mut self) {
        log!("Boid update");
    }

    fn draw(&self, renderer: &Renderer2d) {
        renderer.clear();
        for agent in &self.agents {
            let triangle = agent.triangle();
            renderer.triangle(triangle[0].x, triangle[0].y, triangle[1].x, triangle[1].y, triangle[2].x, triangle[2].y);
        }
        log!("Boid draw");
    }
}