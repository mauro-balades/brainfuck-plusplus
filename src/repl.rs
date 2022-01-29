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

use owo_colors::OwoColorize;

use std::io;
use std::io::{BufRead, BufReader, Write};

use crate::brainfuck::*;

fn info() {
    print!(
        "\n{}\n-----------------------------------------\n| Type {} or press CTRL + C to exit |\n-----------------------------------------\n",
        "Brainfuck REPL".bright_green(),
        "quit".bright_blue());
}

pub fn repl(config: BFConfig) {
    info();

    // Make an infinite loop.
    loop {
        let input = get_input().unwrap();
        brainfuck(input, config.clone());
    }
}

fn get_input() -> io::Result<String> {
    print!("\n{} > ", "brainfuck".bright_blue());
    io::stdout().flush()?;
    BufReader::new(io::stdin())
        .lines()
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Cannot read stdin"))
        .and_then(|inner| inner)
}
