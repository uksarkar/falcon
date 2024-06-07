use iced::{widget::Column, Renderer, Theme};

use crate::utils::request::{PendingRequest, PendingRequestItem};

use super::{key_and_value_input_row::key_and_value_input_row, HomeEventMessage};

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
        let item_a = item.clone();
        let item_b = item.clone();
        let item_c = item.clone();

        container_columns = container_columns.push(key_and_value_input_row(
            key,
            value,
            items.len() > 1 && items.len() != index + 1,
            HomeEventMessage::RemoveRequestItem(item_a, index),
            move |input| HomeEventMessage::OnRequestItemKeyInput(item_b.clone(), index, input),
            move |input| HomeEventMessage::OnRequestItemValueInput(item_c.clone(), index, input),
        ));
    }

    container_columns
}
