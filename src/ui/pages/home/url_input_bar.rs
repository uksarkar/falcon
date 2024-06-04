use iced::{
    widget::{button, container, pick_list, row, text_input, Container, Space},
    Length, Padding, Renderer, Theme,
};
use reqwest::Method;

use crate::ui::{
    app_theme::{AppBtn, AppContainer, AppInput, AppSelect},
    elements::select_options::{SelectItems, SelectOption},
};

use super::HomeEventMessage;

pub fn url_input_bar(
    url: &str,
    is_requesting: bool,
    method: &Method,
) -> Container<'static, HomeEventMessage, Theme, Renderer> {
    let mut button = button(if is_requesting { "Sending" } else { "Send" })
        .style(AppBtn::Primary)
        .padding(Padding::from([5, 15]));

    if !is_requesting {
        button = button.on_press(HomeEventMessage::SendRequest);
    }

    container(
        row![
            pick_list(get_method_options(), Some(method_to_select_option(method)), |item| {
                HomeEventMessage::OnRequestMethodChanged(item.value)
            })
            .style(AppSelect::Card),
            text_input("https://utpal.io", url)
                .style(AppInput)
                .width(Length::Fill)
                .on_input(HomeEventMessage::UrlInput),
            Space::with_width(10),
            button,
        ]
        .align_items(iced::Alignment::Center),
    )
    .align_y(iced::alignment::Vertical::Center)
    .padding(Padding::from([0, 10]))
    .style(AppContainer::Rounded)
    .height(40)
    .width(Length::Fill)
}

fn get_method_options() -> SelectItems<Method> {
    SelectItems(
        vec!["Get", "Post", "Put", "Patch", "Delete"]
            .into_iter()
            .filter_map(str_to_select_option)
            .collect(),
    )
}

fn str_to_select_option(item: &str) -> Option<SelectOption<Method>> {
    if let Some(method) = Method::from_bytes(item.to_uppercase().as_bytes()).ok() {
        Some(SelectOption {
            label: item.into(),
            value: method,
        })
    } else {
        None
    }
}

fn method_to_select_option(method: &Method) -> SelectOption<Method> {
    let mut label = method.as_str().chars();
    let label = match label.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + label.as_str().to_lowercase().as_str(),
    };

    SelectOption {
        label,
        value: method.clone()
    }
}