use std::borrow::Cow;

use iced::{
    widget::{container, mouse_area, text},
    Element, Length, Padding,
};

#[derive(Debug, PartialEq, Clone)]
pub enum JsonToken {
    String(Cow<'static, str>),
    Key(Cow<'static, str>),
    Number(Cow<'static, str>),
    Bool(bool),
    Null,
    BeginObject,
    EndObject,
    BeginArray,
    EndArray,
    Comma,
    Colon,
}

impl<'a, Message: 'a + Clone> Into<Element<'a, Message>> for JsonToken {
    fn into(self) -> Element<'a, Message> {
        let elm = text(match self.clone() {
            JsonToken::String(value) => format!("\"{}\"", value.replace("\n", " ")),
            JsonToken::Number(value) => format!("{}", value),
            JsonToken::Bool(value) => format!("{}", value),
            JsonToken::Null => "null".to_string(),
            JsonToken::BeginObject => "{".to_string(),
            JsonToken::EndObject => "}".to_string(),
            JsonToken::BeginArray => "[".to_string(),
            JsonToken::EndArray => "]".to_string(),
            JsonToken::Comma => ",".to_string(),
            JsonToken::Colon => ":".to_string(),
            JsonToken::Key(key) => format!("\"{}\"", key),
        });

        match self {
            JsonToken::String(_) => container(
                mouse_area(container(elm.width(Length::Fill)))
                    .interaction(iced::mouse::Interaction::Text),
            ),
            JsonToken::Colon => container(elm).padding(Padding::from([0, 5])),
            _ => container(elm),
        }
        .into()
    }
}
