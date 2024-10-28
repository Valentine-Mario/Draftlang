use draftlang_error::Error;
use draftlang_parser::{
    parse,
    types::{AstNode, DraftLangAst},
};
use std::{collections::HashMap, fs, path::PathBuf};

pub fn read_draftlang_file(file_path: &PathBuf) -> Result<String, Error> {
    match fs::read_to_string(file_path) {
        Ok(data) => Ok(data),
        Err(err) => Err(Error::ErrorReadingFile(Box::leak(
            err.to_string().into_boxed_str(),
        ))),
    }
}

pub fn parse_file(content: &str) -> Result<DraftLangAst, Error> {
    parse(content)
}

pub fn parse_imports(imports: Option<&Vec<AstNode>>) -> HashMap<String, Vec<String>> {
    let mut value = HashMap::new();
    if imports.is_none() {
        value
    } else {
        let imports_list = imports.unwrap();
        for item in imports_list {
            match item {
                AstNode::Import { funcs, module } => {
                    let mut parent_list = vec![];
                    match *module.clone() {
                        AstNode::ModuleImport { parent, child } => {
                            parent_list.push(parent.to_string());
                            parent_list.push(child.to_string());
                        }
                        _ => unreachable!(),
                    }
                    for func in funcs {
                        parent_list.push(func.to_string());
                        value.insert(func.to_string(), parent_list.clone());
                    }
                }
                _ => unreachable!(),
            }
        }
        value
    }
}

pub fn append_default_import() -> HashMap<String, Vec<String>> {
    let mut value = HashMap::new();
    const DEFAULT_IMPORTS: [[&str; 3]; 2] =
        [["core", "default", "print"], ["core", "eval", "exec"]];

    for item in DEFAULT_IMPORTS {
        value.insert(
            item[2].to_string(),
            vec![
                item[0].to_string(),
                item[1].to_string(),
                item[2].to_string(),
            ],
        );
    }
    value
}

pub fn parse_global_scope(global_scope: Vec<AstNode>) -> HashMap<String, AstNode> {
    let mut value = HashMap::new();
    for item in global_scope {
        match item {
            AstNode::Assignment { ident, expr } => {
                value.insert(ident.to_string(), *expr.clone());
            }
            _ => unreachable!(),
        }
    }
    value
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
