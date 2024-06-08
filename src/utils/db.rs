use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{fmt::Display, fs};

use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ui::elements::select_options::{SelectItems, SelectOption};

use super::app::app_config;
use super::request::{HttpMethod, PendingRequest, PendingRequestItem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Env {
    pub id: Uuid,
    pub name: String,
    pub items: Vec<(String, String)>,
    pub is_active: bool,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            name: "Default env".into(),
            items: vec![("".into(), "".into())],
            is_active: Default::default(),
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub base_url: Option<String>,
    pub is_active: bool,
    pub id: Uuid,
    pub requests: HashMap<String, Vec<PendingRequest>>,
    pub active_request_id: Option<Uuid>,
    pub default_env: Option<Uuid>,
}

impl Project {
    pub fn current_request(&self) -> Option<(&String, &PendingRequest)> {
        self.requests
            .iter()
            .find_map(|(folder, reqs)| {
                self.active_request_id
                    .and_then(|id| reqs.iter().find(|req| req.id == id).map(|r| (folder, r)))
            })
            .or_else(|| {
                self.requests
                    .iter()
                    .find_map(|(folder, reqs)| reqs.first().map(|r| (folder, r)))
            })
    }

    pub fn current_request_id(&self) -> Option<Uuid> {
        self.current_request().and_then(|r| Some(r.1.id))
    }

    pub fn current_request_mut(&mut self) -> Option<&mut PendingRequest> {
        if self.requests.is_empty() {
            let req = PendingRequest::default();
            self.active_request_id = Some(req.id);
            self.requests.insert("root".to_string(), vec![req]);
        }

        if self.active_request_id.is_none() {
            self.active_request_id = self
                .requests
                .values()
                .filter_map(|p| p.first())
                .map(|p| p.id)
                .next();
        }

        if let Some(id) = self.active_request_id {
            for reqs in self.requests.values_mut() {
                if let Some(req) = reqs.iter_mut().find(|r| r.id == id) {
                    return Some(req);
                }
            }
        }

        None
    }

    pub fn set_current_request(&mut self, id: Uuid) {
        self.active_request_id = Some(id);
    }

    pub fn update_request_item(
        &mut self,
        item: PendingRequestItem,
        index: usize,
        value: String,
        is_key: bool,
    ) {
        if let Some(req) = self.current_request_mut() {
            if is_key {
                req.update_item_key(item, index, value);
            } else {
                req.update_item_value(item, index, value);
            }
        }
    }

    pub fn update_request_url(&mut self, url: String) {
        if let Some(active_req) = self.current_request_mut() {
            active_req.set_url(url);
        }
    }

    pub fn update_request_method(&mut self, method: HttpMethod) {
        if let Some(active_req) = self.current_request_mut() {
            active_req.set_method(method);
        }
    }

    pub fn add_request(&mut self, folder: String, request: PendingRequest) {
        self.set_current_request(request.id);

        if let Some(reqs) = self.requests.get_mut(&folder) {
            reqs.push(request);
        } else {
            self.requests.insert(folder, vec![request]);
        }
    }

    pub fn remove_request(&mut self, folder: String, id: Uuid) {
        if let Some(reqs) = self.requests.get_mut(&folder) {
            for (i, req) in reqs.iter().enumerate() {
                if req.id == id {
                    reqs.remove(i);
                    break;
                }
            }
        }

        if let Some(id) = self.current_request_id() {
            self.set_current_request(id);
        }
    }

    pub fn set_default_env(&mut self, id: Uuid) {
        self.default_env = Some(id);
    }

    pub fn remove_default_env(&mut self) {
        self.default_env = None
    }
}

impl Default for Project {
    fn default() -> Self {
        let mut requests = HashMap::new();
        requests.insert("root".to_string(), vec![PendingRequest::default()]);

        Self {
            name: String::from("Unknown project"),
            is_active: Default::default(),
            id: Uuid::now_v7(),
            requests,
            active_request_id: None,
            base_url: None,
            default_env: None,
        }
    }
}

impl Into<SelectOption<Uuid>> for Project {
    fn into(self) -> SelectOption<Uuid> {
        SelectOption {
            label: self.name,
            value: self.id,
        }
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Projects {
    #[serde(rename = "projects")]
    items: Vec<Project>,
    envs: Vec<Env>,
}

impl Into<SelectItems<Uuid>> for &Projects {
    fn into(self) -> SelectItems<Uuid> {
        SelectItems(
            <Vec<Project> as Clone>::clone(&self.items)
                .into_iter()
                .map(|project| project.into())
                .collect(),
        )
    }
}

impl Projects {
    pub fn new() -> Self {
        get_projects(&format!("{}/falcon_projects.toml", app_config().DATA_DIR)).unwrap_or(Self {
            items: vec![Project::default()],
            envs: vec![Env::default()],
        })
    }

    pub fn active(&self) -> Option<Project> {
        // Step 1: Check for an existing active project
        if let Some(project) = self.items.iter().find(|itm| itm.is_active) {
            return Some(project.clone());
        }

        // Step 2: If no active project, check if the list is not empty and make the first project active
        if !self.items.is_empty() {
            return Some(self.items[0].clone());
        }

        None
    }

    pub fn active_mut(&mut self) -> Option<&mut Project> {
        if self.items.is_empty() {
            return None;
        }

        if let Some(index) = self.items.iter().position(|i| i.is_active) {
            return Some(&mut self.items[index]);
        }

        Some(&mut self.items[0])
    }

    pub fn set_project_default_env(&mut self, project_id: Uuid, env_id: Option<Uuid>) {
        for project in self.items.iter_mut() {
            if project.id == project_id {
                project.default_env = env_id;
            }
        }
    }

    pub fn get_project_default_env_id(&self) -> Option<Uuid> {
        self.active().and_then(|proj| {
            proj.default_env.and_then(|env_id| {
                self.envs
                    .iter()
                    .find(|env| env.id == env_id)
                    .and_then(|env| Some(env.id.clone()))
            })
        })
    }

    pub fn project_default_env(&self) -> Option<Env> {
        self.active().and_then(|p| {
            p.default_env.and_then(|env_id| {
                self.envs
                    .iter()
                    .find(|env| env.id == env_id)
                    .and_then(|env| Some(env.clone()))
            })
        })
    }

    pub fn active_env(&self) -> Option<Env> {
        // Step 1: Check for an existing active env
        if let Some(env) = self.envs.iter().find(|itm| itm.is_active) {
            return Some(env.clone());
        }

        // Step 2: If no active env, check if the list is not empty and make the first env active
        if !self.envs.is_empty() {
            return Some(self.envs[0].clone());
        }

        None
    }

    pub fn active_env_mut(&mut self) -> Option<&mut Env> {
        if self.envs.is_empty() {
            return None;
        }

        if let Some(index) = self.envs.iter().position(|i| i.is_active) {
            return Some(&mut self.envs[index]);
        }

        Some(&mut self.envs[0])
    }

    pub fn set_active_env(&mut self, id: Uuid) {
        for env in self.envs.iter_mut() {
            env.is_active = env.id == id;
        }
    }

    pub fn delete_env(&mut self, id: Uuid) {
        for (i, env) in self.envs.iter().enumerate() {
            if env.id == id {
                self.envs.remove(i);
                return;
            }
        }
    }

    pub fn duplicate_env(&mut self, id: Uuid) -> Option<Env> {
        for env in self.envs.iter() {
            if env.id == id {
                let env = Env {
                    id: Uuid::now_v7(),
                    ..env.clone()
                };

                self.envs.push(env.clone());
                return Some(env);
            }
        }

        None
    }

    pub fn is_active_env(&self, id: Uuid) -> bool {
        if let Some(env) = self.envs.iter().find(|itm| itm.is_active) {
            return env.id == id;
        }

        // Step 2: If no active project, check if the list is not empty and make the first project active
        if !self.items.is_empty() {
            return self.items[0].id == id;
        }

        false
    }

    pub fn add_env(&mut self, env: Env) {
        self.envs.push(env);
    }

    pub fn sync(&self) -> Result<(), String> {
        set_projects(
            &format!("{}/falcon_projects.toml", app_config().DATA_DIR),
            self,
        )?;
        Ok(())
    }

    pub fn set_active(&mut self, id: &Uuid) {
        for proj in &mut self.items {
            proj.is_active = proj.id == *id;
        }
    }

    pub fn add(&mut self, project: Project) {
        if project.is_active {
            for proj in &mut self.items {
                proj.is_active = false;
            }
        }

        self.items.push(project);
    }

    pub fn into_options(&self) -> SelectItems<Uuid> {
        self.into()
    }

    pub fn selected_project(&self) -> Option<SelectOption<Uuid>> {
        if let Some(active) = self.active() {
            return Some(active.into());
        }

        None
    }

    pub fn delete_project(&mut self, id: Uuid) {
        for (ind, proj) in self.items.iter().enumerate() {
            if proj.id == id {
                self.items.remove(ind);
                return;
            }
        }
    }

    pub fn duplicate_project(&mut self, id: Uuid) -> Option<Project> {
        for proj in self.items.iter() {
            if proj.id == id {
                let project = Project {
                    id: Uuid::now_v7(),
                    ..proj.clone()
                };

                self.items.push(project.clone());
                return Some(project);
            }
        }

        None
    }

    pub fn env_into_options(&self) -> SelectItems<Uuid> {
        SelectItems(
            <Vec<Env> as Clone>::clone(&self.envs)
                .into_iter()
                .map(|env| env.into())
                .collect(),
        )
    }

    pub fn selected_env_as_option(&self) -> Option<SelectOption<Uuid>> {
        self.active_env().and_then(|env| Some(env.into()))
    }
}

fn get_projects(path: &str) -> Result<Projects, String> {
    let path = Path::new(path);

    if !path.exists() {
        return Err("File not found".to_string());
    }

    // Open the file
    let mut file = File::open(path).expect("Unable to open file");

    // Read the content of the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    // Parse the content using toml::from_str
    match toml::from_str(&contents) {
        Ok(items) => Ok(items),
        Err(_) => Err(String::from("Unable to parse toml")),
    }
}

fn set_projects(path: &str, projects: &Projects) -> Result<(), String> {
    let path = Path::new(path);

    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap()).expect("Unable to create directory");
    }

    match toml::to_string(&projects) {
        Ok(contents) => match fs::write(path, contents) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to save file, cause {}", err)),
        },
        Err(err) => Err(format!("Toml save failed, cause {}", err)),
    }
}
