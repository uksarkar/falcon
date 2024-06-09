use iced::widget::{column, container, text, Row, Space};
use iced::{Element, Length, Padding};

use crate::ui::app_theme::{AppColor, AppContainer};

use super::JsonToken;

#[derive(Debug, Clone)]
pub struct Line {
    pub line: usize,
    pub indent: usize,
    pub elements: Vec<JsonToken>,
}

impl<'a, Message: 'a + Clone> Into<Element<'a, Message>> for Line {
    fn into(self) -> Element<'a, Message> {
        let mut items: Vec<Element<'a, Message>> = vec![
            // line number
            container(text(format!("{}", self.line)))
                .padding(Padding::from([2, 5]))
                .style(AppContainer::FlatBg(AppColor::BG_DARKER))
                .into(),
            // space + indent
            Space::with_width(Length::Fixed(((self.indent * 2) + 10) as f32)).into(),
        ];

        for elm in self.elements {
            items.push(elm.into());
        }

        column![
            // actual line
            container(Row::from_vec(items).width(Length::Fill)).style(AppContainer::FlatSecondary),
            // border bottom
            container("")
                .height(1)
                .width(Length::Fill)
                .style(AppContainer::FlatBg(AppColor::BG_DARKER_12)),
        ]
        .width(Length::Fill)
        .into()
    }
}
