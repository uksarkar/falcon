use std::fmt::Display;

use regex::Regex;
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

    pub fn build(self, base_url: &str) -> String {
        // take the variable
        let url = self.0;

        // basic check
        if url.starts_with(RequestUrl::BASE_URL_PLACEHOLDER) {
            return url.replacen(RequestUrl::BASE_URL_PLACEHOLDER, base_url, 1);
        }

        // Check if var matches the variable pattern {{[a-zA-Z0-9_]+}}
        let var_pattern = Regex::new(r"^\{\{[a-zA-Z0-9_]+\}\}$").unwrap();
        let inner_var = if base_url.is_empty() {
            base_url
        } else if var_pattern.is_match(base_url) {
            &base_url[2..base_url.len() - 2]
        } else {
            return url.clone();
        };

        // Replace {{%PROJECT_BASE_URL[extra]%}} with {{SOME[extra]}}
        let re_with_brackets = Regex::new(r"^\{\{%PROJECT_BASE_URL\[(.*?)\]%\}\}").unwrap();

        // replace everything if the base_url is empty
        if base_url.is_empty() {
            return re_with_brackets.replace(&url, "").to_string();
        }

        re_with_brackets
            .replace(&url, format!("{{{{{}[$1]}}}}", inner_var))
            .to_string()
    }

    pub fn extract(self, base_url: &str) -> String {
        // let's take the URL
        let url = self.0;

        // if the base_url is empty then return the URL
        if base_url.is_empty() {
            return url;
        }

        // Check if url starts with var and replace it with {{%PROJECT_BASE_URL%}}
        if url.starts_with(&base_url) {
            return url.replacen(&base_url, RequestUrl::BASE_URL_PLACEHOLDER, 1);
        }

        // Check if base_url matches the variable pattern {{[a-zA-Z0-9_]+}}
        let var_pattern = Regex::new(r"^\{\{[a-zA-Z0-9_\s]+\}\}$").unwrap();
        if !var_pattern.is_match(&base_url) {
            return url;
        }

        // Extract the inner part of the variable, e.g., SOME from {{SOME}}
        let inner_var = &base_url[2..base_url.len() - 2];

        // Check if url starts with {{SOME[anything here]}} and replace accordingly
        let re_with_brackets = Regex::new(&format!(
            r"^\{{\{{{}\[(.*?)\]\}}\}}",
            regex::escape(inner_var)
        ))
        .unwrap();

        re_with_brackets
            .replace(&url, "{{%PROJECT_BASE_URL[$1]%}}")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_with_base_url() {
        let url = "http://example.com/path/to/resource".to_string();
        let var = "http://example.com".to_string();
        let result = RequestUrl::from(url).extract(&var);
        assert_eq!(result, "{{%PROJECT_BASE_URL%}}/path/to/resource");
    }

    #[test]
    fn test_extract_with_variable() {
        let url = "{{SOME}}/path/to/resource".to_string();
        let var = "{{SOME}}".to_string();
        let result = RequestUrl::from(url).extract(&var);
        assert_eq!(result, "{{%PROJECT_BASE_URL%}}/path/to/resource");
    }

    #[test]
    fn test_extract_with_variable_and_brackets() {
        let url = "{{SOME[extra]}}/path/to/resource".to_string();
        let var = "{{SOME}}".to_string();
        let result = RequestUrl::from(url).extract(&var);
        assert_eq!(result, "{{%PROJECT_BASE_URL[extra]%}}/path/to/resource");
    }

    #[test]
    fn test_extract_with_invalid_variable() {
        let url = "http://example.com/path/to/resource".to_string();
        let var = "{{INVALID}}".to_string();
        let result = RequestUrl::from(url.clone()).extract(&var);
        assert_eq!(result, url);
    }

    #[test]
    fn test_build_with_base_url() {
        let url = "{{%PROJECT_BASE_URL%}}/path/to/resource".to_string();
        let var = "http://example.com".to_string();
        let result = RequestUrl::from(url).build(&var);
        assert_eq!(result, "http://example.com/path/to/resource");
    }

    #[test]
    fn test_build_with_variable() {
        let url = "{{%PROJECT_BASE_URL%}}/path/to/resource".to_string();
        let var = "{{SOME}}".to_string();
        let result = RequestUrl::from(url).build(&var);
        assert_eq!(result, "{{SOME}}/path/to/resource");
    }

    #[test]
    fn test_build_with_variable_and_brackets() {
        let url = "{{%PROJECT_BASE_URL[extra]%}}/path/to/resource".to_string();
        let var = "{{SOME}}".to_string();
        let result = RequestUrl::from(url).build(&var);
        assert_eq!(result, "{{SOME[extra]}}/path/to/resource");
    }

    #[test]
    fn test_build_with_different_variable_using_extra() {
        let url = "{{%PROJECT_BASE_URL[extra]%}}/path/to/resource".to_string();
        let var = "{{DIFFERENT}}".to_string();
        let result = RequestUrl::from(url.clone()).build(&var);
        assert_eq!(result, "{{DIFFERENT[extra]}}/path/to/resource");
    }

    #[test]
    fn test_build_with_different_variable() {
        let url = "{{%PROJECT_BASE_URL%}}/path/to/resource".to_string();
        let var = "{{DIFFERENT}}".to_string();
        let result = RequestUrl::from(url.clone()).build(&var);
        assert_eq!(result, "{{DIFFERENT}}/path/to/resource");
    }

    #[test]
    fn test_build_with_empty_variable() {
        let url = "{{%PROJECT_BASE_URL%}}/path/to/resource".to_string();
        let var = "".to_string();
        let result = RequestUrl::from(url.clone()).build(&var);
        assert_eq!(result, "/path/to/resource");
    }

    #[test]
    fn test_build_with_empty_variable_and_brackets() {
        let url = "{{%PROJECT_BASE_URL[abcd]%}}/path/to/resource".to_string();
        let var = "".to_string();
        let result = RequestUrl::from(url.clone()).build(&var);
        assert_eq!(result, "/path/to/resource");
    }
}
