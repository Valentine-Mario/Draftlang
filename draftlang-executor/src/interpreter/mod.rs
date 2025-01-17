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
        match &self.function_body[0] {
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
                            println!("Function Scope {:#?}", self.function_scope);
                        },
                        AstNode::IfExpresion(condition)=>{},
                        AstNode::ForLoop { ident, range_value:range, expr: loop_expression }=>{

                        }
                        AstNode::Return(expr)=>self.return_value=Some(*expr.clone()),
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
