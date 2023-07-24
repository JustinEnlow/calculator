//! Command Line Calculator, based on Shunting Yard algorithm.
//! does not implement order of operations. user should define their own order
//! of operations using parentheses. operations will be calculated left to 
//! right if no precedence is defined
 
mod token;

use token::Token;



pub fn calculate(input: &str) -> String{    // switch to Result<f32, Error> for error handling
    let tokens = tokenize(input);
    if tokens.is_empty(){
        return format!("Empty or invalid input string.")
    }
    let postfix_tokens = to_postfix_tokens(&tokens);
    match evaluate(postfix_tokens){
        Ok(ok) => format!("{ok}"),
        Err(e) => format!("{e}")
    }
}

// consider implementing a power ^ operator. remember ^ is right associative
fn tokenize(input: &str) -> Vec<Token>{
    let mut num_string = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    
    for char in input.chars(){
        if !num_string.is_empty() && !is_floating_point_digit(char){
            tokens.push(Token::Number(num_string.parse().unwrap()));
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
            '^' => tokens.push(Token::Power),
            _ => {}
        }
    }

    // push num_string to tokens and clear num_string if we have reached the end of the input string
    if !num_string.is_empty(){
        tokens.push(Token::Number(num_string.parse().unwrap()));
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

fn to_postfix_tokens(tokens: &[Token]) -> Vec<Token>{
    let mut output = Vec::new();
    let mut operations = Vec::new();

    for i in 0..tokens.len(){
        let token = tokens[i];
        match token{
            Token::Number(_) => {
                output.push(token);
                if !operations.is_empty(){
                    output.push(operations.pop().unwrap());
                }
            }
            Token::OpenParen => {
                let result = to_postfix_tokens(&tokens[i+1..tokens.len()]);
                for idk in result{
                    output.push(idk);
                }
                break;
            },
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
                        None => break
                    }
                }
            },
            Token::AddOp | 
            Token::SubOp | 
            Token::MulOp | 
            Token::DivOp |
            Token::Power => operations.push(token)
        }
    }

    for _ in 0..operations.len(){
        output.push(operations.pop().unwrap());
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
                let (rhs, lhs) = retrieve_operands(&mut num_stack)?;
                num_stack.push(lhs + rhs);
            }
            Token::SubOp => {
                let (rhs, lhs) = retrieve_operands(&mut num_stack)?;
                num_stack.push(lhs - rhs);
            }
            Token::MulOp => {
                let (rhs, lhs) = retrieve_operands(&mut num_stack)?;
                num_stack.push(lhs * rhs);
            }
            Token::DivOp => {
                let (rhs, lhs) = retrieve_operands(&mut num_stack)?;
                num_stack.push(lhs / rhs);
            }
            Token::Power => {
                let (rhs, lhs) = retrieve_operands(&mut num_stack)?;
                num_stack.push(f32::powf(lhs, rhs));
            }
            _ => {}
        }
    }
    
    match num_stack.pop(){
        Some(x) => {Ok(x)},
        None => Err("somehow reached a point that is supposed to be unreachable".to_string())
    }
}

fn retrieve_operands(num_stack: &mut Vec<f32>) -> Result<(f32, f32), String>{
    let rhs = match num_stack.pop(){
        Some(val) => {val},
        None => return Err("could not evaluate equation due to unbalanced operands".to_string())
    };
    let lhs = match num_stack.pop(){
        Some(val) => {val},
        None => return Err("could not evaluate equation due to unbalanced operands".to_string())
    };

    Ok((rhs, lhs))
}


///////////////////////////////////////////////////////////////////////////////


#[test]
fn wiki_test(){
    let input = "3 + 4 * 2 / (1 - 5)";
    println!("{}", input);

    let lexer_result = tokenize(input);
    println!("{:?}", lexer_result);
    assert!(lexer_result == vec![Token::Number(3.0), Token::AddOp, Token::Number(4.0), Token::MulOp, Token::Number(2.0), Token::DivOp, Token::OpenParen, Token::Number(1.0), Token::SubOp, Token::Number(5.0), Token::CloseParen]);

    let shunting_yard_result = to_postfix_tokens(&lexer_result);
    println!("{:#?}", shunting_yard_result);
    assert!(shunting_yard_result == vec![Token::Number(3.0), Token::Number(4.0), Token::AddOp, Token::Number(2.0), Token::MulOp, Token::Number(1.0), Token::Number(5.0), Token::SubOp, Token::DivOp]);

    let result = calculate(input);
    println!("{}", result);
    assert!(result == "-3.5");
}

#[test]
fn integer_operation(){
    let input = "2+2";
    println!("input: {:?}", input);

    let lexer_result = tokenize(input);
    println!("lexer result: {:?}", lexer_result);
    assert!(lexer_result == vec![Token::Number(2.0), Token::AddOp, Token::Number(2.0)]);

    let shunting_yard_result = to_postfix_tokens(&lexer_result);
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

    let shunting_yard_result = to_postfix_tokens(&lexer_result);
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

    let shunting_yard_result = to_postfix_tokens(&lexer_result);
    println!("shunting yard result: {:?}", shunting_yard_result);
    assert!(shunting_yard_result == vec![Token::Number(2.0), Token::Number(4.0), Token::AddOp, Token::Number(3.0), Token::MulOp]);

    let result = calculate(input);
    println!("result: {:?}", result);
    assert!(result == "18");
}

#[test]
fn paren_in_paren(){
    let input = "9 / (3 * (2 + 1))";
    println!("{}", input);

    let lexer_result = tokenize(input);
    println!("{:?}", lexer_result);
    assert!(lexer_result == vec![Token::Number(9.0), Token::DivOp, Token::OpenParen, Token::Number(3.0),Token::MulOp, Token::OpenParen, Token::Number(2.0), Token::AddOp, Token::Number(1.0), Token::CloseParen, Token::CloseParen]);

    let shunting_yard_result = to_postfix_tokens(&lexer_result);
    println!("{:#?}", shunting_yard_result);
    assert!(shunting_yard_result == vec![Token::Number(9.0), Token::Number(3.0), Token::Number(2.0), Token::Number(1.0), Token::AddOp, Token::MulOp, Token::DivOp]);

    let result = calculate(input);
    println!("result: {:?}", result);
    assert!(result == "1");
}

#[test]
fn left_to_right_solving(){
    let input = "2 - 3 + 4";
    println!("{:?}", input);

    let lexer_result = tokenize(input);
    println!("{:?}", lexer_result);
    assert!(lexer_result == vec![Token::Number(2.0), Token::SubOp, Token::Number(3.0), Token::AddOp, Token::Number(4.0)]);

    let shunting_yard_result = to_postfix_tokens(&lexer_result);//to_postfix_tokens(lexer_result);
    println!("{:#?}", shunting_yard_result);
    assert!(shunting_yard_result == vec![Token::Number(2.0), Token::Number(3.0), Token::SubOp, Token::Number(4.0), Token::AddOp]);

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
#[test]
fn david_test(){
    let input = "(-1 + 2) - 3 * 5 / 5";
    println!("{:?}", input);

    let lexer_result = tokenize(input);
    println!("{:?}", lexer_result);
    assert!(lexer_result == vec![Token::OpenParen, Token::Number(-1.0), Token::AddOp, Token::Number(2.0), Token::CloseParen, Token::SubOp, Token::Number(3.0), Token::MulOp, Token::Number(5.0), Token::DivOp, Token::Number(5.0)]);

    let shunting_yard_result = to_postfix_tokens(&lexer_result);
    println!("{:#?}", shunting_yard_result);
    assert!(shunting_yard_result == vec![Token::Number(-1.0), Token::Number(2.0), Token::AddOp, Token::Number(3.0), Token::SubOp, Token::Number(5.0), Token::MulOp, Token::Number(5.0), Token::DivOp]);

    let result = calculate(input);
    println!("{:?}", result);
    assert!(result == "-2")
}

#[test]
fn weird_shit_in_parens(){
    let input = "2 + (3 - 1 + 5)";
    println!("{}", input);

    let lexer_result = tokenize(input);
    println!("{:?}", lexer_result);
    //assert!(lexer_result == vec!)

    let shunting_yard_result = to_postfix_tokens(&lexer_result);//to_postfix_tokens(lexer_result);
    println!("{:?}", shunting_yard_result);
    //2 3 1 - 5 + +
    assert!(shunting_yard_result == vec![Token::Number(2.0), Token::Number(3.0), Token::Number(1.0), Token::SubOp, Token::Number(5.0), Token::AddOp, Token::AddOp]);

    let result = calculate(input);
    println!("{}", result);
    assert!(result == "9");
}