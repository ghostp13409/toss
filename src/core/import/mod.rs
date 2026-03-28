pub mod postman;

pub fn import_collection<P: AsRef<std::path::Path>>(path: P) -> Result<crate::core::collection::Collection, Box<dyn std::error::Error>> {
    postman::import_postman(path)
}
