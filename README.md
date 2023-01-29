# Input-macro
A simple input macro for rust, it works similar to the python input function.

I hated typing stdin everytime because other languages(like c++ and python) had more concise syntax. I've been learning python recently and I decided I was going to make a macro that shortened the syntax and combined print for added convience.


Example (Just copied what was in main.rs):

```
use input_macro_attempt::input;

fn main() {
    let x = input!("Enter a variable: ");
    let y = input!(); 
    println!("The variable is: {} and {}", x, y);
}
```




