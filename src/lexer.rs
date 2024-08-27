use std::{char, io::{Error,ErrorKind}};

#[derive(std::fmt::Debug)]
#[derive(PartialEq)]
pub enum Token{
    Number(f64),
    OpenExp(()), // (
    CloseExp(()), // )
    Plus(()), 
    Minus(()),
    Mult(()),
    Quotient(())
}

pub fn lexer(input:&str)->Result<Vec<Token>, Error>{
    let mut tokens = Vec::new();
    let mut dot_count = 0;
    let mut buffer = String::new();
    for c in input.chars(){
        match c {
            '+' => push_number_and_token(Token::Plus(()), &mut tokens, &mut buffer, &mut dot_count),
            '-' => push_number_and_token(Token::Minus(()), &mut tokens, &mut buffer, &mut dot_count),
            '*' => push_number_and_token(Token::Mult(()), &mut tokens, &mut buffer, &mut dot_count),
            '/' => push_number_and_token(Token::Quotient(()), &mut tokens, &mut buffer, &mut dot_count),
            '(' => push_number_and_token(Token::OpenExp(()), &mut tokens, &mut buffer, &mut dot_count),
            ')' => push_number_and_token(Token::CloseExp(()), &mut tokens, &mut buffer, &mut dot_count),
            '0'..='9' | '.' =>{
                buffer=add_char_to_buffer(buffer, c, &mut dot_count)?;
            },
            ' '|'\n'=>(),
            _ => return Err(Error::new(ErrorKind::InvalidInput, format!("Invalid Character in input: {}",c)))
        }
    }
    insert_number_if_finished(&mut buffer, &mut tokens, &mut dot_count);
    tokens.reverse();
    Ok(tokens)
}

fn push_number_and_token(token:Token, tokens:&mut Vec<Token>, buffer:&mut String, dot_count:&mut i32){
    insert_number_if_finished(buffer, tokens, dot_count);
    if token==Token::Minus(()){
        let last_token = tokens.pop();
        match last_token {
            Some(Token::Number(_))|Some(Token::CloseExp(()))=>{
                tokens.push(last_token.unwrap());
                tokens.push(Token::Plus(()));
            },
            None=>(),
            _=>tokens.push(last_token.unwrap())
        }
        //print!("{:?}", tokens);
    }
    tokens.push(token);
}

fn insert_number_if_finished(buffer:&mut String, tokens:&mut Vec<Token>, dot_count:&mut i32){
    if !buffer.is_empty(){
        tokens
            .push(Token::Number(buffer
                .parse::<f64>()
                .expect(&format!("The buffer content is not a number {}",buffer))));
        buffer.clear();
        *dot_count=0;
    }
}

fn add_char_to_buffer(mut buffer:String, c:char, dot_count:&mut i32)->Result<String, Error>{
    if c=='.' {
        *dot_count+=1;
    }
    if *dot_count > 1 {
        return Err(Error::new(ErrorKind::InvalidInput,"Too many decimal points."));
    }
    buffer.push(c);
    Ok(buffer)
}