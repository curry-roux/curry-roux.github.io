// ボイドモデルシミュレーター
// メモ
// 距離行列持って、差分計算とか頑張るともっと高速化できるかも
use anyhow::{anyhow, Result};
use std::{f64::consts::PI};
use rand::Rng;
use async_trait::async_trait;

use crate::engine::{
    Game, Renderer2d, Point,
};

// Config
const BOID_SIZE: f64 = 15.0;
const BOID_COUNT: usize = 30;

const MAX_FORCE: f64 = 0.3;
const MAX_SPEED: f64 = 2.0;

// ボイドモデルシミュレータ
pub struct Boid {
    agents: Vec<BoidAgent>,
    width: u32,  // 画面の幅
    height: u32, // 画面の高さ
}

impl Boid {
    pub fn new(width: u32, height: u32) -> Self {
        Boid {
            agents: Vec::new(),
            width: width,
            height: height,
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
                    // x: rng.gen_range(0.0..800.0),
                    // y: rng.gen_range(0.0..600.0),
                    x: rng.gen_range(0.0..self.width as f64),
                    y: rng.gen_range(0.0..self.height as f64),
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
            width: self.width,
            height: self.height,
        }))
    }

    fn update(&mut self) {
        // 分離
        self.separate();
        // 整列
        self.alignment();
        // 結合
        self.cohesion();
        for agent in &mut self.agents {
            // 速度を更新
            agent.velocity.x += agent.acceleration.x;
            agent.velocity.y += agent.acceleration.y;
            let speed = (agent.velocity.x.powi(2) + agent.velocity.y.powi(2)).sqrt();
            if speed > MAX_SPEED {
                // let factor = MAX_SPEED / speed;
                // agent.velocity.x *= factor;
                // agent.velocity.y *= factor;
                agent.velocity.x = agent.velocity.x / speed * MAX_SPEED;
                agent.velocity.y = agent.velocity.y / speed * MAX_SPEED;
            }

            // 位置を更新
            agent.position.x += agent.velocity.x;
            agent.position.y += agent.velocity.y;

            // 加速度をリセット
            agent.acceleration.x = 0.0;
            agent.acceleration.y = 0.0;

            // // 画面端で跳ね返る
            // if agent.position.x < 0.0 || agent.position.x > self.width as f64 {
            //     agent.velocity.x *= -1.0;
            // }
            // if agent.position.y < 0.0 || agent.position.y > self.height as f64 {
            //     agent.velocity.y *= -1.0;
            // }

            // 画面端でループ
            if agent.position.x < 0.0 {
                agent.position.x = self.width as f64;
            }
            if agent.position.x > self.width as f64 {
                agent.position.x = 0.0;
            }
            if agent.position.y < 0.0 {
                agent.position.y = self.height as f64;
            }
            if agent.position.y > self.height as f64 {
                agent.position.y = 0.0;
            }
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

impl Boid {
    fn separate(&mut self) {
        let separate_force: f64 = 3.5;
        let separate_distance: f64 = 25.0;
        for i in 0..self.agents.len() {
            let mut count = 0;
            let mut separate = Point{x: 0.0, y: 0.0};
            for j in 0..self.agents.len() {
                if i==j{
                    continue;
                }
                let distance = (self.agents[i].position.x - self.agents[j].position.x).powi(2) + (self.agents[i].position.y - self.agents[j].position.y).powi(2);
                if distance > 0. && distance < separate_distance.powi(2){
                    separate.x += self.agents[i].position.x - self.agents[j].position.x;
                    separate.y += self.agents[i].position.y - self.agents[j].position.y;
                    count += 1;
                }
            }
            if count > 0 {
                separate.x /= count as f64;
                separate.y /= count as f64;
                let length = (separate.x.powi(2) + separate.y.powi(2)).sqrt();
                separate.x /= length;
                separate.y /= length;
                separate.x *= separate_force;
                separate.y *= separate_force;
            }
            self.agents[i].acceleration.x += separate.x;
            self.agents[i].acceleration.y += separate.y;
        }
    }

    fn alignment(&mut self) {
        let alignment_force: f64 = 1.5;
        let alignment_distance: f64 = 50.0;
        for i in 0..self.agents.len() {
            let mut count = 0;
            let mut alignment = Point{x: 0.0, y: 0.0};
            for j in 0..self.agents.len() {
                if i==j{
                    continue;
                }
                let distance = (self.agents[i].position.x - self.agents[j].position.x).powi(2) + (self.agents[i].position.y - self.agents[j].position.y).powi(2);
                if distance > 1.0 && distance < alignment_distance.powi(2){
                    alignment.x += self.agents[j].velocity.x;
                    alignment.y += self.agents[j].velocity.y;
                    count += 1;
                }
            }
            if count > 0 {
                alignment.x /= count as f64;
                alignment.y /= count as f64;
                let length = (alignment.x.powi(2) + alignment.y.powi(2)).sqrt();
                alignment.x /= length;
                alignment.y /= length;
                alignment.x *= alignment_force;
                alignment.y *= alignment_force;
            }
            self.agents[i].acceleration.x += alignment.x;
            self.agents[i].acceleration.y += alignment.y;
        }
    }

    fn cohesion(&mut self) {
        let cohesion_force: f64 = 1.5;
        let cohesion_distance: f64 = 50.0;
        for i in 0..self.agents.len() {
            let mut count = 0;
            let mut cohesion = Point{x: 0.0, y: 0.0};
            for j in 0..self.agents.len() {
                if i==j{
                    continue;
                }
                let distance = (self.agents[i].position.x - self.agents[j].position.x).powi(2) + (self.agents[i].position.y - self.agents[j].position.y).powi(2);
                if distance > 0.5 && distance < cohesion_distance.powi(2){
                    cohesion.x += self.agents[j].position.x;
                    cohesion.y += self.agents[j].position.y;
                    count += 1;
                }
            }
            if count > 0 {
                cohesion.x /= count as f64;
                cohesion.y /= count as f64; // cohesionの平均
                cohesion.x -= self.agents[i].position.x;
                cohesion.y -= self.agents[i].position.y; // agentからみたcohesionのベクトル
                let length = (cohesion.x.powi(2) + cohesion.y.powi(2)).sqrt();
                cohesion.x /= length;
                cohesion.y /= length; // cohesionの単位ベクトル
                cohesion.x *= cohesion_force;
                cohesion.y *= cohesion_force;
            }
            self.agents[i].acceleration.x += cohesion.x;
            self.agents[i].acceleration.y += cohesion.y;
        }
    }
}