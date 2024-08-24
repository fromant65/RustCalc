use std::io;

// enum Token {
//     Number(i32),
//     Float(f64),
//     OpenExp(String), // (
//     CloseExp(String), // )
//     Operation(String) // + - * /
// }

fn lexer(input:&str)->Vec<String>{
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
fn main(){
    let mut input:String= String::new();
    let _ = io::stdin().read_line(&mut input);
    let tokens = lexer(&input);
    for t in tokens{
        print!("{} ",t);
    }
}

