use std::fs::File;
use std::path::PathBuf;
use std::io::stdin;
use std::io::prelude::*;
use structopt::StructOpt;

fn print_file(file_name: &PathBuf) -> Result<(), std::io::Error> {
    let mut string = String::new();
    let mut file = File::open(file_name)?;
    file.read_to_string(&mut string)?;
    print!("{}", string);
    Ok(())
}

fn echo_input(){
    loop {
        let mut string = String::new();
        stdin().read_line(&mut string)
            .expect("ERROR");
        print!("{}", string);
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "kitty")]
struct Kitty {
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let k = Kitty::from_args();
    println!("{:#?}", k);
    match k.files.len() {
        0 => echo_input(), // no args; repeat until ctrl-c
        _ => {
            for file in k.files {
                match print_file(&file) {
                    Err(e) => println!("Kitty: Cannot Open {}: {}", file.display(), e),
                    _ => (),
                }
            }
        }
    }
}
