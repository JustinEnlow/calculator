//! Command Line Calculator, based on Shunting Yard algorithm.
//! does not implement order of operations. user should define their own order
//! of operations using parentheses.
 
mod token;

use token::Token;



pub fn calculate(input: &str) -> String{    // switch to Result<f32, Error> for error handling
    let tokens = tokenize(input);
    if tokens.is_empty(){
        return format!("Empty or invalid input string.")
    }
    let postfix_tokens = to_postfix_tokens(tokens);
    match evaluate(postfix_tokens){
        Ok(ok) => format!("{ok}"),
        Err(e) => format!("{e}")
    }
}

fn tokenize(input: &str) -> Vec<Token>{
    let mut num_string = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    
    for char in input.chars(){
        if !num_string.is_empty() && !is_floating_point_digit(char){
            tokens.push(Token::Number(parse_num_string(&num_string)));
            num_string.clear();
        }
    
        match char{
            '0'..='9' | '.' => num_string.push(char),
            '+' => tokens.push(Token::AddOp),
            '-' => {
                match tokens.last(){
                    Some(prev_token) => match prev_token{
                        // if previous token was a number or close paren, this '-' is a subtraction
                        Token::Number(_) | Token::CloseParen => tokens.push(Token::SubOp), 
                        // otherwise, it is a negative
                        _ => num_string.push(char)
                    },
                    // if no previous token, this '-' is a negative sign
                    None => num_string.push(char)
                }
            },
            '*' => tokens.push(Token::MulOp),
            '/' => tokens.push(Token::DivOp),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            _ => {}
        }
    }

    // push num_string to tokens and clear num_string if we have reached the end of the input string
    if !num_string.is_empty(){
        tokens.push(Token::Number(parse_num_string(&num_string)));
        num_string.clear();
    }

    tokens
}

// this or can also use reg expressions to match char as floating point digit [\d.]
fn is_floating_point_digit(char: char) -> bool{
    if char.is_ascii_digit() || char == '.'{
        return true;
    }
    else{
        return false;
    }
}

fn parse_num_string(num_string: &str) -> f32{
    match num_string.parse(){
        Ok(val) => val,
        _ => unreachable!("numbers validated previously. should not fail to parse.")
    }
}

/// converts tokens to reverse polish/postfix notation
fn to_postfix_tokens(tokens: Vec<Token>) -> Vec<Token>{
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
                                operations.pop().unwrap();
                                break;
                            }
                            else{
                                output.push(operations.pop().unwrap());   
                            }
                        },
                        None => break //inform user of unbalanced paren use?
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
        // can be reached if input equation is unbalanced. ex: 2 +
        None => unimplemented!("unimplemented")
    }
}


///////////////////////////////////////////////////////////////////////////////


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

// may need to implement precedence
// better test is 2 - 3 + 4. expect 3, result is -5
#[test]
fn left_to_right_solving(){
    let input = "2 - 3 + 4";
    println!("{:?}", input);

    let lexer_result = tokenize(input);
    println!("{:?}", lexer_result);
    assert!(lexer_result == vec![
        Token::Number(2.0),
        Token::SubOp,
        Token::Number(3.0),
        Token::AddOp,
        Token::Number(4.0)
    ]);

    let shunting_yard_result = to_postfix_tokens(lexer_result);
    println!("{:#?}", shunting_yard_result);
    assert!(shunting_yard_result == vec![
        Token::Number(2.0),
        Token::Number(3.0),
        Token::Number(4.0),
        Token::AddOp,
        Token::SubOp,
    ]);

    let result = calculate(input);
    println!("{:?}", result);
    assert!(result == "3")
}

#[test]
fn idk(){
    let input = "2 + -3 + 4";
    let result = calculate(input);
    assert!(result == "3");
}

// reconsider this test once previous is working correctly
//#[test]
//fn david_test(){
//    let input = "(-1 + 2) - 3 * 5 / 5";
//    println!("{:?}", input);
//
//    let lexer_result = tokenize(input);
//    println!("{:?}", lexer_result);
//    assert!(lexer_result == vec![
//        Token::OpenParen,
//        Token::Number(-1.0),
//        Token::AddOp,
//        Token::Number(2.0),
//        Token::CloseParen,
//        Token::SubOp,
//        Token::Number(3.0),
//        Token::MulOp,
//        Token::Number(5.0),
//        Token::DivOp,
//        Token::Number(5.0),
//    ]);
//
//    let shunting_yard_result = to_postfix_tokens(lexer_result);
//    println!("{:#?}", shunting_yard_result);
//    assert!(shunting_yard_result == vec![
//        Token::Number(-1.0),
//        Token::Number(2.0),
//        Token::AddOp,
//        Token::Number(3.0),
//        Token::Number(5.0),
//        Token::Number(5.0),
//        Token::DivOp,
//        Token::MulOp,
//        Token::SubOp,
//    ]);
//
//    let result = calculate(input);
//    println!("{:?}", result);
//    assert!(result == "-2")
//}



// figure out some way to emit an error when input has operators of differing 
// precedence without parentheses.
// example: 2 + 4 * 3 ::: error: ambiguous operator precedence. use 
//                    ::: parentheses to clarify
// since we don't assume operator precedence, the user must clarify their
// intent.
// example: 2 + 4 + 3 would be fine, because they share operator precedence.
// example: 2 + 4 - 3 would be fine as well, for the same reason.