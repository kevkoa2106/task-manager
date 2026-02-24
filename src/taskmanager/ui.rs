use eframe::egui::{self, Color32, FontId, Frame, Vec2, Visuals, include_image};
use egui_plot::{Line, Plot};

use std::time::{Duration, Instant};
use sysinfo::System;

pub fn bytes_to_gb(bytes: u64) -> f32 {
    bytes as f32 / 1_000_000_000.0
}

pub fn memory_usage_percent(used: f64, total: f64) -> f64 {
    if total > 0.0 {
        (used / total) * 100.0
    } else {
        0.0
    }
}

pub fn format_uptime(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    format!("{hours}:{minutes}    Total in secs: {seconds}")
}

pub fn mhz_to_ghz(mhz: u64) -> f32 {
    mhz as f32 / 1000.0
}

#[derive(PartialEq)]
enum SelectedTab {
    Cpu,
    Memory,
}

pub struct TaskManager {
    sys: System,
    last_cpu_refresh: Instant,
    total_mem: f32,
    cpu_usage: f32,
    cpu_frequency: f32,
    cpu_history: Vec<f64>,
    memory_usage: f64,
    memory_history: Vec<f64>,
    uptime: String,
    selected_tab: SelectedTab,
}

impl Default for TaskManager {
    fn default() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let total_mem = bytes_to_gb(sys.total_memory());

        let uptime = String::new();

        Self {
            sys,
            last_cpu_refresh: Instant::now(),
            total_mem,
            cpu_usage: 0.0,
            cpu_frequency: 0.0,
            cpu_history: Vec::new(),
            memory_usage: 0.0,
            memory_history: Vec::new(),
            uptime,
            selected_tab: SelectedTab::Cpu,
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
            self.sys.refresh_memory();
            self.cpu_usage = self.sys.global_cpu_usage();
            self.cpu_history.push(self.cpu_usage as f64);

            self.memory_usage = memory_usage_percent(
                self.sys.used_memory() as f64,
                self.sys.total_memory() as f64,
            );
            self.memory_history.push(self.memory_usage);

            self.last_cpu_refresh = Instant::now();
        }

        self.uptime = format_uptime(sysinfo::System::uptime());

        for cpu in self.sys.cpus() {
            self.cpu_frequency = mhz_to_ghz(cpu.frequency());
        }

        ctx.request_repaint_after(Duration::from_secs(1));
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("sidepanel")
            .default_size(50.)
            .resizable(false)
            .frame(Frame {
                fill: Color32::LIGHT_GRAY,
                ..Default::default()
            })
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
        egui::Panel::left("list of buttons")
            .resizable(false)
            .show_inside(ui, |ui| {
                // CPU button
                let cpu_group = ui.group(|ui| {
                    ui.horizontal(|ui| {
                        let cpu_points: Vec<[f64; 2]> = self
                            .cpu_history
                            .iter()
                            .enumerate()
                            .map(|(i, &usage)| [i as f64, usage])
                            .collect();
                        let cpu_line =
                            Line::new("CPU %", cpu_points).color(Color32::from_rgb(0, 255, 255));

                        Plot::new("cpu_thumb")
                            .height(50.0)
                            .width(120.0)
                            .show_axes([false, false])
                            .show_grid([false, false])
                            .include_y(0.0)
                            .include_y(100.0)
                            .allow_drag(false)
                            .allow_zoom(false)
                            .allow_scroll(false)
                            .allow_boxed_zoom(false)
                            .show(ui, |plot_ui| {
                                plot_ui.line(cpu_line);
                            });

                        ui.label("CPU");
                    });
                });

                let cpu_response = cpu_group.response.interact(egui::Sense::click());
                if cpu_response.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if cpu_response.clicked() {
                    self.selected_tab = SelectedTab::Cpu;
                }

                // Memory button
                let mem_group = ui.group(|ui| {
                    ui.horizontal(|ui| {
                        let mem_points: Vec<[f64; 2]> = self
                            .memory_history
                            .iter()
                            .enumerate()
                            .map(|(i, &usage)| [i as f64, usage])
                            .collect();
                        let mem_line =
                            Line::new("Mem %", mem_points).color(Color32::from_rgb(180, 0, 255));

                        Plot::new("mem_thumb")
                            .height(50.0)
                            .width(120.0)
                            .show_axes([false, false])
                            .show_grid([false, false])
                            .include_y(0.0)
                            .include_y(100.0)
                            .allow_drag(false)
                            .allow_zoom(false)
                            .allow_scroll(false)
                            .allow_boxed_zoom(false)
                            .allow_axis_zoom_drag(false)
                            .allow_double_click_reset(false)
                            .show(ui, |plot_ui| {
                                plot_ui.line(mem_line);
                            });

                        ui.label("Memory");
                    });
                });

                let mem_response = mem_group.response.interact(egui::Sense::click());
                if mem_response.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if mem_response.clicked() {
                    self.selected_tab = SelectedTab::Memory;
                }
            });
        egui::CentralPanel::default().show_inside(ui, |ui| match self.selected_tab {
            SelectedTab::Cpu => {
                let cpu_points: Vec<[f64; 2]> = self
                    .cpu_history
                    .iter()
                    .enumerate()
                    .map(|(i, &usage)| [i as f64, usage])
                    .collect();
                let cpu_line = Line::new("CPU %", cpu_points).color(Color32::from_rgb(0, 255, 255));

                Plot::new("cpu_usage_plot")
                    .view_aspect(3.0)
                    .y_axis_label("CPU %")
                    .x_axis_label("Time (s)")
                    .include_y(0.0)
                    .include_y(100.0)
                    .allow_drag(false)
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .allow_boxed_zoom(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(cpu_line);
                    });

                ui.add_space(10.0);
                ui.label(format!("CPU usage: {}%", self.cpu_usage));
                ui.label(format!("CPU frequency: {} GHz", self.cpu_frequency));
                ui.label(format!("Up time: {}", self.uptime));
            }
            SelectedTab::Memory => {
                let mem_points: Vec<[f64; 2]> = self
                    .memory_history
                    .iter()
                    .enumerate()
                    .map(|(i, &usage)| [i as f64, usage])
                    .collect();
                let mem_line = Line::new("Mem %", mem_points).color(Color32::from_rgb(180, 0, 255));

                Plot::new("mem_usage_plot")
                    .view_aspect(3.0)
                    .y_axis_label("Memory %")
                    .x_axis_label("Time (s)")
                    .include_y(0.0)
                    .include_y(100.0)
                    .allow_drag(false)
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .allow_boxed_zoom(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(mem_line);
                    });

                ui.add_space(10.0);
                ui.label(format!("Memory usage: {:.1}%", self.memory_usage));
                ui.label(format!("Total memory: {} GB", self.total_mem));
            }
        });
    }
}
