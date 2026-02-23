pub mod taskmanager;

use taskmanager::ui::*;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([860.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Task Manager",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<TaskManager>::default())
        }),
    )
}
