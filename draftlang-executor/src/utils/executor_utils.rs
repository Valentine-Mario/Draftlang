use draftlang_parser::types::AstNode;

use crate::interpreter::types::FunctionExecxutor;

use super::{for_loop::execute_loop, if_cond::execute_condition};

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
    //check if ident is a native type
    match ident {
        AstNode::Str(_) => return ident.clone(),
        AstNode::Number(_) => return ident.clone(),
        AstNode::Boolean(_) => return ident.clone(),
        AstNode::Null => return ident.clone(),
        _ => {}
    }
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
            AstNode::FunctionCaller(func) => {
                let function_name = func.name.to_string();
                let function_params = &func.params;
                //pipes point to the function's pipe expressions if any
                let pipes = &func.pipe;
                // Check if the function is imported
                if function.import_value.contains_key(&function_name) {
                    // If the function is imported, execute it with the provided parameters.
                }
            }
            AstNode::Return(expr) => function.return_value = Some(*expr.clone()),
            _ => {}
        }
    }
}
