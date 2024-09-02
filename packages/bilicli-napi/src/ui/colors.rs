use ratatui::style::Color;
use std::sync::OnceLock;

pub const USER_COLORS: [&str; 4] = ["#967E76", "#FF7C28", "#E17AFF", "#00D1F1"];

pub static GRAY_COLOR: OnceLock<Color> = OnceLock::new();
