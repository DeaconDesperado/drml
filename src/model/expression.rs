use serde::{Deserialize, Serialize};

use super::function::{FunctionMap, Input, ReturnMap};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    RDFNodeConstant {
        constant: String,
    },
    Template {
        template: String,
    },
    Reference {
        reference: String,
    },
    FunctionExecution {
        function_map: Box<FunctionMap>,
        return_map: Option<Box<ReturnMap>>,
        inputs: Vec<Input>,
    },
}

impl Expression {
    /// Obtain a Vec of reference expressions, eg "$.foo" in case of JSONPath formulation
    /// See <https://kg-construct.github.io/rml-core/spec/docs/#dfn-reference-expression>
    pub fn references(&self) -> Vec<String> {
        match self {
            Expression::RDFNodeConstant { .. } => Vec::new(),
            Expression::Template { template } => extract_template_references(template),
            Expression::Reference { reference } => vec![reference.clone()],
            Expression::FunctionExecution { .. } => Vec::new(), // Functions don't have direct references
        }
    }
}

fn extract_template_references(template: &str) -> Vec<String> {
    let mut references = Vec::new();
    let mut chars = template.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' && chars.peek() != Some(&'\\') {
            let mut reference = String::new();
            let mut escaped = false;

            while let Some(ch) = chars.next() {
                if ch == '\\' && !escaped {
                    escaped = true;
                    continue;
                }

                if ch == '}' && !escaped {
                    break;
                }

                reference.push(ch);
                escaped = false;
            }

            if !reference.is_empty() {
                references.push(reference);
            }
        }
    }

    references
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpressionMap {
    pub expression: Expression,
}

impl ExpressionMap {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }

    pub fn get_references(&self) -> Vec<String> {
        self.expression.references()
    }
}
