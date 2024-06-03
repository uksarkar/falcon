use iced::{Application, Command, Element, Theme};

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
            home_state: HomePage::new(()).0,
            profile_state: AboutPage::new(()).0,
        }
    }

    fn update(&mut self, message: MessageBus) -> Command<MessageBus> {
        match message {
            MessageBus::NavigateTo(route) => {
                self.route = route;
                Command::none()
            }
            MessageBus::HomeMessage(msg) => match msg {
                HomeEventMessage::NavigateTo(route) => {
                    self.route = route;
                    Command::none()
                },
                _ => self.home_state.update(msg).map(MessageBus::HomeMessage),
            },
            MessageBus::SetTheme(theme) => {
                self.theme = theme.clone();
                self.home_state.set_theme(theme);
                Command::none()
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

impl Application for AppState {
    type Message = MessageBus;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (AppState, Command<Self::Message>) {
        (AppState::new(), Command::none())
    }

    fn theme(&self) -> iced::Theme {
        self.theme.theme()
    }

    fn title(&self) -> String {
        match self.route {
            Route::Home => self.home_state.title(),
            Route::Profile => self.profile_state.title(),
        }
    }

    fn update(&mut self, message: Self::Message) -> Command::<Self::Message> {
        self.update(message)
    }

    fn view(&self) -> Element<Self::Message> {
        self.view()
    }
}
