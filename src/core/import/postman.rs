use crate::core::collection::{Collection, CollectionItem, Folder, Request};
use crate::cli::args::Method;
use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PmCollection {
    info: PmInfo,
    item: Vec<PmItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PmInfo {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PmItem {
    name: Option<String>,
    request: Option<PmRequest>,
    item: Option<Vec<PmItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PmRequest {
    method: String,
    url: serde_json::Value,
    header: Option<serde_json::Value>,
    body: Option<PmBody>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PmBody {
    raw: Option<String>,
}

pub fn import_postman<P: AsRef<Path>>(path: P) -> Result<Collection, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let pm: PmCollection = serde_json::from_str(&content)?;
    
    let mut items = Vec::new();
    for item in pm.item {
        items.push(convert_item(item));
    }
    
    Ok(Collection {
        id: uuid::Uuid::new_v4().to_string(),
        name: pm.info.name,
        items,
        expanded: false,
    })
}

fn convert_item(pm_item: PmItem) -> CollectionItem {
    if let Some(request) = pm_item.request {
        let method = match request.method.to_uppercase().as_str() {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "PATCH" => Method::Patch,
            "DELETE" => Method::Delete,
            _ => Method::Get,
        };

        let url = if request.url.is_string() {
            request.url.as_str().unwrap_or_default().to_string()
        } else if let Some(raw) = request.url.get("raw") {
            raw.as_str().unwrap_or_default().to_string()
        } else {
            String::new()
        };

        let mut headers = HashMap::new();
        if let Some(h_val) = request.header {
            if let Some(arr) = h_val.as_array() {
                for h in arr {
                    if let (Some(k), Some(v)) = (h.get("key"), h.get("value")) {
                        headers.insert(k.as_str().unwrap_or_default().to_string(), v.as_str().unwrap_or_default().to_string());
                    }
                }
            }
        }

        let body = request.body.and_then(|b| b.raw);

        CollectionItem::Request(Request {
            id: uuid::Uuid::new_v4().to_string(),
            name: pm_item.name.unwrap_or_else(|| "Unnamed Request".to_string()),
            method,
            url,
            headers,
            body,
        })
    } else {
        let mut items = Vec::new();
        if let Some(pm_items) = pm_item.item {
            for item in pm_items {
                items.push(convert_item(item));
            }
        }

        CollectionItem::Folder(Folder {
            id: uuid::Uuid::new_v4().to_string(),
            name: pm_item.name.unwrap_or_else(|| "Unnamed Folder".to_string()),
            items,
            expanded: false,
        })
    }
}
