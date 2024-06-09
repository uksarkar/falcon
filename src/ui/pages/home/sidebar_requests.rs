use iced::widget::svg::Handle;
use iced::widget::{button, container, pick_list, row, svg, text, tooltip, Column, Container};
use iced::{Element, Length};

use crate::constants::{ADD_DOC_SVG, COG_API_SVG};
use crate::ui::app_theme::{AppBtn, AppColor, AppContainer, AppSelect};
use crate::utils::request::{PendingRequest, RequestUrl};

use super::events::{EnvEvent, RequestEvent};
use super::http_badge_column::HttpBadgeColumn;
use super::{HomeEventMessage, HomePage};

pub fn sidebar_requests(page: &HomePage) -> Element<'static, HomeEventMessage> {
    let mut requests = Column::new()
        .push(
            container(row![
                pick_list(
                    page.db.env_into_options(),
                    page.db.selected_env_as_option(),
                    |env| EnvEvent::Select(env.value).into()
                )
                .padding(2)
                .text_size(14)
                .width(Length::Fill)
                .style(AppSelect::Card),
                tooltip(
                    button(svg(Handle::from_memory(COG_API_SVG)).width(15).height(15))
                        .style(AppBtn::Basic)
                        .padding(3)
                        .on_press(HomeEventMessage::OnChangePageState(
                            super::HomePageState::Envs
                        )),
                    container(text("Environments").size(10))
                        .style(AppContainer::Bg(AppColor::BG_DARKEST))
                        .padding(4),
                    tooltip::Position::FollowCursor
                ),
                tooltip(
                    button(svg(Handle::from_memory(ADD_DOC_SVG)).width(15).height(15))
                        .style(AppBtn::Basic)
                        .padding(3)
                        .on_press(RequestEvent::New.into()),
                    container(text("New request").size(10))
                        .style(AppContainer::Bg(AppColor::BG_DARKEST))
                        .padding(4),
                    tooltip::Position::FollowCursor
                ),
            ])
            .style(AppContainer::FlatSecondary)
            .padding(2),
        )
        .push(
            container("")
                .style(AppContainer::Bg(AppColor::BG_DARKER))
                .height(1)
                .width(Length::Fill),
        );

    if let Some(project) = page.db.active() {
        let base_url = project.base_url.unwrap_or_default();

        for (_, reqs) in project.requests {
            for req in reqs {
                let card: Container<'static, HomeEventMessage> = HttpBadgeColumn {
                    label: if req.name.clone().is_some_and(|n| n.trim().len() > 0) {
                        req.name.unwrap_or_default().trim().to_string()
                    } else {
                        RequestUrl::from(req.url.clone()).build(&base_url)
                    },
                    on_click: RequestEvent::Select(req.id).into(),
                    on_duplicate: RequestEvent::Add(PendingRequest {
                        cookies: req.cookies.clone(),
                        headers: req.headers.clone(),
                        method: req.method.clone(),
                        queries: req.queries.clone(),
                        url: req.url.clone(),
                        ..Default::default()
                    })
                    .into(),
                    on_remove: RequestEvent::Delete(req.id).into(),
                    method: req.method,
                    is_active: page
                        .db
                        .active()
                        .is_some_and(|p| p.current_request_id().is_some_and(|id| id == req.id)),
                }
                .into();

                requests = requests.push(card).push(
                    container("")
                        .style(AppContainer::Bg(AppColor::BG_DARKER))
                        .height(1)
                        .width(Length::Fill),
                );
            }
        }
    }

    requests.into()
}
