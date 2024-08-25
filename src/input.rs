use std::fs::File;
use std::io::{self,BufRead,BufReader};

pub fn read_lines(filename:&str) -> io::Result<Vec<(String,f64)>>{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut expressions = Vec::new();
    for line in reader.lines(){
        let line = line?;
        if let Some((expr,res))=parse_line(&line){
            expressions.push((expr,res));
        }else{
            return Err(io::Error::new(io::ErrorKind::InvalidData, 
                "Incorrect line format"));
        }
    }
    Ok(expressions)
}

fn parse_line(line:&str)->Option<(String,f64)>{
    let parts: Vec<&str> = line.split('=').collect();
    if parts.len()==2{
        let expr = parts[0].trim().to_string();
        if let Ok(res) = parts[1].trim().parse::<f64>(){
            return Some((expr,res));
        }
    }
    None
}