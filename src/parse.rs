use oxrdf::Dataset;

pub struct RmlMappingParser {
    dataset: Dataset,
    mapping_path: String,
}

impl RmlMappingParser {
    pub fn new(dataset: Dataset, mapping_path: String) -> Self {
        Self {
            dataset,
            mapping_path,
        }
    }

    pub fn parse(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Parsing RML mappings from path: {}", self.mapping_path);
        Ok(())
    }
}
