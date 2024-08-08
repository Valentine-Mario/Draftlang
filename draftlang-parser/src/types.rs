use std::collections::HashMap;

use crate::json::JSONValue;

#[derive(Debug, Clone)]
pub struct DraftLangAst {
    pub json: JSONValue,
    pub script: HashMap<String, Vec<AstNode>>,
}

impl DraftLangAst {
    pub fn new(data: HashMap<String, String>, script: HashMap<String, Vec<AstNode>>) -> Self {
        DraftLangAst {
            json: JSONValue { data },
            script,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Ident(String),
    Str(String),
    Number(f64),
}
