/*
Una expresión se define como:
1. Numero
2. 1. + {1.}
3. {OpUnaria} + 2.
4. 3. + OpBinaria + 3.
5. 4. + {OpBinaria + 3.}
6. ( + 3. + )
7. ( + 4. + )
8. ( + 5. + ) 
*/

use std::{fs::File, io::{Error, Write}};

use rand::prelude::*;

use crate::lexer::Token;

pub fn write_to_testfile()->Result<(),Error>{
    let mut file = File::create("src/input.txt")?;
    for _ in 0..1000{
        let expr = generate_expr();
        //print!("{}\n", expr);
        writeln!(file, "{}=0", expr)?;
    }
    Ok(())
}

fn generate_expr()->String{
    let mut expr = String::new();
    let length = random::<u32>()%10+10;
    let mut open_parenthesis = 0;
    let mut i = 0;
    let mut ends_with_op = false;
    //print!("l {}\n", length);
    while i<length || open_parenthesis>0 || ends_with_op{
        if i>0 {
            match  expr.chars().last().unwrap(){
                '+'|'-'|'/'|'*'|'('=> ends_with_op=true,
                _=>ends_with_op=false
            }
        }
        //println!("e: {}", expr);
        let tokens: Vec<Token> =get_possible_tokens(&expr, &open_parenthesis, i>length);
        
        let rand_index = random::<usize>()%tokens.len();
        let chosen = &tokens[rand_index];
        
        match chosen {
            Token::CloseExp(())=>{
                expr.push(')');
                open_parenthesis-=1;
            },
            Token::OpenExp(())=>{
                expr.push('(');
                open_parenthesis+=1;
            },
            Token::Minus(())=>expr.push('-'),
            Token::Plus(())=>expr.push('+'),
            Token::Mult(())=>expr.push('*'),
            Token::Quotient(())=>expr.push('/'),
            Token::Number(_)=>expr.push_str(&(random::<i8>()%10).to_string()),
        }
        i+=1;
    }
    
    match  expr.chars().last().unwrap(){
        '+'|'-'|'/'|'*'|'('=> print!("{}\n",expr),
        _=>()
    }
        
    
    expr
}

fn get_possible_tokens(expr:&String, open_parenthesis:&i32, len_exceeded: bool)->Vec<Token>{
    let mut possible_tokens;
    if expr.is_empty(){
        return vec![Token::OpenExp(()), Token::Minus(()), Token::Number(0.0)];
    }
    match expr.chars().last().unwrap() {
        '+'|'*'|'/'=> possible_tokens= Vec::from([
            Token::Number(0.0), 
            Token::Minus(()), 
            Token::OpenExp(())]),
        '-'=> possible_tokens= Vec::from([
            Token::Number(0.0),
            Token::Minus(()), 
            Token::OpenExp(())]),
        '0'..='9'=>possible_tokens= Vec::from([
            Token::Number(0.0), 
            Token::Plus(()),
            Token::Mult(()),
            Token::Quotient(())]),
        '('=>possible_tokens= Vec::from([
            Token::Number(0.0),
            Token::Minus(()),
            Token::OpenExp(())]),
        ')'=>possible_tokens=Vec::from([
            Token::Minus(()),
            Token::Plus(()),
            Token::Mult(()),
            Token::Quotient(()),
        ]),
        other=>panic!("This shouln't be here... {}", other)
    }
    if len_exceeded{
        let __;
        match  expr.chars().last().unwrap() {
            '+' | '-' | '*' | '/' =>__= possible_tokens.pop(),
            _=>__=Some(Token::OpenExp(()))
        }
    }
    if *open_parenthesis>0{
        match expr.chars().last().unwrap() {
            '0'..='9'=>possible_tokens.push(Token::CloseExp(())),
            _=>()
        }
    }
    possible_tokens
}