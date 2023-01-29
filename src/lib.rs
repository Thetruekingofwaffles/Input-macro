

#[macro_export]
macro_rules! input{
    ($a:expr) =>{
       
        {
            use std::io::Write;
        print!("{}", $a);
        ::std::io::stdout().flush().unwrap();
        let mut input = String::new();
        ::std::io::stdin().read_line(&mut input).unwrap();
        let y = input.trim().to_owned(); //trim the whitespace out
        y
       }
    };

   () =>{{
        use std::io::Write;
        let mut input = String::new();
        ::std::io::stdin().read_line(&mut input).unwrap();
        let y = input.trim().to_owned(); //trim the whitespace out
        y
        }
    };


}

