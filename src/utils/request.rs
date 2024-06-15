use http_method::HttpMethod;
use reqwest::cookie::Jar;
use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
use reqwest::{Body, Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::Arc;
use std::time::{Instant, SystemTime};
use uuid::Uuid;

use super::db::env::Env;
use super::falcon_duration::FalconDuration;

pub mod http_method;
pub mod request_url;

pub use request_url::RequestUrl;

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FalconAuthorization {
    Bearer { prefix: String, token: String },
}

impl Default for FalconAuthorization {
    fn default() -> Self {
        FalconAuthorization::Bearer {
            prefix: "Bearer".to_string(),
            token: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FlBody {
    ApplicationJson(String),
}

impl Default for FlBody {
    fn default() -> Self {
        FlBody::ApplicationJson("".to_string())
    }
}

impl Display for FlBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlBody::ApplicationJson(json) => write!(f, "{}", json),
        }
    }
}

impl Into<Body> for FlBody {
    fn into(self) -> Body {
        match self {
            FlBody::ApplicationJson(json) => Body::from(json),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PendingRequest {
    pub id: Uuid,
    pub name: Option<String>,
    pub url: String,
    pub method: HttpMethod,
    pub headers: Vec<(String, String)>,
    pub cookies: Vec<(String, String)>,
    pub queries: Vec<(String, String)>,
    pub authorization: FalconAuthorization,
    pub body: FlBody,
}

impl Default for PendingRequest {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            name: None,
            url: "https://".to_string(),
            method: Default::default(),
            headers: vec![("".to_string(), "".to_string())],
            cookies: vec![("".to_string(), "".to_string())],
            queries: vec![("".to_string(), "".to_string())],
            authorization: FalconAuthorization::default(),
            body: FlBody::default(),
        }
    }
}

impl PendingRequest {
    pub async fn send(&self, env: &Env, base_url: &str) -> anyhow::Result<FalconResponse> {
        // Create a cookie jar
        let cookie_jar = Arc::new(Jar::default());

        let url = RequestUrl::from(self.url.clone()).build(base_url);

        // Add a cookie manually (if needed)
        let mut url = url::Url::parse(&env.replace_variables(url))?;

        for (name, value) in self.queries.iter() {
            if !name.trim().is_empty() {
                url.query_pairs_mut()
                    .append_pair(&env.replace_variables(name), &env.replace_variables(value));
            }
        }

        println!(
            "{:<10}[FALCON]: ({}) {}",
            "INFO",
            self.method.0.clone(),
            url.clone()
        );

        for (name, value) in self.cookies.iter() {
            cookie_jar.add_cookie_str(
                &format!(
                    "{}={}",
                    env.replace_variables(name),
                    env.replace_variables(value)
                ),
                &url,
            );
        }

        let mut headers = HeaderMap::new();

        for (key, value) in self.headers.iter() {
            if !key.trim().is_empty() {
                headers.insert(
                    HeaderName::from_bytes(env.replace_variables(key).as_bytes())?,
                    HeaderValue::from_str(&env.replace_variables(value))?,
                );
            }
        }

        match self.body {
            FlBody::ApplicationJson(_) => {
                headers.insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str("application/json")?,
                );
            }
        }

        match self.authorization.clone() {
            FalconAuthorization::Bearer { prefix, token } => {
                if !token.trim().is_empty() {
                    headers.insert(
                        header::AUTHORIZATION,
                        HeaderValue::from_str(&format!(
                            "{} {}",
                            env.replace_variables(prefix),
                            env.replace_variables(token)
                        ))?,
                    );
                }
            }
        }

        // Create a reqwest client with the cookie jar
        let client = Client::builder()
            .default_headers(headers)
            .cookie_provider(cookie_jar)
            .build()?;

        // Start timing the request
        let start = Instant::now();

        // Send a request
        let res = client
            .request(self.method.clone().into(), url)
            .body(self.body.clone())
            .send()
            .await?;

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

    pub fn set_method(&mut self, method: HttpMethod) {
        self.method = method;
    }

    pub fn set_auth(&mut self, auth: FalconAuthorization) {
        self.authorization = auth;
    }

    pub fn set_body(&mut self, body: FlBody) {
        self.body = body;
    }
}
