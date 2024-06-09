use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    ui::elements::select_options::SelectOption,
    utils::request::{http_method::HttpMethod, PendingRequest, PendingRequestItem},
};

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
