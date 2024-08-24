use std::collections::HashMap;

use draftlang_parser::types::AstNode;

pub struct FunctionExecxutor {
    pub name: AstNode,
    pub parameters: Vec<AstNode>,
    pub function_scope: HashMap<String, AstNode>,
    pub global_scope: HashMap<String, AstNode>,
    pub function_body: Vec<AstNode>,
    pub return_value: AstNode,
}
