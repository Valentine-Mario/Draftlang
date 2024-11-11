use draftlang_parser::types::AstNode;

pub fn str_len(value: &Vec<AstNode>) -> AstNode {
    if value.is_empty() {
        panic!("Invalid parameter length. Please suply a parameter")
    }
    let is_str = super::check_if_string(value.first().unwrap());
    if is_str == false {
        panic!("Invalid parameter type. Please suply a string")
    }
    let value = super::get_string_value(value.first().unwrap());
    AstNode::Number(value.len() as f64)
}

pub fn str_concat(value: &Vec<AstNode>) -> AstNode {
    let concat_string = value.iter().map(|x| x.to_string()).collect();
    AstNode::Str(concat_string)
}
