use iced::{
    widget::{column, container, row, text, text_input},
    Element, Length, Padding, Renderer, Theme,
};

use crate::ui::app_theme::{AppColor, AppContainer, AppInput};

use super::{HomeEventMessage, HomePage};

pub fn project_tabs_block(page: &HomePage) -> Element<'static, HomeEventMessage, Theme, Renderer> {
    column![container(column![
        row![
            text("Project name: ").size(14),
            text_input(
                "Unknown project",
                &page
                    .projects
                    .active()
                    .and_then(|p| Some(p.name))
                    .unwrap_or_default()
            )
            .style(AppInput)
            .width(Length::Fill)
            .on_input(HomeEventMessage::OnProjectNameInput),
        ]
        .align_items(iced::Alignment::Center),
        container("")
            .style(AppContainer::Bg(AppColor::BG_DARKER))
            .height(1)
            .width(Length::Fill),
        row![
            text("Base URL: ").size(14),
            text_input(
                "https://utpal.io",
                &page
                    .projects
                    .active()
                    .and_then(|p| p.base_url)
                    .unwrap_or_default()
            )
            .style(AppInput)
            .on_input(HomeEventMessage::OnProjectBaseUrlInput)
            .width(Length::Fill)
        ]
        .align_items(iced::Alignment::Center),
    ])
    .align_y(iced::alignment::Vertical::Center)
    .padding(Padding::from([0, 10]))
    .style(AppContainer::Rounded)
    .width(Length::Fill)]
    .padding(10)
    .into()
}
