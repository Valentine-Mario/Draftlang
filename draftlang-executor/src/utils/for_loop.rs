use draftlang_parser::types::AstNode;

use crate::interpreter::types::FunctionExecxutor;

use super::executor_utils::execute_body;

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
