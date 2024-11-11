use draftlang_parser::types::AstNode;

pub(crate) mod utils;

fn check_if_string(item: &AstNode) -> bool {
    match item {
        AstNode::Str(_) => true,
        _ => false,
    }
}

fn get_string_value(item: &AstNode) -> String {
    match item {
        AstNode::Str(value) => value.to_string(),
        _ => "".to_string(),
    }
}

/// The string module provides functions to manipulate strings
///
/// # Examples
///
///
pub fn string(ident: &str, payload: &Vec<AstNode>) -> AstNode {
    match ident {
        "str_len" => utils::str_len(payload),
        "str_concat" => utils::str_concat(payload),
        _ => panic!("cannot find module {} in string", ident),
    }
}
