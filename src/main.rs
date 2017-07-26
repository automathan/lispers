use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

mod types;
mod parser;
mod interpreter;

fn main() {
    println!("Lispe.rs v0.01");
    let mut stdout = io::stdout();
    
    let mut global_env : HashMap<String, types::LispItem> = HashMap::new();

    global_env.insert("x".to_string(), types::LispItem::Atom(types::LispType::Integer(42)));
    loop { // REPL
        write!(&mut stdout, "lispers>").ok();
        
        stdout.flush()
        .ok()
        .expect("Failed");
        
        let mut input = String::new();

        io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed");

        input.pop(); // remove '\n'

        let list = parser::parse_string(input);
        //print_list(vec);
        let first = list[0].clone();
        match first{
            types::LispItem::List(_) => {
                print_item(interpreter::eval(first.clone(), &global_env));
            },
            _ => println!("Not a list!")
        }
        println!("");
    }
}

#[allow(dead_code)]
fn print_list(list : Vec<types::LispItem>){
    for item in list{
        match item{
            types::LispItem::Atom(types::LispType::Integer(ref val)) => print!(" {}:i32 ", val),
            types::LispItem::Atom(types::LispType::Float(ref val)) => print!(" {}:f32 ", val),
            types::LispItem::Atom(types::LispType::Symbol(ref val)) => print!(" \"{}\":sym ", val),
            types::LispItem::List(inner) => {
                print!(" ( ");
                print_list(inner);
                print!(" ) ");
            }
        }
    }
}

#[allow(dead_code)]
fn print_item(item : types::LispItem){
    match item{
        types::LispItem::Atom(types::LispType::Integer(ref val)) => print!(" {}:i32 ", val),
        types::LispItem::Atom(types::LispType::Float(ref val)) => print!(" {}:f32 ", val),
        types::LispItem::Atom(types::LispType::Symbol(ref val)) => print!(" \"{}\":sym ", val),
        types::LispItem::List(inner) => {
            print!(" ( ");
            print_list(inner);
            print!(" ) ");
        }
    }
}