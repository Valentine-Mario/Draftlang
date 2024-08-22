use std::collections::HashMap;

use pest::iterators::Pair;

use crate::{script::parse_func_signature, types::AstNode, util, Rule};

//drftlang json interface only takes in a json key
//of string and value of string. Other types are
//not supported on the json interface
#[derive(Debug, Clone)]
pub struct JSONValue {
    pub data: HashMap<String, Vec<(AstNode, Vec<AstNode>)>>,
}

pub fn parse_json(pair: Pair<Rule>, json_map: &mut HashMap<String, Vec<(AstNode, Vec<AstNode>)>>) {
    match pair.as_rule() {
        Rule::json_object => {
            for item in pair.into_inner() {
                parse_json(item, json_map)
            }
        }
        Rule::json_pair => {
            let mut inner_rules = pair.into_inner();
            let key = util::unescape_string(inner_rules.next().unwrap().as_str());
            let value = inner_rules.next().unwrap().into_inner();
            let function_signature: Vec<(AstNode, Vec<AstNode>)> =
                value.map(|x| parse_func_signature(x)).collect();
            json_map.insert(key.to_string(), function_signature);
        }
        _ => {}
    }
}
