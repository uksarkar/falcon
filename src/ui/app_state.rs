use iced::{Element, Sandbox};

use super::{
    app_component::AppComponent,
    app_theme::AppTheme,
    message_bus::{MessageBus, Route},
    pages::{
        about::AboutPage,
        home::{HomeEventMessage, HomePage},
    },
};

pub struct AppState {
    theme: AppTheme,
    route: Route,
    home_state: HomePage,
    profile_state: AboutPage,
}

impl AppState {
    fn new() -> Self {
        AppState {
            theme: AppTheme::default(),
            route: Route::Home,
            home_state: HomePage::new(),
            profile_state: AboutPage::new(),
        }
    }

    fn update(&mut self, message: MessageBus) {
        match message {
            MessageBus::NavigateTo(route) => {
                self.route = route;
            }
            MessageBus::HomeMessage(msg) => match msg {
                HomeEventMessage::NavigateTo(route) => self.route = route,
                _ => self.home_state.update(msg),
            },
            MessageBus::SetTheme(theme) => {
                self.theme = theme.clone();
                self.home_state.set_theme(theme);
            }
        }
    }

    fn view(&self) -> Element<MessageBus> {
        match self.route {
            Route::Home => self.home_state.view().map(MessageBus::HomeMessage),
            Route::Profile => self.profile_state.view(),
        }
    }
}

impl Sandbox for AppState {
    type Message = MessageBus;

    fn theme(&self) -> iced::Theme {
        self.theme.theme()
    }

    fn new() -> Self {
        AppState::new()
    }

    fn title(&self) -> String {
        match self.route {
            Route::Home => self.home_state.title(),
            Route::Profile => self.profile_state.title(),
        }
    }

    fn update(&mut self, message: Self::Message) {
        self.update(message);
    }

    fn view(&self) -> Element<Self::Message> {
        self.view()
    }
}
