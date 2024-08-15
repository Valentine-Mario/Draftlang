use std::collections::HashMap;

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
        Rule::import_stat => {
            let inner_rule = pair.into_inner();
            let mut imported_funcs: Vec<AstNode> = vec![];
            let mut parent: AstNode = AstNode::Null;
            let mut child: AstNode = AstNode::Null;
            for item in inner_rule {
                match item.as_rule() {
                    Rule::ident => imported_funcs.push(parse_script(item)),
                    Rule::module_call => {
                        let mut module_rule = item.into_inner();
                        parent = parse_script(module_rule.next().unwrap());
                        child = parse_script(module_rule.next().unwrap());
                    }
                    _ => unreachable!(),
                }
            }
            AstNode::Import {
                funcs: imported_funcs,
                module: Box::new(AstNode::ModuleImport {
                    parent: Box::new(parent),
                    child: Box::new(child),
                }),
            }
        }
        Rule::func_expression => {
            let mut inner_rule = pair.into_inner();
            let function_name = parse_script(inner_rule.next().unwrap());
            let function_param: Vec<AstNode> = inner_rule
                .next()
                .unwrap()
                .into_inner()
                .map(|x| parse_script(x))
                .collect();
            let mut function_body: Vec<AstNode> = vec![];

            for item in inner_rule {
                function_body.push(parse_script(item))
            }

            AstNode::Function {
                name: Box::new(function_name),
                params: function_param,
                expr: function_body,
            }
        }

        Rule::types => {
            let mut inner_rules = pair.into_inner();
            parse_types(inner_rules.next().unwrap())
        }
        Rule::inline_expr => {
            let mut inner_rules = pair.into_inner();
            parse_script(inner_rules.next().unwrap())
        }
        Rule::return_statement => {
            let mut inner_rule = pair.into_inner();
            AstNode::Return(Box::new(parse_script(inner_rule.next().unwrap())))
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
        Rule::map => {
            let mut map_vector: Vec<(AstNode, AstNode)> = vec![];
            let inner_rule = pair.into_inner();

            for item in inner_rule {
                let (key, value) = parse_map_item(item);
                map_vector.push((key, value))
            }
            AstNode::Map(map_vector)
        }
        _ => {
            println!("unreachable {:?}", pair);
            AstNode::Null
        }
    }
}
fn parse_map_item(pair: Pair<Rule>) -> (AstNode, AstNode) {
    match pair.as_rule() {
        Rule::map_item => {
            let mut inner_rules = pair.into_inner();
            let key = parse_script(inner_rules.next().unwrap());
            let value = parse_script(inner_rules.next().unwrap());
            (key, value)
        }
        _ => unreachable!(),
    }
}
