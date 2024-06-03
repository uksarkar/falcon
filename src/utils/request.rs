use reqwest::cookie::Jar;
use reqwest::header::HeaderMap;
use reqwest::{Client, Method, StatusCode};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct FalconResponse {
    pub status_code: StatusCode,
    pub body: String,
    pub headers: HeaderMap,
    pub duration: Duration,
    pub size_kb: f64
}

#[derive(Debug, Clone)]
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
        let res = client
            .request(self.method.clone(), url)
            .send()
            .await?;

        // Calculate the duration
        let duration = start.elapsed();

        // Get the status, body, headers, and cookies
        let status_code = res.status();
        let headers = res.headers().clone();
        let body = res.text().await.unwrap_or_default();

        // Calculate response size in kilobytes
        let size_kb = (body.len() as f64 / 1024.0).ceil();

        Ok(FalconResponse {
            body,
            duration,
            headers,
            size_kb,
            status_code
        })
    }
}

#[derive(Default, Clone)]
pub struct NoUrl;

#[derive(Default, Clone)]
pub struct Url(String);

#[derive(Default, Clone)]
pub struct Locked;

#[derive(Default, Clone)]
pub struct Unlocked;

#[derive(Debug, Clone, Default)]
pub struct RequestBuilder<U, L> {
    url: U,
    method: Method,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    _locked: PhantomData<L>,
}

impl RequestBuilder<NoUrl, Unlocked> {
    pub fn new() -> RequestBuilder<NoUrl, Unlocked> {
        Self::default()
    }
}

impl<U> RequestBuilder<U, Unlocked> {
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<Url, Unlocked> {
        RequestBuilder {
            url: Url(url.into()),
            headers: self.headers,
            cookies: self.cookies,
            method: self.method,
            _locked: PhantomData,
        }
    }

    pub fn method(self, method: Method) -> RequestBuilder<U, Unlocked> {
        RequestBuilder {
            url: self.url,
            headers: self.headers,
            cookies: self.cookies,
            method,
            _locked: PhantomData,
        }
    }

    pub fn add_header(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.headers.insert(name.into(), value.into());
    }

    pub fn add_cookie(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.cookies.insert(name.into(), value.into());
    }

    pub fn lock(self) -> RequestBuilder<U, Locked> {
        RequestBuilder {
            url: self.url,
            headers: self.headers,
            cookies: self.cookies,
            method: self.method,
            _locked: PhantomData,
        }
    }
}

impl<U> RequestBuilder<U, Locked> {
    pub fn unlock(self) -> RequestBuilder<U, Unlocked> {
        RequestBuilder {
            url: self.url,
            headers: self.headers,
            cookies: self.cookies,
            method: self.method,
            _locked: PhantomData,
        }
    }
}

impl<L> RequestBuilder<Url, L> {
    pub fn build(self) -> PendingRequest {
        PendingRequest {
            url: self.url.0,
            method: self.method,
            headers: self.headers,
            cookies: self.cookies,
        }
    }
}
