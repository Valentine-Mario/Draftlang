use color_print::cprintln;
use draftlang_parser::parse_file;

fn main() {
    let unparsed_file = std::fs::read_to_string("DRAFTLANG").expect("cannot read elixir file");
    // println!("{:?}", unparsed_file);
    let v = parse_file(&unparsed_file);
    match v {
        Ok(v) => {
            println!("parsed values {:#?}", v)
        }
        Err(e) => {
            match e.line_col {
                pest::error::LineColLocation::Pos((line, position)) => {
                    cprintln!(
                        "<red>>>> Syntax Error on line {:?} position {:?}<red>",
                        line,
                        position
                    )
                }
                _ => todo!(),
            }
            cprintln!("<red>>>> {:?}<red>", e.line())
        }
    }

    let mut strings = vec!["first".to_string(), "second".to_string(), "third".to_string()];

    // Convert the vector into the nested structure
    strings.reverse();
    let nested_struct = Nested::from_vec(strings);

    println!("{:#?}", nested_struct);
}


#[derive(Debug)]
struct Nested {
    value: String,
    is_next: bool,
    next: Option<Box<Nested>>,
}

impl Nested {
    // A constructor function for convenience
    fn new(value: String, next: Option<Box<Nested>>) -> Self {
        if next.is_some(){
            Nested{value, is_next: true, next}
        }else{
            Nested { value, is_next: false,  next }

        }
    }

    // A function to convert a vector of strings into the recursive `Nested` structure
    fn from_vec(mut vec: Vec<String>) -> Option<Box<Nested>> {
        match vec.pop() {
            Some(value) => {
                // Recursively call from_vec with the remaining elements
                let next = Nested::from_vec(vec);
                // Return a new Nested struct encapsulated in a Box with the current value
                Some(Box::new(Nested::new(value, next)))
            },
            None => None, 
        }
    }
}