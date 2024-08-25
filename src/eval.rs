use std::io::Error;

use crate::{parser::ASTNode, lexer::Token};


pub fn print_ast(root:&Box<ASTNode>)->Result<String,Error>{
    let mut str = String::new();
    match &(root.value) {
        Token::Number(n)=> {
            let s = n.to_string();
            str = str + &s;
        }
        Token::Plus(()) => str+= get_str_from_op(Token::Plus(()), &root)?.as_str(),
        Token::Mult(()) =>str += get_str_from_op(Token::Mult(()), &root)?.as_str(),
        Token::Quotient(())=>str +=  get_str_from_op(Token::Quotient(()), &root)?.as_str(),
        Token::Minus(())=>{
            let str_r=print_ast(root.right.as_ref().expect("error"))?;
            str = format!("-({})", str_r);
        },
        _=>return Err(Error::new(std::io::ErrorKind::Unsupported, 
            "Unrecognized Token"))
    }
    Ok(str)
}

fn get_str_from_op(token:Token, node:&ASTNode)->Result<String, Error>{
    let str_l =print_ast(node.left.as_ref().expect("error"));
    let str_r =  print_ast(node.right.as_ref().expect("error"));
    let token_str = match token {
        Token::Plus(())=>"+",
        Token::Mult(())=>"*",
        Token::Quotient(())=>"/",
        _=> return Err(Error::new(std::io::ErrorKind::Unsupported, "Not a valid token to print"))
    };
    let str = format!("({}{}{})", str_l?,token_str, str_r?);
    Ok(str)
}

pub fn eval(ast:&Box<ASTNode>)->f64{
    match &ast.value {
        Token::Plus(())=>return eval(ast.left.as_ref().expect("err in eval before +")) 
                            + eval(ast.right.as_ref().expect("err in eval after +")),
        Token::Minus(())=>return - eval(ast.right.as_ref().expect("err in eval after -")),
        Token::Mult(())=>return eval(ast.left.as_ref().expect("err in eval before *")) 
                                * eval(ast.right.as_ref().expect("err in eval after *")),
        Token::Quotient(())=>return eval(ast.left.as_ref().expect("err in eval before /")) 
                                / eval(ast.right.as_ref().expect("err in eval after /")),
        Token::Number(n)=> *n as f64,
        _=>0.0
    }
}
