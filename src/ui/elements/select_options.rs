use std::{borrow::Borrow, fmt::Display, ops::Deref};



#[derive(PartialEq, Clone)]
pub struct SelectOption<V> {
    pub label: String,
    pub value: V
}

impl<V> Display for SelectOption<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

pub struct SelectItems<V> (pub Vec<SelectOption<V>>);

impl<V> Borrow<[SelectOption<V>]> for SelectItems<V> {
    fn borrow(&self) -> &[SelectOption<V>] {
        &self.0
    }
}

impl<V> Deref for SelectItems<V> {
    type Target = Vec<SelectOption<V>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}