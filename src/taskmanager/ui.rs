use iced::{
    Alignment::Center,
    Color, Element,
    Length::Fill,
    Subscription,
    widget::{button, button::Style, center, column, container, row, svg, text},
};
use std::time::Duration;
use sysinfo::System;

#[derive(Default)]
pub struct TaskManager {
    total_mem: u64,
    // used_mem: u64,
    cpu_usage: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    ShowProcesses,
    ShowPerformance,
    Tick,
}

impl TaskManager {
    pub fn boot() -> TaskManager {
        TaskManager::default()
    }

    pub fn update(state: &mut TaskManager, message: Message) {
        match message {
            Message::Tick => {
                let mut sys = System::new_all();
                sys.refresh_all();

                state.total_mem = sys.total_memory();

                std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
                sys.refresh_cpu_usage();
                state.cpu_usage = sys.global_cpu_usage();
            }
            Message::ShowProcesses => {}
            Message::ShowPerformance => {}
        }
    }

    pub fn view(state: &TaskManager) -> Element<'_, Message> {
        let sidebar = container(column![
            button(row![
                svg("assets/tetris-svgrepo-com.svg").width(50).height(50),
            ])
            .on_press(Message::ShowProcesses)
            .style(|_theme, status| {
                let bg = match status {
                    button::Status::Hovered => Color::from_rgb8(106, 109, 111),
                    _ => Color::from_rgb8(238, 244, 248),
                };
                Style {
                    background: Some(bg.into()),
                    ..Style::default()
                }
            }),
            button(row![
                svg("assets/image-removebg-preview.svg")
                    .width(50)
                    .height(50),
            ])
            .on_press(Message::ShowPerformance)
            .style(|_theme, status| {
                let bg = match status {
                    button::Status::Hovered => Color::from_rgb8(106, 109, 111),
                    _ => Color::from_rgb8(238, 244, 248),
                };
                Style {
                    background: Some(bg.into()),
                    ..Style::default()
                }
            }),
        ])
        .height(Fill)
        .style(|_theme| container::Style {
            background: Some(Color::from_rgb8(238, 244, 248).into()),
            ..container::Style::default()
        });

        let content = column![
            text(format!("Total memory: {}", state.total_mem)).size(28),
            text(format!("CPU usage: {}%", state.cpu_usage)).size(28)
        ]
        .width(Fill)
        .align_x(Center);

        center(row![sidebar, content]).into()
    }

    pub fn subscription(_state: &TaskManager) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
