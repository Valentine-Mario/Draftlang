use draftlang_parser::types::AstNode;

pub fn str_len(value: &String) -> AstNode {
    AstNode::Number(value.len() as f64)
}

pub fn is_str_empty(value: &String) -> AstNode {
    AstNode::Boolean(value.len() < 1)
}
