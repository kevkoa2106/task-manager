use std::time::Duration;
use sysinfo::{Disks, System};

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Background, Color, Element, Length, Subscription};

use crate::charts::*;
use crate::theme::Theme;
use crate::utilities::*;
use plotters::prelude::*;
use plotters_iced2::ChartWidget;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectedTab {
    Cpu,
    Memory,
    Disk,
}

pub struct State {
    sys: System,
    total_mem: f32,
    cpu_usage: f32,
    cpu_frequency: f32,
    num_of_cpus: i32,
    cpu_history: Vec<f64>,
    memory_usage: f64,
    memory_history: Vec<f64>,
    uptime: String,
    disks: Disks,
    disk_usage: f64,
    disk_history: Vec<f64>,
    prev_disk_read: u64,
    prev_disk_written: u64,
    selected_tab: SelectedTab,
    processes_icon: iced::widget::image::Handle,
    performance_icon: iced::widget::image::Handle,
}

impl Default for State {
    fn default() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let total_mem = bytes_to_gb(sys.total_memory());
        let num_of_cpus = sys.cpus().len() as i32;

        Self {
            sys,
            total_mem,
            cpu_usage: 0.0,
            cpu_frequency: 0.0,
            cpu_history: Vec::new(),
            num_of_cpus,
            memory_usage: 0.0,
            memory_history: Vec::new(),
            uptime: String::new(),
            disks: Disks::new_with_refreshed_list(),
            disk_usage: 0.0,
            disk_history: Vec::new(),
            prev_disk_read: 0,
            prev_disk_written: 0,
            selected_tab: SelectedTab::Cpu,
            processes_icon: iced::widget::image::Handle::from_bytes(
                include_bytes!("../assets/tetris-svgrepo-com.png").as_slice(),
            ),
            performance_icon: iced::widget::image::Handle::from_bytes(
                include_bytes!("../assets/image-removebg-preview.png").as_slice(),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    SelectCpu,
    SelectMemory,
    SelectDisk,
    OpenProcesses,
    OpenPerformance,
}

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::Tick => {
            state.sys.refresh_cpu_usage();
            state.sys.refresh_memory();

            state.cpu_usage = state.sys.global_cpu_usage();
            state.cpu_history.push(state.cpu_usage as f64);

            state.memory_usage = memory_usage_percent(
                state.sys.used_memory() as f64,
                state.sys.total_memory() as f64,
            );
            state.memory_history.push(state.memory_usage);

            state.uptime = format_uptime(sysinfo::System::uptime());

            for cpu in state.sys.cpus() {
                state.cpu_frequency = mhz_to_ghz(cpu.frequency());
            }

            for disk in state.disks.list() {
                let read = disk.usage().read_bytes;
                let written = disk.usage().written_bytes;

                let read_delta = read.saturating_sub(state.prev_disk_read);
                let written_delta = written.saturating_sub(state.prev_disk_written);

                state.disk_usage = (read_delta + written_delta) as f64 / 1_000_000.0;
                state.disk_history.push(state.disk_usage);

                state.prev_disk_read = read;
                state.prev_disk_written = written;
            }
        }
        Message::SelectCpu => {
            state.selected_tab = SelectedTab::Cpu;
        }
        Message::SelectMemory => {
            state.selected_tab = SelectedTab::Memory;
        }
        Message::SelectDisk => {
            state.selected_tab = SelectedTab::Disk;
        }
        Message::OpenProcesses => {
            todo!()
        }
        Message::OpenPerformance => {
            todo!()
        }
    }
}

fn tail(data: &[f64], max: usize) -> &[f64] {
    if data.len() > max {
        &data[data.len() - max..]
    } else {
        data
    }
}

pub fn view(state: &State) -> Element<'_, Message, Theme> {
    let cpu_color = RGBColor(0, 255, 255);
    let mem_color = RGBColor(180, 0, 255);
    let disk_color = RGBColor(53, 189, 60);

    // Sidebar with icon buttons
    let processes_icon = iced::widget::image(state.processes_icon.clone())
        .width(30)
        .height(30);

    let performance_icon = iced::widget::image(state.performance_icon.clone())
        .width(30)
        .height(30);

    let processes_btn = button(processes_icon)
        .width(50)
        .height(50)
        .on_press(Message::OpenProcesses)
        .style(|_, status| button::Style {
            background: Some(Background::Color(match status {
                button::Status::Hovered => Color::from_rgb(0.59215686, 0.60784314, 0.64313725),
                _ => Color::from_rgb(0.53333333, 0.54117647, 0.56470588),
            })),
            text_color: Color::WHITE,
            ..Default::default()
        });

    let performance_btn = button(performance_icon)
        .width(50)
        .height(50)
        .on_press(Message::OpenPerformance)
        .style(|_: &Theme, status| button::Style {
            background: Some(Background::Color(match status {
                button::Status::Hovered => Color::from_rgb(0.59215686, 0.60784314, 0.64313725),
                _ => Color::from_rgb(0.53333333, 0.54117647, 0.56470588),
            })),
            text_color: Color::WHITE,
            ..Default::default()
        });

    let sidebar = container(
        column![processes_btn, performance_btn,]
            .spacing(10)
            .padding(5),
    )
    .style(|_: &_| container::Style {
        background: Some(Background::Color(Color::from_rgb(
            0.09803922, 0.09803922, 0.14901961,
        ))),
        ..Default::default()
    });

    // Tab selector panel
    let cpu_thumb: Element<'_, Message, Theme> = ChartWidget::new(ThumbChart {
        data: tail(&state.cpu_history, 60),
        color: cpu_color,
    })
    .width(Length::Fixed(120.0))
    .height(Length::Fixed(50.0))
    .into();

    let mem_thumb: Element<'_, Message, Theme> = ChartWidget::new(ThumbChart {
        data: tail(&state.memory_history, 60),
        color: mem_color,
    })
    .width(Length::Fixed(120.0))
    .height(Length::Fixed(50.0))
    .into();

    let cpu_btn = button(
        row![cpu_thumb, text("CPU").size(16)]
            .spacing(10)
            .align_y(Alignment::Center),
    )
    .on_press(Message::SelectCpu)
    .width(Length::Fill)
    .style(|_, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => Color::from_rgb(0.59215686, 0.60784314, 0.64313725),
            _ => Color::from_rgb(0.53333333, 0.54117647, 0.56470588),
        })),
        text_color: Color::WHITE,
        ..Default::default()
    });

    let mem_btn = button(
        row![mem_thumb, text("Memory").size(16)]
            .spacing(10)
            .align_y(Alignment::Center),
    )
    .on_press(Message::SelectMemory)
    .width(Length::Fill)
    .style(|_, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => Color::from_rgb(0.59215686, 0.60784314, 0.64313725),
            _ => Color::from_rgb(0.53333333, 0.54117647, 0.56470588),
        })),
        text_color: Color::WHITE,
        ..Default::default()
    });

    let mut tab_children: Vec<Element<'_, Message, Theme>> = vec![cpu_btn.into(), mem_btn.into()];

    for disk in state.disks.list() {
        let disk_name = disk.name().display().to_string();

        let disk_thumb: Element<'_, Message, Theme> = ChartWidget::new(ThumbChart {
            data: tail(&state.disk_history, 60),
            color: disk_color,
        })
        .width(Length::Fixed(120.0))
        .height(Length::Fixed(50.0))
        .into();

        let disk_btn = button(
            row![disk_thumb, text(disk_name).size(16)]
                .spacing(10)
                .align_y(Alignment::Center),
        )
        .on_press(Message::SelectDisk)
        .width(Length::Fill)
        .style(|_, status| button::Style {
            background: Some(Background::Color(match status {
                button::Status::Hovered => Color::from_rgb(0.59215686, 0.60784314, 0.64313725),
                _ => Color::from_rgb(0.53333333, 0.54117647, 0.56470588),
            })),
            text_color: Color::WHITE,
            ..Default::default()
        });

        tab_children.push(disk_btn.into());
    }

    let tab_panel = container(
        iced::widget::Column::with_children(tab_children)
            .spacing(10)
            .padding(10),
    )
    .width(220);

    // Main content area
    let main_content: Element<'_, Message, Theme> = match state.selected_tab {
        SelectedTab::Cpu => {
            let chart: Element<'_, Message, Theme> = ChartWidget::new(DetailChart {
                data: tail(&state.cpu_history, 120),
                color: cpu_color,
                y_label: "CPU %",
                max_size: 100.0,
            })
            .width(Length::Fill)
            .height(Length::Fixed(300.0))
            .into();

            column![
                chart,
                text(format!("CPU usage: {:.1}%", state.cpu_usage)).size(18),
                text(format!("CPU frequency: {:.2} GHz", state.cpu_frequency)).size(18),
                text(format!("Number of CPUs: {}", state.num_of_cpus)).size(18),
                text(format!("Up time: {}", state.uptime)).size(18),
            ]
            .spacing(10)
            .padding(20)
            .into()
        }
        SelectedTab::Memory => {
            let chart: Element<'_, Message, Theme> = ChartWidget::new(DetailChart {
                data: tail(&state.memory_history, 120),
                color: mem_color,
                y_label: "Memory %",
                max_size: 100.0,
            })
            .width(Length::Fill)
            .height(Length::Fixed(300.0))
            .into();

            column![
                chart,
                text(format!("Memory usage: {:.1}%", state.memory_usage)).size(18),
                text(format!(
                    "Used memory: {:.1} GB",
                    bytes_to_gb(state.sys.used_memory())
                ))
                .size(18),
                text(format!("Total memory: {:.1} GB", state.total_mem)).size(18),
            ]
            .spacing(10)
            .padding(20)
            .into()
        }
        SelectedTab::Disk => {
            let chart: Element<'_, Message, Theme> = ChartWidget::new(DetailChart {
                data: tail(&state.disk_history, 120),
                color: disk_color,
                y_label: "MB/s",
                max_size: 100.0,
            })
            .width(Length::Fill)
            .height(Length::Fixed(300.0))
            .into();

            column![
                chart,
                text(format!("Disk usage: {:.2} MB/s", state.disk_usage)).size(18),
                // text(format!("Total memory: {:.1} GB", state.total_mem)).size(18),
            ]
            .spacing(10)
            .padding(20)
            .into()
        }
    };

    // Combine into a row layout
    row![
        sidebar,
        tab_panel,
        container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
    ]
    .height(Length::Fill)
    .into()
}

pub fn subscription(_state: &State) -> Subscription<Message> {
    iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
}
