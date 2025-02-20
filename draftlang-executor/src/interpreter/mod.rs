pub mod types;

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
                            let index = &ident.0;
                            let value = &ident.1;
                            let mut range_value = &AstNode::EmptyValue;
                            let cloned_func_scope = self.function_scope.clone();
                            let cloned_global_scope = self.global_scope.clone();

                            match range.as_ref() {
                                AstNode::Ident(ident) => {
                                    // Attempt to retrieve the range value from the function scope
                                    let variable_ref = cloned_func_scope.get(&ident.to_string());
                                    match variable_ref {
                                        // If the variable is found in the function scope, set it as the range value
                                        Some(variable_ref) => {
                                            range_value = variable_ref;
                                        }
                                        // If not found in the function scope, attempt to retrieve it from the global scope
                                        None => {
                                            range_value = cloned_global_scope
                                                .get(&ident.to_string())
                                                .unwrap();
                                        }
                                    }
                                }
                                // If the range is an array, set it as the range value
                                AstNode::Array(_array) => {
                                    range_value = range;
                                }
                                // If the range is a string, set it as the range value
                                AstNode::Str(_str) => {
                                    range_value = range;
                                }
                                //if range is neither an array or a string panic
                                _ => {
                                    panic!("Invalid range type");
                                }
                            }
                            execute_loop(self, (index, value), loop_expression, range_value);
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

fn execute_loop(
    function: &mut FunctionExecxutor,
    range_pointers: (&AstNode, &AstNode),
    loop_body: &Vec<AstNode>,
    range: &AstNode,
) {
    //iterate through the range item, and use the range pointer to update the function scope through the mut ref
    function
        .function_scope
        .insert(range_pointers.0.to_string(), range.clone());
}
