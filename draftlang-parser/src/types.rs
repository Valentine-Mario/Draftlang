use std::collections::HashMap;

use pest::iterators::Pair;

use crate::{json::JSONValue, Rule};

#[derive(Debug, Clone)]
pub struct DraftLangAst {
    pub json: JSONValue,
    pub script: HashMap<String, Vec<AstNode>>,
}

impl DraftLangAst {
    pub fn new(data: HashMap<String, String>, script: HashMap<String, Vec<AstNode>>) -> Self {
        DraftLangAst {
            json: JSONValue { data },
            script,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Ident(String),
    Str(String),
    Number(f64),
    Null,
    Map(HashMap<String, AstNode>),
    Array(Vec<AstNode>),
    Boolean(bool),
    Assignment {
        ident: Box<AstNode>,
        expr: Box<AstNode>,
    },
    Import {
        funcs: Vec<AstNode>,
        module: Box<AstNode>,
    },
}

#[derive(Clone, Debug)]
pub enum Verb {
    Plus,
    NotEqual,
    LessThan,
    LargerThan,
    LessThanOrEqual,
    LargerThanOrEqual,
    Equal,
    And,
    Or,
    Pipe,
}

pub fn parse_verb(pair: Pair<Rule>) -> Verb {
    match pair.as_str() {
        "+" => Verb::Plus,
        "<" => Verb::LessThan,
        "==" => Verb::Equal,
        ">" => Verb::LargerThan,
        "&&" => Verb::And,
        "||" => Verb::Or,
        "|>" => Verb::Pipe,
        "!=" => Verb::NotEqual,
        "<=" => Verb::LessThanOrEqual,
        ">=" => Verb::LargerThanOrEqual,
        _ => panic!("Unexpected  verb: {}", pair.as_str()),
    }
}
