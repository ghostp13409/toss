use crate::cli::args::Method;
use crate::core::collection::{Auth, Collection, CollectionItem, Folder, KVParam, Request, RequestBody};
use crate::core::parser::SourceParser;
use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;

pub struct SpringParser;

impl SourceParser for SpringParser {
    fn parse(&self, project_path: &Path) -> anyhow::Result<Collection> {
        let mut collection = Collection::new(format!(
            "{} (Spring Boot)",
            project_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
        ));

        collection.env_vars.push(KVParam {
            key: "baseUrl".to_string(),
            value: "http://localhost:8080".to_string(),
            enabled: true,
            description: Some("Base URL for the service".to_string()),
        });

        // Matches @GetMapping("/path"), @PostMapping("/path"), etc.
        let mapping_regex = Regex::new(
            r#"@(Get|Post|Put|Delete|Patch)Mapping\s*\(\s*(?:value\s*=\s*)?['"]([^'"]+)['"]"#,
        )
        .unwrap();

        // Matches @RequestMapping(value = "/path", method = RequestMethod.GET)
        let request_mapping_regex = Regex::new(
            r#"@RequestMapping\s*\(\s*(?:value\s*=\s*)?['"]([^'"]+)['"](?:.*method\s*=\s*RequestMethod\.(GET|POST|PUT|DELETE|PATCH))?"#,
        )
        .unwrap();

        for entry in WalkDir::new(project_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "java" || ext == "kt"))
        {
            let path_str = entry.path().to_string_lossy();
            if path_str.contains("target") || path_str.contains(".git") {
                continue;
            }

            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                // Skip Feign Clients and only include actual Controllers
                if content.contains("@FeignClient") {
                    continue;
                }
                
                if !content.contains("@RestController") && 
                   !content.contains("@Controller") && 
                   !content.contains("@RequestMapping") {
                    continue;
                }

                let mut requests = Vec::new();

                // Check for @XMapping
                for cap in mapping_regex.captures_iter(&content) {
                    let method_prefix = &cap[1];
                    let url_path = &cap[2];

                    let method = match method_prefix.to_lowercase().as_str() {
                        "post" => Method::Post,
                        "put" => Method::Put,
                        "patch" => Method::Patch,
                        "delete" => Method::Delete,
                        _ => Method::Get,
                    };

                    requests.push(CollectionItem::Request(Request {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: format!("{} {}", method_prefix.to_uppercase(), url_path),
                        method,
                        url: format!("{{{{baseUrl}}}}{}", url_path),
                        params: Vec::new(),
                        headers: Vec::new(),
                        auth: Auth::None,
                        body: RequestBody::None,
                        pre_request_script: None,
                        post_response_script: None,
                    }));
                }

                // Check for @RequestMapping
                for cap in request_mapping_regex.captures_iter(&content) {
                    let url_path = &cap[1];
                    let method_str = cap.get(2).map(|m| m.as_str()).unwrap_or("GET");

                    let method = match method_str.to_uppercase().as_str() {
                        "POST" => Method::Post,
                        "PUT" => Method::Put,
                        "PATCH" => Method::Patch,
                        "DELETE" => Method::Delete,
                        _ => Method::Get,
                    };

                    requests.push(CollectionItem::Request(Request {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: format!("{} {}", method_str.to_uppercase(), url_path),
                        method,
                        url: format!("{{{{baseUrl}}}}{}", url_path),
                        params: Vec::new(),
                        headers: Vec::new(),
                        auth: Auth::None,
                        body: RequestBody::None,
                        pre_request_script: None,
                        post_response_script: None,
                    }));
                }

                if !requests.is_empty() {
                    let file_name = entry
                        .path()
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let mut folder = Folder::new(file_name);
                    folder.items = requests;
                    collection.items.push(CollectionItem::Folder(folder));
                }
            }
        }

        Ok(collection)
    }
}
