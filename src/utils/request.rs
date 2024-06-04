use reqwest::cookie::Jar;
use reqwest::header::HeaderMap;
use reqwest::{Client, Method, StatusCode};
use std::fmt::Display;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use crate::utils::helpers::format_duration;

#[derive(Debug, Clone)]
pub struct FalconDuration(Duration);

impl From<Duration> for FalconDuration {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

impl Display for FalconDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_duration(self.0))
    }
}

#[derive(Debug, Clone)]
pub enum PendingRequestItem {
    Header,
    Cookie,
    Query,
}

#[derive(Debug, Clone)]
pub struct FalconCookie {
    pub name: String,
    pub value: Option<String>,
    pub http_only: bool,
    pub expires: Option<SystemTime>,
}

#[derive(Debug, Clone)]
pub struct FalconResponse {
    pub status_code: StatusCode,
    pub body: String,
    pub headers: HeaderMap,
    pub cookies: Vec<FalconCookie>,
    pub duration: FalconDuration,
    pub size_kb: f64,
}

#[derive(Debug, Clone)]
pub struct PendingRequest {
    pub url: String,
    pub method: Method,
    pub headers: Vec<(String, String)>,
    pub cookies: Vec<(String, String)>,
    pub queries: Vec<(String, String)>,
}

impl Default for PendingRequest {
    fn default() -> Self {
        Self {
            url: "https://".to_string(),
            method: Default::default(),
            headers: vec![("".to_string(), "".to_string())],
            cookies: vec![("".to_string(), "".to_string())],
            queries: vec![("".to_string(), "".to_string())],
        }
    }
}

impl PendingRequest {
    pub async fn send(&self) -> anyhow::Result<FalconResponse> {
        // Create a cookie jar
        let cookie_jar = Arc::new(Jar::default());

        // Add a cookie manually (if needed)
        let url = url::Url::parse(&self.url)?;

        for (name, value) in self.cookies.iter() {
            cookie_jar.add_cookie_str(&format!("{}={}", name, value), &url);
        }

        // Create a reqwest client with the cookie jar
        let client = Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()?;

        // Start timing the request
        let start = Instant::now();

        // Send a request
        let res = client.request(self.method.clone(), url).send().await?;

        // Calculate the duration
        let duration = start.elapsed();

        // Get the status, body, headers, and cookies
        let status_code = res.status();
        let headers = res.headers().clone();

        let cookies: Vec<FalconCookie> = res
            .cookies()
            .map(|c| FalconCookie {
                name: c.name().to_string(),
                http_only: c.http_only(),
                value: Some(c.value().to_string()),
                expires: c.expires(),
            })
            .collect();

        let body = res.text().await.unwrap_or_default();

        // Calculate response size in kilobytes
        let size_kb = (body.len() as f64 / 1024.0).ceil();

        Ok(FalconResponse {
            body,
            duration: duration.into(),
            headers,
            size_kb,
            status_code,
            cookies,
        })
    }

    pub fn set_url(&mut self, url: impl Into<String>) {
        self.url = url.into();
    }

    pub fn add_header(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.headers.push((name.into(), value.into()));
    }

    pub fn add_cookie(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.cookies.push((name.into(), value.into()));
    }

    pub fn add_query(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.queries.push((name.into(), value.into()));
    }

    pub fn update_item_key(
        &mut self,
        item: PendingRequestItem,
        index: usize,
        name: impl Into<String>,
    ) {
        match item {
            PendingRequestItem::Header => self.update_header_key(index, name),
            PendingRequestItem::Cookie => self.update_cookie_key(index, name),
            PendingRequestItem::Query => self.update_query_key(index, name),
        };
    }

    pub fn update_item_value(
        &mut self,
        item: PendingRequestItem,
        index: usize,
        value: impl Into<String>,
    ) {
        match item {
            PendingRequestItem::Header => self.update_header_value(index, value),
            PendingRequestItem::Cookie => self.update_cookie_value(index, value),
            PendingRequestItem::Query => self.update_query_value(index, value),
        };
    }

    pub fn remove_item(&mut self, item: PendingRequestItem, index: usize) {
        match item {
            PendingRequestItem::Header => self.headers.remove(index),
            PendingRequestItem::Cookie => self.cookies.remove(index),
            PendingRequestItem::Query => self.queries.remove(index),
        };
    }

    pub fn update_header_key(&mut self, index: usize, name: impl Into<String>) {
        if let Some((_, value)) = self.headers.get(index) {
            self.headers[index] = (name.into(), value.clone());
        }

        if self.headers.len() == index + 1 {
            self.add_header("", "");
        }
    }

    pub fn update_header_value(&mut self, index: usize, value: impl Into<String>) {
        if let Some((key, _)) = self.headers.get(index) {
            self.headers[index] = (key.clone(), value.into());
        }

        if self.headers.len() == index + 1 {
            self.add_header("", "");
        }
    }

    pub fn update_cookie_key(&mut self, index: usize, name: impl Into<String>) {
        if let Some((_, value)) = self.cookies.get(index) {
            self.cookies[index] = (name.into(), value.clone());
        }

        if self.cookies.len() == index + 1 {
            self.add_cookie("", "");
        }
    }

    pub fn update_cookie_value(&mut self, index: usize, value: impl Into<String>) {
        if let Some((key, _)) = self.cookies.get(index) {
            self.cookies[index] = (key.clone(), value.into());
        }

        if self.cookies.len() == index + 1 {
            self.add_cookie("", "");
        }
    }

    pub fn update_query_key(&mut self, index: usize, name: impl Into<String>) {
        if let Some((_, value)) = self.queries.get(index) {
            self.queries[index] = (name.into(), value.clone());
        }

        if self.queries.len() == index + 1 {
            self.add_query("", "");
        }
    }

    pub fn update_query_value(&mut self, index: usize, value: impl Into<String>) {
        if let Some((key, _)) = self.queries.get(index) {
            self.queries[index] = (key.clone(), value.into());
        }

        if self.queries.len() == index + 1 {
            self.add_query("", "");
        }
    }

    pub fn set_method(&mut self, method: Method) {
        self.method = method;
    }
}
