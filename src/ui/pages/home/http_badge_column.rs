use iced::widget::svg::Handle;
use iced::widget::{mouse_area, Container};
use iced::{
    widget::{button, container, row, svg, text, Space},
    Length, Padding,
};
use iced::{Renderer, Theme};

use crate::{
    constants::{CROSS_SMALL_SVG, DUPLICATE_SVG},
    ui::app_theme::{AppBtn, AppColor, AppContainer},
    utils::{helpers::ellipse_text, request::http_method::HttpMethod},
};

pub struct HttpBadgeColumn<Message> {
    pub label: String,
    pub method: HttpMethod,
    pub on_duplicate: Message,
    pub on_remove: Message,
    pub on_click: Message,
    pub is_active: bool,
}

impl<'a, Message> Into<Container<'a, Message, Theme, Renderer>> for HttpBadgeColumn<Message>
where
    Message: 'a + Clone,
{
    fn into(self) -> Container<'a, Message, Theme, Renderer> {
        let color = match self.method.to_string().as_str() {
            "Post" => AppColor::BG_DARKEST,
            "Put" => AppColor::PURPLE,
            "Patch" => AppColor::YELLOW,
            "Delete" => AppColor::RED,
            _ => AppColor::GREEN,
        };

        container(
            mouse_area(
                container(
                    row![
                        container(
                            row![
                                container("")
                                    .width(4)
                                    .height(Length::Fill)
                                    .style(AppContainer::Bg(color)),
                                Space::with_width(4),
                                text(self.method),
                                Space::with_width(4),
                            ]
                            .align_items(iced::Alignment::Center)
                        )
                        .height(25)
                        .style(AppContainer::BadgePrimary)
                        .padding(2),
                        Space::with_width(5),
                        text(ellipse_text(&self.label, 20)),
                        Space::with_width(Length::Fill),
                        button(svg(Handle::from_memory(DUPLICATE_SVG)).width(20).height(20))
                            .style(AppBtn::Basic)
                            .padding(5)
                            .on_press(self.on_duplicate),
                        button(
                            svg(Handle::from_memory(CROSS_SMALL_SVG))
                                .width(20)
                                .height(20)
                        )
                        .style(AppBtn::Basic)
                        .padding(5)
                        .on_press(self.on_remove),
                    ]
                    .align_items(iced::Alignment::Center),
                )
                .align_y(iced::alignment::Vertical::Center)
                .style(if self.is_active {
                    AppContainer::FlatSecondary
                } else {
                    AppContainer::Flat
                })
                .padding(Padding::from([5, 0])),
            )
            .on_press(self.on_click)
            .interaction(iced::mouse::Interaction::Pointer),
        )
        .into()
    }
}
