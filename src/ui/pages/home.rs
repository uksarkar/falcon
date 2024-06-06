use iced::widget::svg::Handle;
use iced::widget::{button, column, container, mouse_area, row, svg, text, tooltip, Row, Space};
use iced::{Application, Command, Element, Length, Theme};
use request_and_response_card::request_and_response_card;
use sidebar_requests::sidebar_requests;
use tob_bar::tob_bar;
use uuid::Uuid;

use crate::constants::{ADD_DOC_SVG, COG_API_SVG};
use crate::ui::app_component::AppComponent;
use crate::ui::app_theme::{AppBtn, AppColor, AppContainer, AppTheme};
use crate::ui::elements::tabs::TabNode;
use crate::ui::elements::tabs::Tabs;
use crate::ui::message_bus::Route;
use crate::utils::db::{Project, Projects};
use crate::utils::helpers::page_title;
use crate::utils::request::{FalconResponse, HttpMethod, PendingRequest, PendingRequestItem};

mod http_badge_column;
mod request_and_response_card;
mod request_tabs_block;
mod response_tabs_block;
mod sidebar_envs;
mod sidebar_projects;
mod sidebar_requests;
mod tob_bar;
mod url_input_bar;

pub struct HomePage {
    theme: Option<AppTheme>,
    request_tabs: Tabs,
    response_tabs: Tabs,
    projects: Projects,
    response: Option<FalconResponse>,
    is_requesting: bool,
    sidebar_closed: bool,
}

impl Default for HomePage {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            sidebar_closed: Default::default(),
            request_tabs: Tabs::new(
                vec!["Query", "Header", "Body", "Authorization", "Cookies"],
                "Query",
            ),
            response_tabs: Tabs::new(vec!["Header", "Body", "Cookies"], "Body"),
            projects: Projects::new(),
            is_requesting: false,
            response: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum HomeEventMessage {
    NavigateTo(Route),
    UrlInput(String),
    OnRequestTabChange(TabNode),
    OnResponseTabChange(TabNode),
    MinimizeRequestTabs,
    SendRequest,
    NewProject(String),
    OnProjectChange(Uuid),
    RequestFinished(FalconResponse),
    RequestErr(String),
    OnRequestMethodChanged(HttpMethod),
    OnRequestItemKeyInput(PendingRequestItem, usize, String),
    OnRequestItemValueInput(PendingRequestItem, usize, String),
    RemoveRequestItem(PendingRequestItem, usize),
    ToggleSidebar,
    AddNewRequest(PendingRequest),
    SelectRequest(Uuid),
    DeleteRequest(Uuid),
}

impl HomePage {
    fn pending_request(&self) -> (String, PendingRequest) {
        if let Some(current) = self
            .projects
            .active()
            .and_then(|p| p.current_request().map(|(s, r)| (s.clone(), r.clone())))
        {
            (current.0.clone(), current.1.clone())
        } else {
            ("root".to_string(), PendingRequest::default())
        }
    }
}

impl AppComponent for HomePage {
    fn app_theme(&self) -> crate::ui::app_theme::AppTheme {
        if let Some(theme) = self.theme.clone() {
            return theme;
        }

        AppTheme::Light
    }

    fn set_theme(&mut self, theme: AppTheme) {
        self.theme = Some(theme);
    }
}

impl Application for HomePage {
    type Message = HomeEventMessage;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (HomePage, Command<Self::Message>) {
        (HomePage::default(), Command::none())
    }

    fn title(&self) -> String {
        page_title("Home")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let command: Option<Command<Self::Message>> = match message {
            HomeEventMessage::ToggleSidebar => {
                self.sidebar_closed = !self.sidebar_closed;
                None
            }
            HomeEventMessage::UrlInput(url) => {
                if let Some(project) = self.projects.active_mut() {
                    project.update_request_url(url);
                }
                self.response = None;
                self.request_tabs.activate();
                None
            }
            HomeEventMessage::OnRequestTabChange(node) => {
                self.request_tabs.set_active(&node.label);
                self.request_tabs.activate();
                None
            }
            HomeEventMessage::OnResponseTabChange(node) => {
                self.response_tabs.set_active(&node.label);
                None
            }
            HomeEventMessage::MinimizeRequestTabs => {
                self.request_tabs.toggle_activation();
                None
            }
            HomeEventMessage::NewProject(name) => {
                match self.projects.add(Project {
                    name,
                    is_active: true,
                    ..Default::default()
                }) {
                    Ok(()) => (),
                    Err(e) => println!("{:<8}{}", "DB: ", e),
                };
                None
            }
            HomeEventMessage::OnProjectChange(id) => {
                self.projects.set_active(&id);
                None
            }
            HomeEventMessage::SendRequest => {
                if let Some(project) = self.projects.active() {
                    if let Some(req) = project.current_request() {
                        self.is_requesting = true;
                        return Command::perform(send_request(req.1.clone()), |response| {
                            match response {
                                Ok(res) => HomeEventMessage::RequestFinished(res),
                                Err(err) => HomeEventMessage::RequestErr(err.to_string()),
                            }
                        });
                    }
                }

                None
            }
            HomeEventMessage::RequestFinished(res) => {
                self.response = Some(res);
                self.is_requesting = false;
                None
            }
            HomeEventMessage::RequestErr(msg) => {
                self.is_requesting = false;
                println!("Request failed: {}", msg);
                None
            }
            HomeEventMessage::OnRequestItemKeyInput(item, index, name) => {
                if let Some(project) = self.projects.active_mut() {
                    project.update_request_item(item, index, name, true);
                }
                None
            }
            HomeEventMessage::OnRequestItemValueInput(item, index, val) => {
                if let Some(project) = self.projects.active_mut() {
                    project.update_request_item(item, index, val, false);
                }
                None
            }
            HomeEventMessage::RemoveRequestItem(item, index) => {
                if let Some(project) = self.projects.active_mut() {
                    if let Some(req) = project.current_request_mut() {
                        req.remove_item(item, index);
                    }
                }
                None
            }
            HomeEventMessage::OnRequestMethodChanged(method) => {
                if let Some(project) = self.projects.active_mut() {
                    project.update_request_method(method);
                }
                None
            }
            HomeEventMessage::AddNewRequest(req) => {
                if let Some(project) = self.projects.active_mut() {
                    project.add_request("root".into(), req);
                }
                None
            }
            HomeEventMessage::SelectRequest(id) => {
                if let Some(project) = self.projects.active_mut() {
                    project.set_current_request(id)
                }
                None
            }
            HomeEventMessage::DeleteRequest(id) => {
                if let Some(project) = self.projects.active_mut() {
                    project.remove_request("root".into(), id);
                }
                None
            }
            _ => None,
        };

        if let Some(cmd) = command {
            return cmd;
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut base_row = Row::new();

        if !self.sidebar_closed {
            base_row = base_row.push(
                container(column![
                    container(row![
                        text("Default env").size(14),
                        Space::with_width(Length::Fill),
                        tooltip(
                            button(svg(Handle::from_memory(COG_API_SVG)).width(15).height(15))
                                .style(AppBtn::Basic)
                                .padding(3)
                                .on_press(HomeEventMessage::AddNewRequest(
                                    PendingRequest::default()
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
                                .on_press(HomeEventMessage::AddNewRequest(
                                    PendingRequest::default()
                                )),
                            container(text("New request").size(10))
                                .style(AppContainer::Bg(AppColor::BG_DARKEST))
                                .padding(4),
                            tooltip::Position::FollowCursor
                        ),
                    ])
                    .style(AppContainer::FlatSecondary)
                    .padding(2),
                    container("")
                        .style(AppContainer::Bg(AppColor::BG_DARKER))
                        .height(1)
                        .width(Length::Fill),
                    sidebar_requests(self),
                    Space::with_height(Length::Fill),
                    row![
                        Space::with_width(Length::Fill),
                        mouse_area(text(env!("CARGO_PKG_VERSION")))
                            .interaction(iced::mouse::Interaction::Pointer)
                            .on_press(HomeEventMessage::NavigateTo(Route::Profile)),
                        Space::with_width(Length::Fill)
                    ]
                    .width(iced::Length::Fill),
                ])
                .style(AppContainer::Flat)
                .height(Length::Fill)
                .width(350),
            );
        };

        base_row = base_row.push(container(request_and_response_card(self)).padding(10));

        column![
            tob_bar(
                self.projects.into_options(),
                self.projects.selected_project(),
                self.sidebar_closed,
            ),
            base_row
        ]
        .into()
    }
}

async fn send_request(pending_request: PendingRequest) -> anyhow::Result<FalconResponse> {
    pending_request.send().await
}
