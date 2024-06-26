use iced::widget::svg::Handle;
use iced::widget::tooltip;
use iced::{
    widget::{button, column, container, pick_list, row, svg, text, text_input, Row, Space},
    Element, Length, Padding, Renderer, Theme,
};
use uuid::Uuid;

use crate::constants::CROSS_SMALL_SVG;
use crate::ui::{
    app_theme::{AppBtn, AppColor, AppContainer, AppInput, AppSelect},
    elements::select_options::SelectOption,
};

use super::events::ProjectEvent;
use super::{HomeEventMessage, HomePage};

pub fn project_tabs_block(page: &HomePage) -> Element<'static, HomeEventMessage, Theme, Renderer> {
    let active_project = page.db.active();
    let project_env: Option<SelectOption<Uuid>> = page
        .db
        .project_default_env()
        .and_then(|env| Some(env.into()));

    let mut env_row = Row::new()
        .push(text("Default Environments:").size(14))
        .push(Space::with_width(10))
        .push(
            pick_list(
                page.db.env_into_options(),
                project_env.clone(),
                |env| ProjectEvent::DefaultEnvSelect(Some(env.value)).into(),
            )
            .placeholder("Select Environment")
            .style(AppSelect::Card),
        );

    if project_env.is_some() {
        env_row = env_row.push(tooltip(
            button(
                svg(Handle::from_memory(CROSS_SMALL_SVG))
                    .width(20)
                    .height(20),
            )
            .style(AppBtn::Basic)
            .padding(5)
            .on_press(HomeEventMessage::ProjectEvent(
                ProjectEvent::DefaultEnvSelect(None),
            )),
            container(text("Remove Environment").size(14))
                .padding(5)
                .style(AppContainer::Bg(AppColor::BG_DARKER)),
            tooltip::Position::FollowCursor,
        ));
    }

    column![container(column![
        row![
            text("Project name: ").size(14),
            text_input(
                "Unknown project",
                &active_project
                    .clone()
                    .and_then(|p| Some(p.name))
                    .unwrap_or_default()
            )
            .style(AppInput)
            .width(Length::Fill)
            .on_input(|name| HomeEventMessage::ProjectEvent(ProjectEvent::NameInput(name))),
        ]
        .align_items(iced::Alignment::Center),
        container(
            container("")
                .style(AppContainer::Bg(AppColor::BG_DARKER))
                .height(1)
                .width(Length::Fill)
        )
        .padding(Padding::from([10, 0])),
        env_row.align_items(iced::Alignment::Center),
    ])
    .align_y(iced::alignment::Vertical::Center)
    .padding(10)
    .style(AppContainer::Rounded)
    .width(Length::Fill)]
    .padding(10)
    .into()
}
