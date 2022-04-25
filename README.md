
# Brainfuck++

A brainfuck interpreter and compiler with some extra functionality.

## About brainfuck

The idea behind `brainfuck` is memory manipulation. Basically you are given an array of 30,000 1byte memory blocks. The array size is actually dependent upon the implementation used in the compiler or interpretor, but standard brainfuck states 30,000. Within this array, you can increase the memory pointer, increase the value at the memory pointer, etc. Let me first present to you the 10 operators available to us.

| Symbol 	| Description                                                            	|
|--------	|-------------------------------------------------------------------------	|
| `#`    	| This symbol is used to debug the current's cell value.                 	|
| `!`    	| The `!` symbol will be used to exit the programm with an exit code `2` 	|
| `>`    	| increases memory pointer, or moves the pointer to the right 1 block.  	|
| `<`    	| decreases memory pointer, or moves the pointer to the left 1 block.   	|
| `+`    	| increases value stored at the block pointed to by the memory pointer  	|
| `-`    	| decreases value stored at the block pointed to by the memory pointer  	|
| `]`    	| like c `while(cur_block_value != 0)` loop.                               	|
| `[`    	| if block currently pointed to's value is not zero, jump back to `[`      	|
| `,`    	| like c `getchar()`. input 1 character. 	                                |
| `.`    	| like c `putchar()`. print 1 character to the console                     	|


#### Some rules:

- Any arbitrary character besides the 10 listed above should be ignored by the
compiler or interpretor. Characters besides the 10 operators should be con-
sidered comments.

- All memory blocks on the "array" are set to zero at the beginning of the
program. And the memory pointer starts out on the very left most memory
block.

- Loops may be nested as many times as you want. But all `[` must have a corre-
sponding `]`.

## Installation

To install it, simply add the following line to your `Cargo.toml`

```
brainfuck-plusplus = "0.1.3"
```

You can also install the CLI by doing:
```shell
$ cargo install brainfuck-plusplus
$ brainfuck -V # Check the version for brainfuck.
```

## Usage

Humans need to learn in some way. This interpreter haves 2 parts, a CLI and a module that you can use to interpret brainfuck code.

### CLI

```
$ brainfuck [OPTIONS] [FILE]
```

To add verbosity, add `-v` to the `[OPTIONS]` if you whant to add more levels, add `-vv` or `-vvv` and so on...

To see the help menu, type the following in your command line.
```
$ brainfuck --help
```

### Module

You will need to import the crate and a function called `brainfuck` will be imported. It takes 1 argument as a `String` wich will be brainfuck's source code and a second argument `i32` wich will be debug's level.

```rust
use brainfuck::*;

fn main() {
    let bf_config =  BFConfig {
        ..default_bf_config()
    };

    brainfuck("+++++++>++>-.", config);
}
```

You can also get the used cells as a return value of the fuction.

```rust
let cells: [u8, 3000] = brainfuck(...);
```

### Configuration


```rust
use brainfuck::*;

...

let bf_config =  BFConfig {
    debug: 0,
    ..default_bf_config() // support for default values
};

```

This is the definition of the configuration's struct.
```rust
pub struct BFConfig {

    // The level of verbosity (default to 0)
    pub(crate) debug: i32,
}
```

## Examples

This is a hello world example in brainfuck.

```bf
>++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<+
+.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-
]<+.
```

* [more examples](https://github.com/mauro-balades/brainfuck-plusplus/tree/main/examples)

## Ideas

Brainfuck is a very small language that can be used for beginers to make their first turing complete programing language. Since it is so easy, there is always new ways to make this more interesting.

Here are some ideas:

* Make a REPL
* compiler to other language
    * [`todo`] Python
    * [`todo`] C
    * [`todo`] JS
* More language extensions
    * `#` for cell debuging :tick:
    * `!` for programm exit :tick:
    * `^`   for importing other files
    * `{}`  if statemets (for checking if cells are the same).

## License

This project is under the license of [MIT](https://github.com/mauro-balades/brainfuck-plusplus/blob/main/LICENSE)