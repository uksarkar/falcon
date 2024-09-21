use iced::widget::svg::Handle;
use iced::widget::{button, Column};
use iced::{
    widget::{container, mouse_area, row, svg, text, Space},
    Element, Length, Padding, Renderer, Theme,
};

use crate::constants::{ADD_DOC_SVG, ANGLE_LEFT_SVG};
use crate::ui::app_theme::AppContainer;
use crate::ui::app_theme::{AppBtn, AppColor};

use super::events::ProjectEvent;
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
                        text("Back").vertical_alignment(iced::alignment::Vertical::Center),
                        Space::with_width(Length::Fill),
                        button(row![
                            svg(Handle::from_memory(ADD_DOC_SVG)).width(15).height(15),
                            Space::with_width(10),
                            text("New project").size(14)
                        ])
                        .padding(5)
                        .style(AppBtn::Basic)
                        .on_press(ProjectEvent::Add("Unknown project".into()).into())
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

    for project in page.db.into_options().iter() {
        items = items
            .push(sidebar_item(
                &project.label,
                page.db.active().is_some_and(|p| p.id == project.value),
                ProjectEvent::Select(project.value).into(),
                ProjectEvent::Remove(project.value).into(),
                ProjectEvent::Duplicate(project.value).into(),
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
