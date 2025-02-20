mod cli;
mod core;
mod interpreter;
mod types;

use color_print::cprintln;
use draftlang_error::Error;

mod util;
/// The main entry point for the application.
///
/// It takes a file path as an argument and execute the draftlang
/// file. It will parse the file, retrieve the imports and the
/// function key from the json object, parse the function body
/// and execute it. If the function is not found, it will
/// panic.
///
///
fn main() {
    let args = cli::Args::new();

    let raw_file =
        util::read_draftlang_file(&args.file_path).expect("error reading DRAFTLANG file.");

    // Parse the draftlang file and execute the specified function.
    match util::parse_file(&raw_file) {
        // If parsing is successful, proceed with execution
        Ok(ast) => {
            // Retrieve and parse import statements from the script
            let imports = ast.script.get("import");
            let parsed_import = util::parse_imports(imports);

            // Retrieve the function key from the JSON object
            let function = ast.json.data.get(&args.func);
            if function.is_none() {
                cprintln!("<red>Function not found<red>");
                panic!("Function not found");
            }

            // Get all items in the global scope of the script
            let global_scope = ast.script.get("global").unwrap().clone();

            // Iterate over each function and execute it
            function.unwrap().iter().for_each(|(name, parameters)| {
                let function_body = ast.script.get(&name.clone().to_string()).unwrap();

                // Create a new instance of FunctionExecxutor
                let mut executor = interpreter::types::FunctionExecxutor::new(
                    ast.script.clone(),
                    parsed_import.clone(),
                    name.clone(),
                    parameters.clone(),
                    function_body.clone(),
                    util::parse_global_scope(global_scope.clone()),
                );

                // Execute the function
                executor.execute();
            })
        }
        // Handle parsing errors
        Err(err) => match err {
            Error::ParsingError(message) => {
                cprintln!("<red>{}<red>", message)
            }
            _ => unreachable!(), // Other errors should not occur
        },
    }
}
