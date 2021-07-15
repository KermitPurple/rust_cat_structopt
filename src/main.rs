use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::stdin;
use std::path::PathBuf;
use structopt::StructOpt;

fn print_file(file_name: &PathBuf, opts: &Opt) -> Result<(), std::io::Error> {
    let mut string = String::new();
    let mut file = File::open(file_name)?;
    print_lines(&mut file, opts)?;
    Ok(())
}

fn echo_input(opt: &Opt) {
    print_lines(&mut stdin(), &opt);
}

fn print_lines(file: &mut dyn Read, opts: &Opt) -> Result<(), std::io::Error> {
    let mut i = 0;
    for line in BufReader::new(file).lines() {
        if opts.number{
            println!("{} {}", i, line.unwrap());
            i += 1;
        } else {
            println!("{}", line.unwrap());
        }
    }
    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "kitty")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,

    #[structopt(short, long)]
    number: bool,
}

fn main() {
    let opt = Opt::from_args();
    match opt.files.len() {
        0 => echo_input(&opt), // no args; repeat until ctrl-c
        _ => {
            for file in &opt.files {
                if file.as_os_str().to_str() == Some("-") {
                    echo_input(&opt);
                } else {
                    match print_file(&file, &opt) {
                        Err(e) => println!("Opt: Cannot Open {}: {}", file.display(), e),
                        _ => (),
                    }
                }
            }
        }
    }
}
