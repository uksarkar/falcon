use iced::{
    widget::{container, mouse_area, row, svg, text, Column, Space},
    Element, Length, Padding, Renderer, Theme,
};

use iced::widget::svg::Handle;

use crate::{
    constants::ANGLE_LEFT_SVG,
    ui::app_theme::{AppColor, AppContainer},
};

use super::{sidebar_item::sidebar_item, HomeEventMessage, HomePage};

pub fn get_env_items(page: &HomePage) -> Element<'static, HomeEventMessage, Theme, Renderer> {
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

    for env in page.projects.env_into_options().iter() {
        items = items
            .push(sidebar_item(
                env.label.clone().as_str(),
                page.projects.is_active_env(env.value),
                HomeEventMessage::OnEnvSelect(env.value),
                HomeEventMessage::OnEnvDelete(env.value),
                HomeEventMessage::OnEnvDuplicate(env.value),
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
