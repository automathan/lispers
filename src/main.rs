use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

mod types;
mod parser;
mod interpreter;

fn main() {
    println!("Lispe.rs v0.02");
    let mut stdout = io::stdout();
    
    let mut global_env : HashMap<String, types::LispItem> = HashMap::new();

    global_env.insert("pi".to_string(), types::LispItem::Atom(types::LispType::Float(3.1416)));
    global_env.insert("e".to_string(), types::LispItem::Atom(types::LispType::Float(2.7183)));
    global_env.insert("t".to_string(), types::LispItem::Atom(types::LispType::Bool(true)));
    global_env.insert("nil".to_string(), types::LispItem::Atom(types::LispType::Bool(false)));
    

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
        let first = list[0].clone(); // because the input is actually a list with 1 element, the input list (or something else)
        match first{
            types::LispItem::List(_) => print_item(interpreter::eval(first.clone(), &mut global_env)),
            types::LispItem::Atom(types::LispType::Symbol(ref val)) => {
                match global_env.get(val){
                    Some(var_val) => {
                        print!("{} = ", val);
                        print_item(var_val.clone());
                    },
                    None => println!("symbol \"{}\" is not in the symbol table", val)
                }
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
            types::LispItem::Atom(types::LispType::Bool(ref val)) => print!(" {}:bool ", if *val {"t"} else {"nil"}),
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
        types::LispItem::Atom(types::LispType::Bool(ref val)) => print!(" {}:bool ", if *val {"t"} else {"nil"}),
        types::LispItem::Atom(types::LispType::Float(ref val)) => print!(" {}:f32 ", val),
        types::LispItem::Atom(types::LispType::Symbol(ref val)) => print!(" \"{}\":sym ", val),
        types::LispItem::List(inner) => {
            print!(" ( ");
            print_list(inner);
            print!(" ) ");
        }
    }
}