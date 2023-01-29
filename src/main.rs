use input_macro_attempt::input;


fn main() {
    let x = input!("Enter a variable: ");
    let y = input!();
    println!("The variable is: {} and {}", x, y);

}
