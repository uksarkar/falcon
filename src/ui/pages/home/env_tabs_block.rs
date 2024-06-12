use crate::{
    constants::FILE_CIRCLE_INFO_SVG,
    ui::app_theme::{AppBtn, AppColor, AppContainer, AppInput},
    utils::db::env::Env,
};
use iced::widget::{svg::Handle, tooltip};
use iced::{
    widget::{button, column, container, row, svg, text, text_input, Column, Space},
    Element, Length, Padding, Renderer, Theme,
};

use super::{events::EnvEvent, key_and_value_input_row::key_and_value_input_row, HomeEventMessage};

pub fn env_tabs_block<'a>(
    active_env: Option<Env>,
    show_examples: bool,
) -> Element<'a, HomeEventMessage, Theme, Renderer> {
    // env name input
    let env_name_input = container(
        container(column![
            row![
                Space::with_width(10),
                text("Env name:"),
                Space::with_width(10),
                text_input(
                    "value",
                    &active_env
                        .clone()
                        .and_then(|env| Some(env.name))
                        .unwrap_or_default()
                )
                .style(AppInput)
                .on_input(|name| EnvEvent::NameInput(name).into()),
            ]
            .align_items(iced::Alignment::Center),
            container(
                container("")
                    .style(AppContainer::Bg(AppColor::BG_DARKER))
                    .height(1)
                    .width(Length::Fill)
            )
            .padding(Padding::from([10, 0])),
            row![
                Space::with_width(10),
                text("Base URL: ").size(14),
                Space::with_width(10),
                text_input(
                    "https://example.com",
                    &active_env
                        .clone()
                        .and_then(|env| env.base_url)
                        .unwrap_or_default()
                )
                .style(AppInput)
                .on_input(|url| EnvEvent::BaseUrlInput(url).into())
                .width(Length::Fill)
            ]
            .align_items(iced::Alignment::Center),
        ])
        .align_y(iced::alignment::Vertical::Center)
        .padding(2)
        .width(Length::Fill)
        .style(AppContainer::Rounded),
    );

    // label variables
    let var_label = container(row![
        text("Variables"),
        Space::with_width(10),
        tooltip(
            button(
                svg(Handle::from_memory(FILE_CIRCLE_INFO_SVG))
                    .width(12)
                    .height(12)
            )
            .padding(5)
            .style(AppBtn::Basic)
            .on_press(HomeEventMessage::ToggleEnvExample),
            container(text("Usage?").size(14))
                .padding(5)
                .style(AppContainer::Bg(AppColor::BG_DARK)),
            tooltip::Position::FollowCursor
        )
    ])
    .padding(Padding::from([10, 0]));

    // build the column
    let mut items = Column::new().push(env_name_input).push(var_label);

    if show_examples {
        items = items.push(container(column![
            example_container(
                "Example 1:",
                "DOMAIN",
                "https://example.com",
                "{{DOMAIN}}/users",
                "https://example.com/users"
            ),
            Space::with_height(5),
            example_container(
                "Example 2:",
                "API_PATH",
                "https://$0.example.com/v$1",
                "{{API_PATH[www,3]}}/users",
                "https://www.example.com/v3/users"
            ),
            Space::with_height(5),
            example_container(
                "Example 3:",
                "API_PATH",
                "https://$sub.example.com/v$version",
                "{{API_PATH[version: 2, sub: app]}}/users",
                "https://app.example.com/v2/users"
            ),
            Space::with_height(10),
        ]))
    }

    if let Some(env) = active_env {
        for (index, (key, value)) in env.items.iter().enumerate() {
            items = items.push(key_and_value_input_row(
                key,
                value,
                env.items.len() > 1 && env.items.len() != index + 1,
                EnvEvent::ItemRemove(index).into(),
                move |key| EnvEvent::ItemKeyInput(index, key).into(),
                move |value| EnvEvent::ItemValueInput(index, value).into(),
            ));
        }
    }

    items.padding(10).into()
}

fn example_container<'a>(
    title: &str,
    key: &str,
    value: &str,
    usage_key: &str,
    usage_value: &str,
) -> Element<'a, HomeEventMessage> {
    container(column![
        text(title),
        row![
            container(text(format!("Key: {}", key))).padding(5),
            container(text("=")).padding(5),
            container(text(format!("Value: {}", value))).padding(5)
        ]
        .align_items(iced::Alignment::Center),
        row![
            container(text(usage_key)).padding(5),
            container(text("=")).padding(5),
            container(text(usage_value)).padding(5)
        ]
        .align_items(iced::Alignment::Center)
    ])
    .width(Length::Fill)
    .style(AppContainer::Rounded)
    .padding(10)
    .into()
}
