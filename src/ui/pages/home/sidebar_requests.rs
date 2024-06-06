use iced::widget::{container, Column, Container};
use iced::{Element, Length};

use crate::ui::app_theme::{AppColor, AppContainer};
use crate::utils::request::PendingRequest;

use super::http_badge_column::HttpBadgeColumn;
use super::{HomeEventMessage, HomePage};

pub fn sidebar_requests(page: &HomePage) -> Element<'static, HomeEventMessage> {
    let mut requests = Column::new();

    if let Some(project) = page.projects.active() {
        for (_, reqs) in project.requests {
            for req in reqs {
                let card: Container<'static, HomeEventMessage> = HttpBadgeColumn {
                    label: req.url.clone(),
                    on_click: HomeEventMessage::SelectRequest(req.id),
                    on_duplicate: HomeEventMessage::AddNewRequest(PendingRequest {
                        cookies: req.cookies.clone(),
                        headers: req.headers.clone(),
                        method: req.method.clone(),
                        queries: req.queries.clone(),
                        url: req.url.clone(),
                        ..Default::default()
                    }),
                    on_remove: HomeEventMessage::ToggleSidebar,
                    method: req.method,
                    is_active: page
                        .projects
                        .active()
                        .is_some_and(|p| p.active_request_id.is_some_and(|id| id == req.id)),
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
