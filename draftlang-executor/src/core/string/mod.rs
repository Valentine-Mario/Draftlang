use draftlang_parser::types::AstNode;

mod utils;

fn check_if_string(item: &AstNode) -> bool {
    match item {
        AstNode::Str(_) => true,
        _ => false,
    }
}

fn check_if_array_of_strings(item: &AstNode) -> bool {
    match item {
        AstNode::Array(array_value) => array_value.iter().all(|x| match x {
            AstNode::Str(_) => true,
            _ => false,
        }),
        _ => false,
    }
}
fn get_string_value(item: &AstNode) -> String {
    match item {
        AstNode::Str(value) => value.to_string(),
        _ => "".to_string(),
    }
}

fn get_array_values(item: &AstNode) -> Vec<AstNode> {
    match item {
        AstNode::Array(value) => value.to_vec(),
        _ => vec![],
    }
}

/// The string module provides functions to manipulate strings
///
/// # Examples
///
///
pub fn string(ident: &str, payload: &AstNode) -> AstNode {
    if !check_if_string(payload) {
        panic!("unable to run operation on none string type");
    }
    let payload = &get_string_value(payload);
    match ident {
        "str_len" => utils::str_len(payload),
        "is_str_empty" => utils::is_str_empty(payload),
        _ => panic!("cannot find module {} in string", ident),
    }
}

/// The string module provides functions to manipulate array of strings
///
/// # Examples
///
///
pub fn string_utility(ident: &str, payload: &AstNode) {
    if !check_if_array_of_strings(payload) {
        panic!("unable to run operation on none string array type");
    }
    let payload = get_array_values(&payload);

    match ident {
        _ => panic!("cannot find module {} in string utility", ident),
    }
}
