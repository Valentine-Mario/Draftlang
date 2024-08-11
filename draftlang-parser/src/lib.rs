mod json;
mod script;
mod types;
mod util;

use std::collections::HashMap;

use pest::iterators::Pair;
use pest::{error::Error, Parser};
use pest_derive::Parser;
use types::{AstNode, DraftLangAst};

#[derive(Parser)]
#[grammar = "grammer/draftlang.pest"]
pub struct DraftLangParser;

pub fn parse_file(source: &str) -> Result<DraftLangAst, Box<Error<Rule>>> {
    let pairs = DraftLangParser::parse(Rule::program, source)?
        .next()
        .unwrap();

    let mut json_map = HashMap::new();
    let mut script_map = HashMap::new();
    let mut ast_script: Vec<AstNode> = vec![];

    match pairs.as_rule() {
        Rule::draft_lang => {
            for pair in pairs.into_inner() {
                parse_draftlang_script(pair, &mut ast_script, &mut json_map)
            }
        }
        _ => {}
    }
    println!("ast script {:#?}", ast_script);

    Ok(DraftLangAst::new(json_map, script_map))
}

fn parse_draftlang_script(
    pair: Pair<Rule>,
    ast_script: &mut Vec<AstNode>,
    json_map: &mut HashMap<String, String>,
) {
    match pair.as_rule() {
        Rule::expr => {
            ast_script.push(script::parse_script(pair.into_inner().next().unwrap()));
        }
        Rule::json_object => json::parse_json(pair, json_map),
        _ => {}
    }
}
