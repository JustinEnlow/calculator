//! Command Line Calculator.
//! does not implement order of operations. user should define their own order
//! of operations using parentheses.



use std::io::{self, Write};

// change prompt to whatever you would like
const USER_PROMPT: &'static str = ">> ";

fn main() {
    loop{
        print!("{USER_PROMPT}");
        io::stdout().flush().unwrap();

        let user_input = retrieve_user_input();        
    
        let tokens = tokenize(&user_input);
    
        let postfix_tokens = parse_tokens(tokens);

        println!("{}", evaluate(postfix_tokens));
    }
}

fn retrieve_user_input() -> String{
    let mut user_input = String::new();
    
    io::stdin()
        .read_line(&mut user_input)
        .unwrap();

    user_input
}

fn tokenize(input: &str) -> Vec<Token>{
    let mut last_char_was_number = false;
    let mut num_string = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    
    for char in input.chars(){
        if last_char_was_number && !char.is_ascii_digit() && char != '.'{
            match num_string.parse(){
                Ok(val) => tokens.push(Token::Number(val)),
                Err(_) => {}
            }
            num_string.clear();
        }
    
        match char{
            '0'..='9' | '.' => num_string.push(char),
            '+' => tokens.push(Token::AddOp),
            '-' => {
                match tokens.last(){
                    None => num_string.push(char),
                    Some(prev_token) => match prev_token{
                        Token::Number(_) | Token::CloseParen => tokens.push(Token::SubOp), 
                        _ => num_string.push(char)
                    }
                }
            },
            '*' => tokens.push(Token::MulOp),
            '/' => tokens.push(Token::DivOp),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            _ => {}
        }

        last_char_was_number = char.is_ascii_digit() || char == '.';
    }

    tokens
}

fn parse_tokens(tokens: Vec<Token>) -> Vec<Token>{
    let mut output = Vec::new();
    let mut operations = Vec::new();
    
    for token in tokens{
        match token{
            Token::Number(_) => output.push(token),
            Token::OpenParen => operations.push(token),
            Token::CloseParen => {
                loop{
                    match operations.last(){
                        Some(&last_token) => {
                            if last_token == Token::OpenParen{break;}
        
                            output.push(operations.pop().unwrap());
                        },
                        None => {}
                    }
                }
                operations.pop().expect("missing '(' in operator stack");
            },
            Token::AddOp | Token::SubOp | Token::MulOp | Token::DivOp => operations.push(token)
        }
    }

    for &op in operations.iter().rev(){
        output.push(op);
    }

    output
}

fn evaluate(tokens: Vec<Token>) -> f32{
    let mut num_stack = Vec::new();

    for &token in tokens.iter(){
        match token{
            Token::Number(val) => {
                num_stack.push(val);
                continue;
            },
            Token::AddOp => {
                let rhs = num_stack.pop().unwrap();
                let lhs = num_stack.pop().unwrap();
                num_stack.push(lhs + rhs);
            }
            Token::SubOp => {
                let rhs = num_stack.pop().unwrap();
                let lhs = num_stack.pop().unwrap();
                num_stack.push(lhs - rhs);
            }
            Token::MulOp => {
                let rhs = num_stack.pop().unwrap();
                let lhs = num_stack.pop().unwrap();
                num_stack.push(lhs * rhs);
            }
            Token::DivOp => {
                let rhs = num_stack.pop().unwrap();
                let lhs = num_stack.pop().unwrap();
                num_stack.push(lhs / rhs);
            }
            _ => {}
        }
    }
    
    num_stack.pop().unwrap()
}

#[derive(PartialEq, Clone, Copy)]
enum Token{
    Number(f32),
    AddOp,
    SubOp,
    MulOp,
    DivOp,
    OpenParen,
    CloseParen,
}
impl std::fmt::Debug for Token{
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match *self{
            Token::Number(char) => write!(f, "Number({})", char),
            Token::AddOp => write!(f, "AddOp"),
            Token::SubOp => write!(f, "SubOp"),
            Token::MulOp => write!(f, "MulOp"),
            Token::DivOp => write!(f, "DivOp"),
            Token::OpenParen => write!(f, "OpenParen"),
            Token::CloseParen => write!(f, "CloseParen"),
        }
    }
}
// figure out how to manually implement partial equivalence
//impl std::cmp::PartialEq for Token{
//    fn eq(&self, other: &Self) -> bool {}
//    fn ne(&self, other: &Self) -> bool {}
//}