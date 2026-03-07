use iced::Color;

pub const LIGHT_THEME_HOVER: Color = Color::from_rgb(0.68627451, 0.70196078, 0.74509804);
pub const DARK_THEME_HOVER: Color = Color::from_rgb(0.59215686, 0.60784314, 0.64313725);
pub const LIGHT_THEME_IDLE: Color = Color::from_rgb(0.82745098, 0.84705882, 0.89019608);
pub const DARK_THEME_IDLE: Color = Color::from_rgb(0.53333333, 0.54117647, 0.56470588);

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
