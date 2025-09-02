use std::collections::HashSet;

use super::maps::{PredicateObjectMap, SubjectMap};

pub trait LogicalSource {
    fn get_nulls(&self) -> HashSet<String>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct TriplesMap {
    pub uri: String,
    pub logical_source: LogicalSourceType,
    pub subject_map: SubjectMap,
    pub predicate_object_maps: Vec<PredicateObjectMap>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalSourceType {
    File(FileLogicalSource),
    Database(DatabaseLogicalSource),
}

impl LogicalSource for LogicalSourceType {
    fn get_nulls(&self) -> HashSet<String> {
        match self {
            LogicalSourceType::File(file_source) => file_source.get_nulls(),
            LogicalSourceType::Database(db_source) => db_source.get_nulls(),
        }
    }
}

impl TriplesMap {
    pub fn new(
        uri: String,
        logical_source: LogicalSourceType,
        subject_map: SubjectMap,
        predicate_object_maps: Vec<PredicateObjectMap>,
    ) -> Self {
        Self {
            uri,
            logical_source,
            subject_map,
            predicate_object_maps,
        }
    }

    pub fn joins(&self) -> Vec<&PredicateObjectMap> {
        self.predicate_object_maps
            .iter()
            .filter(|pom| !pom.ref_object_maps.is_empty())
            .collect()
    }

    pub fn get_all_reference_fields(&self) -> HashSet<String> {
        let mut references = HashSet::new();

        // Subject map references
        references.extend(self.subject_map.expression.references());

        // Predicate and object map references
        for pom in &self.predicate_object_maps {
            // Referencing object map references
            for rom in &pom.ref_object_maps {
                references.extend(rom.get_references());
            }

            // Object map references
            for om in &pom.object_maps {
                references.extend(om.expression.references());
            }

            // Predicate map references
            for pm in &pom.predicate_maps {
                references.extend(pm.expression.references());
            }
        }

        references
            .into_iter()
            .map(|s| s.trim_start_matches(&['$', '.'][..]).to_string())
            .collect()
    }
}

// Basic logical source implementations
#[derive(Debug, Clone, PartialEq)]
pub struct FileLogicalSource {
    pub source: String,
    pub reference_formulation: Option<String>,
    pub iterator: Option<String>,
    pub nulls: HashSet<String>,
}

impl LogicalSource for FileLogicalSource {
    fn get_nulls(&self) -> HashSet<String> {
        self.nulls.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseLogicalSource {
    pub source: String,
    pub query: Option<String>,
    pub nulls: HashSet<String>,
}

impl LogicalSource for DatabaseLogicalSource {
    fn get_nulls(&self) -> HashSet<String> {
        self.nulls.clone()
    }
}
