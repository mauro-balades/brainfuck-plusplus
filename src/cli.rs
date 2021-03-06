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

use std::fs::*;
use std::io::prelude::*;
use std::path::Path;
use structopt::StructOpt;

mod brainfuck;
mod errors;
mod repl;

pub use brainfuck::*;
pub use errors::*;
pub use repl::*;

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

    // Declare the brainfuck config with
    // it's default values.
    let bf_config = BFConfig {
        debug: opt.verbose,
        ..default_bf_config()
    };

    if !opt.file.is_none() {
        let file_path = opt.file.as_ref().map(String::as_str).unwrap();

        // Open file
        let file = File::open(file_path).map_err(|err| report_error("IOERR".to_string(), err.to_string()));

        // Define file's contents as a new String
        let mut contents = String::new();

        // Read file's contents and store it as a String
        // in [contents]. If it does not work, throw an
        // error.
        file.unwrap().read_to_string(&mut contents)
            .expect("Unable to read to string");

        // interpret the brainfuck code
        brainfuck(contents, bf_config);
        return;
    } else {
        repl(bf_config);
    }
}
