mod json;
mod script;
pub mod types;
mod util;

use std::collections::HashMap;

use draftlang_error::Error as DError;
use pest::iterators::Pair;
use pest::{error::Error, Parser};
use pest_derive::Parser;
use types::{AstNode, DraftLangAst};

#[derive(Parser)]
#[grammar = "grammer/draftlang.pest"]
pub struct DraftLangParser;

fn parse_file(source: &str) -> Result<DraftLangAst, Box<Error<Rule>>> {
    let pairs = DraftLangParser::parse(Rule::program, source)?
        .next()
        .unwrap();

    let mut json_map = HashMap::new();
    let mut script_map = HashMap::new();
    let mut ast_script: Vec<AstNode> = vec![];

    if pairs.as_rule() == Rule::draft_lang {
        for pair in pairs.into_inner() {
            parse_draftlang_script(pair, &mut ast_script, &mut json_map)
        }
    }

    script_map.insert("global".to_string(), vec![]);
    script_map.insert("import".to_string(), vec![]);
    let script_parsed = map_script_ast(ast_script, script_map);

    Ok(DraftLangAst::new(json_map, script_parsed))
}

pub fn parse(source: &str) -> Result<DraftLangAst, DError> {
    match parse_file(source) {
        Ok(map) => Ok(map),
        Err(e) => {
            let mut s = "".to_string();
            match e.line_col {
                pest::error::LineColLocation::Pos((line, position)) => {
                    let error_pos =
                        format!("Syntax Error on line {:?} position {:?}\n", line, position);
                    s.push_str(error_pos.as_str());
                }
                _ => todo!(),
            }
            s.push_str(e.line());
            Err(DError::ParsingError(Box::leak(s.into_boxed_str())))
        }
    }
}

fn parse_draftlang_script(
    pair: Pair<Rule>,
    ast_script: &mut Vec<AstNode>,
    json_map: &mut HashMap<String, Vec<(AstNode, Vec<AstNode>)>>,
) {
    match pair.as_rule() {
        Rule::expr => {
            ast_script.push(script::parse_script(pair.into_inner().next().unwrap()));
        }
        Rule::json_object => json::parse_json(pair, json_map),
        _ => {}
    }
}

fn map_script_ast(
    ast: Vec<AstNode>,
    mut map: HashMap<String, Vec<AstNode>>,
) -> HashMap<String, Vec<AstNode>> {
    for item in ast {
        match item {
            AstNode::Import { funcs, module } => {
                let import_vector = map.get_mut("import").unwrap();
                import_vector.push(AstNode::Import { funcs, module });
            }
            AstNode::Assignment { ident, expr } => {
                let assigment_vector = map.get_mut("global").unwrap();
                assigment_vector.push(AstNode::Assignment { ident, expr })
            }
            AstNode::Function { name, params, expr } => match *name.clone() {
                AstNode::Ident(key) => {
                    map.insert(key, vec![AstNode::Function { name, params, expr }]);
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    map
}
