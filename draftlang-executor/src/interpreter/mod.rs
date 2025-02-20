pub mod types;

use core::panic;
use std::collections::HashMap;

use draftlang_parser::types::AstNode;
use types::FunctionExecxutor;

use crate::util;

impl FunctionExecxutor {
    /// Create a new instance of the FunctionExecutor
    ///
    /// # Parameters
    ///
    /// * `scripts`: The scripts that would be executed by the function
    /// * `imports`: The imports that are defined in the function scope
    /// * `name`: The name of the function
    /// * `parameters`: The parameters of the function
    /// * `function_body`: The body of the function
    /// * `global_scope`: The global scope of the function
    ///
    /// # Returns
    ///
    /// A new instance of FunctionExecutor
    pub fn new(
        scripts: HashMap<String, Vec<AstNode>>,
        mut imports: HashMap<String, Vec<String>>,
        name: AstNode,
        parameters: Vec<AstNode>,
        function_body: Vec<AstNode>,
        global_scope: HashMap<String, AstNode>,
    ) -> Self {
        let mut function_scope: HashMap<String, AstNode> = HashMap::new();
        //function scope would be originally empty execpt with the values
        //of the parameters inserted
        //it would be appended as we transverse the function body
        match &function_body[0] {
            AstNode::Function {
                name: _,
                params,
                expr: _,
            } => {
                for (i, param) in params.iter().enumerate() {
                    function_scope.insert(param.to_string(), parameters[i].clone());
                }
            }
            // AstNode::ForLoop { ident, range_value, expr }=>{

            // }
            _ => unreachable!(),
        }

        imports.extend(util::append_default_import());
        FunctionExecxutor {
            scripts,
            name,
            parameters,
            function_scope,
            global_scope,
            function_body,
            return_value: None,
            import_value: imports,
        }
    }

    ///Execute the function body by iterating over each expression.
    ///Each expression could be either a variable assignment, an if expression,
    ///a for loop, a return statement or a function call.
    ///The function scope would be mutated accordingly.
    ///The return value would be the value of the last expression to be evaluated.

    pub fn execute(&mut self) {
        let function_body = &self.function_body.clone()[0];
        match function_body {
            AstNode::Function {
                name: _,
                params: _,
                expr: function_expressions,
            } => {
                for expression in function_expressions {
                    match expression {
                        AstNode::Assignment { ident, expr } => {
                            let variable_name = ident.to_string();
                            self.function_scope
                                .insert(variable_name, expr.as_ref().to_owned());
                        }
                        AstNode::IfExpresion(condition) => {}
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
                                AstNode::Ident(_) => get_ident_value(self, range),
                                _ => range.as_ref().to_owned(),
                            };

                            // Execute the loop with the resolved index and value names, loop expression, and range value.
                            execute_loop(
                                self,
                                (index_name, value_name),
                                loop_expression,
                                &range_value,
                            );
                        }
                        AstNode::Return(expr) => self.return_value = Some(*expr.clone()),
                        _ => {}
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn execute_function(&mut self) {}
    pub fn extract_types(&mut self) {}
}

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
fn get_ident_value(function: &FunctionExecxutor, ident: &AstNode) -> AstNode {
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

fn execute_loop(
    function: &mut FunctionExecxutor,
    range_pointers: (&AstNode, &AstNode),
    loop_body: &[AstNode],
    range: &AstNode,
) {
    match range {
        AstNode::Array(array) => {}
        AstNode::Str(str_data) => {}
        AstNode::Map(map_item) => {}
        AstNode::Number(num_data) => {}
        _ => {
            panic!("Invalid range type")
        }
    }
    //iterate through the range item, and use the range pointer to update the function scope through the mut ref
    function
        .function_scope
        .insert(range_pointers.0.to_string(), range.clone());
}
