use iced::widget::svg::Handle;
use iced::widget::{button, column, container, mouse_area, row, svg, text, Column, Space};
use iced::{Element, Length, Padding, Sandbox};
use url_input_bar::url_input_bar;

use crate::ui::app_component::AppComponent;
use crate::ui::app_theme::{AppBtn, AppContainer, AppTheme};
use crate::ui::elements::tabs::Tabs;
use crate::ui::message_bus::Route;
use crate::utils::helpers::page_title;
use crate::{create_tabs, ui::elements::tabs::TabNode};

mod url_input_bar;

pub struct HomePage {
    theme: Option<AppTheme>,
    url: String,
    request_tabs: Tabs,
    response_tabs: Tabs,
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
    NewProject,
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

impl Sandbox for HomePage {
    type Message = HomeEventMessage;

    fn new() -> Self {
        HomePage::default()
    }

    fn title(&self) -> String {
        page_title("Home")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            HomeEventMessage::UrlInput(url) => self.url = url,
            HomeEventMessage::OnRequestTabChange(node) => {
                self.request_tabs.set_active(&node.label);
                self.request_tabs.activate();
            }
            HomeEventMessage::OnResponseTabChange(node) => {
                self.response_tabs.set_active(&node.label);
            }
            HomeEventMessage::MinimizeRequestTabs => {
                self.request_tabs.toggle_activation();
            }
            _ => (),
        };
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
                .height(330)
                .width(Length::Fill)
                .style(AppContainer::Rounded),
            );

            req_tab = req_tab.push(Space::with_height(10));
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
                    container("Unknown project").padding(Padding::from([0.0, 5.0])),
                    button("New")
                        .style(AppBtn::Secondary)
                        .padding(Padding::from([5, 15]))
                        .on_press(HomeEventMessage::NewProject),
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
                    url_input_bar(&self.url),
                    Space::with_height(10),
                    create_tabs!(
                        self.request_tabs,
                        HomeEventMessage::OnRequestTabChange,
                        Some(HomeEventMessage::MinimizeRequestTabs),
                        Some(if self.request_tabs.is_active() {
                            container("-").padding(Padding::from([5, 10])).style(AppContainer::Outlined)
                        } else {
                            container("+").padding(Padding::from([5, 10])).style(AppContainer::Outlined)
                        })
                    ),
                    container("")
                        .width(Length::Fill)
                        .height(1)
                        .style(AppContainer::Hr),
                    Space::with_height(10),
                    req_tab,
                    container(column![create_tabs!(
                        self.response_tabs,
                        HomeEventMessage::OnResponseTabChange,
                        None,
                        None
                    ),])
                    .height(Length::Fill)
                    .padding(10)
                    .width(Length::Fill)
                    .style(AppContainer::Rounded),
                ]
                .padding(24.0)
            ],
        ]
        .into()
    }
}
