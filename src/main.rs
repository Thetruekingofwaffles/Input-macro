use input_macro_attempt::input;
use std::io::Write;

fn main() {
    let x = input!("Enter a variable: ");
    let y = input!();
    println!("The variable is: {} and {}", x, y);

}
