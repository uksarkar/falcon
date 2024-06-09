use std::fmt::Display;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestUrl(String);

impl Default for RequestUrl {
    fn default() -> Self {
        RequestUrl("".to_string())
    }
}

impl From<String> for RequestUrl {
    fn from(value: String) -> Self {
        RequestUrl(value)
    }
}

impl Display for RequestUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for RequestUrl {
    fn into(self) -> String {
        format!("{}", self.0)
    }
}

impl RequestUrl {
    pub const BASE_URL_PLACEHOLDER: &'static str = "{{%PROJECT_BASE_URL%}}";

    pub fn build(&self, base_url: &str) -> String {
        self.0
            .clone()
            .replace(RequestUrl::BASE_URL_PLACEHOLDER, base_url)
    }

    pub fn extract(&self, base_url: &str) -> String {
        if !self.0.starts_with(base_url) {
            return self.0.clone();
        }

        self.0
            .clone()
            .replace(base_url, RequestUrl::BASE_URL_PLACEHOLDER)
    }
}
