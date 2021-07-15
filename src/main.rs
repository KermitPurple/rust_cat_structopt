use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::stdin;
use std::path::PathBuf;
use structopt::StructOpt;

fn print_file(file_name: &PathBuf, opts: &Opt, line_count: &mut i32) -> Result<(), std::io::Error> {
    let mut file = File::open(file_name)?;
    print_lines(&mut file, opts, line_count)?;
    Ok(())
}

fn echo_input(opt: &Opt, line_count: &mut i32) {
    print_lines(&mut stdin(), &opt, line_count).unwrap();
}

fn print_lines(file: &mut dyn Read, opts: &Opt, line_count: &mut i32) -> Result<(), std::io::Error> {
    for line in BufReader::new(file).lines() {
        if opts.number{
            *line_count += 1;
            println!("{: >6}: {}", line_count, line.unwrap());
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
    let mut line_count = 0;
    match opt.files.len() {
        0 => echo_input(&opt, &mut line_count), // no args; repeat until ctrl-c
        _ => {
            for file in &opt.files {
                if file.as_os_str().to_str() == Some("-") {
                    echo_input(&opt, &mut line_count);
                } else {
                    match print_file(&file, &opt, &mut line_count) {
                        Err(e) => println!("Opt: Cannot Open {}: {}", file.display(), e),
                        _ => (),
                    }
                }
            }
        }
    }
}
