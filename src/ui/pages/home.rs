use std::thread::sleep;
use std::time::{Duration, Instant};

use env_tabs_block::env_tabs_block;
use iced::widget::text_editor::Action;
use iced::widget::{column, container, mouse_area, row, text, text_editor, Row, Space};
use iced::{Application, Command, Element, Length, Theme};
use project_tabs_block::project_tabs_block;
use request_and_response_card::request_and_response_card;
use sidebar_envs::get_env_items;
use sidebar_projects::get_sidebar_projects_items;
use sidebar_requests::sidebar_requests;
use tob_bar::tob_bar;
use uuid::Uuid;

// use crate::ui::app_component::AppComponent;
use crate::ui::app_theme::AppContainer;
use crate::ui::elements::tabs::TabNode;
use crate::ui::elements::tabs::Tabs;
use crate::ui::message_bus::Route;
use crate::utils::db::{Env, Project, Projects};
use crate::utils::helpers::page_title;
use crate::utils::request::{
    FalconAuthorization, FalconResponse, FlBody, HttpMethod, PendingRequest, PendingRequestItem,
};

mod env_tabs_block;
mod http_badge_column;
mod key_and_value_input_row;
mod project_tabs_block;
mod request_and_response_card;
mod request_tabs_block;
mod response_tabs_block;
mod sidebar_envs;
mod sidebar_item;
mod sidebar_projects;
mod sidebar_requests;
mod tob_bar;
mod url_input_bar;

#[derive(Default, Debug, Clone)]
pub enum HomePageState {
    #[default]
    Requests,
    Projects,
    Envs,
}

pub struct HomePage {
    // theme: Option<AppTheme>,
    request_tabs: Tabs,
    response_tabs: Tabs,
    projects: Projects,
    response: Option<FalconResponse>,
    is_requesting: bool,
    sidebar_closed: bool,
    state: HomePageState,
    request_body_context: text_editor::Content,
    scheduled_sync_at: Instant,
    show_env_examples: bool,
}

impl Default for HomePage {
    fn default() -> Self {
        Self {
            // theme: Default::default(),
            sidebar_closed: Default::default(),
            state: Default::default(),
            request_tabs: Tabs::new(
                vec!["Query", "Header", "Body", "Authorization", "Cookies"],
                "Query",
            ),
            response_tabs: Tabs::new(vec!["Header", "Body", "Cookies"], "Body"),
            projects: Projects::new(),
            is_requesting: false,
            response: None,
            request_body_context: text_editor::Content::new(),
            scheduled_sync_at: Instant::now(),
            show_env_examples: true,
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
    OnRequestNameInput(String),
    RemoveRequestItem(PendingRequestItem, usize),
    ToggleSidebar,
    AddNewRequest(PendingRequest),
    SelectRequest(Uuid),
    DeleteRequest(Uuid),
    OnChangePageState(HomePageState),
    OnProjectNameInput(String),
    OnProjectBaseUrlInput(String),
    OnProjectRemove(Uuid),
    OnProjectDuplicate(Uuid),
    OnEnvSelect(Uuid),
    OnEnvDuplicate(Uuid),
    OnEnvDelete(Uuid),
    OnEnvItemKeyInput(usize, String),
    OnEnvItemValueInput(usize, String),
    OnEnvItemRemove(usize),
    OnEnvNameInput(String),
    OnEnvAdd,
    OnProjectDefaultEnvSelect(Option<Uuid>),
    OnAuthorizationTabChange(TabNode),
    OnAuthorizationInput(FalconAuthorization),
    OnBodyTabChange(TabNode),
    OnRequestBodyContextAction(Action),
    SyncProjects,
    SyncedDone,
    ToggleEnvExample,
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

    fn schedule_sync(&mut self) -> Command<HomeEventMessage> {
        self.scheduled_sync_at = Instant::now();
        Command::perform(
            async {
                sleep(Duration::from_millis(500));
                HomeEventMessage::SyncProjects
            },
            |msg| msg,
        )
    }

    fn perform_sync(&self) -> Command<HomeEventMessage> {
        if Instant::now().duration_since(self.scheduled_sync_at) > Duration::from_millis(500) {
            let projects = self.projects.clone();

            return Command::perform(
                async move {
                    match projects.sync() {
                        Ok(_) => {}
                        Err(err) => println!("{:<10}: Failed to sync, {:?}", "DB", err),
                    }
                    HomeEventMessage::SyncedDone
                },
                |msg| msg,
            );
        }

        Command::none()
    }
}

// impl AppComponent for HomePage {
// fn app_theme(&self) -> crate::ui::app_theme::AppTheme {
//     if let Some(theme) = self.theme.clone() {
//         return theme;
//     }

//     AppTheme::Light
// }

// fn set_theme(&mut self, theme: AppTheme) {
//     self.theme = Some(theme);
// }
// }

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
        match message {
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
                Some(self.schedule_sync())
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
                self.projects.add(Project {
                    name,
                    is_active: true,
                    ..Default::default()
                });
                self.state = HomePageState::Projects;
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnProjectChange(id) => {
                self.projects.set_active(&id);
                Some(self.schedule_sync())
            }
            HomeEventMessage::SendRequest => {
                if let Some(project) = self.projects.active() {
                    if let Some((_, req)) = project.current_request() {
                        self.is_requesting = true;
                        let env = self.projects.active_env().unwrap_or_default();
                        let request = req.clone();
                        return Command::perform(
                            async move { request.send(&env).await },
                            |response| match response {
                                Ok(res) => HomeEventMessage::RequestFinished(res),
                                Err(err) => HomeEventMessage::RequestErr(err.to_string()),
                            },
                        );
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
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnRequestItemValueInput(item, index, val) => {
                if let Some(project) = self.projects.active_mut() {
                    project.update_request_item(item, index, val, false);
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::RemoveRequestItem(item, index) => {
                if let Some(project) = self.projects.active_mut() {
                    if let Some(req) = project.current_request_mut() {
                        req.remove_item(item, index);
                    }
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnRequestMethodChanged(method) => {
                if let Some(project) = self.projects.active_mut() {
                    project.update_request_method(method);
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::AddNewRequest(req) => {
                if let Some(project) = self.projects.active_mut() {
                    project.add_request("root".into(), req);
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::SelectRequest(id) => {
                if let Some(project) = self.projects.active_mut() {
                    project.set_current_request(id)
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::DeleteRequest(id) => {
                if let Some(project) = self.projects.active_mut() {
                    project.remove_request("root".into(), id);
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnRequestNameInput(name) => {
                if let Some(project) = self.projects.active_mut() {
                    if let Some(req) = project.current_request_mut() {
                        req.name = Some(name);
                    }
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnChangePageState(state) => {
                self.state = state;
                None
            }
            HomeEventMessage::OnProjectNameInput(name) => {
                if let Some(project) = self.projects.active_mut() {
                    project.name = name;
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnProjectBaseUrlInput(url) => {
                if let Some(project) = self.projects.active_mut() {
                    project.base_url = Some(url);
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnProjectDuplicate(id) => {
                if let Some(project) = self.projects.duplicate_project(id) {
                    self.projects.set_active(&project.id);
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnProjectRemove(id) => {
                self.projects.delete_project(id);
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnEnvDelete(id) => {
                self.projects.delete_env(id);
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnEnvDuplicate(id) => {
                self.projects.duplicate_env(id);
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnEnvItemKeyInput(index, key) => {
                if let Some(env) = self.projects.active_env_mut() {
                    env.update_item_key(index, key);
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnEnvItemRemove(index) => {
                if let Some(env) = self.projects.active_env_mut() {
                    env.remove_item(index);
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnEnvItemValueInput(index, value) => {
                if let Some(env) = self.projects.active_env_mut() {
                    env.update_item_value(index, value);
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnEnvSelect(id) => {
                self.projects.set_active_env(id);
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnEnvNameInput(name) => {
                if let Some(env) = self.projects.active_env_mut() {
                    env.name = name;
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnProjectDefaultEnvSelect(id) => {
                if let Some(env_id) = id.clone() {
                    self.projects.set_active_env(env_id);
                }
                if let Some(project) = self.projects.active_mut() {
                    project.default_env = id;
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnEnvAdd => {
                let env = Env::default();

                self.projects.add_env(env.clone());
                self.projects.set_active_env(env.id);
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnAuthorizationInput(auth) => {
                if let Some(project) = self.projects.active_mut() {
                    if let Some(req) = project.current_request_mut() {
                        req.set_auth(auth);
                    }
                }
                Some(self.schedule_sync())
            }
            HomeEventMessage::OnRequestBodyContextAction(action) => {
                self.request_body_context.perform(action);

                if let Some(project) = self.projects.active_mut() {
                    if let Some(req) = project.current_request_mut() {
                        req.set_body(FlBody::ApplicationJson(self.request_body_context.text()));
                    }
                }

                Some(self.schedule_sync())
            }
            HomeEventMessage::ToggleEnvExample => {
                self.show_env_examples = !self.show_env_examples;
                None
            }
            HomeEventMessage::SyncProjects => Some(self.perform_sync()),
            HomeEventMessage::SyncedDone => {
                println!("{:<10}: Synced to local file", "DB");
                None
            }
            HomeEventMessage::OnAuthorizationTabChange(_) => None,
            HomeEventMessage::OnBodyTabChange(_) => None,
            HomeEventMessage::NavigateTo(_) => None,
        }
        .unwrap_or(Command::none())
    }

    fn view(&self) -> Element<Self::Message> {
        let mut base_row = Row::new();

        // handle sidebar based on the page state
        if !self.sidebar_closed {
            let active_sidebar_items = match self.state {
                HomePageState::Requests => sidebar_requests(self),
                HomePageState::Projects => get_sidebar_projects_items(self),
                HomePageState::Envs => get_env_items(self),
            };

            base_row = base_row.push(
                container(column![
                    active_sidebar_items,
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

        // handle contents based on the page state
        match self.state {
            HomePageState::Requests => {
                base_row = base_row.push(container(request_and_response_card(self)).padding(10));
            }
            HomePageState::Projects => {
                base_row = base_row.push(project_tabs_block(self));
            }
            HomePageState::Envs => {
                base_row = base_row.push(env_tabs_block(self.projects.active_env(), self.show_env_examples));
            }
        }

        // build main view here
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
