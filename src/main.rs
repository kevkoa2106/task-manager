use task_manager::theme::Theme;
use task_manager::ui::*;

fn theme(state: &State) -> Theme {
    state.theme_selected.unwrap()
}

fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .title("Task Manager")
        .subscription(subscription)
        .theme(theme)
        .run()
}
