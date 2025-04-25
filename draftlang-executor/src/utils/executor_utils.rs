use draftlang_parser::types::{AstNode, IfCondition, IfExpr, Verb};

use crate::interpreter::types::FunctionExecxutor;

///This function is used to get the value of an identifier in a function.
///It looks up the value of the identifier in the function scope first,
///if it can't find it, then it looks up the value in the global scope.
///If it can't find it in the global scope, it panics.
/// ///# Parameters
///
///* `function`: The function executor instance
///* `ident`: The identifier to look up
///
///# Returns
///
///The value of the identifier
pub fn get_ident_value(function: &FunctionExecxutor, ident: &AstNode) -> AstNode {
    let var_item = function
        .function_scope
        .get(&ident.to_string())
        .or_else(|| function.global_scope.get(&ident.to_string()))
        .cloned()
        .unwrap_or_else(|| {
            panic!("Variable not found. Make sure you have declared the variable before using it")
        });

    let value = match var_item {
        AstNode::Ident(_) => {
            //If the value is an identifier, recursively call `get_ident_value` to get the value.
            get_ident_value(function, &var_item)
        }
        _ => {
            //If the value is not an identifier, just return the value.
            var_item
        }
    };

    return value;
}

/// Execute the given body of a function.
///
/// This function iterates over each expression in the body and executes it.
/// Currently, it only supports assignments and if expressions.
/// For assignments, it inserts the assigned value into the function scope.
/// For if expressions, it does nothing (TODO: implement if expression execution).
///
/// # Parameters
///
/// * `function`: The function executor instance
/// * `body`: The body of the function to execute
pub fn execute_body(function: &mut FunctionExecxutor, body: &Vec<AstNode>) {
    for expression in body {
        match expression {
            AstNode::Assignment { ident, expr } => {
                let variable_name = ident.to_string();
                function
                    .function_scope
                    .insert(variable_name, expr.as_ref().to_owned());
            }
            AstNode::IfExpresion(condition) => {
                execute_condition(condition, function);
            }
            AstNode::ForLoop {
                ident,
                range_value: range,
                expr: loop_expression,
            } => {
                // Extract the index and value identifiers from the loop's 'ident' tuple.
                let index_name = &ident.0;
                let value_name = &ident.1;

                // Determine the range value for the loop.
                // If it's an identifier, retrieve its value from the function scope.
                // Otherwise, use the range's original value.
                let range_value = match range.as_ref() {
                    AstNode::Ident(_) => get_ident_value(function, range),
                    _ => range.as_ref().to_owned(),
                };

                // Execute the loop with the resolved index and value names, loop expression, and range value.
                execute_loop(
                    function,
                    (index_name, value_name),
                    loop_expression,
                    &range_value,
                );
            }
            AstNode::Return(expr) => function.return_value = Some(*expr.clone()),
            _ => {}
        }
    }
}

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
        }
    }
    if !condition.executed {
        let fallback = &condition.fallback;
        execute_body(function, &fallback);
        condition.executed = true;
    }
}

fn retrieve_if_condition(expression: &Vec<IfCondition>, function: &mut FunctionExecxutor) -> bool {
    println!("retrieve_if_condition {:?}", expression);
    for item in expression {
        match item {
            IfCondition::Expr((left, verb, right)) => {
                let left_value = get_ident_value(function, left);
                let right_value = match right {
                    Some(right) => get_ident_value(function, right),
                    None => AstNode::Null,
                };
                match verb {
                    Some(verb) => {
                        match verb {
                            Verb::Equal => {
                                if left_value != right_value {
                                    return false;
                                }
                            }
                            Verb::NotEqual => {
                                if left_value == right_value {
                                    return false;
                                }
                            }
                            Verb::LargerThanOrEqual => {
                                //todo: write utility function to compare values
                                if left_value >= right_value {
                                    return false;
                                }
                            }
                            Verb::LessThan => {
                                if left_value <= right_value {
                                    return false;
                                }
                            }
                            Verb::LessThanOrEqual => {
                                if left_value < right_value {
                                    return false;
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    None => {}
                }
            }
            IfCondition::Cond(cond) => match cond {
                Verb::And => {
                    if !retrieve_if_condition(&expression[1..].to_vec(), function) {
                        return false;
                    }
                }
                Verb::Or => {
                    if retrieve_if_condition(&expression[1..].to_vec(), function) {
                        return true;
                    }
                }
                _ => unreachable!(),
            },
        }
    }

    return true;
}
/// Execute a loop with a given range.
///
/// This function will iterate over the range, assigning the current value to the variable
/// specified by `range_pointers.1` and the current index to the variable specified by
/// `range_pointers.0`. It will then execute the given loop body.
///
/// # Parameters
///
/// * `function`: The function executor instance
/// * `range_pointers`: A tuple containing the index variable name and the value variable name
/// * `loop_body`: The body of the loop to execute
/// * `range_value`: The range value to iterate over
pub fn execute_loop(
    function: &mut FunctionExecxutor,
    range_pointers: (&AstNode, &AstNode),
    loop_body: &Vec<AstNode>,
    range_value: &AstNode,
) {
    match range_value {
        AstNode::Array(array) => {
            for (index, value) in array.iter().enumerate() {
                function
                    .function_scope
                    .insert(range_pointers.1.to_string(), value.clone());
                function
                    .function_scope
                    .insert(range_pointers.0.to_string(), AstNode::Number(index as f64));
                execute_body(function, loop_body);
            }
        }
        AstNode::Str(string) => {
            for (index, char) in string.chars().enumerate() {
                function
                    .function_scope
                    .insert(range_pointers.1.to_string(), AstNode::Str(char.to_string()));
                function
                    .function_scope
                    .insert(range_pointers.0.to_string(), AstNode::Number(index as f64));
                execute_body(function, loop_body);
            }
        }
        AstNode::Map(map_items) => {
            for (index, (_, value)) in map_items.iter().enumerate() {
                function
                    .function_scope
                    .insert(range_pointers.1.to_string(), value.clone());
                function
                    .function_scope
                    .insert(range_pointers.0.to_string(), AstNode::Number(index as f64));
                execute_body(function, loop_body);
            }
        }
        AstNode::Number(number) => {
            for index in 0..*number as u32 {
                function
                    .function_scope
                    .insert(range_pointers.1.to_string(), AstNode::Number(index as f64));
                function
                    .function_scope
                    .insert(range_pointers.0.to_string(), AstNode::Number(index as f64));
                execute_body(function, loop_body);
            }
        }
        _ => panic!("Invalid range type"),
    }

    //when done, clean up range variable from function scope
    function
        .function_scope
        .remove(&range_pointers.1.to_string());
    function
        .function_scope
        .remove(&range_pointers.0.to_string());
}
