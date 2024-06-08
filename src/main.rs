use std::borrow::Cow;

use constants::ROBOTO_FONT;
use iced::{window, Application, Font, Settings, Size};

mod constants;
mod models;
mod ui;
mod utils;

use ui::app_state::AppState;

#[tokio::main]
pub async fn main() -> iced::Result {
    println!("{:<10} An HTTP request client.", "Falcon:");

    AppState::run(Settings {
        window: window::Settings {
            min_size: Some(Size::new(500.0, 500.0)),
            size: Size::INFINITY,
            position: window::Position::Centered,
            ..Default::default()
        },
        fonts: vec![Cow::Borrowed(ROBOTO_FONT)],
        default_font: Font::with_name("Roboto"),
        ..Default::default()
    })
}
