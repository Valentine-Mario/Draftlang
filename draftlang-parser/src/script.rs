use std::{collections::HashMap, vec};

use pest::iterators::{Pair, Pairs};

use crate::{
    types::{parse_verb, AstNode, FunctionCall, IfCondition, IfExpr, Verb},
    util, Rule,
};

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
        Rule::global_assignment => {
            let mut inner_rules = pair.into_inner();
            let ident = parse_script(inner_rules.next().unwrap());
            let value = parse_script(inner_rules.next().unwrap());
            AstNode::Assignment {
                ident: Box::new(ident),
                expr: Box::new(value),
            }
        }
        Rule::assign_func => {
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
        Rule::block => {
            let mut inner_rules = pair.into_inner();
            parse_script(inner_rules.next().unwrap())
        }
        Rule::inline_expr => {
            let mut inner_rules = pair.into_inner();
            parse_script(inner_rules.next().unwrap())
        }
        Rule::return_statement => {
            let mut inner_rule = pair.into_inner();
            AstNode::Return(Box::new(parse_script(inner_rule.next().unwrap())))
        }
        Rule::cond_expr => {
            let mut inner_rules = pair.into_inner();
            parse_script(inner_rules.next().unwrap())
        }
        Rule::func_call => {
            //get function name and parameters
            let mut inner_rule = pair.into_inner();
            let (name, param) = parse_func_signature(inner_rule.next().unwrap());

            //if the function has pipe children recursvely add them to the pipe
            if inner_rule.clone().next().is_some() {
                let pipe = construct_pipe(inner_rule);

                let call = FunctionCall::new(name, param, pipe);
                AstNode::FunctionCaller(call)
            } else {
                let call = FunctionCall::new(name, param, None);
                AstNode::FunctionCaller(call)
            }
        }
        Rule::for_loop => {
            let mut inner_rule = pair.into_inner();
            let index = parse_script(inner_rule.next().unwrap());
            let value = parse_script(inner_rule.next().unwrap());
            let range_value = parse_script(inner_rule.next().unwrap());
            let mut loop_body: Vec<AstNode> = vec![];

            for item in inner_rule {
                loop_body.push(parse_script(item))
            }

            AstNode::ForLoop {
                ident: (Box::new(index), Box::new(value)),
                range_value: Box::new(range_value),
                expr: loop_body,
            }
        }
        Rule::if_statement => {
            let inner_rule = pair.clone().into_inner();

            let mut else_block: Vec<AstNode> = vec![];
            let mut if_expression: Vec<(Vec<IfCondition>, Vec<AstNode>)> = vec![];
            let (condition, if_block) = parse_if_block(inner_rule);
            if_expression.push((condition, if_block));

            for item in pair.into_inner() {
                if item.as_rule() == Rule::else_if_statement {
                    let inner_elif_rule = item.clone().into_inner();

                    let (condition, if_block) = parse_if_block(inner_elif_rule);
                    if_expression.push((condition, if_block));
                }

                if item.as_rule() == Rule::else_statement {
                    for item in item.into_inner() {
                        else_block.push(parse_script(item))
                    }
                }
            }

            let ifexpr = IfExpr {
                if_expr: if_expression,
                executed: false,
                fallback: else_block,
            };

            AstNode::IfExpresion(ifexpr)
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
            let mut map_item: HashMap<String, AstNode> = HashMap::new();
            let inner_rule = pair.into_inner();

            for item in inner_rule {
                let (key, value) = parse_map_item(item);
                map_item.insert(key, value);
            }
            AstNode::Map(map_item)
        }
        _ => {
            println!("unreachable {:?}", pair);
            AstNode::Null
        }
    }
}
fn parse_map_item(pair: Pair<Rule>) -> (String, AstNode) {
    match pair.as_rule() {
        Rule::map_item => {
            let mut inner_rules = pair.into_inner();
            let key = util::unescape_string(inner_rules.next().unwrap().as_str());
            let value = parse_script(inner_rules.next().unwrap());
            (key, value)
        }
        _ => unreachable!(),
    }
}

fn construct_pipe(mut pair: Pairs<Rule>) -> Option<Box<FunctionCall>> {
    //pass the rule and children to function
    let inner_rule = pair.next();
    //if there are more children, construct a nexted strut
    // else terminate the function
    if inner_rule.is_some() {
        let (pipename, pipeparam) = parse_func_signature(inner_rule.unwrap());

        Some(Box::new(FunctionCall::new(
            pipename,
            pipeparam,
            construct_pipe(pair),
        )))
    } else {
        None
    }
}

pub fn parse_func_signature(pair: Pair<Rule>) -> (AstNode, Vec<AstNode>) {
    match pair.as_rule() {
        Rule::func_signature => {
            let mut inner_rule = pair.into_inner();
            let function_name = parse_script(inner_rule.next().unwrap());
            let function_param: Vec<AstNode> = inner_rule
                .next()
                .unwrap()
                .into_inner()
                .map(|x| parse_script(x))
                .collect();
            (function_name, function_param)
        }
        _ => unreachable!(),
    }
}

fn parse_if_block(mut inner_rule: Pairs<Rule>) -> (Vec<IfCondition>, Vec<AstNode>) {
    let cond_pair = inner_rule.next().unwrap();
    let condition = parse_condition(cond_pair);
    let mut if_block: Vec<AstNode> = vec![];
    for item in inner_rule {
        if item.as_rule() == Rule::block {
            if_block.push(parse_script(item.clone()))
        }
    }
    (condition, if_block)
}
fn parse_condition(pair: Pair<Rule>) -> Vec<IfCondition> {
    let inner_rule = pair.into_inner();
    let mut return_value: Vec<IfCondition> = vec![];

    for item in inner_rule {
        if item.as_rule() == Rule::cond {
            let condition = item.into_inner();
            let mut cond_tuplue: (AstNode, Option<Verb>, Option<AstNode>) =
                (AstNode::Null, None, None);

            for item in condition {
                if item.as_rule() == Rule::cond_expr {
                    if cond_tuplue.1.is_some() {
                        cond_tuplue.2 = Some(parse_script(item))
                    } else {
                        cond_tuplue.0 = parse_script(item)
                    }
                } else {
                    cond_tuplue.1 = Some(parse_verb(item))
                }
            }
            return_value.push(IfCondition::Expr(cond_tuplue))
        } else {
            let join = parse_verb(item);
            return_value.push(IfCondition::Cond(join))
        }
    }

    return_value
}
