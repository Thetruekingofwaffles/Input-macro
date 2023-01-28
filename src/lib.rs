use std::io;
use std::io::Write;


#[macro_export]
macro_rules! input{
    ($a:expr) =>{
       {
        
        print!("{}", $a);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
       }
    };

   () =>{{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        input
        }
    };


}

