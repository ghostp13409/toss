use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::cli::args::Method;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub items: Vec<CollectionItem>,
    #[serde(default)]
    pub expanded: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum CollectionItem {
    Folder(Folder),
    Request(Request),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub items: Vec<CollectionItem>,
    #[serde(default)]
    pub expanded: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub id: String,
    pub name: String,
    pub method: Method,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Collection {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            items: Vec::new(),
            expanded: false,
        }
    }

    pub fn find_request_mut(&mut self, id: &str) -> Option<&mut Request> {
        for item in &mut self.items {
            if let Some(req) = item.find_request_mut(id) {
                return Some(req);
            }
        }
        None
    }
}

impl Folder {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            items: Vec::new(),
            expanded: false,
        }
    }
}

impl CollectionItem {
    pub fn find_request_mut(&mut self, id: &str) -> Option<&mut Request> {
        match self {
            CollectionItem::Request(req) => {
                if req.id == id {
                    Some(req)
                } else {
                    None
                }
            }
            CollectionItem::Folder(f) => {
                for item in &mut f.items {
                    if let Some(req) = item.find_request_mut(id) {
                        return Some(req);
                    }
                }
                None
            }
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            CollectionItem::Folder(f) => &f.name,
            CollectionItem::Request(r) => &r.name,
        }
    }

    #[allow(dead_code)]
    pub fn set_name(&mut self, name: String) {
        match self {
            CollectionItem::Folder(f) => f.name = name,
            CollectionItem::Request(r) => r.name = name,
        }
    }
}
