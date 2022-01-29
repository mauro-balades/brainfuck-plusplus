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

mod errors;

use std::io::Read;
use std::str::Chars;

use crate::errors as bf_error;

#[derive(Clone)]
pub struct BFConfig {
    // The level of verbosity (default to 0)
    pub(crate) debug: i32,

    // Add support for the `!` symbol.
    pub(crate) exit_support: bool,
}

pub fn default_bf_config() -> BFConfig {
    BFConfig {
        debug: 0,
        exit_support: false,
    }
}

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

    return ix;
}

fn do_right_bracket(chars: Chars, index: i32) -> i32 {
    let mut ix: i32 = index;
    let mut close = 1;
    while close != 0 {
        ix -= 1;

        if ix >= chars.clone().count().try_into().unwrap() {
            bf_error::error(
                "Syntax error".to_string(),
                "couldn't find next matching ']'".to_string(),
            );
        }

        match chars.clone().nth(ix.try_into().unwrap()).unwrap() {
            '[' => close -= 1,
            ']' => close += 1,
            _ => (),
        }
    }

    return ix;
}

pub fn brainfuck(programm: String, config: BFConfig) -> [u8; 3000] {
    let mut cells: [u8; 3000] = [0; 3000];
    let mut max: i32 = 1;
    let possition: &mut usize = &mut 0;
    let chars: Chars = programm.chars();

    // configuration
    let debug: i32 = config.debug;
    let exit_support: bool = config.exit_support;

    let mut index: i32 = 0;
    while index < chars.clone().count().try_into().unwrap() {
        let cur_char = chars.clone().nth(index.try_into().unwrap()).unwrap();

        match cur_char {
            // # does not actually appear in brainfuck's docs.
            // This character is used to debug only supported
            // in this interpreter (if debug option is set to
            // true)
            '#' => {
                if debug > 0 {
                    println!("{}", cells[*possition])
                }
            }

            // `!` is a custom symbold, it can be used to exit
            // the programm with code 2. You can use it by
            // enable it by declaring exit_support as true
            // in the configuration.
            '!' => {
                if exit_support {
                    std::process::exit(2);
                }
            }

            // Increment value by 1 in current cell possition.
            // if the curren't value for the cell is 255,
            // we will set it to 0.
            '+' => {
                let mut cell: u8 = cells[*possition];

                if cell == 255 {
                    cell = 0;
                } else {
                    cell = cell.wrapping_add(1);
                }

                cells[*possition] = cell;
            }

            // Decrement value by 1 in current cell possition
            '-' => {
                let mut cell: u8 = cells[*possition];

                if cell == 0 {
                    cell = 255;
                } else {
                    cell = cell.wrapping_sub(1);
                }

                cells[*possition] = cell;
            }

            // Move the current possition to the next cell
            '>' => {
                if *possition as i32 == 2999 {
                    *possition = 0
                } else {
                    *possition += 1
                }
            }

            // Go back one cell
            '<' => {
                if (*possition as i32) == 0 {
                    *possition = 2999;
                } else {
                    *possition =
                        *&mut ((*possition as usize).checked_sub(1)).unwrap_or_default() as usize;
                }
            }

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
                    Err(_) => {
                        bf_error::error(
                            "IO error".to_string(),
                            "Error while trying to get an input from stdin".to_string(),
                        );
                    }
                }
            }

            // Left brackets are like c while(cur_block_value != 0) loop.
            '[' => {
                if cells[*possition] == 0 {
                    index = do_left_bracket(chars.clone(), index)
                }
            }

            // if block currently pointed to's value is not zero, jump back to [
            ']' => {
                if cells[*possition] != 0 {
                    index = do_right_bracket(chars.clone(), index)
                }
            }

            // In Brainfuck, other ASCII characters that
            // are not ["+", ",", "-", "<", ">", ".", "[", "]"]
            // they are considered as comments, so we do nothing.
            _ => (),
        }

        index += 1;

        if (*possition as i32) + 1 > max {
            max = (*possition as i32) + 1;
        }
    }

    if debug > 1 {
        println!("\n\n=== CELLS ===");
        for i in 0..max {
            println!("c[{}]: {}", i, cells[i as usize]);
        }
    }

    return cells;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_add() {
        let code: String = "+".to_string();
        let config: BFConfig = BFConfig {
            ..default_bf_config()
        };
        let cells = brainfuck(code, config);
        let mut res: [u8; 3000] = [0; 3000];

        res[0] = 1;
        assert_eq!(res, cells);
    }

    #[test]
    fn cell_sub() {
        let code: String = "-".to_string();
        let config: BFConfig = BFConfig {
            ..default_bf_config()
        };

        let cells = brainfuck(code, config);
        let mut res: [u8; 3000] = [0; 3000];

        res[0] = 255;
        assert_eq!(res, cells);
    }

    #[test]
    fn cell_sub_plus() {
        let code: String = "++-".to_string();
        let config: BFConfig = BFConfig {
            ..default_bf_config()
        };

        let cells = brainfuck(code, config);
        let mut res: [u8; 3000] = [0; 3000];

        res[0] = 1;
        assert_eq!(res, cells);
    }

    #[test]
    fn cell_left() {
        let code: String = ">+".to_string();
        let config: BFConfig = BFConfig {
            ..default_bf_config()
        };

        let cells = brainfuck(code, config);
        let mut res: [u8; 3000] = [0; 3000];

        res[1] = 1;
        assert_eq!(res, cells);
    }

    #[test]
    fn cell_right() {
        let code: String = "><+".to_string();
        let config: BFConfig = BFConfig {
            ..default_bf_config()
        };

        let cells = brainfuck(code, config);
        let mut res: [u8; 3000] = [0; 3000];

        res[0] = 1;
        assert_eq!(res, cells);
    }

    #[test]
    fn cell_right_end() {
        let code: String = "<+".to_string();
        let config: BFConfig = BFConfig {
            ..default_bf_config()
        };

        let cells = brainfuck(code, config);
        let mut res: [u8; 3000] = [0; 3000];

        res[2999] = 1;
        assert_eq!(res, cells);
    }

    #[test]
    fn cell_left_end() {
        let code: String = "<>+".to_string();
        let config: BFConfig = BFConfig {
            ..default_bf_config()
        };

        let cells = brainfuck(code, config);
        let mut res: [u8; 3000] = [0; 3000];

        res[0] = 1;
        assert_eq!(res, cells);
    }

    #[test]
    fn cell_loop() {
        // Move 5 to the left
        // Loop and adding 1 to c1
        // and removing 1 to c2
        let code: String = ">+++++[<+>-]".to_string();
        let config: BFConfig = BFConfig {
            ..default_bf_config()
        };

        let cells = brainfuck(code, config);
        let mut res: [u8; 3000] = [0; 3000];

        res[0] = 5;
        assert_eq!(res, cells);
    }

    #[test]
    fn comments() {
        let code: String = "Lets add one (+) move to left (>) and add 2 (++)".to_string();
        let config: BFConfig = BFConfig {
            ..default_bf_config()
        };

        let cells = brainfuck(code, config);
        let mut res: [u8; 3000] = [0; 3000];

        res[0] = 1;
        res[1] = 2;
        assert_eq!(res, cells);
    }
}
