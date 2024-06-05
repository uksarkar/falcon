use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{fmt::Display, fs};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ui::elements::select_options::{SelectItems, SelectOption};

use super::app::app_config;
use super::request::{HttpMethod, PendingRequest, PendingRequestItem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub is_active: bool,
    pub id: Uuid,
    pub requests: HashMap<String, Vec<PendingRequest>>,
    pub active_request_id: Option<Uuid>,
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
        if let Some(reqs) = self.requests.get_mut(&folder) {
            reqs.push(request);
        } else {
            self.requests.insert(folder, vec![request]);
        }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Projects {
    #[serde(rename = "projects")]
    items: Vec<Project>,
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

    pub fn add(&mut self, project: Project) -> Result<(), String> {
        if project.is_active {
            for proj in &mut self.items {
                proj.is_active = false;
            }
        }

        self.items.push(project);

        self.sync()
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
