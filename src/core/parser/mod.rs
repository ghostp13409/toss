pub mod detector;
pub mod frameworks;

use crate::core::collection::{Collection, CollectionItem, Folder};
use std::path::Path;

pub trait SourceParser {
    fn parse(&self, project_path: &Path) -> anyhow::Result<Collection>;
}

pub fn parse_project<P: AsRef<Path>>(path: P) -> anyhow::Result<Collection> {
    let path = path.as_ref();
    
    // First try direct detection at root
    let framework = detector::detect_framework(path);
    if framework != detector::Framework::Unknown {
        let col = parse_single_project(path, framework)?;
        if col.items.is_empty() {
            anyhow::bail!("Found project at {:?} but no endpoints were extracted", path);
        }
        return Ok(col);
    }

    // If not found at root, discover recursively
    let discovered = detector::discover_projects(path);
    if discovered.is_empty() {
        anyhow::bail!("Unsupported or unknown framework at {:?}", path);
    }

    let mut master_collection = Collection::new(format!(
        "{} (Workspace)",
        path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Root".to_string())
    ));

    for (p, f) in discovered {
        match parse_single_project(&p, f) {
            Ok(col) => {
                if !col.items.is_empty() {
                    let mut folder = Folder::new(col.name.clone());
                    folder.items = col.items;
                    master_collection.items.push(CollectionItem::Folder(folder));
                }
            }
            Err(e) => {
                eprintln!("Warning: failed to parse project at {:?}: {}", p, e);
            }
        }
    }

    if master_collection.items.is_empty() {
        anyhow::bail!("Found projects but no endpoints were extracted from any of them at {:?}", path);
    }

    Ok(master_collection)
}

fn parse_single_project(path: &Path, framework: detector::Framework) -> anyhow::Result<Collection> {
    match framework {
        detector::Framework::Express => {
            let parser = frameworks::express::ExpressParser;
            parser.parse(path)
        }
        detector::Framework::FastAPI => {
            let parser = frameworks::fastapi::FastApiParser;
            parser.parse(path)
        }
        detector::Framework::Spring => {
            let parser = frameworks::spring::SpringParser;
            parser.parse(path)
        }
        _ => anyhow::bail!("Unsupported framework {:?} at {:?}", framework, path),
    }
}
