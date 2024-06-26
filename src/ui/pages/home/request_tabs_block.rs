use iced::{
    widget::{
        column, container, row, text,
        text_editor::{self, Content},
        text_input, Column, Space, TextEditor,
    },
    Element, Length, Padding, Renderer, Theme,
};

use crate::{
    create_tabs,
    ui::{
        app_theme::{AppColor, AppContainer, AppInput, FalconTextarea},
        elements::tabs::Tabs,
    },
    utils::request::{FalconAuthorization, FlBody, PendingRequest, PendingRequestItem},
};

use super::{
    events::RequestEvent, key_and_value_input_row::key_and_value_input_row, HomeEventMessage,
};

pub fn request_tab_container<'a>(
    label: &str,
    pending_request: &PendingRequest,
    body_context: &'a text_editor::Content,
) -> Column<'a, HomeEventMessage, Theme, Renderer> {
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
        "Body" => {
            container_columns = container_columns.push(body_block(&pending_request, &body_context));
        }
        "Authorization" => {
            container_columns = container_columns.push(authorization_block(pending_request));
        }
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
            RequestEvent::RemoveItem(item_a, index).into(),
            move |input| RequestEvent::ItemKeyInput(item_b.clone(), index, input).into(),
            move |input| RequestEvent::ItemValueInput(item_c.clone(), index, input).into(),
        ));
    }

    container_columns
}

fn authorization_block<'a>(req: &PendingRequest) -> Element<'a, HomeEventMessage, Theme, Renderer> {
    column![
        create_tabs!(
            Tabs::new(vec!["Bearer"], "Bearer"),
            HomeEventMessage::OnAuthorizationTabChange,
            None,
            None
        ),
        Space::with_height(10),
        container(match req.authorization.clone() {
            FalconAuthorization::Bearer { prefix, token } => {
                let token_a = token.clone();
                let prefix_a = prefix.clone();

                column![
                    row![
                        text("Prefix"),
                        Space::with_width(5),
                        text_input("Bearer", &prefix)
                            .style(AppInput)
                            .on_input(move |p| {
                                RequestEvent::AuthorizationInput(FalconAuthorization::Bearer {
                                    prefix: p,
                                    token: token_a.clone(),
                                })
                                .into()
                            })
                    ]
                    .align_items(iced::Alignment::Center),
                    container(
                        container("")
                            .style(AppContainer::Bg(AppColor::BG_DARKER))
                            .height(1)
                            .width(Length::Fill),
                    )
                    .padding(Padding::from([10, 0])),
                    container(text("Token")).padding(Padding::from([10, 0])),
                    text_input("Token", &token)
                        .width(Length::Fill)
                        .style(AppInput)
                        .on_input(move |t| {
                            RequestEvent::AuthorizationInput(FalconAuthorization::Bearer {
                                prefix: prefix_a.clone(),
                                token: t,
                            })
                            .into()
                        }),
                ]
            }
        })
        .style(AppContainer::Rounded)
        .padding(10),
    ]
    .into()
}

fn body_block<'a>(
    req: &PendingRequest,
    body: &'a Content,
) -> Element<'a, HomeEventMessage, Theme, Renderer> {
    column![
        create_tabs!(
            Tabs::new(vec!["application/json"], "application/json"),
            HomeEventMessage::OnBodyTabChange,
            None,
            None
        ),
        Space::with_height(10),
        container(match req.body.clone() {
            FlBody::ApplicationJson(_) => {
                TextEditor::new(&body)
                    .height(Length::Fill)
                    .on_action(HomeEventMessage::OnRequestBodyContextAction)
                    .style(FalconTextarea)
            }
        })
        .style(AppContainer::Rounded)
        .padding(10),
    ]
    .into()
}
