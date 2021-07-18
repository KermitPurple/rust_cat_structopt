use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::io::stdout;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

fn echo_input(opt: &Opt) {
    print_lines(vec![Ok(Box::new(stdin()))], &opt).unwrap();
}

fn print_lines(
    files: Vec<Result<Box<dyn Read>, String>>,
    opts: &Opt,
) -> Result<(), std::io::Error> {
    let mut line_count = 0;
    let mut prev_blank = false;
    let mut out = stdout();
    for res in files {
        match res {
            Ok(file) => {
                for line in BufReader::new(file).split(b'\n') {
                    let mut v = line.unwrap();
                    let blank = v.is_empty() || v[0] == b'\r';
                    if opts.squeeze_blank && prev_blank && blank {
                        continue;
                    }
                    if opts.number {
                        line_count += 1;
                        print!("{: >6}: ", line_count);
                    }
                    if opts.show_ends {
                        out.write(
                            &v.into_iter()
                                .filter(|item| *item != b'\r')
                                .collect::<Vec<u8>>(),
                        )?;
                        print!("$");
                    } else {
                        out.write(&v)?;
                    }
                    println!();
                    prev_blank = blank;
                }
            }
            Err(file_name) => println!("Kitty: couldn't find file \"{}\"", file_name),
        }
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

    #[structopt(short = "E", long, help = "Print $ at the end of each line")]
    show_ends: bool,
}

fn main() {
    let opt = Opt::from_args();
    match opt.files.len() {
        0 => echo_input(&opt), // no args; repeat until ctrl-c
        _ => {
            let files: Vec<Result<Box<dyn Read>, String>> = opt
                .files
                .clone()
                .into_iter()
                .map(|file| -> Result<Box<dyn Read>, String> {
                    let file_name = file.clone().into_os_string().into_string().unwrap();
                    if file_name == "-" {
                        Ok(Box::new(stdin()))
                    } else {
                        match File::open(file) {
                            Ok(f) => Ok(Box::new(f)),
                            Err(_) => Err(file_name),
                        }
                    }
                })
                .collect();
            print_lines(files, &opt).unwrap();
        }
    }
}
