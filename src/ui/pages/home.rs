use chrono::{DateTime, Utc};
use iced::widget::svg::Handle;
use iced::widget::{
    button, column, container, mouse_area, pick_list, row, scrollable, svg, text, Column, Space,
};
use iced::{Application, Command, Element, Length, Padding, Theme};
use reqwest::Method;
use url_input_bar::url_input_bar;

use crate::ui::app_component::AppComponent;
use crate::ui::app_theme::{AppBtn, AppContainer, AppSelect, AppTheme};
use crate::ui::elements::tabs::Tabs;
use crate::ui::message_bus::Route;
use crate::utils::db::{Project, Projects};
use crate::utils::helpers::page_title;
use crate::utils::request::{FalconResponse, PendingRequest};
use crate::{create_tabs, ui::elements::tabs::TabNode};

mod url_input_bar;

pub struct HomePage {
    theme: Option<AppTheme>,
    pending_request: PendingRequest,
    request_tabs: Tabs,
    response_tabs: Tabs,
    projects: Projects,
    response: Option<FalconResponse>,
    is_requesting: bool,
}

impl Default for HomePage {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            request_tabs: Tabs::new(
                vec!["Query", "Header", "Body", "Authorization", "Cookies"],
                "Query",
            ),
            response_tabs: Tabs::new(vec!["Header", "Body", "Cookies"], "Body"),
            projects: Projects::new(),
            pending_request: PendingRequest {
                url: "https://".to_string(),
                ..Default::default()
            },
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
    OnProjectChange(Project),
    RequestFinished(FalconResponse),
    RequestErr(String),
    OnRequestMethodChanged(Method),
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
            HomeEventMessage::UrlInput(url) => {
                self.pending_request.set_url(url);
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
            HomeEventMessage::OnProjectChange(project) => {
                self.projects.set_active(&project.id);
                None
            }
            HomeEventMessage::SendRequest => {
                self.is_requesting = true;
                Some(Command::perform(
                    send_request(self.pending_request.clone()),
                    |response| match response {
                        Ok(res) => HomeEventMessage::RequestFinished(res),
                        Err(err) => HomeEventMessage::RequestErr(err.to_string()),
                    },
                ))
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
            _ => None,
        };

        if let Some(cmd) = command {
            return cmd;
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut conditional_container = Column::new();

        if let Some(tab) = self.request_tabs.get_active() {
            conditional_container = conditional_container.push(
                container(column![
                    text(format!("Inserted URL: {}", self.pending_request.url)),
                    match tab.label.as_str() {
                        "Query" => container(text("This is for Query tab")),
                        "Header" => container(text("This is for Header")),
                        "Body" => container(text("This is for Body")),
                        "Authorization" => container(text("This is for Authorization")),
                        "Cookies" => container(text("This is for Cookies")),
                        _ => container("Unknown tab"),
                    }
                ])
                .padding(10)
                .height(Length::Fill)
                .width(Length::Fill)
                .style(AppContainer::Rounded),
            );
        }

        if let Some(response) = self.response.clone() {
            conditional_container = conditional_container.push(Space::with_height(10));

            let mut response_tab = Column::new()
                .push(row![
                    text("Response"),
                    Space::with_width(Length::Fill),
                    text("Status: "),
                    text(response.status_code),
                    Space::with_width(10),
                    text(response.duration),
                    Space::with_width(10),
                    text(format!("Size: {}kb", response.size_kb)),
                ])
                .push(
                    container("")
                        .width(Length::Fill)
                        .height(1)
                        .style(AppContainer::Hr),
                )
                .push(create_tabs!(
                    self.response_tabs,
                    HomeEventMessage::OnResponseTabChange,
                    None,
                    None
                ));

            let mut tab_container = Column::new();

            if let Some(tab) = self.response_tabs.get_active() {
                match tab.label.as_str() {
                    "Body" => {
                        tab_container = tab_container.push(
                            container(text(response.body))
                                .padding(10)
                                .width(Length::Fill),
                        );
                    }
                    "Header" => {
                        for (name, value) in response.headers {
                            let header_name = if let Some(header_name) = name {
                                format!("{}", header_name.as_str())
                            } else {
                                "Unknown".to_string()
                            };

                            let header_value = format!("{:?}", value);

                            if !header_name.trim().is_empty() && !header_value.trim().is_empty() {
                                tab_container = tab_container.push(
                                    container(column![
                                        container(row![
                                            text(header_name),
                                            Space::with_width(10),
                                            text(":"),
                                            Space::with_width(10),
                                            text(format!("{:?}", value))
                                        ])
                                        .width(Length::Fill)
                                        .padding(5),
                                        container("")
                                            .width(Length::Fill)
                                            .height(1)
                                            .style(AppContainer::Hr)
                                    ])
                                    .padding(5),
                                );
                            }
                        }
                    }
                    "Cookies" => {
                        for cookie in response.cookies {
                            tab_container = tab_container.push(
                                container(column![
                                    container(row![
                                        text(cookie.name),
                                        Space::with_width(10),
                                        text(":"),
                                        Space::with_width(10),
                                        text(format!(
                                            "{}, exp: {}, http_only: {}",
                                            if let Some(val) = cookie.value {
                                                val
                                            } else {
                                                "".to_string()
                                            },
                                            if let Some(val) = cookie.expires {
                                                let datetime: DateTime<Utc> = val.into();
                                                format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"))
                                            } else {
                                                "".to_string()
                                            },
                                            cookie.http_only
                                        ))
                                    ])
                                    .width(Length::Fill)
                                    .padding(5),
                                    container("")
                                        .width(Length::Fill)
                                        .height(1)
                                        .style(AppContainer::Hr)
                                ])
                                .padding(5),
                            );
                        }
                    }
                    _ => {}
                };
            };

            response_tab =
                response_tab.push(container(scrollable(tab_container)).width(Length::Fill));

            conditional_container = conditional_container.push(
                container(response_tab)
                    .height(Length::Fill)
                    .padding(10)
                    .width(Length::Fill)
                    .style(AppContainer::Rounded),
            );
        }

        column![
            container(
                row![
                    container(
                        svg(Handle::from_memory(
                            include_bytes!("../../../assets/Logo.svg").as_slice()
                        ))
                        .width(30)
                        .height(30)
                    )
                    .padding(Padding::from([0.0, 5.0])),
                    container("Layout").padding(Padding::from([0.0, 5.0])),
                    Space::with_width(5),
                    pick_list(self.projects.clone(), self.projects.active(), |proj| {
                        HomeEventMessage::OnProjectChange(proj)
                    })
                    .style(AppSelect::Card),
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
                container(column![
                    text("Side bar"),
                    text("list items"),
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
                column![
                    url_input_bar(&self.pending_request.url, self.is_requesting),
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
