use iced::widget::{Column, Container};
use iced::Element;

use super::http_badge_column::HttpBadgeColumn;
use super::{HomeEventMessage, HomePage};

pub fn sidebar_requests(page: &HomePage) -> Element<'static, HomeEventMessage> {
    let mut requests = Column::new();

    if let Some(project) = page.projects.active() {
        for (_, reqs) in project.requests {
            for req in reqs {
                let container: Container<'static, HomeEventMessage> = HttpBadgeColumn {
                    label: req.url,
                    on_click: HomeEventMessage::ToggleSidebar,
                    on_duplicate: HomeEventMessage::ToggleSidebar,
                    on_remove: HomeEventMessage::ToggleSidebar,
                    method: req.method,
                }
                .into();

                requests = requests.push(container);
            }
        }
    }

    requests.into()
}
