

use std::io;
use std::io::prelude::*;
use std::env;
use std::fs::*;
use std::path::Path;

mod brainfuck;

pub use brainfuck::*;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if user has typed an argument
    if args.len() > 0 {

        // Get filename.
        // We asume that filename is in
        // the possition [1].
        // TODO: check for more commands
        let filename = &args[1];

        // check if file eixsts
        if Path::new(filename).exists() {

            // Open file
            let mut file = File::open(&filename).expect("Error opening File");

            // Define file's contents as a new String
            let mut contents = String::new();

            // Read file's contents and store it as a String
            // in [contents]. If it does not work, throw an
            // error.
            file.read_to_string(&mut contents).expect("Unable to read to string");


            return
        }

        // TODO: print error

    } else {
        // Error, filename was not given
    }
}
