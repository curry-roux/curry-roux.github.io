// ボイドモデルシミュレーター
use anyhow::{anyhow, Result};
use std::{f64::consts::PI};
use rand::Rng;
use async_trait::async_trait;

use crate::engine::{
    Game, Renderer2d, Point,
};

// Config
const BOID_SIZE: f64 = 15.0;
const BOID_COUNT: usize = 5;

// ボイドモデルシミュレータ
pub struct Boid {
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
        let angle: f64 = if self.velocity.x == 0.0 && self.velocity.y == 0.0 {
            0.0
        } else {
            self.velocity.y.atan2(self.velocity.x)
        };

        let sendo = 0.6; // 三角形の尖り具合

        let front_x = self.position.x + BOID_SIZE*angle.cos();
        let front_y = self.position.y + BOID_SIZE*angle.sin();

        let left_angle = angle + (2.0*PI / 3.0);
        let left_x = self.position.x + (BOID_SIZE*sendo) * left_angle.cos();
        let left_y = self.position.y + (BOID_SIZE*sendo) * left_angle.sin();

        let right_angle = angle - (2.0*PI / 3.0);
        let right_x = self.position.x + (BOID_SIZE*sendo) * right_angle.cos();
        let right_y = self.position.y + (BOID_SIZE*sendo) * right_angle.sin();

        [
            Point{x: front_x, y: front_y},
            Point{x: left_x, y: left_y},
            Point{x: right_x, y: right_y},
        ]
    }
}

#[async_trait(?Send)]
impl Game for Boid {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        log!("Boid initialize");
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
        for agent in &mut self.agents {
            // 速度を更新
            agent.velocity.x += agent.acceleration.x;
            agent.velocity.y += agent.acceleration.y;

            // 位置を更新
            agent.position.x += agent.velocity.x;
            agent.position.y += agent.velocity.y;

            // 加速度をリセット
            agent.acceleration.x = 0.0;
            agent.acceleration.y = 0.0;
        }
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