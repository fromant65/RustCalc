use core::f64;
use std::io;
#[derive(std::fmt::Debug)]
#[derive(Clone)]
enum Token {
    Number(i32),
    Float(f64),
    OpenExp(()), // (
    CloseExp(()), // )
    Operation(String) // + - * /
}
#[derive(Clone)]
#[derive(Debug)]
struct AST{
    value:Token,
    left:Option<Box<AST>>,
    right: Option<Box<AST>>
}

fn string_lexer(input:&str)->Vec<String>{
    let mut tokens_str:Vec<String> = Vec::new();
    let mut last_char = ' ';
    for c in input.chars(){
        match c {
            '+'| '-' | '*' | '/' | '(' | ')' => {
                tokens_str.push(String::from(c)); 
                last_char=c},
            '0'..='9' | '.' =>{
                if matches!(last_char, '+'| '-' | '*' | '/' | '(' | ')' | ' '){
                    tokens_str.push(String::from(c));
                }else{
                    tokens_str.last_mut().unwrap().push(c);
                }
                last_char=c;
            },
            _ => ()
        }
    }
    tokens_str
}

fn to_float(number:&String)->Option<f64>{
    let return_value: Option<f64>;
    let mut dot_count = 0;
    for c in number.chars(){
        match c {
            '.'=>dot_count+=1,
            _=>()
        }
    }
    if dot_count>1{
        return_value=None
    }else{
        return_value = number.parse::<f64>().ok();
    }
    return_value
}

fn lexer(tokens_str:&Vec<String>)->Vec<Token>{
    let mut tokens:Vec<Token> = Vec::new();
    for t in tokens_str{
        let tok = t.clone();
        if tok.len()>1 || tok.chars().any(|v| matches!(v,'0'..='9')){
            if tok.contains('.'){
                let float = to_float(&tok);
                match float {
                    None => (),
                    Some(v)=>tokens.push(Token::Float(v))
                }
            }else {
                let int = tok.parse::<i32>().ok();
                match int {
                    None => (),
                    Some(v)=>tokens.push(Token::Number(v))
                }
            }
        }else{
            let iter = tok.chars();
            if iter.to_owned().all(|c| matches!(c, '+' | '-' | '*' | '/')) {
                tokens.push(Token::Operation(tok));
            }else if iter.to_owned().all(|c| matches!(c, '(')){
                //print!("open {:?}\n", iter);
                tokens.push(Token::OpenExp(()));
            }else if iter.to_owned().all(|c| matches!(c, ')')){
                tokens.push(Token::CloseExp(()));
            }
        }
    }
    tokens
}

fn add_plus_before_minus(mut tokens:Vec<Token>)->Vec<Token>{
    let mut i = 0;
    while i<tokens.len(){
        match &tokens[i] {
            Token::Operation(o)=>{
                if *o==String::from("-"){
                    tokens.insert(i, Token::Operation(String::from("+")));
                    i+=1;
                }
            }
            _=>()
        }
        i+=1
    }
    
    tokens
}

fn reverse_tokens(mut tokens:Vec<Token>)->Vec<Token>{
    tokens.reverse();
    tokens
}

fn parse_expression(tokens:&mut Vec<Token>)->Box<AST>{
    //print!("exp:{:?}\n",tokens);
    let mut node = parse_term(tokens);
    while tokens.len()>0{
        match &tokens[tokens.len()-1] {
            Token::Operation(o)=>{
                if *o == String::from("+"){
                    //print!("+ entered\n");
                    let t = tokens.pop();
                    let right = parse_term(tokens);
                    //print!("right {:?} {:?}",&node, right);
                    node.left=Some(node.clone());
                    node.right=Some(right);
                    node.value=t.expect("Error en la formula");
                }else{
                    print_ast(&node);
                    //print!("* / or - entered\n");
                    break;
                }
            },
            Token::CloseExp(()) => break,
            other=>{print!("error en exp:{:?} ",other); break;}
        }
    }
    node
}

fn parse_term(tokens:&mut Vec<Token>)->Box<AST>{
    //print!("term:{:?}\n",tokens);
    let mut node = parse_factor(tokens);
    while tokens.len()>0{
        match &tokens[tokens.len()-1] {
            Token::Operation(o)=>{
                //print!("op entered: {}\n", *o);
                if *o == String::from("*") || *o == String::from("/"){
                    let t = tokens.pop();
                    let right = parse_factor(tokens);
                    node.left = Some(node.clone());
                    node.right=Some(right);
                    node.value=t.expect("Error en la formula");
                    return node;
                }else{ // + 
                    break;
                }
            },
            Token::CloseExp(_) => break,
            other=>{print!("error en term: {:?} ", other); break;}
        }
    }
    node
}

fn parse_factor(tokens:&mut Vec<Token>)->Box<AST>{
    let mut node = Box::new(AST{value:Token::Number(0), left:None, right:None});
    //print!("factor:{:?}\n",tokens);
    
    if tokens.len()>0{
        match &tokens[tokens.len()-1] {
            Token::Operation(o)=>{
                //print!("- entered ");
                if *o==String::from("-"){
                    let t = tokens.pop();
                    let right = parse_factor(tokens);
                    node=Box::new(AST{
                        left:None,
                        right:Some(right),
                        value:t.expect("Error en la formula")
                    });
                    return node;
                }
            },
            Token::OpenExp(_)=>{
                //print!("( entered ");
                tokens.pop(); //popping (
                node = parse_expression(tokens);
                //print!("1: {:?} ", tokens);
                tokens.pop(); //popping )
                //print!("2: {:?} ", tokens);
                return node;
            },
            Token::Number(_)=>{
                //print!("number entered ");
                let n = tokens.pop();
                node = Box::new(AST{
                    value:n.expect("Error en la formula"),
                    left:None,
                    right:None
                });
                //print!("number:{:?}\n",tokens);
                return node;
            },
            Token::Float(_)=>{
                //print!("float entered ");
                let n = tokens.pop();
                node = Box::new(AST{
                    value:n.expect("Error en la formula"),
                    left:None,
                    right:None
                });
                return node;
            },
            _=>print!("error en factor")
        }
    }
    node
}

fn print_ast(ast:&Box<AST>)->String{
    let mut str = String::new();
    match &(ast.value) {
        Token::Number(n)=> {
            let s = n.to_string();
            str = str + &s;
        },
        Token::Float(n)=> {
            let s = n.to_string();
            str = str + &s;
        },
        Token::Operation(o)=>{
            if *o==String::from("+")||*o==String::from("*")||*o==String::from("/"){
                let str_l =print_ast(ast.left.as_ref().expect("error"));
                let str_r =  print_ast(ast.right.as_ref().expect("error"));
                str = format!("({}{}{})", str_l,&o, str_r);
            }else{
                let str_r=print_ast(ast.right.as_ref().expect("error"));
                str = format!("-({})", str_r);
            }
        },
        _=>()
    }
    str
}

fn eval(ast:&Box<AST>)->f64{
    match &ast.value {
        Token::Operation(o)=>{
            if *o == "+" {return eval(ast.left.as_ref().expect("err in eval")) 
                                + eval(ast.right.as_ref().expect("err in eval"));}
            if *o == "-" {return - eval(ast.right.as_ref().expect("err in eval"));}
            if *o == "*" {return eval(ast.left.as_ref().expect("err in eval")) 
                                * eval(ast.right.as_ref().expect("err in eval"));}
            return eval(ast.left.as_ref().expect("err in eval")) 
                                / eval(ast.right.as_ref().expect("err in eval"));
        },
        Token::Number(n)=> *n as f64,
        Token::Float(n) => *n,
        _=>0.0
    }
}

fn main(){
    
    let mut input:String= String::new();
    //print!(">>> ");
    let _ = io::stdin().read_line(&mut input);
    let tokens_str = string_lexer(&input);
    let tokens = lexer(&tokens_str);
    let tokens = add_plus_before_minus(tokens);
    let mut tokens = reverse_tokens(tokens);
    ////print!("Tokens getted {:?}\n", tokens);
    let ast = parse_expression(&mut tokens);
    ////print!("Ast getted\n");
    println!("{}={}",print_ast(&ast), eval(&ast));

}
