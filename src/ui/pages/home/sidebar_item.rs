use iced::widget::svg::Handle;
use iced::Padding;
use iced::{
    widget::{button, container, mouse_area, row, svg, text, Space},
    Element, Length, Renderer, Theme,
};

use crate::constants::{DUPLICATE_SVG, TRASH_SVG};
use crate::ui::app_theme::{AppBtn, AppContainer};
use crate::utils::helpers::ellipse_text;

use super::HomeEventMessage;

pub fn sidebar_item(
    name: &str,
    is_active: bool,
    on_click: HomeEventMessage,
    on_remove: HomeEventMessage,
    on_duplicate: HomeEventMessage,
) -> Element<'static, HomeEventMessage, Theme, Renderer> {
    container(
        mouse_area(
            container(
                row![
                    text(ellipse_text(name, 20)),
                    Space::with_width(Length::Fill),
                    button(svg(Handle::from_memory(DUPLICATE_SVG)).width(20).height(20))
                        .style(AppBtn::Basic)
                        .padding(5)
                        .on_press(on_duplicate),
                    button(svg(Handle::from_memory(TRASH_SVG)).width(20).height(20))
                        .style(AppBtn::Basic)
                        .padding(5)
                        .on_press(on_remove),
                ]
                .align_items(iced::Alignment::Center),
            )
            .align_y(iced::alignment::Vertical::Center)
            .style(if is_active {
                AppContainer::FlatSecondary
            } else {
                AppContainer::Flat
            })
            .padding(Padding {
                top: 5.0,
                bottom: 5.0,
                left: 5.0,
                right: 0.0,
            }),
        )
        .on_press(on_click)
        .interaction(iced::mouse::Interaction::Pointer),
    )
    .into()
}
