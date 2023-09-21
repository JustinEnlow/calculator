//! for non interactive use, arguments must be passed either without spaces, 
//! 2+(2*3), or in single or double quotes, "2 + (2 * 3)".
//! it seems arguments beginning with parens does not work, at least with bash, does work with cicada shell

use calculator::calculate;
use std::io::{self, Write};



// change prompt to whatever you would like
const USER_PROMPT: &'static str = "calc: ";



fn main(){
    let mut args = std::env::args();
    if args.len() < 2{
        loop{
            println!("{}", calculate(retrieve_user_input().as_ref()));
        }
    }
    else if args.len() > 2{
        println!("cannot accept more than one argument")
    }
    else{
        println!("{}", calculate(&args.nth(1).unwrap()));
    }
}

// figure out how to allow user to navigate input string for editing while in interactive mode. left, right, home, end, etc.
// delete already works
fn retrieve_user_input() -> String{
    let mut user_input = String::new();

    print!("{USER_PROMPT}");
    io::stdout().flush().unwrap();
    
    io::stdin()
        .read_line(&mut user_input)
        .unwrap();

    user_input
}