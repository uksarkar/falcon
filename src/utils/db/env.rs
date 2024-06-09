use std::{collections::HashMap, fmt::Display};

use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ui::elements::select_options::SelectOption;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Env {
    pub id: Uuid,
    pub name: String,
    pub items: Vec<(String, String)>,
    pub is_active: bool,
    pub base_url: Option<String>,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            name: "Default env".into(),
            items: vec![("".into(), "".into())],
            is_active: Default::default(),
            base_url: None
        }
    }
}

impl Into<SelectOption<Uuid>> for Env {
    fn into(self) -> SelectOption<Uuid> {
        SelectOption {
            label: self.name,
            value: self.id,
        }
    }
}

impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Env {
    pub fn update_item_key(&mut self, index: usize, key: String) {
        if let Some((_, value)) = self.items.get(index) {
            self.items[index] = (key, value.clone());

            if index + 1 == self.items.len() {
                self.add_item("", "");
            }
        }
    }
    pub fn update_item_value(&mut self, index: usize, value: String) {
        if let Some((key, _)) = self.items.get(index) {
            self.items[index] = (key.clone(), value);

            if index + 1 == self.items.len() {
                self.add_item("", "");
            }
        }
    }
    pub fn remove_item(&mut self, index: usize) {
        self.items.remove(index);
    }
    pub fn add_item(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.items.push((key.into(), value.into()));
    }
    pub fn replace_variables(&self, input: impl Into<String>) -> String {
        let mut result: String = input.into();

        // Create a hashmap from the envs vector for easier lookup
        let env_map: HashMap<_, _> = self.items.iter().cloned().collect();

        // Regex to match placeholders with or without arguments
        let re = Regex::new(r"\{\{([A-Z0-9_]+)(\[(.*?)\])?\}\}").unwrap();

        // Iterate over all matches
        for cap in re.captures_iter(&result.clone()) {
            let key = &cap[1];
            let args = cap.get(3).map_or("", |m| m.as_str());

            if let Some(value_template) = env_map.get(key) {
                let mut replaced_value = value_template.clone();

                if !args.is_empty() {
                    // Split the arguments by comma
                    let args_vec: Vec<&str> = args
                        .split(',')
                        .map(|arg| arg.trim().trim_matches('"'))
                        .collect();

                    // Replace positional arguments ($0, $1, etc.)
                    for (i, arg) in args_vec.iter().enumerate() {
                        let placeholder = format!("${}", i);
                        replaced_value = replaced_value.replace(&placeholder, arg);
                    }

                    // Replace named arguments ($name)
                    let named_re = Regex::new(r"\$([a-zA-Z0-9_]+)").unwrap();
                    replaced_value = named_re
                        .replace_all(&replaced_value, |caps: &regex::Captures| {
                            if let Some(index) = args_vec.iter().position(|&a| *a == caps[1]) {
                                args_vec[index].to_string()
                            } else {
                                caps[0].to_string()
                            }
                        })
                        .to_string();
                }

                // Replace the original placeholder with the resolved value
                result = result.replace(&cap[0], &replaced_value);
            }
        }

        result
    }
}