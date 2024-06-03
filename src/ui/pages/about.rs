use iced::widget::{button, column, container, row, text, Space};
use iced::{Application, Command, Element, Length, Padding, Theme};

use crate::ui::app_theme::{AppBtn, AppContainer};
use crate::ui::message_bus::{MessageBus, Route};
use crate::utils::helpers::page_title;

#[derive(Default)]
pub struct AboutPage;

impl Application for AboutPage {
    type Message = MessageBus;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (AboutPage, Command<Self::Message>) {
        (AboutPage::default(), Command::none())
    }

    fn title(&self) -> String {
        page_title("About")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {Command::none()}

    fn view(&self) -> Element<Self::Message> {
        column![
            Space::with_height(Length::Fill),
            row![
                Space::with_width(Length::Fill),
                container(column![
                    text("App name: Falcon"),
                    text("Repo: https://github.com/uksarkar/falcon"),
                    text(format!("Version: {}", env!("CARGO_PKG_VERSION"))),
                    text("Author: Utpal Sarkar"),
                    Space::with_height(15),
                    row![
                        Space::with_width(Length::Fill),
                        button("Go Back")
                            .padding(Padding::from([5, 15]))
                            .style(AppBtn::Primary)
                            .on_press(MessageBus::NavigateTo(Route::Home)),
                        Space::with_width(Length::Fill)
                    ]
                ])
                .padding(15)
                .width(500)
                .style(AppContainer::Rounded),
                Space::with_width(Length::Fill),
            ],
            Space::with_height(Length::Fill),
        ]
        .into()
    }
}
