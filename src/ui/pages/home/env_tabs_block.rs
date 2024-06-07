use iced::{
    widget::{container, row, text, text_input, Column, Space},
    Element, Length, Padding, Renderer, Theme,
};

use crate::{
    ui::app_theme::{AppContainer, AppInput},
    utils::db::Env,
};

use super::{key_and_value_input_row::key_and_value_input_row, HomeEventMessage};

pub fn env_tabs_block<'a>(
    active_env: Option<Env>,
) -> Element<'a, HomeEventMessage, Theme, Renderer> {
    let mut items = Column::new()
        .push(
            container(
                container(
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
                        .on_input(|name| HomeEventMessage::OnEnvNameInput(name)),
                    ]
                    .align_items(iced::Alignment::Center),
                )
                .align_y(iced::alignment::Vertical::Center)
                .padding(2)
                .width(Length::Fill)
                .style(AppContainer::Rounded),
            )
            .padding(Padding::from([2, 0])),
        )
        .push(container(text("Variables")).padding(Padding::from([10, 0])));

    if let Some(env) = active_env {
        for (index, (key, value)) in env.items.iter().enumerate() {
            items = items.push(key_and_value_input_row(
                key,
                value,
                env.items.len() > 1 && env.items.len() != index + 1,
                HomeEventMessage::OnEnvItemRemove(index),
                move |key| HomeEventMessage::OnEnvItemKeyInput(index, key),
                move |value| HomeEventMessage::OnEnvItemValueInput(index, value),
            ));
        }
    }

    items.padding(10).into()
}
