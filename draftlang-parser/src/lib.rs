use pest::iterators::Pair;
use pest::{error::Error, Parser};
use pest_derive::Parser;


#[derive(Parser)]
#[grammar = "grammer/draftlang.pest"]
pub struct DraftLangParser;

pub fn parse_file(source: &str){
    let pairs = DraftLangParser::parse(Rule::program, source);
    println!("{:?}", pairs)
}