use reqwest::cookie::Jar;
use reqwest::header::HeaderMap;
use reqwest::{Client, Method, StatusCode};
use std::collections::HashMap;
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

#[derive(Debug, Clone, Default)]
pub struct PendingRequest {
    pub url: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
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

        let cookies: Vec<FalconCookie> = res.cookies().map(|c| {
            FalconCookie {
                name: c.name().to_string(),
                http_only: c.http_only(),
                value: Some(c.value().to_string()),
                expires: c.expires(),
            }
        }).collect();

        let body = res.text().await.unwrap_or_default();

        // Calculate response size in kilobytes
        let size_kb = (body.len() as f64 / 1024.0).ceil();

        Ok(FalconResponse {
            body,
            duration: duration.into(),
            headers,
            size_kb,
            status_code,
            cookies
        })
    }

    pub fn set_url(&mut self, url: impl Into<String>) {
        self.url = url.into();
    }
    
    pub fn add_header(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.headers.insert(name.into(), value.into());
    }

    pub fn add_cookie(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.cookies.insert(name.into(), value.into());
    }

    pub fn set_method(&mut self, method: Method) {
        self.method = method;
    }
}
