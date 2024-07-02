use asdf_util::check_asdf;
use color_print::cprintln;

fn main() {
    match check_asdf(){
        Ok(_)=>{

        },
        Err(val)=>{
            cprintln!("<red>{:?}<red>", val)
        }
    }
}
