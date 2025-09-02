pub const NS: &str = "http://w3id.org/rml/";

pub struct TermType;

impl TermType {
    pub const BLANK_NODE: &'static str = "http://w3id.org/rml/BlankNode";
    pub const IRI: &'static str = "http://w3id.org/rml/IRI";
    pub const LITERAL: &'static str = "http://w3id.org/rml/Literal";
}

pub struct ReferenceFormulation;

impl ReferenceFormulation {
    pub const CSV: &'static str = "http://w3id.org/rml/CSV";
    pub const JSON_PATH: &'static str = "http://w3id.org/rml/JSONPath";
    pub const XPATH: &'static str = "http://w3id.org/rml/XPath";
    pub const SQL2008_TABLE: &'static str = "http://w3id.org/rml/SQL2008Table";
    pub const SQL2008_QUERY: &'static str = "http://w3id.org/rml/SQL2008Query";
}

pub struct Classes;

impl Classes {
    pub const TRIPLES_MAP: &'static str = "http://w3id.org/rml/TriplesMap";
    pub const LOGICAL_SOURCE: &'static str = "http://w3id.org/rml/LogicalSource";
    pub const REF_OBJECT_MAP: &'static str = "http://w3id.org/rml/RefObjectMap";
}

pub struct Properties;

impl Properties {
    pub const CLASS: &'static str = "http://w3id.org/rml/class";
    pub const CHILD_MAP: &'static str = "http://w3id.org/rml/childMap";
    pub const CONSTANT: &'static str = "http://w3id.org/rml/constant";
    pub const DATATYPE_MAP: &'static str = "http://w3id.org/rml/datatypeMap";
    pub const FUNCTION_EXECUTION: &'static str = "http://w3id.org/rml/functionExecution";
    pub const FUNCTION_MAP: &'static str = "http://w3id.org/rml/functionMap";
    pub const GATHER: &'static str = "http://w3id.org/rml/gather";
    pub const GATHER_AS: &'static str = "http://w3id.org/rml/gatherAs";
    pub const GRAPH_MAP: &'static str = "http://w3id.org/rml/graphMap";
    pub const INPUT: &'static str = "http://w3id.org/rml/input";
    pub const INPUT_VALUE_MAP: &'static str = "http://w3id.org/rml/inputValueMap";
    pub const ITERATOR: &'static str = "http://w3id.org/rml/iterator";
    pub const JOIN_CONDITION: &'static str = "http://w3id.org/rml/joinCondition";
    pub const LANGUAGE_MAP: &'static str = "http://w3id.org/rml/languageMap";
    pub const LOGICAL_SOURCE: &'static str = "http://w3id.org/rml/logicalSource";
    pub const NULL: &'static str = "http://w3id.org/rml/null";
    pub const OBJECT_MAP: &'static str = "http://w3id.org/rml/objectMap";
    pub const PARAMETER_MAP: &'static str = "http://w3id.org/rml/parameterMap";
    pub const PARENT_MAP: &'static str = "http://w3id.org/rml/parentMap";
    pub const PARENT_TRIPLES_MAP: &'static str = "http://w3id.org/rml/parentTriplesMap";
    pub const PREDICATE: &'static str = "http://w3id.org/rml/predicate";
    pub const PREDICATE_MAP: &'static str = "http://w3id.org/rml/predicateMap";
    pub const PREDICATE_OBJECT_MAP: &'static str = "http://w3id.org/rml/predicateObjectMap";
    pub const REFERENCE: &'static str = "http://w3id.org/rml/reference";
    pub const REFERENCE_FORMULATION: &'static str = "http://w3id.org/rml/referenceFormulation";
    pub const RETURN_MAP: &'static str = "http://w3id.org/rml/returnMap";
    pub const SOURCE: &'static str = "http://w3id.org/rml/source";
    pub const SUBJECT_MAP: &'static str = "http://w3id.org/rml/subjectMap";
    pub const TEMPLATE: &'static str = "http://w3id.org/rml/template";
    pub const TERM_TYPE: &'static str = "http://w3id.org/rml/termType";
}
