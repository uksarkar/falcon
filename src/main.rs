use iced::{window, Application, Settings, Size};

mod models;
mod ui;
mod utils;
mod constants;

use ui::app_state::AppState;
use utils::app::app_config;

pub fn main() -> iced::Result {
    println!("{:#?}", app_config());

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
