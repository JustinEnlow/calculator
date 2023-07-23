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

// this or can also use reg expressions to match char as floating point digit [\d.]
fn is_floating_point_digit(char: char) -> bool{
    if char.is_ascii_digit() || char == '.'{
        return true;
    }

    false
}

fn tokenize(input: &str) -> Vec<Token>{
    //let mut last_char_was_number = false;
    let mut num_string = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    
    for char in input.chars(){
        // can we replace 'last_char_was_number' with '!num_string.is_empty()'? ...seems that we can
        //if last_char_was_number && !is_floating_point_digit(char){
        if !num_string.is_empty() && !is_floating_point_digit(char){
            tokens.push(Token::Number(parse_num_string(&num_string)));
            num_string.clear();
        }
    
        match char{
            '0'..='9' | '.' => num_string.push(char),
            '+' => tokens.push(Token::AddOp),
            '-' => {
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

        //last_char_was_number = is_floating_point_digit(char);
    }

    // push num_string to tokens and clear num_string if we have reached the end of the input string
    if !num_string.is_empty(){
        tokens.push(Token::Number(parse_num_string(&num_string)));
        num_string.clear();
    }

    tokens
}

fn parse_num_string(num_string: &str) -> f32{
    // this bit of code is our older style, handled at each location this function is now called
    //match num_string.parse(){
    //    Ok(val) => tokens.push(Token::Number(val)),
    //    _ => {numbers validated previously. should not fail to parse.}
    //}
    // end of older code
    match num_string.parse(){
        Ok(val) => val,
        _ => unreachable!("numbers validated previously. should not fail to parse.")
    }
}

/// converts tokens to reverse polish/postfix notation
fn to_postfix_tokens(tokens: Vec<Token>) -> Vec<Token>{ // -> Result<Vec<Token>, Err>
    let mut output = Vec::new();
    let mut operations = Vec::new();
    
    for token in tokens{
        match token{
            Token::Number(_) => output.push(token),
            Token::CloseParen => {
                loop{
                    match operations.last(){
                        Some(&last_token) => {
                            if last_token == Token::OpenParen{
                                match operations.pop(){
                                    Some(_) => {},
                                    None => {println!("missing '(' in operator stack")} // make actual error later
                                }
                                break;
                            }
                            else{
                                output.push(operations.pop().unwrap());   
                            }
                        },
                        None => break
                    }
                }
            },
            Token::AddOp | 
            Token::SubOp | 
            Token::MulOp | 
            Token::DivOp | 
            Token::OpenParen => operations.push(token)
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

// figure out how to make this work properly
#[test]
fn twos(){
    let input = vec![
        Token::Number(2.0),
        Token::AddOp,
        Token::Number(2.0),
        Token::SubOp,
        Token::Number(2.0)
    ];
    let output = to_postfix_tokens(input);
    println!("{:#?}", output);
    assert!(output == vec![
        Token::Number(2.0),
        Token::Number(2.0),
        Token::AddOp,
        Token::Number(2.0),
        Token::SubOp
    ]);
}

// feature, not a bug. should fail, because i have elected not to assume operator precedence
// how can we code this so that equal precedence operators don't need parens. ex: 2 + 2 + 2
#[test]
#[should_panic]
fn david_test(){
    // (-1 + 2) - 3 * 4 / 5
    let input = vec![
        Token::OpenParen,
        Token::Number(-1.0),
        Token::AddOp,
        Token::Number(2.0),
        Token::CloseParen,
        Token::SubOp,
        Token::Number(3.0),
        Token::MulOp,
        Token::Number(4.0),
        Token::DivOp,
        Token::Number(5.0)
    ];
    let output = to_postfix_tokens(input);
    println!("{:#?}", output);
    assert!(output == vec![
        Token::Number(-1.0),
        Token::Number(2.0),
        Token::AddOp,
        Token::Number(3.0),
        Token::SubOp,
        Token::Number(4.0),
        Token::MulOp,
        Token::Number(5.0),
        Token::DivOp
    ]);
}

#[test]
fn paren_in_paren(){
    // 9 / ( 3 * (2 + 1))
    // 9 3 2 1 + * /
    let input = vec![
        Token::Number(9.0),
        Token::DivOp,
        Token::OpenParen,
        Token::Number(3.0),
        Token::MulOp, 
        Token::OpenParen,
        Token::Number(2.0),
        Token::AddOp,
        Token::Number(1.0),
        Token::CloseParen,
        Token::CloseParen
    ];
    let output = to_postfix_tokens(input);
    println!("{:#?}", output);
    assert!(output == vec![
        Token::Number(9.0),
        Token::Number(3.0),
        Token::Number(2.0),
        Token::Number(1.0),
        Token::AddOp,
        Token::MulOp,
        Token::DivOp
    ]);
}



// figure out some way to emit an error when input has operators of differing 
// precedence without parentheses.
// example: 2 + 4 * 3 ::: error: ambiguous operator precedence. use 
//                    ::: parentheses to clarify
// since we don't assume operator precedence, the user must clarify their
// intent.
// example: 2 + 4 + 3 would be fine, because they share operator precedence.
// example: 2 + 4 - 3 would be fine as well, for the same reason.