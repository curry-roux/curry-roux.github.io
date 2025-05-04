// ボイドモデルシミュレーター
use anyhow::{anyhow, Result};
use std::{f64::consts::PI};
use rand::Rng;
use async_trait::async_trait;

use crate::engine::{
    self, Game, Point, Renderer2d
};

// ボイドモデルシミュレータ
pub struct Boid {
    agents: Vec<BoidAgent>,
    width: u32,  // 画面の幅
    height: u32, // 画面の高さ
    parameters: BoidParameters,
}

impl Boid {
    pub fn new(width: u32, height: u32) -> Self {
        let mut rng = rand::thread_rng();
        let parameter = BoidParameters {
            alignment_distance: rng.gen_range(10.0..100.0),
            cohesion_distance: rng.gen_range(10.0..100.0),
            separate_distance: rng.gen_range(10.0..100.0),
            alignment_force : rng.gen_range(0.05..0.7),
            cohesion_force : rng.gen_range(0.05..0.7),
            separate_force: rng.gen_range(0.05..0.7),
            ..BoidParameters::default()
        };
        Boid {
            agents: Vec::new(),
            width: width,
            height: height,
            parameters: parameter,
        }
    }

    pub fn update_alignment_distance(&mut self, distance: f64) {
        self.parameters.alignment_distance = distance;
    }
    pub fn update_cohesion_distance(&mut self, distance: f64) {
        self.parameters.cohesion_distance = distance;
    }
    pub fn update_separate_distance(&mut self, distance: f64) {
        self.parameters.separate_distance = distance;
    }
    pub fn update_alignment_force(&mut self, force: f64) {
        self.parameters.alignment_force = force;
    }
    pub fn update_cohesion_force(&mut self, force: f64) {
        self.parameters.cohesion_force = force;
    }
    pub fn update_separate_force(&mut self, force: f64) {
        self.parameters.separate_force = force;
    }
    pub fn update_boid_count(&mut self, size: usize) {
        self.parameters.boid_count = size;
    }
}

// コンフィグ
#[derive(Debug, Clone)]
pub struct BoidParameters {
    pub boid_size: f64,
    pub boid_count: usize,
    pub max_speed: f64,
    pub separate_force: f64,
    pub separate_distance: f64,
    pub alignment_force: f64,
    pub alignment_distance: f64,
    pub cohesion_force: f64,
    pub cohesion_distance: f64,
}

impl Default for BoidParameters {
    fn default() -> Self {
        BoidParameters {
            boid_size: 15.0,
            boid_count: 100,
            max_speed: 3.0,
            separate_force: 0.35,
            separate_distance: 25.0,
            alignment_force: 0.15,
            alignment_distance: 50.0,
            cohesion_force: 0.15,
            cohesion_distance: 50.0,
        }
    }
}

struct BoidAgent {
    position: Point,
    velocity: Point,
    acceleration: Point,
    size: f64,
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

        let front_x = self.position.x + self.size*angle.cos();
        let front_y = self.position.y + self.size*angle.sin();

        let left_angle = angle + (2.0*PI / 3.0);
        let left_x = self.position.x + (self.size*sendo) * left_angle.cos();
        let left_y = self.position.y + (self.size*sendo) * left_angle.sin();

        let right_angle = angle - (2.0*PI / 3.0);
        let right_x = self.position.x + (self.size*sendo) * right_angle.cos();
        let right_y = self.position.y + (self.size*sendo) * right_angle.sin();

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
        for _ in 0..self.parameters.boid_count {
            let agent = BoidAgent {
                position: Point {
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
                size: self.parameters.boid_size,
            };
            agents.push(agent);
        }

        Ok(Box::new(Boid {
            agents: agents,
            width: self.width,
            height: self.height,
            parameters: self.parameters.clone(),
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
            if speed > self.parameters.max_speed {
                agent.velocity.x = agent.velocity.x / speed * self.parameters.max_speed;
                agent.velocity.y = agent.velocity.y / speed * self.parameters.max_speed;
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
        //log!("Boid update");
    }

    fn draw(&self, renderer: &Renderer2d) {
        renderer.clear();
        for agent in &self.agents {
            let triangle = agent.triangle();
            renderer.triangle(triangle[0].x, triangle[0].y, triangle[1].x, triangle[1].y, triangle[2].x, triangle[2].y);
        }
        //log!("Boid draw");
    }

    fn update_parameter_from_html(&mut self){
        let param_list: Vec<&str> = vec![
            "boidcount",
            "separateforce",
            "alignforce",
            "cohesionforce",
            "separatedistance",
            "aligndistance",
            "cohesiondistance",
        ];
        for name in param_list {
            let value = match engine::get_parameter_ui_value(name) {
                Ok(val) => val,
                Err(err) => {
                    log!("Failed to get parameter value for {}: {:?}", name, err);
                    continue;
                }
            };
            match name {
                "boidcount" => self.update_boid_count(value as usize),
                "separateforce" => self.update_separate_force(value),
                "alignforce" => self.update_alignment_force(value),
                "cohesionforce" => self.update_cohesion_force(value),
                "separatedistance" => self.update_separate_distance(value),
                "aligndistance" => self.update_alignment_distance(value),
                "cohesiondistance" => self.update_cohesion_distance(value),
                _ => log!("Unknown parameter: {}", name),
            }
        }
    }
}

impl Boid {
    fn separate(&mut self) {
        for i in 0..self.agents.len() {
            let mut count = 0;
            let mut separate = Point{x: 0.0, y: 0.0};
            for j in 0..self.agents.len() {
                if i==j{
                    continue;
                }
                let distance = (self.agents[i].position.x - self.agents[j].position.x).powi(2) + (self.agents[i].position.y - self.agents[j].position.y).powi(2);
                if distance > 0. && distance < self.parameters.separate_distance.powi(2){
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
                separate.x *= self.parameters.separate_force;
                separate.y *= self.parameters.separate_force;
            }
            self.agents[i].acceleration.x += separate.x;
            self.agents[i].acceleration.y += separate.y;
        }
    }

    fn alignment(&mut self) {
        for i in 0..self.agents.len() {
            let mut count = 0;
            let mut alignment = Point{x: 0.0, y: 0.0};
            for j in 0..self.agents.len() {
                if i==j{
                    continue;
                }
                let distance = (self.agents[i].position.x - self.agents[j].position.x).powi(2) + (self.agents[i].position.y - self.agents[j].position.y).powi(2);
                if distance > 1.0 && distance < self.parameters.alignment_distance.powi(2){
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
                alignment.x *= self.parameters.alignment_force;
                alignment.y *= self.parameters.alignment_force;
            }
            self.agents[i].acceleration.x += alignment.x;
            self.agents[i].acceleration.y += alignment.y;
        }
    }

    fn cohesion(&mut self) {
        for i in 0..self.agents.len() {
            let mut count = 0;
            let mut cohesion = Point{x: 0.0, y: 0.0};
            for j in 0..self.agents.len() {
                if i==j{
                    continue;
                }
                let distance = (self.agents[i].position.x - self.agents[j].position.x).powi(2) + (self.agents[i].position.y - self.agents[j].position.y).powi(2);
                if distance > 0.5 && distance < self.parameters.cohesion_distance.powi(2){
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
                cohesion.x *= self.parameters.cohesion_force;
                cohesion.y *= self.parameters.cohesion_force;
            }
            self.agents[i].acceleration.x += cohesion.x;
            self.agents[i].acceleration.y += cohesion.y;
        }
    }
}