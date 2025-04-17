// アナログ時計
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::{
    f32::consts::PI,
};
use web_sys::js_sys;

use crate::engine::{
    Game, Renderer2d, Point,
};

pub struct AnalogClock {
    width: u32,  // 画面の幅
    height: u32, // 画面の高さ
    pub hour: f32,
    pub minute: f32,
    pub second: f32,
}

impl AnalogClock {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
            hour: 0.0,
            minute: 0.0,
            second: 0.0,
        }
    }
}

#[async_trait(?Send)]
impl Game for AnalogClock {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        Ok(Box::new(AnalogClock::new(self.width, self.height)))
    }

    fn update(&mut self) {
        let now = js_sys::Date::new_0();
        self.second = now.get_seconds() as f32;
        self.minute = now.get_minutes() as f32 + self.second / 60.0;
        self.hour = now.get_hours() as f32 + self.minute / 60.0;

        // log!("Hour: {}, Minute: {}, Second: {}", self.hour, self.minute, self.second);
    }

    fn draw(&self, renderer: &Renderer2d) {
        renderer.clear();
        let center = Point::new_fron_uint(self.width / 2, self.height / 2);
        let radius = (self.width.min(self.height) / 2 - 20 ) as f64;

        // Draw clock face
        renderer.circle(center, radius, "black");

        // Draw hour hand
        let hour_angle = (self.hour / 12.0) * PI * 2.0 - PI / 2.0;
        let hour_hand_length = radius * 0.5;
        let hour_hand_end_x = hour_hand_length * hour_angle.cos() as f64;
        let hour_hand_end_y = hour_hand_length * hour_angle.sin() as f64;
        let hour_hand_end = Point::new(center.x + hour_hand_end_x, center.y + hour_hand_end_y);
        renderer.line(center, hour_hand_end, 6.0, "blue");

        // Draw minute hand
        let minute_angle = (self.minute / 60.0) * PI * 2.0 - PI / 2.0;
        let minute_hand_length = radius * 0.7;
        let minute_hand_end_x = minute_hand_length * minute_angle.cos() as f64;
        let minute_hand_end_y = minute_hand_length * minute_angle.sin() as f64;
        let minute_hand_end = Point::new(center.x + minute_hand_end_x, center.y + minute_hand_end_y);
        renderer.line(center, minute_hand_end, 4.0, "green");

        // Draw second hand
        let second_angle = (self.second / 60.0) * PI * 2.0 - PI / 2.0;
        let second_hand_length = radius * 0.9;
        let second_hand_end_x = second_hand_length * second_angle.cos() as f64;
        let second_hand_end_y = second_hand_length * second_angle.sin() as f64;
        let second_hand_end = Point::new(center.x + second_hand_end_x, center.y + second_hand_end_y);
        renderer.line(center, second_hand_end, 2.0, "red");

        // todo 文字盤の数字を描画する
    }
}