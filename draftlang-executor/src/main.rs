mod cli;
mod core;
mod interpreter;
mod types;

use color_print::cprintln;
use draftlang_error::Error;

mod util;
fn main() {
    let args = cli::Args::new();

    let raw_file =
        util::read_draftlang_file(&args.file_path).expect("error reading DRAFTLANG file.");

    match util::parse_file(&raw_file) {
        Ok(ast) => {
            let imports = ast.script.get("import");
            let parsed_import = util::parse_imports(imports);
            let function = ast.json.data.get(&args.func);
            if function.is_none() {
                cprintln!("<red>Function not found<red>");
                panic!("Function not found");
            }
            let global_scope = ast.script.get("global").unwrap().clone();

            function.unwrap().iter().for_each(|(name, parameters)| {
                let function_body = ast.script.get(&name.clone().to_string()).unwrap();
                let mut executor = interpreter::types::FunctionExecxutor::new(
                    ast.script.clone(),
                    parsed_import.clone(),
                    name.clone(),
                    parameters.clone(),
                    function_body.clone(),
                    util::parse_global_scope(global_scope.clone()),
                );
                executor.execute();
            })
        }
        Err(err) => match err {
            Error::ParsingError(message) => {
                cprintln!("<red>{}<red>", message)
            }
            _ => unreachable!(),
        },
    }
}
