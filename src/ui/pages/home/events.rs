use uuid::Uuid;

use crate::utils::{
    db::{env::Env, project::Project, DB},
    request::{FalconAuthorization, http_method::HttpMethod, PendingRequest, PendingRequestItem},
};

use super::HomeEventMessage;

#[derive(Debug, Clone)]
pub enum EnvEvent {
    Select(Uuid),
    Duplicate(Uuid),
    Delete(Uuid),
    ItemKeyInput(usize, String),
    ItemValueInput(usize, String),
    ItemRemove(usize),
    NameInput(String),
    BaseUrlInput(String),
    Add,
}

impl EnvEvent {
    pub fn handle(self, db: &mut DB) {
        match self {
            EnvEvent::Select(id) => {
                db.set_active_env(id);
            }
            EnvEvent::Duplicate(id) => {
                db.duplicate_env(id);
            }
            EnvEvent::Delete(id) => {
                db.delete_env(id);
            }
            EnvEvent::ItemKeyInput(index, key) => {
                if let Some(env) = db.active_env_mut() {
                    env.update_item_key(index, key);
                }
            }
            EnvEvent::ItemValueInput(index, value) => {
                if let Some(env) = db.active_env_mut() {
                    env.update_item_value(index, value);
                }
            }
            EnvEvent::ItemRemove(index) => {
                if let Some(env) = db.active_env_mut() {
                    env.remove_item(index);
                }
            }
            EnvEvent::NameInput(name) => {
                if let Some(env) = db.active_env_mut() {
                    env.name = name;
                }
            }
            EnvEvent::Add => {
                let env = Env::default();

                db.add_env(env.clone());
                db.set_active_env(env.id);
            }
            EnvEvent::BaseUrlInput(base) => {
                let base = if base.is_empty() {None} else {Some(base)};
                db.set_active_base_url(base);
            }
        }
    }
}

impl Into<HomeEventMessage> for EnvEvent {
    fn into(self) -> HomeEventMessage {
        HomeEventMessage::EnvEvent(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectEvent {
    NameInput(String),
    Remove(Uuid),
    Duplicate(Uuid),
    DefaultEnvSelect(Option<Uuid>),
    Add(String),
    Select(Uuid),
}

impl Into<HomeEventMessage> for ProjectEvent {
    fn into(self) -> HomeEventMessage {
        HomeEventMessage::ProjectEvent(self)
    }
}

impl ProjectEvent {
    pub fn handle(self, db: &mut DB) -> bool {
        match self {
            ProjectEvent::NameInput(name) => {
                if let Some(project) = db.active_mut() {
                    project.name = name;
                }
                false
            }
            ProjectEvent::Remove(id) => {
                db.delete_project(id);
                false
            }
            ProjectEvent::Duplicate(id) => {
                db.duplicate_project(id);
                false
            }
            ProjectEvent::DefaultEnvSelect(id) => {
                if let Some(env_id) = id.clone() {
                    db.set_active_env(env_id);
                }
                if let Some(project) = db.active_mut() {
                    project.default_env = id;
                }
                false
            }
            ProjectEvent::Add(name) => {
                db.add(Project {
                    name,
                    is_active: true,
                    ..Default::default()
                });
                true
            }
            ProjectEvent::Select(id) => {
                db.set_active(&id);
                if let Some(id) = db.get_project_default_env_id() {
                    db.set_active_env(id);
                }
                false
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum RequestEvent {
    SelectMethod(HttpMethod),
    ItemKeyInput(PendingRequestItem, usize, String),
    ItemValueInput(PendingRequestItem, usize, String),
    NameInput(String),
    RemoveItem(PendingRequestItem, usize),
    Add(PendingRequest),
    Select(Uuid),
    Delete(Uuid),
    AuthorizationInput(FalconAuthorization),
    UrlInput(String),
    New
}

impl Into<HomeEventMessage> for RequestEvent {
    fn into(self) -> HomeEventMessage {
        HomeEventMessage::RequestEvent(self)
    }
}

impl RequestEvent {
    pub fn handle(self, project: &mut Project, base_url: &str) {
        match self {
            RequestEvent::SelectMethod(method) => {
                project.update_request_method(method);
            }
            RequestEvent::ItemKeyInput(item, index, name) => {
                project.update_request_item(item, index, name, true);
            }
            RequestEvent::ItemValueInput(item, index, value) => {
                project.update_request_item(item, index, value, false);
            }
            RequestEvent::NameInput(name) => {
                if let Some(req) = project.current_request_mut() {
                    req.name = Some(name);
                }
            }
            RequestEvent::RemoveItem(item, index) => {
                if let Some(req) = project.current_request_mut() {
                    req.remove_item(item, index);
                }
            }
            RequestEvent::Add(request) => {
                project.add_request("root", request);
            }
            RequestEvent::Select(id) => {
                project.set_current_request(id);
            }
            RequestEvent::Delete(id) => {
                project.remove_request("root".into(), id);
            }
            RequestEvent::AuthorizationInput(auth) => {
                if let Some(req) = project.current_request_mut() {
                    req.set_auth(auth);
                }
            }
            RequestEvent::UrlInput(url) => {
                project.update_request_url(url.into(), base_url);
            }
            RequestEvent::New => {
                project.add_new_request(base_url);
            }
        }
    }
}
