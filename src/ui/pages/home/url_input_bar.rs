use iced::{
    widget::{button, container, pick_list, row, text_input, Container, Space},
    Length, Padding, Renderer, Theme,
};

use crate::{
    ui::{
        app_theme::{AppBtn, AppContainer, AppInput, AppSelect},
        elements::select_options::{SelectItems, SelectOption},
    },
    utils::request::HttpMethod,
};

use super::HomeEventMessage;

pub fn url_input_bar(
    url: &str,
    is_requesting: bool,
    method: &HttpMethod,
) -> Container<'static, HomeEventMessage, Theme, Renderer> {
    let mut button = button(if is_requesting { "Sending" } else { "Send" })
        .style(AppBtn::Primary)
        .padding(Padding::from([5, 15]));

    if !is_requesting {
        button = button.on_press(HomeEventMessage::SendRequest);
    }

    let selected_method: SelectOption<HttpMethod> = method.clone().into();

    container(
        row![
            pick_list(get_method_options(), Some(selected_method), |item| {
                HomeEventMessage::OnRequestMethodChanged(item.value.into())
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

fn get_method_options() -> SelectItems<HttpMethod> {
    SelectItems(
        vec!["Get", "Post", "Put", "Patch", "Delete"]
            .into_iter()
            .map(|method| {
                let method: HttpMethod = method.into();
                let method: SelectOption<HttpMethod> = method.into();
                method
            })
            .collect(),
    )
}
