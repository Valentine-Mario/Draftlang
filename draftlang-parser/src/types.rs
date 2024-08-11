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
pub struct FunctionCall {
    pub name: String,
    pub params: Vec<AstNode>,
    pub pipe: Option<Box<FunctionCall>>,
}

///All if and elif expression are to to stored in the if block
/// since elif is just a kind of if statement
/// With the structure (condition, expression_block)
/// While the else block is the fallback block
#[derive(Debug, Clone)]
pub struct IfExpr {
    pub if_expr: Vec<(Vec<AstNode>, Vec<AstNode>)>,
    pub executed: bool,
    pub fallback: Vec<AstNode>,
}

///These are the AST tokens for draftlang
#[derive(Debug, Clone)]
pub enum AstNode {
    Ident(String),
    Str(String),
    Number(f64),
    Null,
    Map(HashMap<AstNode, AstNode>),
    Array(Vec<AstNode>),
    Boolean(bool),
    Return(Box<AstNode>),
    Assignment {
        ident: Box<AstNode>,
        expr: Box<AstNode>,
    },
    Import {
        funcs: Vec<AstNode>,
        module: Box<AstNode>,
    },
    ForLoop {
        ident: (Box<AstNode>, Box<AstNode>),
        range_value: Box<AstNode>,
        expr: Vec<AstNode>,
    },
    Function {
        name: String,
        params: Vec<AstNode>,
        expr: Vec<AstNode>,
    },
    FunctionCaller(FunctionCall),
    IfExpresion(IfExpr),
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
