// /$$$$$$$                     /$$            /$$$$$$                     /$$
// | $$__  $$                   |__/           /$$__  $$                   | $$
// | $$  \ $$  /$$$$$$  /$$$$$$  /$$ /$$$$$$$ | $$  \__//$$   /$$  /$$$$$$$| $$   /$$
// | $$$$$$$  /$$__  $$|____  $$| $$| $$__  $$| $$$$   | $$  | $$ /$$_____/| $$  /$$/
// | $$__  $$| $$  \__/ /$$$$$$$| $$| $$  \ $$| $$_/   | $$  | $$| $$      | $$$$$$/
// | $$  \ $$| $$      /$$__  $$| $$| $$  | $$| $$     | $$  | $$| $$      | $$_  $$
// | $$$$$$$/| $$     |  $$$$$$$| $$| $$  | $$| $$     |  $$$$$$/|  $$$$$$$| $$ \  $$
// |_______/ |__/      \_______/|__/|__/  |__/|__/      \______/  \_______/|__/  \__/
//
// -----------------------------------------------------------------------------------------------
// Copiright 2021 <mauro.balades@tutanota.com>
//

use std::io::prelude::*;
use std::fs::*;
use std::path::Path;
use structopt::StructOpt;

mod errors;
mod brainfuck;

pub use errors::*;
pub use brainfuck::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "Brainfuck")]
struct Opt {

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: i32,

    #[structopt(name = "FILE")]
    file: Option<String>,
}

fn main() {

    // Parse arguments with StructOpt
    let opt = Opt::from_args();

    if ! opt.file.is_none() {
        let file_path = opt.file.as_ref().map(String::as_str).unwrap();

        // check if file eixsts
        if Path::new(file_path).exists() {

            // Open file
            let mut file = File::open(file_path).expect("Error opening File");

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

    println!("REPL IN PROGRESS");
}
