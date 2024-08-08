mod json;
mod types;
mod util;

use std::collections::HashMap;

use pest::iterators::{Pair, Pairs};
use pest::{error::Error, Parser};
use pest_derive::Parser;
use types::DraftLangAst;

#[derive(Parser)]
#[grammar = "grammer/draftlang.pest"]
pub struct DraftLangParser;

pub fn parse_file(source: &str) -> Result<DraftLangAst, Error<Rule>> {
    let pairs = DraftLangParser::parse(Rule::program, source)?
        .next()
        .unwrap();
    let mut json_map = HashMap::new();
    let mut script_map = HashMap::new();
    match pairs.as_rule() {
        Rule::draft_lang => {
            for pair in pairs.into_inner() {
                parse_draftlang_script(pair, &mut json_map)
            }
        }
        _ => {}
    }
    Ok(DraftLangAst::new(json_map, script_map))
}

fn parse_draftlang_script(pair: Pair<Rule>, json_map: &mut HashMap<String, String>) {
    match pair.as_rule() {
        Rule::expr => {}
        Rule::json_object => json::parse_json(pair, json_map),
        _ => {}
    }
}
