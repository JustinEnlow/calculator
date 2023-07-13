use calculator::calculate;
use std::io::{self, Write};



// change prompt to whatever you would like
const USER_PROMPT: &'static str = ">> ";



fn main(){
    loop{
        println!("{}", calculate(retrieve_user_input().as_ref()));
    }
}

fn retrieve_user_input() -> String{
    let mut user_input = String::new();

    print!("{USER_PROMPT}");
    io::stdout().flush().unwrap();
    
    io::stdin()
        .read_line(&mut user_input)
        .unwrap();

    user_input
}