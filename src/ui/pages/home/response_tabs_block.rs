use chrono::{DateTime, Utc};
use iced::{
    widget::{column, container, row, scrollable, text, Column, Container, Space},
    Length, Renderer, Theme,
};

use crate::{
    create_tabs,
    ui::{app_theme::AppContainer, elements::tabs::Tabs},
    utils::request::FalconResponse,
};

use super::HomeEventMessage;

pub fn response_tab_container(
    response: FalconResponse,
    tabs: &Tabs,
) -> Container<'static, HomeEventMessage, Theme, Renderer> {
    let mut response_tab = Column::new()
        .push(row![
            text("Response"),
            Space::with_width(Length::Fill),
            text("Status: "),
            text(response.status_code),
            Space::with_width(10),
            text(response.duration),
            Space::with_width(10),
            text(format!("Size: {}kb", response.size_kb)),
        ])
        .push(
            container("")
                .width(Length::Fill)
                .height(1)
                .style(AppContainer::Hr),
        )
        .push(create_tabs!(
            tabs,
            HomeEventMessage::OnResponseTabChange,
            None,
            None
        ));

    let mut tab_container = Column::new();

    if let Some(tab) = tabs.get_active() {
        match tab.label.as_str() {
            "Body" => {
                tab_container = tab_container.push(
                    container(text(response.body))
                        .padding(10)
                        .width(Length::Fill),
                );
            }
            "Header" => {
                for (name, value) in response.headers {
                    let header_name = if let Some(header_name) = name {
                        format!("{}", header_name.as_str())
                    } else {
                        "Unknown".to_string()
                    };

                    let header_value = format!("{:?}", value);

                    if !header_name.trim().is_empty() && !header_value.trim().is_empty() {
                        tab_container = tab_container.push(
                            container(column![
                                container(row![
                                    text(header_name),
                                    Space::with_width(10),
                                    text(":"),
                                    Space::with_width(10),
                                    text(format!("{:?}", value))
                                ])
                                .width(Length::Fill)
                                .padding(5),
                                container("")
                                    .width(Length::Fill)
                                    .height(1)
                                    .style(AppContainer::Hr)
                            ])
                            .padding(5),
                        );
                    }
                }
            }
            "Cookies" => {
                for cookie in response.cookies {
                    tab_container = tab_container.push(
                        container(column![
                            container(row![
                                text(cookie.name),
                                Space::with_width(10),
                                text(":"),
                                Space::with_width(10),
                                text(format!(
                                    "{}, exp: {}, http_only: {}",
                                    if let Some(val) = cookie.value {
                                        val
                                    } else {
                                        "".to_string()
                                    },
                                    if let Some(val) = cookie.expires {
                                        let datetime: DateTime<Utc> = val.into();
                                        format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"))
                                    } else {
                                        "".to_string()
                                    },
                                    cookie.http_only
                                ))
                            ])
                            .width(Length::Fill)
                            .padding(5),
                            container("")
                                .width(Length::Fill)
                                .height(1)
                                .style(AppContainer::Hr)
                        ])
                        .padding(5),
                    );
                }
            }
            _ => {}
        };
    };

    response_tab = response_tab.push(container(scrollable(tab_container)).width(Length::Fill));

    container(response_tab)
        .height(Length::Fill)
        .padding(10)
        .width(Length::Fill)
        .style(AppContainer::Rounded)
}
