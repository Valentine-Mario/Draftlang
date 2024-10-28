pub mod types;

use std::collections::HashMap;

use draftlang_parser::types::AstNode;
use types::FunctionExecxutor;

use crate::util;

impl FunctionExecxutor {
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
            return_value: AstNode::EmptyValue,
            import_value: imports,
        }
    }
    pub fn execute(&self) {}
}
