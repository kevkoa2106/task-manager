pub mod taskmanager;

use taskmanager::ui::*;

fn theme(_state: &TaskManager) -> iced::Theme {
    iced::Theme::Light
}

fn main() -> iced::Result {
    iced::application(TaskManager::boot, TaskManager::update, TaskManager::view)
        .subscription(TaskManager::subscription)
        .theme(theme)
        .run()
}
