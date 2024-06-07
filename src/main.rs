use iced::{window, Application, Settings, Size};

mod models;
mod ui;
mod utils;
mod constants;

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
        ..Default::default()
    })
}
