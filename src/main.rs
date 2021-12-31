use std::env;

fn brainfuck() {
    
}

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


    } else {
        // Error, filename was not given
    }
}
