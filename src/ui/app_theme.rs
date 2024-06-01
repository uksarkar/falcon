use iced::{
    theme::Palette,
    widget::{button, container, text_input},
    Border, Color, Theme,
};

use crate::utils::color::darken_color;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AppTheme {
    #[default]
    Light,
    Dark,
}

impl AppTheme {
    pub fn theme(&self) -> Theme {
        Theme::custom("Falcon".to_string(), self.palette())
    }
    pub fn palette(&self) -> Palette {
        match self {
            AppTheme::Light => Palette {
                background: Color::from_rgba8(223, 220, 212, 1.0),
                text: Color::from_rgba8(1, 5, 13, 1.0),
                primary: Color::from_rgba8(72, 74, 86, 1.0),
                success: Color::from_rgba8(24, 172, 0, 1.0),
                danger: Color::from_rgba8(172, 0, 0, 1.0),
            },
            AppTheme::Dark => todo!(),
        }
    }
}

pub enum AppBtn {
    Primary,
    Secondary,
}

impl button::StyleSheet for AppBtn {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match self {
            AppBtn::Primary => button::Appearance {
                border: Border {
                    color: style.palette().primary,
                    radius: 5.into(),
                    width: 1.0,
                },
                text_color: style.palette().background,
                background: Some(iced::Background::Color(style.palette().primary)),
                ..Default::default()
            },
            AppBtn::Secondary => button::Appearance {
                border: Border {
                    color: style.palette().primary,
                    radius: 5.into(),
                    width: 1.0,
                },
                text_color: style.palette().primary,
                background: Some(iced::Background::Color(Color::TRANSPARENT)),
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match self {
            AppBtn::Primary => button::Appearance {
                background: Some(iced::Background::Color(
                    style.extended_palette().primary.strong.color,
                )),
                ..self.active(style)
            },
            AppBtn::Secondary => button::Appearance {
                text_color: style.palette().background,
                background: Some(iced::Background::Color(style.palette().primary)),
                ..self.active(style)
            },
        }
    }
}

impl Into<iced::theme::Button> for AppBtn {
    fn into(self) -> iced::theme::Button {
        iced::theme::Button::custom(self)
    }
}

pub enum AppContainer {
    Rounded,
    Flat,
    Hr,
    SuccessIndicator,
}

impl container::StyleSheet for AppContainer {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let border = match self {
            AppContainer::Flat => Border::default(),
            AppContainer::SuccessIndicator => Border::with_radius(2.0),
            _ => Border::with_radius(5.0),
        };

        let background = match self {
            AppContainer::Hr => Some(iced::Background::Color(style.palette().text)),
            AppContainer::SuccessIndicator => {
                Some(iced::Background::Color(style.palette().success))
            }
            _ => Some(iced::Background::Color(Color::from_rgb(
                237.0, 233.0, 220.0,
            ))),
        };

        container::Appearance {
            background,
            border,
            ..Default::default()
        }
    }
}

impl Into<iced::theme::Container> for AppContainer {
    fn into(self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(self))
    }
}

pub struct AppInput;

impl text_input::StyleSheet for AppInput {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(darken_color(
                Color::from_rgb(237.0, 233.0, 220.0),
                2,
            )),
            border: Border::default(),
            icon_color: style.palette().primary,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(darken_color(
                Color::from_rgb(237.0, 233.0, 220.0),
                5,
            )),
            ..self.active(style)
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        darken_color(Color::from_rgb(237.0, 233.0, 220.0), 30)
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        style.palette().text
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        style.palette().background
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        style.extended_palette().primary.strong.color
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let color = style.extended_palette().primary.weak.color;

        text_input::Appearance {
            background: iced::Background::Color(color.clone()),
            border: Border::default(),
            icon_color: color,
        }
    }
}

impl Into<iced::theme::TextInput> for AppInput {
    fn into(self) -> iced::theme::TextInput {
        iced::theme::TextInput::Custom(Box::new(self))
    }
}
