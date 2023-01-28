# Input-macro
A simple input macro for rust, it works similar to the python input function


Example:

use input_macro_attempt::input;
use std::io::Write;

fn main() {
    let x = input!("Enter a variable: ");
    let y = input!();
    println!("The variable is: {} and {}", x, y);

}

^It's just main.rs

Use main.rs as an example because it's where i'm basically testing functionality and stuff