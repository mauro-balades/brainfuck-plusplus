use std::mem;
use std::str::Chars;
use std::io::Read;


pub fn brainfuck(programm: String) {
    let mut cells: [u8; 3000] = [0; 3000];
    let possition: &mut usize = &mut 0;
    let chars: Chars = programm.chars();

    let mut index: i32 = 1;
    while index < chars.clone().count().try_into().unwrap() {
        let cur_char = chars.clone().nth(index.try_into().unwrap()).unwrap();

        match cur_char {
            // Increment value by 1 in current cell possition
            '+' => cells[*possition] += 1,

            // Decrement value by 1 in current cell possition
            '-' => cells[*possition] -= 1,

            // Move the current possition to the next cell
            '>' => *possition += 1,

            // Go back one cell
            '<' => *possition -= 1,

            // Print the current cell's ASCII value.
            '.' => print!("{}", cells[*possition] as char),

            // Set value from stdin to the current cell
            ',' => {
                // declare a new buffer array containing a
                // 'u8' type number to store a character code
                let mut buf = [0; 1];

                // Read input and check if an error has occoured.
                match std::io::stdin().read_exact(&mut buf) {
                    Ok(_) => cells[*possition] = buf[0], // Add buffer from input
                    Err(_) => {}, // TODO: error
                }
            },

            // In Brainfuck, other ASCII characters that
            // are not ["+", ",", "-", "<", ">", ".", "[", "]"]
            // they are considered as comments, so we do nothing.
            _ => (),
        }

        index += 1;
    }

    // TODO: add a debug option
    // println!("{:?}", cells);
}