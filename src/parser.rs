use std::io::Error;

use crate::Token;

#[derive(Debug)]
pub struct ASTNode{
    pub value:Token,
    pub left:Option<Box<ASTNode>>,
    pub right: Option<Box<ASTNode>>
}

pub struct AST{
    pub root: Option<Box<ASTNode>>
}

pub fn parse_expression(tokens:&mut Vec<Token>)->Result<AST, Error>{
    //print!("exp:{:?}\n",tokens);
    let mut node = parse_term(tokens)?;
    while !tokens.is_empty(){
        let last = &tokens[tokens.len()-1];
        match &last {
            Token::Plus(())=>{
                let t = tokens.pop();
                let right = parse_term(tokens)?;
                //print!("right {:?} {:?}",&node, right);
                node = Box::new(ASTNode { 
                    value: t.expect("Error in the formula"), 
                    left: Some(node), 
                    right: Some(right) });
            },
            Token::CloseExp(())=>break,
            _=> return Err(Error::new(std::io::ErrorKind::InvalidInput,
                 "There was an error while parsing the expression."))
        }
    }
    Ok(AST { root: Some(node) })
}

fn parse_term(tokens:&mut Vec<Token>)->Result<Box<ASTNode>, Error>{
    //print!("term:{:?}\n",tokens);
    let mut node = parse_factor(tokens)?;
    while tokens.len()>0{
        //print!("right {:?} \n",&node);
        match &tokens[tokens.len()-1] {
            Token::Mult(())|Token::Quotient(())=>{
                let t = tokens.pop();
                let right = parse_factor(tokens)?;
                node = Box::new(ASTNode { 
                    value: t.expect("Error in the formula"), 
                    left: Some(node), 
                    right: Some(right) });
            },
            Token::Plus(())|Token::CloseExp(())=>break,
            token=> return Err(Error::new(std::io::ErrorKind::InvalidInput,
                format!("There was an error while parsing the term: {:?}", token)))
        }
    }
    Ok(node)
}

fn parse_factor(tokens:&mut Vec<Token>)->Result<Box<ASTNode>,Error>{
    let node:Box<ASTNode>;
    //print!("factor:{:?}\n",tokens);
    
    if tokens.len()>0{
        match &tokens[tokens.len()-1] {
            Token::Minus(())=>{
                let t = tokens.pop();
                let right = parse_factor(tokens)?;
                node=Box::new(ASTNode{
                    left:None,
                    right:Some(right),
                    value:t.expect("Error while parsing - operator")
                });
                return Ok(node);
            },
            Token::OpenExp(_)=>{
                //print!("( entered ");
                tokens.pop(); //popping (
                node = parse_expression(tokens)?
                    .root
                    .expect("Error while parsing the expression in ().");
                //print!("1: {:?} ", tokens);
                tokens.pop(); //popping )
                //print!("2: {:?} ", tokens);
                return Ok(node);
            },
            Token::Number(_)=>{
                ////print!("number entered ");
                let n = tokens.pop();
                node = Box::new(ASTNode{
                    value:n.expect("Error while unwrapping Number Token."),
                    left:None,
                    right:None
                });
                ////print!("number:{:?}\n",tokens);
                return Ok(node);
            },
            _=>()
        }
    }
    Err(Error::new(std::io::ErrorKind::InvalidInput,
        "Error while parsing factor."))
}