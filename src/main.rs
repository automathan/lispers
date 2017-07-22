use std::io;
use std::io::prelude::*;

/*
    Obviously I do not intend for everything to happen here
    I will split shit into meaningful pieces and this file
    will at last only function as a REPL
*/ 

enum Type{
    Integer(i32),
    Float(f32),
    Symbol(String)
}

fn main() {
    println!("Hello, world!");
    let mut stdout = io::stdout();
    loop {
        write!(&mut stdout, "lispers>").ok();
        
        stdout.flush()
        .ok()
        .expect("Failed");
        
        let mut input = String::new();

        io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed");

        input.pop(); // remove '\n'

        println!("raw:\"{}\"", input);

        let result = eval(input);
        match result{
            Type::Integer(v) => println!("type: int"),
            Type::Float(v) => println!("type: float"),
            Type::Symbol(v) => println!("type: symbol")
        }
    }
}

fn eval(atom : String) -> Type{
    if atom.parse::<i32>().is_ok(){
        return Type::Integer(atom.parse::<i32>().unwrap())
    }
    if atom.parse::<f32>().is_ok(){
        return Type::Float(atom.parse::<f32>().unwrap())
    }
    return Type::Symbol(atom)    
}