use std::borrow::Borrow;
use std::ops::Deref;
use std::{fmt::Display, fs};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::app::app_config;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(PartialEq)]
pub struct Project {
    pub name: String,
    pub is_active: bool,
    pub id: Uuid,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: String::from("Unknown project"),
            is_active: Default::default(),
            id: Uuid::now_v7(),
        }
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Deref for Projects {
    type Target = Vec<Project>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Projects {
    #[serde(rename = "projects")]
    items: Vec<Project>,
}

impl Borrow<[Project]> for Projects {
    fn borrow(&self) -> &[Project] {
        &self.items
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

    pub fn sync(&self) -> Result<(), String> {
        set_projects(&format!("{}/falcon_projects.toml", app_config().DATA_DIR), self)?;
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
            Err(err) => Err(format!("Failed to save file, cause {}", err))
        }
        Err(err) => Err(format!("Toml save failed, cause {}", err)),
    }
}
