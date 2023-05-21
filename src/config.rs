use crossterm::style;
use termion::color;
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Rgb(u8, u8, u8),
}

impl From<Color> for color::Rgb {
    fn from(value: Color) -> Self {
        match value {
            Color::Rgb(r, g, b) => color::Rgb(r, g, b),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub fg_color_default: String,
    pub bg_color_default: String,
    pub fg_color_match: String,
    pub fg_color_mismatch: String,
    pub fg_color_status_bar: String,
    pub bg_color_status_bar: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            fg_color_default: style::SetForegroundColor(style::Color::Reset).to_string(),
            bg_color_default: style::SetBackgroundColor(style::Color::Reset).to_string(),
            fg_color_match: style::SetForegroundColor((38, 161, 104).into()).to_string(),
            fg_color_mismatch: style::SetForegroundColor((216, 101, 99).into()).to_string(),
            fg_color_status_bar: style::SetForegroundColor((239, 239, 239).into()).to_string(),
            bg_color_status_bar: style::SetBackgroundColor((63, 63, 63).into()).to_string(),
        }
    }
}
