use iced::widget::svg::Handle;
use iced::widget::Column;
use iced::{
    widget::{container, mouse_area, row, svg, text, Space},
    Element, Length, Padding, Renderer, Theme,
};

use crate::constants::ANGLE_LEFT_SVG;
use crate::ui::app_theme::AppColor;
use crate::ui::app_theme::AppContainer;

use super::sidebar_item::sidebar_item;
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
            .push(sidebar_item(
                &project.label,
                page.projects
                    .active()
                    .is_some_and(|p| p.id == project.value),
                HomeEventMessage::OnProjectChange(project.value),
                HomeEventMessage::OnProjectRemove(project.value),
                HomeEventMessage::OnProjectDuplicate(project.value),
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

