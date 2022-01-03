
use std::io::prelude::*;
use std::fs::*;
use std::path::Path;
use structopt::StructOpt;

mod brainfuck;

pub use brainfuck::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "Brainfuck")]
struct Opt {

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: i32,

    #[structopt(name = "FILE")]
    file: String,
}

fn main() {

    // Parse arguments with StructOpt
    let opt = Opt::from_args();

    // check if file eixsts
    if Path::new(&opt.file).exists() {

        // Open file
        let mut file = File::open(&opt.file).expect("Error opening File");

        // Define file's contents as a new String
        let mut contents = String::new();

        // Read file's contents and store it as a String
        // in [contents]. If it does not work, throw an
        // error.
        file.read_to_string(&mut contents).expect("Unable to read to string");

        // interpret the brainfuck code
        brainfuck(contents, opt.verbose);
        return
    }
}
