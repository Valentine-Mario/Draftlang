use std::{collections::HashMap, fmt};

use pest::iterators::Pair;

use crate::{json::JSONValue, Rule};

#[derive(Debug, Clone)]
pub struct DraftLangAst {
    pub json: JSONValue,
    pub script: HashMap<String, Vec<AstNode>>,
}

impl DraftLangAst {
    pub fn new(
        data: HashMap<String, Vec<(AstNode, Vec<AstNode>)>>,
        script: HashMap<String, Vec<AstNode>>,
    ) -> Self {
        DraftLangAst {
            json: JSONValue { data },
            script,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: Box<AstNode>,
    pub params: Vec<AstNode>,
    pub pipe: Option<Box<FunctionCall>>,
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
#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub if_expr: Vec<(Vec<IfCondition>, Vec<AstNode>)>,
    pub executed: bool,
    pub fallback: Vec<AstNode>,
}

//here the expr follows (astnode optional<verb> optional<astnode>)
//take for example if(a != b) the following enum would be (ident(a) some(verb::not_equal), some(ident(b)) )
//but in situations where say if(a) => (ident(a) None, None)
//in between 2 expressions we have the and|or verb
#[derive(Debug, Clone, PartialEq)]
pub enum IfCondition {
    Expr((AstNode, Option<Verb>, Option<AstNode>)),
    Cond(Verb),
}

///These are the AST tokens for draftlang
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Ident(String),
    Str(String),
    Number(f64),
    Null,
    EmptyValue,
    Map(HashMap<String, AstNode>),
    Array(Vec<AstNode>),
    Boolean(bool),
    Return(Box<AstNode>),
    Buffer(Vec<u8>),
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

#[derive(Clone, Debug, PartialEq)]
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

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AstNode::*;
        let val = match self {
            Ident(data) => data.to_string(),
            Str(data) => data.to_string(),
            Number(data) => format!("{}", data),
            Null => String::from("nil"),
            Map(data) => format!("{:#?}", data),
            Boolean(data) => format!("{}", data),
            Array(data) => format!("{:#?}", data),
            Buffer(data) => format!("{:#?}", data),
            _ => unreachable!(),
        };
        write!(f, "{}", val)
    }
}
