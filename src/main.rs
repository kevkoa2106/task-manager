use iced::{Color, Theme, theme::Palette};
use task_manager::taskmanager::ui::*;

fn theme(_state: &State) -> Theme {
    Theme::custom(
        "Win11",
        Palette {
            background: Color::from_rgb(0.09803922, 0.09803922, 0.14901961),
            ..Palette::DARK
        },
    )
}

fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .title("Task Manager")
        .subscription(subscription)
        .theme(theme)
        .run()
}
