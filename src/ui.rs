use std::time::Duration;
use sysinfo::{Disks, Pid, ProcessesToUpdate, System};

use iced::widget::{Space, button, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Background, Color, Element, Length, Subscription, Task};

use crate::charts::*;
use crate::process_table::*;
use crate::theme::Theme;
use crate::utilities::*;
use plotters::prelude::*;
use plotters_iced2::ChartWidget;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectedView {
    Processes,
    Performance,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectedTab {
    Cpu,
    Memory,
    Disk,
}

pub struct State {
    pub theme_selected: Option<Theme>,
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
    disk_size: f32,
    disk_available_space: f32,
    disk_usage: f64,
    disk_history: Vec<f64>,
    prev_disk_read: u64,
    prev_disk_written: u64,
    selected_tab: SelectedTab,
    selected_view: SelectedView,
    process_table: ProcessTableState,
    processes_icon: iced::widget::image::Handle,
    performance_icon: iced::widget::image::Handle,
    settings_icon: iced::widget::image::Handle,
}

impl Default for State {
    fn default() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let total_mem = bytes_to_gb(sys.total_memory());
        let num_of_cpus = sys.cpus().len() as i32;

        Self {
            theme_selected: Some(crate::theme::Theme::Dark),
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
            disk_size: 0.0,
            disk_available_space: 0.0,
            disk_usage: 0.0,
            disk_history: Vec::new(),
            prev_disk_read: 0,
            prev_disk_written: 0,
            selected_tab: SelectedTab::Cpu,
            selected_view: SelectedView::Processes,
            process_table: ProcessTableState::default(),
            processes_icon: iced::widget::image::Handle::from_bytes(
                include_bytes!("../assets/tetris-svgrepo-com.png").as_slice(),
            ),
            performance_icon: iced::widget::image::Handle::from_bytes(
                include_bytes!("../assets/image-removebg-preview.png").as_slice(),
            ),
            settings_icon: iced::widget::image::Handle::from_bytes(
                include_bytes!("../assets/icons8-settings-52.png").as_slice(),
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
    OpenSettings,
    EndTask,
    ProcessTable(ProcessTableMessage),
    ThemeSelected(Theme),
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Tick => {
            state.sys.refresh_cpu_usage();
            state.sys.refresh_memory();
            state.sys.refresh_processes(ProcessesToUpdate::All, true);

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
                state.disk_size = bytes_to_gb(disk.total_space());
                state.disk_available_space = bytes_to_gb(disk.available_space());

                state.prev_disk_read = read;
                state.prev_disk_written = written;
            }

            state.process_table.rows = collect_processes(&state.sys);
            crate::process_table::apply_filter(&mut state.process_table);
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
            state.selected_view = SelectedView::Processes;
        }
        Message::OpenPerformance => {
            state.selected_view = SelectedView::Performance;
        }
        Message::OpenSettings => {
            state.selected_view = SelectedView::Settings;
        }
        Message::EndTask => {
            if let Some(pid) = state.process_table.selected_pid {
                if let Some(process) = state.sys.process(Pid::from_u32(pid)) {
                    process.kill();
                }
                state.process_table.selected_pid = None;
            }
        }
        Message::ProcessTable(msg) => {
            return crate::process_table::update(&mut state.process_table, msg)
                .map(Message::ProcessTable);
        }
        Message::ThemeSelected(theme) => state.theme_selected = Some(theme),
    }
    Task::none()
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

    let settings_icon = iced::widget::image(state.settings_icon.clone())
        .width(30)
        .height(30);

    let mut processes_btn = button(processes_icon)
        .width(50)
        .height(50)
        .on_press(Message::OpenProcesses);

    let mut performance_btn = button(performance_icon)
        .width(50)
        .height(50)
        .on_press(Message::OpenPerformance);

    let mut settings_btn = button(settings_icon)
        .width(50)
        .height(50)
        .on_press(Message::OpenSettings);

    match state.theme_selected {
        Some(Theme::Dark) => {
            processes_btn = processes_btn.style(|_, status| button::Style {
                background: Some(Background::Color(match status {
                    button::Status::Hovered => DARK_THEME_HOVER,
                    _ => DARK_THEME_IDLE,
                })),
                text_color: Color::WHITE,
                ..Default::default()
            });

            performance_btn = performance_btn.style(|_: &Theme, status| button::Style {
                background: Some(Background::Color(match status {
                    button::Status::Hovered => DARK_THEME_HOVER,
                    _ => DARK_THEME_IDLE,
                })),
                text_color: Color::WHITE,
                ..Default::default()
            });

            settings_btn = settings_btn.style(|_: &Theme, status| button::Style {
                background: Some(Background::Color(match status {
                    button::Status::Hovered => DARK_THEME_HOVER,
                    _ => DARK_THEME_IDLE,
                })),
                text_color: Color::WHITE,
                ..Default::default()
            });
        }
        Some(Theme::Light) => {
            processes_btn = processes_btn.style(|_, status| button::Style {
                background: Some(Background::Color(match status {
                    button::Status::Hovered => LIGHT_THEME_HOVER,
                    _ => LIGHT_THEME_IDLE,
                })),
                text_color: Color::WHITE,
                ..Default::default()
            });

            performance_btn = performance_btn.style(|_: &Theme, status| button::Style {
                background: Some(Background::Color(match status {
                    button::Status::Hovered => LIGHT_THEME_HOVER,
                    _ => LIGHT_THEME_IDLE,
                })),
                text_color: Color::WHITE,
                ..Default::default()
            });

            settings_btn = settings_btn.style(|_: &Theme, status| button::Style {
                background: Some(Background::Color(match status {
                    button::Status::Hovered => LIGHT_THEME_HOVER,
                    _ => LIGHT_THEME_IDLE,
                })),
                text_color: Color::WHITE,
                ..Default::default()
            });
        }
        None => {}
    }

    let sidebar = container(
        column![
            processes_btn,
            performance_btn,
            Space::new().height(Length::Fill),
            settings_btn
        ]
        .spacing(10)
        .padding(5),
    );

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

    let mut cpu_btn = button(
        row![cpu_thumb, text("CPU").size(16)]
            .spacing(10)
            .align_y(Alignment::Center),
    )
    .on_press(Message::SelectCpu)
    .width(Length::Fill);

    let mut mem_btn = button(
        row![mem_thumb, text("Memory").size(16)]
            .spacing(10)
            .align_y(Alignment::Center),
    )
    .on_press(Message::SelectMemory)
    .width(Length::Fill);

    match state.theme_selected {
        Some(Theme::Light) => {
            cpu_btn = cpu_btn.style(|_: &Theme, status| button::Style {
                background: Some(Background::Color(match status {
                    button::Status::Hovered => LIGHT_THEME_HOVER,
                    _ => LIGHT_THEME_IDLE,
                })),
                text_color: Color::BLACK,
                ..Default::default()
            });

            mem_btn = mem_btn.style(|_: &Theme, status| button::Style {
                background: Some(Background::Color(match status {
                    button::Status::Hovered => LIGHT_THEME_HOVER,
                    _ => LIGHT_THEME_IDLE,
                })),
                text_color: Color::BLACK,
                ..Default::default()
            });
        }
        Some(Theme::Dark) => {
            cpu_btn = cpu_btn.style(|_: &Theme, status| button::Style {
                background: Some(Background::Color(match status {
                    button::Status::Hovered => DARK_THEME_HOVER,
                    _ => DARK_THEME_IDLE,
                })),
                text_color: Color::WHITE,
                ..Default::default()
            });

            mem_btn = mem_btn.style(|_: &Theme, status| button::Style {
                background: Some(Background::Color(match status {
                    button::Status::Hovered => DARK_THEME_HOVER,
                    _ => DARK_THEME_IDLE,
                })),
                text_color: Color::WHITE,
                ..Default::default()
            });
        }
        None => {}
    }

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

        let mut disk_btn = button(
            row![
                disk_thumb,
                text(disk_name).wrapping(text::Wrapping::Glyph).size(16)
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        )
        .on_press(Message::SelectDisk)
        .width(Length::Fill);

        match state.theme_selected {
            Some(Theme::Light) => {
                disk_btn = disk_btn.style(|_: &Theme, status| button::Style {
                    background: Some(Background::Color(match status {
                        button::Status::Hovered => LIGHT_THEME_HOVER,
                        _ => LIGHT_THEME_IDLE,
                    })),
                    text_color: Color::BLACK,
                    ..Default::default()
                });
            }
            Some(Theme::Dark) => {
                disk_btn = disk_btn.style(|_: &Theme, status| button::Style {
                    background: Some(Background::Color(match status {
                        button::Status::Hovered => DARK_THEME_HOVER,
                        _ => DARK_THEME_IDLE,
                    })),
                    text_color: Color::WHITE,
                    ..Default::default()
                });
            }
            None => {}
        }

        tab_children.push(disk_btn.into());
    }

    let tab_panel = container(
        iced::widget::Column::with_children(tab_children)
            .spacing(10)
            .padding(10),
    )
    .width(220);

    // Main content area
    let main_content: Element<'_, Message, Theme> = match state.selected_view {
        SelectedView::Processes => {
            let mut end_btn = button(text("End Task").size(18.0).color(Color::BLACK))
                .on_press(Message::EndTask)
                .style(|_, status| button::Style {
                    background: Some(Background::Color(match status {
                        button::Status::Hovered => DARK_THEME_HOVER,
                        _ => DARK_THEME_IDLE,
                    })),
                    text_color: Color::WHITE,
                    ..Default::default()
                });

            match state.theme_selected {
                Some(Theme::Light) => {
                    end_btn = end_btn.style(|_: &Theme, status| button::Style {
                        background: Some(Background::Color(match status {
                            button::Status::Hovered => LIGHT_THEME_HOVER,
                            _ => LIGHT_THEME_IDLE,
                        })),
                        text_color: Color::WHITE,
                        ..Default::default()
                    });
                }
                Some(Theme::Dark) => {
                    end_btn = end_btn.style(|_: &Theme, status| button::Style {
                        background: Some(Background::Color(match status {
                            button::Status::Hovered => DARK_THEME_HOVER,
                            _ => DARK_THEME_IDLE,
                        })),
                        text_color: Color::WHITE,
                        ..Default::default()
                    });
                }
                None => {}
            }

            let search_input = text_input("Search processes...", &state.process_table.search_query)
                .on_input(|s| Message::ProcessTable(ProcessTableMessage::SearchChanged(s)))
                .width(200);

            let space_on_top = container(
                row![end_btn, Space::new().width(Length::Fill), search_input]
                    .align_y(Alignment::Center),
            );

            column![
                space_on_top,
                crate::process_table::view(&state.process_table).map(Message::ProcessTable),
            ]
            .spacing(10)
            .padding(20)
            .into()
        }
        SelectedView::Performance => match state.selected_tab {
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
                    text(format!("Total storage: {:.2} GB", state.disk_size)).size(18),
                    text(format!(
                        "Available storage: {:.2} GB",
                        state.disk_available_space
                    ))
                    .size(18),
                ]
                .spacing(10)
                .padding(20)
                .into()
            }
        },
        SelectedView::Settings => {
            let themes = pick_list(Theme::ALL, state.theme_selected, Message::ThemeSelected)
                .placeholder("Choose a theme...");

            let theme_choose = row![text("Theme: ").size(18), Space::new().width(10), themes]
                .align_y(iced::Center);

            column![theme_choose]
                .width(Length::Fill)
                .align_x(iced::Center)
                .into()
        }
    };

    let mut children: Vec<Element<'_, Message, Theme>> = vec![sidebar.into()];

    if state.selected_view == SelectedView::Performance {
        children.push(tab_panel.into());
    }

    children.push(
        container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
    );

    iced::widget::Row::with_children(children)
        .height(Length::Fill)
        .into()
}

pub fn subscription(_state: &State) -> Subscription<Message> {
    iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
}
