use std::fmt::Display;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::ui::elements::select_options::SelectOption;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HttpMethod(pub Method);

impl Into<SelectOption<HttpMethod>> for HttpMethod {
    fn into(self) -> SelectOption<HttpMethod> {
        SelectOption {
            label: format!("{}", self),
            value: self,
        }
    }
}

impl Serialize for HttpMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for HttpMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let method_str = String::deserialize(deserializer)?;
        let method = Method::from_bytes(method_str.as_bytes()).map_err(serde::de::Error::custom)?;
        Ok(HttpMethod(method))
    }
}

impl Into<Method> for HttpMethod {
    fn into(self) -> Method {
        self.0
    }
}

impl From<Method> for HttpMethod {
    fn from(value: Method) -> Self {
        Self(value)
    }
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        Self(
            value
                .to_uppercase()
                .as_bytes()
                .try_into()
                .unwrap_or_default(),
        )
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut val = self.0.as_str().chars();

        let val = match val.next() {
            None => "Get".to_string(),
            Some(char) => char.to_uppercase().to_string() + val.as_str().to_lowercase().as_str(),
        };

        write!(f, "{}", val)
    }
}
