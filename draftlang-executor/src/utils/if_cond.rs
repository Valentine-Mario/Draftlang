use draftlang_parser::types::{IfCondition, IfExpr, Verb};

use crate::interpreter::types::FunctionExecxutor;

use super::{
    executor_utils::{execute_body, get_ident_value},
    helpers::{compare_ast_nodes, get_left_value_truty},
};

/// Execute an if expression within a function.
///
/// This function evaluates the conditions in the `if_expr` of the given
/// `IfExpr` and executes the corresponding block of expressions if the condition
/// evaluates to true. If none of the conditions are met, it executes the fallback
/// block of the `IfExpr`.
///
/// # Parameters
///
/// * `condition`: A mutable reference to the `IfExpr`, containing the if conditions
///   and expression blocks.
/// * `function`: A mutable reference to the `FunctionExecxutor`, which holds the
///   current execution context and scope.

pub fn execute_condition(condition: &IfExpr, function: &mut FunctionExecxutor) {
    let if_expr = &condition.if_expr;
    let condition = &mut condition.clone();
    for cond_block in if_expr {
        let execute_true = retrieve_if_condition(&cond_block.0, function);
        if execute_true {
            execute_body(function, &cond_block.1);
            condition.executed = true;
            break;
        }
    }
    if !condition.executed {
        let fallback = &condition.fallback;
        execute_body(function, fallback);
        condition.executed = true;
    }
}

fn retrieve_if_condition(expression: &Vec<IfCondition>, function: &mut FunctionExecxutor) -> bool {
    let mut result = false;
    for (i, item) in expression.iter().enumerate() {
        match item {
            IfCondition::Expr((left, verb, right)) => {
                let left_value = get_ident_value(function, left);
                let right_value = right;
                if right_value.is_none() {
                    // If the right value is None, we can only compare with the left value
                    result = get_left_value_truty(&left_value);
                }
                match verb {
                    Some(verb) => {
                        //we can unwrap right_value sinve it has been checked before
                        let right = get_ident_value(function, &right.clone().unwrap());
                        result = compare_ast_nodes(&left_value, &right, verb);
                    }
                    None => {}
                }
            }
            IfCondition::Cond(cond) => match cond {
                Verb::And => {
                    result =
                        result && retrieve_if_condition(&expression[i + 1..].to_vec(), function);
                }
                Verb::Or => {
                    result =
                        result || retrieve_if_condition(&expression[i + 1..].to_vec(), function);
                }
                _ => unreachable!(),
            },
        }
    }

    result
}
