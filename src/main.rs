use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::io::stdout;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

fn echo_input(opt: &Opt) {
    print_lines(vec![Ok(Box::new(stdin()))], &opt).unwrap(); // print stdin as a file
}

fn print_lines(
    files: Vec<Result<Box<dyn Read>, String>>,
    opts: &Opt,
) -> Result<(), std::io::Error> {
    let mut line_count = 0;
    let mut prev_blank = false;
    let mut out = stdout();
    for res in files { // loop over results in files
        match res { // match the result
            Ok(file) => { // if there is no error
                for line in BufReader::new(file).split(b'\n') { // loop over lines in current file
                    let v = line.unwrap(); // receive the current line as Vec<u8>
                    let blank = v.is_empty() || v[0] == b'\r'; // the line is blank if it is empty or the first character is carriage return
                    if opts.squeeze_blank && prev_blank && blank { // if squeeze-blank was passed and the previous line was blank and the current line is blank
                        continue; // go to next iteration
                    }
                    if opts.number { // if number was passed
                        line_count += 1; // increment line_count
                        print!("{: >6}: ", line_count); // print the line count before the line
                    }
                    if opts.show_ends { // if show-ends was passed
                        out.write(
                            &v.into_iter() // convert Vec<u8> to iter
                                .filter(|item| *item != b'\r') // filter carriage returns out
                                .collect::<Vec<u8>>(), // convert iter to Vec<u8>
                        )?;
                        print!("$"); // print the $ char
                    } else {
                        out.write(&v)?; // print the line
                    }
                    println!(); // print newline
                    prev_blank = blank; // set previous blank to blank
                }
            }
            Err(file_name) => println!("Kitty: couldn't find file \"{}\"", file_name), // print error message
        }
    }
    Ok(()) // return no error
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
    let opt = Opt::from_args(); // parse command line args
    match opt.files.len() { // number of args
        0 => echo_input(&opt), // no args; repeat until ctrl-c
        _ => { // any other number of args
            let files: Vec<Result<Box<dyn Read>, String>> = opt
                .files // the files
                .clone() // clone vector
                .into_iter() // convert from vector to iter
                .map(|file| -> Result<Box<dyn Read>, String> {
                    let file_name = file.clone().into_os_string().into_string().unwrap(); // convert PathBuf to string
                    if file_name == "-" { // if file name is "-"
                        Ok(Box::new(stdin())) // pass standard input as a file object
                    } else {
                        match File::open(file) { // Open the file
                            Ok(f) => Ok(Box::new(f)), // return an Ok file if it opens
                            Err(_) => Err(file_name), // return an error with the file name
                        }
                    }
                })
                .collect(); // convert from iter to vector
            print_lines(files, &opt).unwrap(); // print files
        }
    }
}
