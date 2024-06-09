use anyhow::Result;
use serde_json::Value;

use std::borrow::Cow;

mod json_line;
mod json_token;

pub use json_line::Line;
pub use json_token::JsonToken;

pub fn tokenize(json: &str) -> Result<Vec<Line>, String> {
    match serde_json::from_str(json) {
        Ok(value) => Ok(format_tokens(process_value(&value))),
        Err(err) => Err(err.to_string()),
    }
}

fn process_value(value: &Value) -> Vec<JsonToken> {
    let mut tokens = Vec::new();
    match value {
        Value::Object(obj) => {
            tokens.push(JsonToken::BeginObject);
            for (key, val) in obj.iter() {
                tokens.push(JsonToken::Key(Cow::Owned(key.to_string())));
                tokens.push(JsonToken::Colon);
                tokens.extend(process_value(val));
                tokens.push(JsonToken::Comma);
            }
            tokens.pop(); // Remove the last comma
            tokens.push(JsonToken::EndObject);
        }
        Value::Array(arr) => {
            tokens.push(JsonToken::BeginArray);
            for val in arr.iter() {
                tokens.extend(process_value(val));
                tokens.push(JsonToken::Comma);
            }
            tokens.pop(); // Remove the last comma
            tokens.push(JsonToken::EndArray);
        }
        Value::String(s) => tokens.push(JsonToken::String(Cow::Owned(s.clone()))),
        Value::Number(n) => tokens.push(JsonToken::Number(Cow::Owned(n.to_string()))),
        Value::Bool(b) => tokens.push(JsonToken::Bool(*b)),
        Value::Null => tokens.push(JsonToken::Null),
    }
    tokens
}

fn format_tokens(tokens: Vec<JsonToken>) -> Vec<Line> {
    let total = tokens.len();
    let mut lines = Vec::new();
    let mut current_line_vec = Vec::new();

    let mut indent = 0;
    let mut current_line = 1;
    let mut skip_comma = false;

    for (index, token) in tokens.into_iter().enumerate() {
        match token {
            JsonToken::BeginObject | JsonToken::BeginArray => {
                current_line_vec.push(token);
                lines.push(Line {
                    elements: current_line_vec.clone(),
                    indent,
                    line: current_line,
                });
                current_line += 1;
                indent += 4;
                current_line_vec.clear();
            }
            JsonToken::Comma => {
                if skip_comma {
                    skip_comma = false;
                } else {
                    current_line_vec.push(token);
                    lines.push(Line {
                        elements: current_line_vec.clone(),
                        indent,
                        line: current_line,
                    });
                    current_line_vec.clear();
                    current_line += 1;
                }
            }
            JsonToken::EndObject | JsonToken::EndArray => {
                if !current_line_vec.is_empty() {
                    lines.push(Line {
                        elements: current_line_vec.clone(),
                        indent,
                        line: current_line,
                    });
                    current_line += 1;
                    current_line_vec.clear();
                }

                let elements = if index == total - 1 {
                    vec![token]
                } else {
                    skip_comma = true;
                    vec![token, JsonToken::Comma]
                };

                indent -= 4;
                lines.push(Line {
                    elements,
                    indent,
                    line: current_line,
                });
                current_line += 1;
            }
            _ => current_line_vec.push(token),
        }
    }

    if !current_line_vec.is_empty() {
        lines.push(Line {
            elements: current_line_vec,
            indent,
            line: current_line,
        });
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Line {
        pub fn begin_obj(line: usize, indent: usize) -> Self {
            Self {
                line,
                indent,
                elements: vec![JsonToken::BeginObject],
            }
        }
        pub fn begin_arr(line: usize, indent: usize) -> Self {
            Self {
                line,
                indent,
                elements: vec![JsonToken::BeginArray],
            }
        }
        pub fn end_obj(line: usize, indent: usize) -> Self {
            Self {
                line,
                indent,
                elements: vec![JsonToken::EndObject],
            }
        }
        pub fn end_arr(line: usize, indent: usize) -> Self {
            Self {
                line,
                indent,
                elements: vec![JsonToken::EndArray],
            }
        }
    }

    #[test]
    fn test_basic_json_tokenization() -> Result<(), String> {
        let json = r#"{"hello": "world"}"#;
        let tokenized = tokenize(json)?;

        let result = [
            Line::begin_obj(1, 0),
            Line {
                elements: vec![
                    JsonToken::String(Cow::Owned("hello".to_string())),
                    JsonToken::Colon,
                    JsonToken::String(Cow::Owned("world".to_string())),
                ],
                line: 2,
                indent: 4,
            },
            Line::end_obj(3, 0),
        ];

        assert_eq!(format!("{:?}", tokenized), format!("{:?}", result));

        Ok(())
    }

    #[test]
    fn test_json_array() -> Result<(), String> {
        let json = r#"[{"items": ["a", 1, null, false]}]"#;
        let result = [
            Line::begin_arr(1, 0),
            Line::begin_obj(2, 4),
            Line {
                elements: vec![
                    JsonToken::String(Cow::Borrowed("items")),
                    JsonToken::Colon,
                    JsonToken::BeginArray,
                ],
                line: 3,
                indent: 8,
            },
            Line {
                elements: vec![JsonToken::String(Cow::Borrowed("a")), JsonToken::Comma],
                line: 4,
                indent: 12,
            },
            Line {
                elements: vec![JsonToken::Number(Cow::Borrowed("1")), JsonToken::Comma],
                line: 5,
                indent: 12,
            },
            Line {
                elements: vec![JsonToken::Null, JsonToken::Comma],
                line: 6,
                indent: 12,
            },
            Line {
                elements: vec![JsonToken::Bool(false)],
                line: 7,
                indent: 12,
            },
            Line::end_arr(8, 8),
            Line::end_obj(9, 4),
            Line::end_arr(10, 0),
        ];

        assert_eq!(format!("{:?}", tokenize(json)?), format!("{:?}", result));

        Ok(())
    }
}
