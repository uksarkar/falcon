use super::{
    app_theme::AppTheme,
    pages::home::HomeEventMessage
};

#[derive(Debug, Clone, Copy)]
pub enum Route {
    Home,
    Profile,
}

#[derive(Debug, Clone)]
pub enum MessageBus {
    NavigateTo(Route),
    HomeMessage(HomeEventMessage),
    SetTheme(AppTheme),
}