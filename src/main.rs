use std::io;
mod lexer;
mod parser;
mod eval;
mod input;
mod expr_generator;
use lexer::{Token,lexer};
use parser::parse_expression;
use eval::{print_ast,eval};
use expr_generator::write_to_testfile;
use input::read_lines;
use std::path::Path;
fn main(){
    let test_res = write_to_testfile();
    match test_res {
        Ok(_)=>(),
        Err(e)=>print!("Couldn't write expressions: {:?}", e)
    }
    let path = Path::new("src/input.txt");
    println!("{:?}",path);
    let read_input = read_lines(path.to_str().expect("Invalid Path"));
    match &read_input {
        Err(e)=>println!("{e}"),
        _=>()
    }
    let mut inputs=Vec::new();
    let mut read_from_txt = true;
    match read_input {
        Ok(inp)=>inputs=inp,
        Err(err)=>{
            println!("Error while reading file {:?}. Insert expression manualy.\n>>>", err);
            let mut inp=String::new() ;
            let _ = io::stdin().read_line(&mut inp);
            inputs.push((inp,0.0));
            read_from_txt=false;
        }
    }
    let tolerance=0.01;
    let mut i = 0;
    for input in inputs{
        let (input,res)=input;
        let tokens_result = lexer(&input);
        match tokens_result {
            Ok(mut tokens)=>{
                let ast = parse_expression(&mut tokens);
                match ast {
                    Ok(ast)=>{
                        let root = ast.root.expect("Empty AST");
                        let str_expr=print_ast(&root).unwrap();
                        let evaluation = eval(&root);
                        //println!("{}. {:?}={}", i, str_expr, evaluation);
                        // if read_from_txt{
                        //     assert!((res-evaluation)<tolerance);
                        // }
                    },
                    Err(err)=>println!("Error while parsing input {}:{:?}",i,err)
                }
            },
            Err(err)=> println!("Error while processing input {}:{:?}",i,err)
        }
        i+=1;
    }
    
}
