use pest_derive::Parser;


#[derive(Parser)]
#[grammar = "grammer/draftlang.pest"]
pub struct DraftLangParser;