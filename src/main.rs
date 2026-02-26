use task_manager::taskmanager::theme::Theme;
use task_manager::taskmanager::ui::*;

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
