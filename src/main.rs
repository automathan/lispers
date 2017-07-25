use std::io;
use std::io::prelude::*;
mod parser;

/*
    Obviously I do not intend for everything to happen here
    I will split shit into meaningful pieces and this file
    will at last only function as a REPL
*/ 



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


        let vec = parser::parse_string(input);
        print_list(vec);
        /*
        match vec[0]{
            parser::LispItem::Atom(parser::LispType::Integer(ref val)) => println!("raw:\"{}\"", val),
            parser::LispItem::Atom(parser::LispType::Float(ref val)) => println!("raw:\"{}\"", val),
            parser::LispItem::Atom(parser::LispType::Symbol(ref val)) => println!("raw:\"{}\"", val),
            _ =>  println!("non-atom")
        }
        */
        /*
        println!("raw:\"{}\"", input);

        let result = eval(input);
        match result{
            Type::Integer(v) => println!("type: int"),
            Type::Float(v) => println!("type: float"),
            Type::Symbol(v) => println!("type: symbol")
        }
        */
    }
}

fn print_list(list : Vec<parser::LispItem>){
    for item in list{
        match item{
            parser::LispItem::Atom(parser::LispType::Integer(ref val)) => print!("int:\"{}\"", val),
            parser::LispItem::Atom(parser::LispType::Float(ref val)) => print!("float:\"{}\"", val),
            parser::LispItem::Atom(parser::LispType::Symbol(ref val)) => print!("symbol:\"{}\"", val),
            parser::LispItem::List(inner) => {
                print!(" ( [{}]", inner.len());
                print_list(inner);
                print!(" ) ");
            }
        }
    }
}