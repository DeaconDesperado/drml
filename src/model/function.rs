use oxrdf::NamedNode;
use std::collections::{HashMap, HashSet};

use super::expression::Expression;
use crate::vocab::rml;

pub trait RmlIteration {
    fn get_values_for(&self, reference: &str) -> Vec<String>;
    fn get_strings_for(&self, reference: &str) -> Vec<String>;
    fn get_nulls(&self) -> HashSet<String>;
}

pub trait Gatherable {
    fn get_gather_map(&self) -> Option<GatherMapMixin>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct GatherMapMixin {
    // Placeholder for gather map functionality
    // This would be implemented based on specific gather map requirements
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionMap {
    pub expression: Expression,
    pub term_type: NamedNode, // Always IRI for function maps
}

impl FunctionMap {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression,
            term_type: NamedNode::new_unchecked(rml::TermType::IRI),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterMap {
    pub expression: Expression,
    pub term_type: NamedNode, // Always IRI for parameter maps
}

impl ParameterMap {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression,
            term_type: NamedNode::new_unchecked(rml::TermType::IRI),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputValueMap {
    pub expression: Expression,
    pub term_type: String,
    pub datatype_map: Option<DatatypeMap>,
    pub language_map: Option<LanguageMap>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatatypeMap {
    pub expression: Expression,
}

impl DatatypeMap {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LanguageMap {
    pub expression: Expression,
}

impl LanguageMap {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnMap {
    pub expression: Expression,
    pub term_type: NamedNode, // Always IRI for return maps
}

impl ReturnMap {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression,
            term_type: NamedNode::new_unchecked(rml::TermType::IRI),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Input {
    pub parameter_map: ParameterMap,
    pub input_value_map: InputValueMap,
}

impl Input {
    pub fn new(parameter_map: ParameterMap, input_value_map: InputValueMap) -> Self {
        Self {
            parameter_map,
            input_value_map,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub returns: HashMap<String, String>,
    pub default_value: Option<String>,
}

impl Return {
    pub fn new(default_value: Option<String>) -> Self {
        Self {
            returns: HashMap::new(),
            default_value,
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.returns.get(key)
    }

    pub fn put(&mut self, key: String, value: String) -> Option<String> {
        self.returns.insert(key, value)
    }

    pub fn get_default_value(&self) -> Option<&String> {
        self.default_value.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RMLFunction {
    // Placeholder for RML function definitions
    pub uri: String,
    pub description: Option<String>,
}
