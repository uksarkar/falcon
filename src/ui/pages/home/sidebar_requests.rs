use iced::widget::svg::Handle;
use iced::{
    widget::{button, column, container, row, svg, text, Space},
    Element, Length, Padding,
};

use crate::{
    constants::{ADD_DOC_SVG, CROSS_SMALL_SVG, DUPLICATE_SVG},
    ui::app_theme::{AppBtn, AppContainer},
};

use super::{HomeEventMessage, HomePage};

pub fn sidebar_requests(_page: &HomePage) -> Element<'static, HomeEventMessage> {
    column![container(row![
        text("Some request"),
        Space::with_width(Length::Fill),
        button(svg(Handle::from_memory(ADD_DOC_SVG)).width(20).height(20))
            .style(AppBtn::Basic)
            .padding(5)
            .on_press(HomeEventMessage::ToggleSidebar),
        button(svg(Handle::from_memory(DUPLICATE_SVG)).width(20).height(20))
            .style(AppBtn::Basic)
            .padding(5)
            .on_press(HomeEventMessage::ToggleSidebar),
        button(
            svg(Handle::from_memory(CROSS_SMALL_SVG))
                .width(20)
                .height(20)
        )
        .style(AppBtn::Basic)
        .padding(5)
        .on_press(HomeEventMessage::ToggleSidebar),
    ])
    .style(AppContainer::FlatSecondary)
    .padding(Padding::from([5, 0]))]
    .into()
}
