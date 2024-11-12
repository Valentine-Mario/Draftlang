use draftlang_parser::types::AstNode;

use crate::types::*;

pub fn str_len(value: &[AstNode]) -> AstNode {
    if value.is_empty() {
        panic!("Invalid parameter length. Please suply a parameter")
    }
    let is_str = is_string(value.first().unwrap());
    if !is_str {
        panic!("Invalid parameter type. Please suply a string")
    }
    let value = get_string_value(value.first().unwrap());
    AstNode::Number(value.len() as f64)
}

pub fn str_concat(value: &[AstNode]) -> AstNode {
    let concat_string = value.iter().map(|x| x.to_string()).collect();
    AstNode::Str(concat_string)
}

pub fn str_index_of(value: &[AstNode]) -> AstNode {
    if value.len() < 2 {
        panic!("Invalid parameter length. Please suply a parameter")
    }
    let is_str = is_string(value.first().unwrap());
    if !is_str {
        panic!("Invalid parameter type. Please suply a string")
    }

    let is_num = is_number(value.get(1).unwrap());
    if !is_num {
        panic!("Invalid parameter type. Please suply a number")
    }
    let str_val = get_string_value(value.first().unwrap());
    let index = get_string_value(value.get(1).unwrap());
    match str_val.find(index.as_str()) {
        Some(num) => AstNode::Number(num as f64),
        None => AstNode::Number(-1.0),
    }
}

pub fn str_substring(value: &[AstNode]) -> AstNode {
    if value.len() < 3 {
        panic!("Invalid parameter length. Please suply a parameter")
    }
    let is_str = is_string(value.first().unwrap());
    if !is_str {
        panic!("Invalid parameter type. Please suply a string")
    }

    let is_num = is_number(value.get(1).unwrap());
    if !is_num {
        panic!("Invalid parameter type. Please suply a number")
    }
    let is_num2 = is_number(value.get(2).unwrap());
    if !is_num2 {
        panic!("Invalid parameter type. Please suply a number")
    }
    let str_val = get_string_value(value.first().unwrap());
    let start = get_number_value(value.get(1).unwrap()) as usize;
    let end = get_number_value(value.get(2).unwrap()) as usize;
    AstNode::Str(str_val[start..end].to_string())
}

pub fn str_include(value: &[AstNode]) -> AstNode {
    if value.len() < 3 {
        panic!("Invalid parameter length. Please suply a parameter")
    }
    let is_str = is_string(value.first().unwrap());
    if !is_str {
        panic!("Invalid parameter type. Please suply a string")
    }

    let is_sub_str = is_string(value.get(1).unwrap());
    if !is_sub_str {
        panic!("Invalid parameter type. Please suply a string")
    }
    let str_val = get_string_value(value.first().unwrap());
    let sub_str_val = get_string_value(value.get(1).unwrap());
    AstNode::Boolean(str_val.contains(&sub_str_val))
}
