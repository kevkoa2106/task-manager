use eframe::egui::{self, FontId, Vec2, Visuals, include_image};

use std::time::{Duration, Instant};
use sysinfo::System;

pub struct TaskManager {
    sys: System,
    last_cpu_refresh: Instant,
    total_mem: f32,
    cpu_usage: f32,
    uptime: String,
}

impl Default for TaskManager {
    fn default() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let total_mem = sys.total_memory() as f32 / 1000000000.0;

        let uptime = String::new();

        Self {
            sys,
            last_cpu_refresh: Instant::now(),
            total_mem,
            cpu_usage: 0.0,
            uptime,
        }
    }
}

impl eframe::App for TaskManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.global_style_mut(|style| {
            style
                .text_styles
                .insert(egui::TextStyle::Body, FontId::proportional(28.0));
            style
                .text_styles
                .insert(egui::TextStyle::Button, FontId::proportional(28.0));
        });

        ctx.set_visuals(Visuals::light());

        if self.last_cpu_refresh.elapsed() >= Duration::from_secs(1) {
            self.sys.refresh_cpu_usage();
            self.cpu_usage = self.sys.global_cpu_usage();
            self.last_cpu_refresh = Instant::now();
        }

        let up = sysinfo::System::uptime();
        let mut uptime = up;
        let hours = uptime / 3600;
        uptime -= hours * 3600;
        let minutes = uptime / 60;
        self.uptime = format!("{hours}:{minutes}:{up}");

        ctx.request_repaint_after(Duration::from_secs(1));
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("sidepanel")
            .default_size(50.)
            .resizable(true)
            .show_inside(ui, |ui| {
                let proccesses_icon =
                    egui::Image::new(include_image!("../../assets/tetris-svgrepo-com.png"));
                let performance_icon =
                    egui::Image::new(include_image!("../../assets/image-removebg-preview.png"));

                if ui
                    .add(
                        egui::Button::image(proccesses_icon.fit_to_original_size(0.03))
                            .min_size(Vec2::new(50., 50.)),
                    )
                    .clicked()
                {
                    todo!()
                }
                if ui
                    .add(
                        egui::Button::image(performance_icon.fit_to_original_size(0.05))
                            .min_size(Vec2::new(57., 50.)),
                    )
                    .clicked()
                {
                    todo!()
                }
            });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label(format!("Total memory: {} GB", self.total_mem));
            ui.label(format!("CPU usage: {}%", self.cpu_usage));
            ui.label(format!("Up time: {}", self.uptime));
        });
    }
}
