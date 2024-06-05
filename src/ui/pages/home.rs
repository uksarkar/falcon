use iced::widget::svg::Handle;
use iced::widget::{
    button, column, container, mouse_area, pick_list, row, svg, text, Column, Space,
};
use iced::{Application, Command, Element, Length, Padding, Theme};
use request_tabs_block::request_tab_container;
use response_tabs_block::response_tab_container;
use url_input_bar::url_input_bar;
use uuid::Uuid;

use crate::constants::{APP_LOGO, LAYOUT_CLOSED_SVG, LAYOUT_OPENED_SVG, PEN_CLIP_SVG};
use crate::ui::app_component::AppComponent;
use crate::ui::app_theme::{AppBtn, AppContainer, AppSelect, AppTheme};
use crate::ui::elements::tabs::Tabs;
use crate::ui::message_bus::Route;
use crate::utils::db::{Project, Projects};
use crate::utils::helpers::page_title;
use crate::utils::request::{FalconResponse, HttpMethod, PendingRequest, PendingRequestItem};
use crate::{create_tabs, ui::elements::tabs::TabNode};

mod request_tabs_block;
mod response_tabs_block;
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
            _ => None,
        };

        if let Some(cmd) = command {
            return cmd;
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut conditional_container = Column::new();
        let (folder, pending_request) = self.pending_request();

        if let Some(tab) = self.request_tabs.get_active() {
            conditional_container =
                conditional_container.push(request_tab_container(&tab.label, &pending_request));
        }

        if let Some(response) = self.response.clone() {
            conditional_container = conditional_container
                .push(Space::with_height(10))
                .push(response_tab_container(response, &self.response_tabs));
        }

        let sidebar = if self.sidebar_closed {
            container("")
        } else {
            container(column![
                container(row![
                    text(folder),
                    Space::with_width(Length::Fill),
                    button(
                        svg(Handle::from_memory(
                            include_bytes!("../../../assets/pen-clip.svg").as_slice()
                        ))
                        .width(20)
                        .height(20)
                    )
                    .style(AppBtn::Basic)
                    .padding(5)
                    .on_press(HomeEventMessage::ToggleSidebar),
                ])
                .style(AppContainer::FlatSecondary)
                .padding(Padding::from([5, 0])),
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
            .width(350)
        };

        column![
            container(
                row![
                    container(svg(Handle::from_memory(APP_LOGO)).width(30).height(30))
                        .padding(Padding::from([0.0, 10.0])),
                    button(
                        svg(Handle::from_memory(if self.sidebar_closed {
                            LAYOUT_OPENED_SVG
                        } else {
                            LAYOUT_CLOSED_SVG
                        }))
                        .width(20)
                        .height(20)
                    )
                    .style(AppBtn::Basic)
                    .padding(5)
                    .on_press(HomeEventMessage::ToggleSidebar),
                    Space::with_width(10),
                    pick_list(
                        self.projects.into_options(),
                        self.projects.selected_project(),
                        |item| { HomeEventMessage::OnProjectChange(item.value) }
                    )
                    .style(AppSelect::Card),
                    Space::with_width(10),
                    button(
                        svg(Handle::from_memory(PEN_CLIP_SVG))
                        .width(20)
                        .height(20)
                    )
                    .style(AppBtn::Basic)
                    .padding(5)
                    .on_press(HomeEventMessage::ToggleSidebar),
                    Space::with_width(10),
                    button("New")
                        .style(AppBtn::Secondary)
                        .padding(Padding::from([5, 15]))
                        .on_press(HomeEventMessage::NewProject("Something new".to_string())),
                ]
                .padding(8.0)
                .align_items(iced::Alignment::Center)
            )
            .width(Length::Fill)
            .style(AppContainer::Flat),
            container("")
                .width(Length::Fill)
                .height(1)
                .style(AppContainer::Hr),
            row![
                sidebar,
                column![
                    url_input_bar(
                        &pending_request.url,
                        self.is_requesting,
                        &pending_request.method
                    ),
                    Space::with_height(10),
                    match self.response {
                        Some(_) => create_tabs!(
                            self.request_tabs,
                            HomeEventMessage::OnRequestTabChange,
                            Some(HomeEventMessage::MinimizeRequestTabs),
                            Some(if self.request_tabs.is_active() {
                                container("-")
                                    .padding(Padding::from([5, 10]))
                                    .style(AppContainer::Outlined)
                            } else {
                                container("+")
                                    .padding(Padding::from([5, 10]))
                                    .style(AppContainer::Outlined)
                            })
                        ),
                        None => create_tabs!(
                            self.request_tabs,
                            HomeEventMessage::OnRequestTabChange,
                            None,
                            None
                        ),
                    },
                    container("")
                        .width(Length::Fill)
                        .height(1)
                        .style(AppContainer::Hr),
                    Space::with_height(10),
                    conditional_container,
                ]
                .padding(24.0)
            ],
        ]
        .into()
    }
}

async fn send_request(pending_request: PendingRequest) -> anyhow::Result<FalconResponse> {
    pending_request.send().await
}
