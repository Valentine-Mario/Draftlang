use std::collections::HashMap;

use pest::iterators::Pair;

use crate::{json::JSONValue, Rule};

#[derive(Debug, Clone)]
pub struct DraftLangAst {
    pub(crate) json: JSONValue,
    pub(crate) script: HashMap<String, Vec<AstNode>>,
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
    pub(crate) name: Box<AstNode>,
    pub(crate) params: Vec<AstNode>,
    pub(crate) pipe: Option<Box<FunctionCall>>,
}

impl FunctionCall {
    pub fn new(name: AstNode, params: Vec<AstNode>, pipe: Option<Box<FunctionCall>>) -> Self {
        FunctionCall {
            name: Box::new(name),
            params,
            pipe,
        }
    }

    pub fn from_pipe() {}
}
///All if and elif expression are to to stored in the if block
/// since elif is just a kind of if statement
/// With the structure (condition, expression_block)
/// While the else block is the fallback block
#[derive(Debug, Clone)]
pub struct IfExpr {
    //the structure of an if expression is as follows:
    //vec[(vec[condition], code_block)]
    //where condition follows the structure ((node ~ optional_verb ~ optional_node), optional_and_or)
    //the optional_and_or at the end links to the next item in the vector
    pub(crate) if_expr: Vec<(Vec<IfCondition>, Vec<AstNode>)>,
    pub(crate) executed: bool,
    pub(crate) fallback: Vec<AstNode>,
}

#[derive(Debug, Clone)]
pub enum IfCondition {
    Expr((AstNode, Option<Verb>, Option<AstNode>)),
    Cond(Verb),
}

///These are the AST tokens for draftlang
#[derive(Debug, Clone)]
pub enum AstNode {
    Ident(String),
    Str(String),
    Number(f64),
    Null,
    Map(HashMap<String, AstNode>),
    Array(Vec<AstNode>),
    Boolean(bool),
    Return(Box<AstNode>),
    Assignment {
        ident: Box<AstNode>,
        expr: Box<AstNode>,
    },
    ModuleImport {
        parent: Box<AstNode>,
        child: Box<AstNode>,
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
        name: Box<AstNode>,
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
        "and" => Verb::And,
        "or" => Verb::Or,
        "|>" => Verb::Pipe,
        "!=" => Verb::NotEqual,
        "<=" => Verb::LessThanOrEqual,
        ">=" => Verb::LargerThanOrEqual,
        _ => panic!("Unexpected  verb: {}", pair.as_str()),
    }
}
