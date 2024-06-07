use crate::constants::TRASH_SVG;
use crate::ui::app_theme::{AppBtn, AppContainer, AppInput};

use super::HomeEventMessage;

use iced::widget::svg::Handle;
use iced::{
    widget::{button, container, row, svg, text_input, Container, Space},
    Length, Padding, Renderer, Theme,
};

pub fn key_and_value_input_row<
    F1: 'static + Fn(String) -> HomeEventMessage,
    F2: 'static + Fn(String) -> HomeEventMessage,
>(
    key: &str,
    value: &str,
    enabled: bool,
    on_remove_item: HomeEventMessage,
    on_key_input: F1,
    on_value_input: F2,
) -> Container<'static, HomeEventMessage, Theme, Renderer> {
    let mut remove_btn = button(svg(Handle::from_memory(TRASH_SVG)).width(20).height(20))
        .style(AppBtn::Basic)
        .padding(5);

    if enabled {
        remove_btn = remove_btn.on_press(on_remove_item);
    }

    container(
        container(row![
            text_input("key", key)
                .on_input(on_key_input)
                .style(AppInput)
                .width(200),
            Space::with_width(10),
            text_input("value", value)
                .on_input(on_value_input)
                .style(AppInput),
            Space::with_width(10),
            remove_btn
        ])
        .padding(10)
        .width(Length::Fill)
        .style(AppContainer::Rounded),
    )
    .padding(Padding::from([2, 0]))
}
