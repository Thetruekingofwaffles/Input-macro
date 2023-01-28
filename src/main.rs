use input_macro_attempt::input;
use std::io::Write;

fn main() {
    let x = input!("Enter a variable: ");

    println!("The variable is: {}", x);

}
