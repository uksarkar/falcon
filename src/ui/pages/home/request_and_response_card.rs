use iced::widget::svg::Handle;
use iced::widget::{column, container, row, svg, text, text_input, Column, Space};
use iced::{Element, Length, Padding};

use crate::constants::{COMPRESS_SVG, EXPAND_SVG};
use crate::create_tabs;
use crate::ui::app_theme::{AppContainer, AppInput};
use crate::utils::request::RequestUrl;

use super::events::RequestEvent;
use super::request_tabs_block::request_tab_container;
use super::response_tabs_block::response_tab_container;
use super::url_input_bar::url_input_bar;
use super::{HomeEventMessage, HomePage};

pub fn request_and_response_card<'a>(page: &'a HomePage) -> Element<'a, HomeEventMessage> {
    let mut conditional_container = Column::new();
    let (_, pending_request) = page.pending_request();

    if let Some(tab) = page.request_tabs.get_active() {
        conditional_container = conditional_container.push(request_tab_container(
            &tab.label,
            &pending_request,
            &page.request_body_context,
        ));
    }

    if let Some(response) = page.response.clone() {
        conditional_container = conditional_container
            .push(Space::with_height(10))
            .push(response_tab_container(response, &page.response_tabs));
    }

    column![
        container(
            row![
                text("Request name: ").size(14),
                text_input("Unknown request", &pending_request.name.unwrap_or_default())
                    .style(AppInput)
                    .width(Length::Fill)
                    .on_input(|name| RequestEvent::NameInput(name).into()),
            ]
            .align_items(iced::Alignment::Center)
        )
        .align_y(iced::alignment::Vertical::Center)
        .padding(Padding::from([0, 10]))
        .style(AppContainer::Rounded)
        .height(40)
        .width(Length::Fill),
        Space::with_height(10),
        url_input_bar(
            &RequestUrl::from(pending_request.url).build(&page.db.get_active_base_url()),
            page.is_requesting,
            &pending_request.method
        ),
        Space::with_height(10),
        match page.response {
            Some(_) => create_tabs!(
                page.request_tabs,
                HomeEventMessage::OnRequestTabChange,
                Some(HomeEventMessage::MinimizeRequestTabs),
                Some(
                    container(
                        svg(Handle::from_memory(if page.request_tabs.is_active() {
                            EXPAND_SVG
                        } else {
                            COMPRESS_SVG
                        }))
                        .width(12)
                        .height(12)
                    )
                    .padding(5)
                    .style(AppContainer::Outlined)
                )
            ),
            None => create_tabs!(
                page.request_tabs,
                HomeEventMessage::OnRequestTabChange,
                None,
                None
            ),
        },
        container("")
            .width(Length::Fill)
            .height(1)
            .style(AppContainer::Hr),
        Space::with_height(10),
        conditional_container,
    ]
    .into()
}
