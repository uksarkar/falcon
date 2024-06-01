use iced::{
    widget::{button, container, row, text, text_input, Container, Space},
    Length, Padding, Renderer, Theme,
};

use crate::ui::app_theme::{AppBtn, AppContainer};

use super::HomeEventMessage;

pub fn url_input_bar(url: &str) -> Container<'static, HomeEventMessage, Theme, Renderer> {
    container(
        row![
            text("Get"),
            text_input("https://utpal.io", url).on_input(HomeEventMessage::UrlInput),
            Space::with_width(Length::Fill),
            button("Send")
                .style(AppBtn::Primary)
                .padding(Padding::from([5, 15]))
                .on_press(HomeEventMessage::SendRequest),
        ]
        .align_items(iced::Alignment::Center),
    )
    .align_y(iced::alignment::Vertical::Center)
    .padding(Padding::from([0, 10]))
    .style(AppContainer::Rounded)
    .height(40)
    .width(Length::Fill)
}
