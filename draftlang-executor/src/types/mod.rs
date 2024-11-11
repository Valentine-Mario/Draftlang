use std::collections::HashMap;

use draftlang_parser::types::AstNode;

/// Check if the item is a string
pub fn is_string(item: &AstNode) -> bool {
    matches!(item, AstNode::Str(_))
}

/// Check if the item is a number
pub fn is_number(item: &AstNode) -> bool {
    matches!(item, AstNode::Number(_))
}

/// check if the item is a boolean
pub fn is_boolean(item: &AstNode) -> bool {
    matches!(item, AstNode::Boolean(_))
}

/// Check if the item is an array
pub fn is_array(item: &AstNode) -> bool {
    matches!(item, AstNode::Array(_))
}

/// Check if the item is a map
pub fn is_map(item: &AstNode) -> bool {
    matches!(item, AstNode::Map(_))
}

/// Check if the item is a buffer
pub fn is_buffer(item: &AstNode) -> bool {
    matches!(item, AstNode::Buffer(_))
}

/// Check if the item is null
pub fn is_null(item: &AstNode) -> bool {
    matches!(item, AstNode::Null)
}

/// Return the string value of the given AstNode if it is a string.
/// If AstNode is not a string, an empty string is returned.
pub fn get_string_value(ast_node: &AstNode) -> String {
    match ast_node {
        AstNode::Str(string) => string.clone(),
        _ => String::new(),
    }
}

/// Return the number value of the given AstNode if it is a number.
/// If AstNode is not a number, 0.0 is returned.
pub fn get_number_value(ast_node: &AstNode) -> f64 {
    match ast_node {
        AstNode::Number(number) => *number,
        _ => 0.0,
    }
}

/// Return the boolean value of the given AstNode if it is a boolean.
/// If AstNode is not a boolean, false is returned.
pub fn get_boolean_value(ast_node: &AstNode) -> bool {
    match ast_node {
        AstNode::Boolean(boolean) => *boolean,
        _ => false,
    }
}

/// Return the array value of the given AstNode if it is an array.
/// If AstNode is not an array, an empty vector is returned.
pub fn get_array_value(ast_node: &AstNode) -> Vec<AstNode> {
    match ast_node {
        AstNode::Array(array) => array.clone(),
        _ => Vec::new(),
    }
}

/// Return the map value of the given AstNode if it is a map.
/// If AstNode is not a map, an empty map is returned.
pub fn get_map_value(ast_node: &AstNode) -> HashMap<String, AstNode> {
    match ast_node {
        AstNode::Map(map) => map.clone(),
        _ => HashMap::new(),
    }
}

/// Return the buffer value of the given AstNode if it is a buffer.
/// If AstNode is not a buffer, an empty vector is returned.
pub fn get_buffer_value(ast_node: &AstNode) -> Vec<u8> {
    match ast_node {
        AstNode::Buffer(buffer) => buffer.clone(),
        _ => Vec::new(),
    }
}

pub fn check_same_type(arr: &[AstNode]) -> bool {
    let first_variant = arr.first();
    arr.iter().all(|x| {
        matches!(
            (first_variant, x),
            (Some(AstNode::Number(_)), AstNode::Number(_))
                | (Some(AstNode::Str(_)), AstNode::Str(_))
                | (Some(AstNode::Boolean(_)), AstNode::Boolean(_))
                | (Some(AstNode::Map(_)), AstNode::Map(_))
                | (Some(AstNode::Buffer(_)), AstNode::Buffer(_))
                | (Some(AstNode::Array(_)), AstNode::Array(_))
        )
    })
}
