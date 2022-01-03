use std::str::Chars;
use std::io::Read;

fn do_left_bracket(chars: Chars, index: i32) -> i32 {
    let mut ix: i32 = index;
    let mut open = 1;
    while open != 0 {
        ix += 1;

        match chars.clone().nth(ix.try_into().unwrap()).unwrap() {
            '[' => open += 1,
            ']' => open -= 1,
            _ => (),
        }
    }

    return ix
}

fn do_right_bracket(chars: Chars, index: i32) -> i32 {
    let mut ix: i32 = index;
    let mut close = 1;
    while close != 0 {
        ix -= 1;

        if ix >= chars.clone().count().try_into().unwrap() {
            panic!("couldn't find next matching ']'");
        }

        match chars.clone().nth(ix.try_into().unwrap()).unwrap() {
            '[' => close -= 1,
            ']' => close += 1,
            _ => (),
        }

    }

    return ix
}

pub fn brainfuck(programm: String, debug: i32) -> [u8; 3000] {
    let mut cells: [u8; 3000] = [0; 3000];
    let mut max: i32 = 0;
    let possition: &mut usize = &mut 0;
    let chars: Chars = programm.chars();

    let mut index: i32 = 0;
    while index < chars.clone().count().try_into().unwrap() {
        let cur_char = chars.clone().nth(index.try_into().unwrap()).unwrap();

        match cur_char {

            // # does not actually appear in brainfuck's docs.
            // This character is used to debug only supported
            // in this interpreter (if debug option is set to
            // true)
            '#' => if debug > 0 {println!("{}", cells[*possition])},

            // Increment value by 1 in current cell possition
            '+' => {
                if cells[*possition] == 255 {
                    cells[*possition] = 0;
                } else {
                    cells[*possition] += 1
                }
            },

            // Decrement value by 1 in current cell possition
            '-' => {
                if cells[*possition] == 0 {
                    cells[*possition] = 255;
                } else {
                    cells[*possition] = *&mut ((cells[*possition] as usize).checked_sub(1)).unwrap_or_default() as u8;
                }
            },

            // Move the current possition to the next cell
            '>' => *possition += 1,

            // Go back one cell
            '<' => {*possition = *&mut ((*possition as usize).checked_sub(1)).unwrap_or_default() as usize;},

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

            // Left brackets are like c while(cur_block_value != 0) loop.
            '[' => if cells[*possition] == 0  {index = do_left_bracket(chars.clone(), index)},

            // if block currently pointed to's value is not zero, jump back to [
            ']' => if cells[*possition] != 0  {index = do_right_bracket(chars.clone(), index)},

            // In Brainfuck, other ASCII characters that
            // are not ["+", ",", "-", "<", ">", ".", "[", "]"]
            // they are considered as comments, so we do nothing.
            _ => (),
        }

        index += 1;

        if *possition as i32 > max {
            max = *possition as i32;
        }
    }

    if debug > 1 {
        println!("\n\n=== CELLS ===");
        for i in 0..max {
            println!("c[{}]: {}", i, cells[i as usize]);
        }
    }
    // println!("{:?}", cells);
    return cells;
}