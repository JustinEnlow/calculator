//! Command Line Calculator, based on Shunting Yard algorithm.
//! does not implement order of operations. user should define their own order
//! of operations using parentheses.

pub fn calculate(input: &str) -> String{
    let tokens = tokenize(input);
    let postfix_tokens = to_postfix_tokens(tokens);
    match evaluate(postfix_tokens){
        Ok(ok) => format!("{ok}"),
        Err(e) => format!("{e}")
    }
}

fn tokenize(input: &str) -> Vec<Token>{
    let mut last_char_was_number = false;
    let mut num_string = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    
    for char in input.chars(){
        // push num_string to tokens and clear num_string if current char is not a valid number character
        if last_char_was_number && !char.is_ascii_digit() && char != '.'{
            match num_string.parse(){
                Ok(val) => tokens.push(Token::Number(val)),
                _ => {/*numbers validated previously. should not fail to parse.*/}
            }
            num_string.clear();
        }
    
        match char{
            '0'..='9' | '.' => num_string.push(char),
            '+' => tokens.push(Token::AddOp),
            '-' => {
                // handles '-' as negative rather than subtraction
                match tokens.last(){
                    // if no previous token, this '-' is a negative sign
                    None => num_string.push(char),
                    Some(prev_token) => match prev_token{
                        // if previous token was a number or close paren, this '-' is a subtraction
                        Token::Number(_) | Token::CloseParen => tokens.push(Token::SubOp), 
                        // otherwise, it is a negative
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

    // push num_string to tokens and clear num_string if we have reached the end of the input string
    if !num_string.is_empty(){
        match num_string.parse(){
            Ok(val) => tokens.push(Token::Number(val)),
            _ => {/*numbers validated previously. should not fail to parse.*/}
        }
        num_string.clear();
    }

    tokens
}

/// converts tokens to reverse polish/postfix notation
fn to_postfix_tokens(tokens: Vec<Token>) -> Vec<Token>{
    let mut output = Vec::new();
    let mut operations = Vec::new();
    
    for token in tokens{
        match token{
            Token::Number(_) => output.push(token),
            Token::OpenParen => operations.push(token),
            Token::CloseParen => {
                // pushes operations to output, until an opening paren is reached
                loop{// can infinite loop if no open paren present
                    match operations.last(){
                        Some(&last_token) => {
                            if last_token == Token::OpenParen{
                                break;
                            }
        
                            output.push(operations.pop().unwrap());
                        },
                        None => {
                            break;
                        }
                    }
                }
                match operations.pop(){//.expect("missing '(' in operator stack");
                    Some(_) => {},
                    None => {println!("missing '(' in operator stack")} // make actual error later
                }
            },
            Token::AddOp | Token::SubOp | Token::MulOp | Token::DivOp => operations.push(token)
        }
    }

    for &op in operations.iter().rev(){
        output.push(op);
    }

    output
}

fn evaluate(tokens: Vec<Token>) -> Result<f32, String>{
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
    
    // can panic if input is non number characters
    // create error handling. Result<f32, Box<Error>>
    //num_stack.pop().unwrap()
    match num_stack.pop(){
        Some(x) => {Ok(x)},
        None => {Err("token stack is empty. cannot evaluate an empty stack. double check that input is numerical and non empty".to_string())}
    }
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





#[test]
fn integer_operation(){
    let input = "2+2";
    println!("input: {:?}", input);

    let lexer_result = tokenize(input);
    println!("lexer result: {:?}", lexer_result);
    assert!(lexer_result == vec![Token::Number(2.0), Token::AddOp, Token::Number(2.0)]);

    let shunting_yard_result = to_postfix_tokens(lexer_result);
    println!("shunting yard result: {:?}", shunting_yard_result);
    assert!(shunting_yard_result == vec![Token::Number(2.0), Token::Number(2.0), Token::AddOp]);

    let result = calculate(input);
    println!("result: {:?}", result);
    assert!(result == "4");
}

#[test]
fn balanced_parens(){
    let input = "(2 + 4) * 3";
    println!("input: {:?}", input);

    let lexer_result = tokenize(input);
    println!("lexer result: {:?}", lexer_result);
    assert!(lexer_result == vec![Token::OpenParen, Token::Number(2.0), Token::AddOp, Token::Number(4.0), Token::CloseParen, Token::MulOp, Token::Number(3.0)]);

    let shunting_yard_result = to_postfix_tokens(lexer_result);
    println!("shunting yard result: {:?}", shunting_yard_result);
    assert!(shunting_yard_result == vec![Token::Number(2.0), Token::Number(4.0), Token::AddOp, Token::Number(3.0), Token::MulOp]);

    let result = calculate(input);
    println!("result: {:?}", result);
    assert!(result == "18");
}

#[test]
fn negatives(){
    let input = "-2 * -2";
    println!("input: {:?}", input);

    let lexer_result = tokenize(input);
    println!("lexer result: {:?}", lexer_result);
    assert!(lexer_result == vec![Token::Number(-2.0), Token::MulOp, Token::Number(-2.0)]);

    let shunting_yard_result = to_postfix_tokens(lexer_result);
    println!("shunting yard result: {:?}", shunting_yard_result);
    assert!(shunting_yard_result == vec![Token::Number(-2.0), Token::Number(-2.0), Token::MulOp]);

    let result = calculate(input);
    println!("result: {:?}", result);
    assert!(result == "4");
}