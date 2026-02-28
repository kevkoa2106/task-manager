use task_manager::theme::Theme;
use task_manager::ui::*;

fn theme(_state: &State) -> Theme {
    Theme::Dark
}

fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .title("Task Manager")
        .subscription(subscription)
        .theme(theme)
        .run()
}
