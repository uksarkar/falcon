use iced::widget::svg::Handle;
use iced::widget::Column;
use iced::{
    widget::{button, container, mouse_area, row, svg, text, Space},
    Element, Length, Padding, Renderer, Theme,
};

use crate::constants::ANGLE_LEFT_SVG;
use crate::ui::app_theme::AppColor;
use crate::{
    constants::{DUPLICATE_SVG, TRASH_SVG},
    ui::app_theme::{AppBtn, AppContainer},
    utils::helpers::ellipse_text,
};

use super::{HomeEventMessage, HomePage};

pub fn get_sidebar_projects_items(
    page: &HomePage,
) -> Element<'static, HomeEventMessage, Theme, Renderer> {
    let mut items = Column::new()
        .push(
            mouse_area(
                container(
                    row![
                        svg(Handle::from_memory(ANGLE_LEFT_SVG))
                            .width(15)
                            .height(15),
                        Space::with_width(5),
                        text("Back").vertical_alignment(iced::alignment::Vertical::Center)
                    ]
                    .align_items(iced::Alignment::Center),
                )
                .align_y(iced::alignment::Vertical::Center)
                .style(AppContainer::FlatSecondary)
                .width(Length::Fill)
                .padding(Padding::from([2, 5])),
            )
            .on_press(HomeEventMessage::OnChangePageState(
                super::HomePageState::Requests,
            ))
            .interaction(iced::mouse::Interaction::Pointer),
        )
        .push(
            container("")
                .style(AppContainer::Bg(AppColor::BG_DARKER))
                .height(1)
                .width(Length::Fill),
        );

    for project in page.projects.into_options().iter() {
        items = items
            .push(project_column(
                &project.label,
                page.projects
                    .active()
                    .is_some_and(|p| p.id == project.value),
                HomeEventMessage::OnProjectChange(project.value),
                HomeEventMessage::ToggleSidebar,
                HomeEventMessage::ToggleSidebar,
            ))
            .push(
                container("")
                    .style(AppContainer::Bg(AppColor::BG_DARKER))
                    .height(1)
                    .width(Length::Fill),
            );
    }

    items.into()
}

fn project_column(
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
