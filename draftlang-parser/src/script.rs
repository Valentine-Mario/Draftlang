use pest::iterators::Pair;

use crate::{types::AstNode, util, Rule};

pub fn parse_script(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::ident => AstNode::Ident(pair.as_str().to_string()),
        Rule::assignment => {
            let mut inner_rules = pair.into_inner();
            let ident = parse_script(inner_rules.next().unwrap());
            let value = parse_script(inner_rules.next().unwrap());
            AstNode::Assignment {
                ident: Box::new(ident),
                expr: Box::new(value),
            }
        }
        Rule::import_stat => AstNode::Null,
        Rule::func_expression => AstNode::Null,
        Rule::types => {
            let mut inner_rules = pair.into_inner();
            parse_types(inner_rules.next().unwrap())
        }
        _ => AstNode::Null,
    }
}

fn parse_types(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::number => AstNode::Number(pair.as_str().parse().unwrap()),
        Rule::string => AstNode::Str(util::unescape_string(pair.as_str())),
        Rule::single_quote_string => AstNode::Str(util::unescape_string(pair.as_str())),
        Rule::boolean => AstNode::Boolean(pair.as_str().parse().unwrap()),
        Rule::array => AstNode::Array(pair.into_inner().map(|x| parse_script(x)).collect()),
        Rule::null => AstNode::Null,
        _ => unreachable!(),
    }
}
