use draftlang_parser::types::AstNode;
pub(crate) mod utils;

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
