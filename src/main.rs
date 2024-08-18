use core::f64;
use std::io;

enum Token {
    Number(i32),
    Float(f64),
    OpenExp(String), // (
    CloseExp(String), // )
    Operation(String) // + - * /
}

// enum ComplexExpression{
//     Token(Token),
//     Expression(Expression)
// }

// //Una expresión es un vector de tokens y expresiones
// enum Expression{
//     Simple(Vec<Token>),
//     Complex(Vec<ComplexExpression>)
// }
// struct BinaryOperator{
//     operator:Option<Token>,
//     left:Option<Token>,
//     right:Option<Token>,
// }

// impl BinaryOperator {
//     fn add_parenthesis_between_mult_and_div(mut tokens_:Vec<Token>)->Vec<Token>{
//         //let mut tokens:&Vec<Token>=&tokens_;
//         let mut len = tokens_.len();
//         let mut i = 0;
//         while(i<len-1){
//             match &tokens_[i] {
//                 Token::Operation(o)=>{
//                     if o=="*" || o=="/" {
//                         tokens_.insert(i-2, Token::OpenExp(String::from("(")));
//                         tokens_.insert(i+2, Token::CloseExp(String::from("(")));
//                         len+=2
//                     }
//                 }
//                 _=>()
//             }
//             i+=1;
//         }
//         tokens_
//     }
// }

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
            let mut iter = tok.chars();
            if iter.all(|c| matches!(c, '+' | '-' | '*' | '/')) {
                tokens.push(Token::Operation(tok));
            }else if iter.all(|c| matches!(c, '(')){
                tokens.push(Token::OpenExp(tok));
            }else if iter.all(|c| matches!(c, ')')){
                tokens.push(Token::CloseExp(tok));
            }
        }
    }
    tokens
}
fn main(){
    
    let mut input:String= String::new();
    print!(">>> ");
    let _ = io::stdin().read_line(&mut input);
    let tokens_str = string_lexer(&input);
    let tokens = lexer(&tokens_str);
    for t in tokens{
        match t {
            Token::Float(i) => print!("{} ",i),
            Token::Number(i)=> print!("{} ",i),
            Token::Operation(i)=>print!("{} ",i),
            Token::OpenExp(i)=>print!("{i} "),
            Token::CloseExp(i)=>print!("{i} "),   
        }
    }
}

