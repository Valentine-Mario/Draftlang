use std::collections::HashMap;

use draftlang_parser::types::AstNode;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionExecxutor {
    pub scripts: HashMap<String, Vec<AstNode>>,
    pub name: AstNode,
    pub parameters: Vec<AstNode>,
    pub function_scope: HashMap<String, AstNode>,
    pub global_scope: HashMap<String, AstNode>,
    pub function_body: Vec<AstNode>,
    pub return_value: AstNode,
    pub import_value: HashMap<String, Vec<String>>,
}
