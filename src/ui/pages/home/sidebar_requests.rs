use iced::widget::svg::Handle;
use iced::widget::{button, container, row, svg, text, tooltip, Column, Container, Space};
use iced::{Element, Length};

use crate::constants::{ADD_DOC_SVG, COG_API_SVG};
use crate::ui::app_theme::{AppBtn, AppColor, AppContainer};
use crate::utils::request::PendingRequest;

use super::http_badge_column::HttpBadgeColumn;
use super::{HomeEventMessage, HomePage};

pub fn sidebar_requests(page: &HomePage) -> Element<'static, HomeEventMessage> {
    let mut requests = Column::new()
        .push(
            container(row![
                text("Default env").size(14),
                Space::with_width(Length::Fill),
                tooltip(
                    button(svg(Handle::from_memory(COG_API_SVG)).width(15).height(15))
                        .style(AppBtn::Basic)
                        .padding(3)
                        .on_press(HomeEventMessage::OnChangePageState(super::HomePageState::Envs)),
                    container(text("Environments").size(10))
                        .style(AppContainer::Bg(AppColor::BG_DARKEST))
                        .padding(4),
                    tooltip::Position::FollowCursor
                ),
                tooltip(
                    button(svg(Handle::from_memory(ADD_DOC_SVG)).width(15).height(15))
                        .style(AppBtn::Basic)
                        .padding(3)
                        .on_press(HomeEventMessage::AddNewRequest(PendingRequest::default())),
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

    if let Some(project) = page.projects.active() {
        for (_, reqs) in project.requests {
            for req in reqs {
                let card: Container<'static, HomeEventMessage> = HttpBadgeColumn {
                    label: if req.name.clone().is_some_and(|n| n.trim().len() > 0) {
                        req.name.unwrap_or_default().trim().to_string()
                    } else {
                        req.url.clone()
                    },
                    on_click: HomeEventMessage::SelectRequest(req.id),
                    on_duplicate: HomeEventMessage::AddNewRequest(PendingRequest {
                        cookies: req.cookies.clone(),
                        headers: req.headers.clone(),
                        method: req.method.clone(),
                        queries: req.queries.clone(),
                        url: req.url.clone(),
                        ..Default::default()
                    }),
                    on_remove: HomeEventMessage::DeleteRequest(req.id),
                    method: req.method,
                    is_active: page
                        .projects
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
