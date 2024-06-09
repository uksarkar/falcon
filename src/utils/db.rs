use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::fs;

use env::Env;
use project::Project;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ui::elements::select_options::{SelectItems, SelectOption};

use super::app::app_config;

pub mod env;
pub mod project;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DB {
    #[serde(rename = "projects")]
    items: Vec<Project>,
    envs: Vec<Env>,
}

impl Into<SelectItems<Uuid>> for &DB {
    fn into(self) -> SelectItems<Uuid> {
        SelectItems(
            <Vec<Project> as Clone>::clone(&self.items)
                .into_iter()
                .map(|project| project.into())
                .collect(),
        )
    }
}

impl DB {
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

    pub fn get_active_base_url(&self) -> String {
        self.active_env().and_then(|env| env.base_url).unwrap_or_default()
    }

    pub fn set_active_base_url(&mut self, base: Option<String>) {
        if let Some(env) = self.active_env_mut() {
            env.base_url = base;
        }
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

fn get_projects(path: &str) -> Result<DB, String> {
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

fn set_projects(path: &str, projects: &DB) -> Result<(), String> {
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
