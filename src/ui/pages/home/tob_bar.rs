use iced::widget::svg::Handle;
use iced::{
    widget::{button, container, pick_list, row, svg, Space},
    Element, Length, Padding,
};
use uuid::Uuid;

use crate::{
    constants::{APP_LOGO, LAYOUT_CLOSED_SVG, LAYOUT_OPENED_SVG, PEN_CLIP_SVG},
    ui::{
        app_theme::{AppBtn, AppContainer, AppSelect},
        elements::select_options::{SelectItems, SelectOption},
    },
};

use super::events::ProjectEvent;
use super::HomeEventMessage;

pub fn tob_bar(
    projects: SelectItems<Uuid>,
    selected_project: Option<SelectOption<Uuid>>,
    sidebar_closed: bool,
) -> Element<'static, HomeEventMessage> {
    container(
        row![
            container(svg(Handle::from_memory(APP_LOGO)).width(30).height(30))
                .padding(Padding::from([0.0, 10.0])),
            button(
                svg(Handle::from_memory(if sidebar_closed {
                    LAYOUT_OPENED_SVG
                } else {
                    LAYOUT_CLOSED_SVG
                }))
                .width(20)
                .height(20)
            )
            .style(AppBtn::Basic)
            .padding(5)
            .on_press(HomeEventMessage::ToggleSidebar),
            Space::with_width(10),
            pick_list(projects, selected_project, |item| ProjectEvent::Select(
                item.value
            )
            .into())
            .style(AppSelect::Card),
            Space::with_width(10),
            button(svg(Handle::from_memory(PEN_CLIP_SVG)).width(20).height(20))
                .style(AppBtn::Basic)
                .padding(5)
                .on_press(HomeEventMessage::OnChangePageState(
                    super::HomePageState::Projects
                )),
            Space::with_width(10),
            button("New")
                .style(AppBtn::Secondary)
                .padding(Padding::from([5, 15]))
                .on_press(ProjectEvent::Add("Unknown project".to_string()).into()),
        ]
        .padding(8.0)
        .align_items(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .style(AppContainer::Flat)
    .into()
}
