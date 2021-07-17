use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

fn echo_input(opt: &Opt) {
    print_lines(vec![Box::new(stdin())], &opt).unwrap();
}

fn print_lines(files: Vec<Box<dyn Read>>, opts: &Opt) -> Result<(), std::io::Error> {
    let mut line_count = 0;
    let mut prev_blank = false;
    let iter = files
        .into_iter()
        .flat_map(|file| BufReader::new(file).lines());
    for line in iter {
        let s: String = line.unwrap();
        let blank = s == "";
        if opts.squeeze_blank && prev_blank && blank {
            continue;
        }
        if opts.number {
            line_count += 1;
            print!("{: >6}: ", line_count);
        }
        println!("{}", s);
        prev_blank = blank;
    }
    Ok(())
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "kitty")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,

    #[structopt(short, long, help = "Display line number at the start of each line")]
    number: bool,

    #[structopt(short, long, help = "Don't show more than one blank line in a row")]
    squeeze_blank: bool,
}

fn main() {
    let opt = Opt::from_args();
    match opt.files.len() {
        0 => echo_input(&opt), // no args; repeat until ctrl-c
        _ => {
            let files: Vec<Box<dyn Read>> = opt
                .files
                .clone()
                .into_iter()
                .map(|file| -> Box<dyn Read> {
                    if file.as_os_str().to_str() == Some("-") {
                        Box::new(stdin())
                    } else {
                        Box::new(File::open(file).unwrap())
                    }
                })
                .collect();
            print_lines(files, &opt).unwrap();
        }
    }
}
