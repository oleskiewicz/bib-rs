use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::from_utf8;

use biblatex::Bibliography;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// Input file
    #[structopt(name = "FILE")]
    input: Option<String>,
}

fn print_tsv(bib: &Bibliography) {
    for entry in bib.iter() {
        for field in ["author", "title"].iter() {
            match entry.get_as::<String>(field) {
                Some(v) => print!("{}", v),
                None => print!(""),
            };
            print!("\t");
        }
        println!();
    }
}

fn read(input: Option<String>) -> String {
    return from_utf8((match input {
        Some(f) => Box::new(BufReader::new(File::open(f).unwrap())) as Box<dyn BufRead>,
        None => Box::new(BufReader::new(io::stdin())) as Box<dyn BufRead>,
    }).fill_buf().unwrap()).unwrap().to_string();
}

fn main() {
    let opt = Opt::from_args();
    let content = read(opt.input);
    let bib = Bibliography::parse(&content).unwrap();

    print_tsv(&bib);
}
