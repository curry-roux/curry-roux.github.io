// アナログ時計
use anyhow::{anyhow, Result};
use wasm_bindgen::JsValue;
use web_sys::js_sys;

use crate::engine::{
    Game, Renderer2d, Point,
};

pub struct AnalogClock {
    pub hour: f32,
    pub minute: f32,
    pub second: f32,
}

impl AnalogClock {
    pub fn new() -> Self {
        Self {
            hour: 0.0,
            minute: 0.0,
            second: 0.0,
        }
    }

    pub fn update(&mut self) {
        let now = js_sys::Date::new_0();
        self.hour = now.get_hours() as f32 + now.get_minutes() as f32 / 60.0;
        self.minute = now.get_minutes() as f32 + now.get_seconds() as f32 / 60.0;
        self.second = now.get_seconds() as f32;
    }

    fn draw(&self, renderer: &Renderer2d) {
        renderer.clear();
        renderer.draw_circle(Point::new(0.0, 0.0), 100.0, "white", 1.0);
        renderer.draw_line(Point::new(0.0, 0.0), Point::new(50.0 * self.hour.cos(), 50.0 * self.hour.sin()), "red", 2.0);
        renderer.draw_line(Point::new(0.0, 0.0), Point::new(70.0 * self.minute.cos(), 70.0 * self.minute.sin()), "green", 2.0);
        renderer.draw_line(Point::new(0.0, 0.0), Point::new(90.0 * self.second.cos(), 90.0 * self.second.sin()), "blue", 2.0);
    }
}