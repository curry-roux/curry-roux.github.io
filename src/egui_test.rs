use eframe::egui;

pub struct MyApp;

impl Default for MyApp {
    fn default() -> Self {
        MyApp
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, egui + WASM!");
        });
    }
}