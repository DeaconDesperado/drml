use std::path::PathBuf;

use oxrdf::Dataset;

pub struct RmlMappingParser {
    dataset: Dataset,
    mapping_path: PathBuf,
}

impl RmlMappingParser {
    pub fn new(dataset: Dataset, mapping_path: PathBuf) -> Self {
        Self {
            dataset,
            mapping_path,
        }
    }

    pub fn parse(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!(
            "Parsing RML mappings from path: {}",
            self.mapping_path.to_str().unwrap_or("")
        );
        Ok(())
    }
}
