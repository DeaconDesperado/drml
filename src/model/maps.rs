use oxrdf::NamedNode;

use super::expression::Expression;
use super::function::{DatatypeMap, GatherMapMixin, Gatherable, LanguageMap};
use super::ExpressionMap;
use crate::vocab::rml;

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateMap {
    pub expression: Expression,
    pub term_type: NamedNode, // Always IRI for predicate maps
}

impl PredicateMap {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression,
            term_type: NamedNode::new_unchecked(rml::TermType::IRI),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectMap {
    pub expression: Expression,
    pub term_type: NamedNode,
    pub datatype_map: Option<DatatypeMap>,
    pub language_map: Option<LanguageMap>,
    pub gather_map: Option<GatherMapMixin>,
}

impl ObjectMap {
    pub fn new(
        expression: Expression,
        term_type_iri: String,
        datatype_map: Option<DatatypeMap>,
        language_map: Option<LanguageMap>,
        gather_map: Option<GatherMapMixin>,
    ) -> Self {
        Self {
            expression: expression,
            term_type: NamedNode::new_unchecked(term_type_iri),
            datatype_map: datatype_map,
            language_map: language_map,
            gather_map: gather_map,
        }
    }
}

impl Gatherable for ObjectMap {
    fn get_gather_map(&self) -> Option<GatherMapMixin> {
        self.gather_map.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphMap {
    pub expression: Expression,
    pub term_type: NamedNode,
}

impl GraphMap {
    pub fn new(expression: Expression, term_type_iri: String) -> Self {
        Self {
            expression,
            term_type: NamedNode::new_unchecked(term_type_iri),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubjectMap {
    pub expression: Expression,
    pub term_type: NamedNode,
    pub classes: Vec<String>,
    pub graph_maps: Vec<GraphMap>,
    pub gather_map: Option<GatherMapMixin>,
}

impl SubjectMap {
    pub fn new(
        expression: Expression,
        term_type_iri: String,
        classes: Vec<String>,
        graph_maps: Vec<GraphMap>,
        gather_map: Option<GatherMapMixin>,
    ) -> Self {
        Self {
            expression,
            term_type: NamedNode::new_unchecked(term_type_iri),
            classes,
            graph_maps,
            gather_map,
        }
    }
}

impl Gatherable for SubjectMap {
    fn get_gather_map(&self) -> Option<GatherMapMixin> {
        self.gather_map.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct JoinCondition {
    pub parent_map: ExpressionMap,
    pub child_map: ExpressionMap,
}

impl JoinCondition {
    pub fn new(parent_map: ExpressionMap, child_map: ExpressionMap) -> Self {
        Self {
            parent_map,
            child_map,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferencingObjectMap {
    pub uri: Option<String>,
    pub parent_uri: String,
    pub join_conditions: Vec<JoinCondition>,
    pub gather_map: Option<GatherMapMixin>,
}

impl ReferencingObjectMap {
    pub fn new(
        uri: Option<String>,
        parent_uri: String,
        join_conditions: Vec<JoinCondition>,
        gather_map: Option<GatherMapMixin>,
    ) -> Self {
        Self {
            uri,
            parent_uri,
            join_conditions,
            gather_map,
        }
    }

    pub fn get_references(&self) -> Vec<String> {
        self.join_conditions
            .iter()
            .flat_map(|jc| jc.child_map.get_references())
            .collect()
    }
}

impl Gatherable for ReferencingObjectMap {
    fn get_gather_map(&self) -> Option<GatherMapMixin> {
        self.gather_map.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateObjectMap {
    pub uri: Option<String>,
    pub predicate_maps: Vec<PredicateMap>,
    pub object_maps: Vec<ObjectMap>,
    pub ref_object_maps: Vec<ReferencingObjectMap>,
    pub graph_maps: Vec<GraphMap>,
}

impl PredicateObjectMap {
    pub fn new(
        uri: Option<String>,
        predicate_maps: Vec<PredicateMap>,
        object_maps: Vec<ObjectMap>,
        ref_object_maps: Vec<ReferencingObjectMap>,
        graph_maps: Vec<GraphMap>,
    ) -> Self {
        Self {
            uri,
            predicate_maps,
            object_maps,
            ref_object_maps,
            graph_maps,
        }
    }
}
