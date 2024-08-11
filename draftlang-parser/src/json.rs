use std::collections::HashMap;

use pest::iterators::Pair;

use crate::{util, Rule};

//drftlang json interface only takes in a json key
//of string and value of string. Other types are
//not supported on the json interface
#[derive(Debug, Clone)]
pub struct JSONValue {
    pub(crate) data: HashMap<String, String>,
}

pub fn parse_json(pair: Pair<Rule>, json_map: &mut HashMap<String, String>) {
    match pair.as_rule() {
        Rule::json_object => {
            for item in pair.into_inner() {
                parse_json(item, json_map)
            }
        }
        Rule::json_pair => {
            let mut inner_rules = pair.into_inner();
            let key = util::unescape_string(inner_rules.next().unwrap().as_str());
            let value = util::unescape_string(inner_rules.next().unwrap().as_str());
            json_map.insert(key.to_string(), value.to_string());
        }
        _ => {}
    }
}
