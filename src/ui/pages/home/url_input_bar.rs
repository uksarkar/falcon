use iced::{
    widget::{button, container, row, text, text_input, Container, Space},
    Length, Padding, Renderer, Theme,
};

use crate::ui::app_theme::{AppBtn, AppContainer, AppInput};

use super::HomeEventMessage;

pub fn url_input_bar(
    url: &str,
    is_requesting: bool,
) -> Container<'static, HomeEventMessage, Theme, Renderer> {
    let mut button = button(if is_requesting { "Sending" } else { "Send" })
        .style(AppBtn::Primary)
        .padding(Padding::from([5, 15]));

    if !is_requesting {
        button = button.on_press(HomeEventMessage::SendRequest);
    }

    container(
        row![
            text("Get"),
            text_input("https://utpal.io", url)
                .style(AppInput)
                .width(Length::Fill)
                .on_input(HomeEventMessage::UrlInput),
            Space::with_width(10),
            button,
        ]
        .align_items(iced::Alignment::Center),
    )
    .align_y(iced::alignment::Vertical::Center)
    .padding(Padding::from([0, 10]))
    .style(AppContainer::Rounded)
    .height(40)
    .width(Length::Fill)
}
