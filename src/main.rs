use std::fs::File;
use std::path::PathBuf;
use std::io::stdin;
use std::io::prelude::*;
use structopt::StructOpt;

fn print_file(file_name: &PathBuf) {
    let display = file_name.display();
    let mut file = match File::open(file_name) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file
    };
    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => print!("{}", string)
    };
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
        0 => echo_input(),
        _ => {
            for file in k.files {
                print_file(&file);
            }
        }
    }
}
