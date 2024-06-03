use iced::widget::svg::Handle;
use iced::widget::{
    button, column, container, mouse_area, pick_list, row, svg, text, Column, Space,
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
use crate::utils::request::{FalconResponse, RequestBuilder};
use crate::{create_tabs, ui::elements::tabs::TabNode};

mod url_input_bar;

pub struct HomePage {
    theme: Option<AppTheme>,
    url: String,
    method: Method,
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
            url: Default::default(),
            request_tabs: Tabs::new(
                vec!["Query", "Header", "Body", "Authorization", "Cookies"],
                "Query",
            ),
            response_tabs: Tabs::new(vec!["Header", "Body", "Cookies"], "Body"),
            projects: Projects::new(),
            response: None,
            is_requesting: false,
            method: Method::GET
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
                self.url = url;
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
                    send_request(self.url.clone()),
                    |response| match response {
                        Ok(res) => HomeEventMessage::RequestFinished(res),
                        Err(err) => HomeEventMessage::RequestErr(err.to_string())
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
        let mut req_tab = Column::new();

        if let Some(tab) = self.request_tabs.get_active() {
            req_tab = req_tab.push(
                container(column![
                    text(format!("Inserted URL: {}", self.url)),
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

            req_tab = req_tab.push(Space::with_height(10));
        }

        let mut response_card = container("");

        if let Some(response) = self.response.clone() {
            let mut response_tab = Column::new();

            response_tab = response_tab.push(row![
                text("Response"),
                Space::with_width(Length::Fill),
                text("Status: "),
                text(response.status_code),
                Space::with_width(10),
                text(response.duration),
                Space::with_width(10),
                text(format!("Size: {}kb", response.size_kb)),
            ]);

            response_tab = response_tab.push(
                container("")
                    .width(Length::Fill)
                    .height(1)
                    .style(AppContainer::Hr),
            );

            response_tab = response_tab.push(create_tabs!(
                self.response_tabs,
                HomeEventMessage::OnResponseTabChange,
                None,
                None
            ));

            response_tab = response_tab.push(container(text(response.body)).padding(10));

            response_card = container(response_tab)
                .height(Length::Fill)
                .padding(10)
                .width(Length::Fill)
                .style(AppContainer::Rounded);
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
                    url_input_bar(&self.url, self.is_requesting),
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
                        )
                    },
                    container("")
                        .width(Length::Fill)
                        .height(1)
                        .style(AppContainer::Hr),
                    Space::with_height(10),
                    req_tab,
                    response_card,
                ]
                .padding(24.0)
            ],
        ]
        .into()
    }
}

async fn send_request(url: String) -> anyhow::Result<FalconResponse> {
    let builder = RequestBuilder::new().url(url).build();
    builder.send().await
}
