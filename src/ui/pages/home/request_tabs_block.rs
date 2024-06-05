use iced::{
    widget::{button, container, row, svg, text_input, Column, Container, Space},
    Length, Padding, Renderer, Theme,
};
use iced::widget::svg::Handle;

use crate::{
    constants::TRASH_SVG, ui::app_theme::{AppBtn, AppContainer, AppInput}, utils::request::{PendingRequest, PendingRequestItem}
};

use super::HomeEventMessage;

pub fn request_tab_container(
    label: &str,
    pending_request: &PendingRequest,
) -> Column<'static, HomeEventMessage, Theme, Renderer> {
    let mut container_columns = Column::new();

    match label {
        "Query" => {
            container_columns =
                build_key_value_input_columns(&pending_request.queries, PendingRequestItem::Query);
        }
        "Header" => {
            container_columns =
                build_key_value_input_columns(&pending_request.headers, PendingRequestItem::Header);
        }
        "Body" => {}
        "Authorization" => {}
        "Cookies" => {
            container_columns =
                build_key_value_input_columns(&pending_request.cookies, PendingRequestItem::Cookie);
        }
        _ => (),
    };

    container_columns
}

fn build_key_value_input_columns(
    items: &Vec<(String, String)>,
    item: PendingRequestItem,
) -> Column<'static, HomeEventMessage, Theme, Renderer> {
    let mut container_columns = Column::new();

    for (index, (key, value)) in items.iter().enumerate() {
        container_columns = container_columns.push(key_value_input_row(
            index,
            key,
            value,
            items.len() > 1 && items.len() != index + 1,
            item.clone(),
        ));
    }

    container_columns
}

fn key_value_input_row(
    index: usize,
    key: &str,
    value: &str,
    enabled: bool,
    item: PendingRequestItem,
) -> Container<'static, HomeEventMessage, Theme, Renderer> {
    let key_index = index;
    let value_index = index;

    let item_c1 = item.clone();
    let item_c2 = item.clone();

    let mut remove_btn = button(
        svg(Handle::from_memory(TRASH_SVG))
        .width(20)
        .height(20),
    )
    .style(AppBtn::Basic)
    .padding(5);

    if enabled {
        remove_btn = remove_btn.on_press(HomeEventMessage::RemoveRequestItem(item, index));
    }

    container(
        container(row![
            text_input("key", key)
                .on_input(move |input| HomeEventMessage::OnRequestItemKeyInput(
                    item_c1.clone(),
                    key_index,
                    input
                ))
                .style(AppInput)
                .width(200),
            Space::with_width(10),
            text_input("value", value)
                .on_input(move |input| HomeEventMessage::OnRequestItemValueInput(
                    item_c2.clone(),
                    value_index,
                    input
                ))
                .style(AppInput),
            Space::with_width(10),
            remove_btn
        ])
        .padding(10)
        .width(Length::Fill)
        .style(AppContainer::Rounded),
    )
    .padding(Padding::from([2, 0]))
}
