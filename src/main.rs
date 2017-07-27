use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

mod types;
mod parser;
mod interpreter;

fn main() {
    println!("Lispe.rs v0.03");
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
            types::LispItem::List(_, _) => print_item(interpreter::eval(first.clone(), &mut global_env)),
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
            types::LispItem::List(inner, _) => {
                print!(" ( ");
                print_list(inner);
                print!(" ) ");
            },
            types::LispItem::Atom(types::LispType::Function(_, _)) => print!("?")
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
        types::LispItem::List(inner, dm) => {
            print!(" ( ");
            if dm {
                print!(" dm ");
            }
            print_list(inner);
            print!(" ) ");
        },
        types::LispItem::Atom(types::LispType::Function(_, _)) => print!("?")
    }
}

#[cfg(test)]
mod basic_functions{
    use super::*;

    #[test]
    fn eval_add(){
        let mut global_env : HashMap<String, types::LispItem> = HashMap::new();
        
        // adding values to themselves and check if the result is twice the value of i

        for i in -10000..10000{
            let list = parser::parse_string(format!("(+ {} {})", i, i));
            let first = list[0].clone();
            
            match first{
                types::LispItem::List(_, _) => {
                    let res = interpreter::eval(first.clone(), &mut global_env);
                    match res{
                        types::LispItem::Atom(types::LispType::Integer(ref val)) => assert_eq!(*val, i * 2),
                        _ => println!("wrong type")
                    }
                },
                _ => println!("testing: not a list!")
            }
        }
    }

    #[test]
    fn eval_mul(){
        let mut global_env : HashMap<String, types::LispItem> = HashMap::new();
        
        // calculating square numbers

        for i in -1000..1000{
            let list = parser::parse_string(format!("(* {} {})", i, i));
            let first = list[0].clone();
            
            match first{
                types::LispItem::List(_, _) => {
                    let res = interpreter::eval(first.clone(), &mut global_env);
                    match res{
                        types::LispItem::Atom(types::LispType::Integer(ref val)) => assert_eq!(*val, i * i),
                        _ => println!("wrong type")
                    }
                },
                _ => println!("testing: not a list!")
            }
        }
    }

    #[test]
    fn eval_greater(){
        let mut global_env : HashMap<String, types::LispItem> = HashMap::new();
        
        // comparing numbers

        for i in -10000..10000{
            let list = parser::parse_string(format!("(> {} {})", i, -i));
            let first = list[0].clone();
            
            match first{
                types::LispItem::List(_, _) => {
                    let res = interpreter::eval(first.clone(), &mut global_env);
                    match res{
                        types::LispItem::Atom(types::LispType::Bool(ref val)) => assert_eq!(*val, (i > -i)),
                        _ => println!("wrong type")
                    }
                },
                _ => println!("testing: not a list!")
            }
        }
    }

    #[test]
    fn eval_lesser(){
        let mut global_env : HashMap<String, types::LispItem> = HashMap::new();
        
        // comparing numbers

        for i in -10000..10000{
            let list = parser::parse_string(format!("(< {} {})", i, -i));
            let first = list[0].clone();
            
            match first{
                types::LispItem::List(_, _) => {
                    let res = interpreter::eval(first.clone(), &mut global_env);
                    match res{
                        types::LispItem::Atom(types::LispType::Bool(ref val)) => assert_eq!(*val, (i < -i)),
                        _ => println!("wrong type")
                    }
                },
                _ => println!("testing: not a list!")
            }
        }
    }

    #[test]
    fn eval_define(){
        let mut global_env : HashMap<String, types::LispItem> = HashMap::new();
        
        // set a variable named "a" to a given integer value, then check if it actually is mapped to that value

        for i in -10000..10000{
            let list = parser::parse_string(format!("(define a {})", i));
            let first = list[0].clone();
            
            match first{
                types::LispItem::List(_, _) => {
                    let res = interpreter::eval(first.clone(), &mut global_env);
                    match res{
                        types::LispItem::Atom(types::LispType::Symbol(ref val)) => {
                            assert_eq!(*val, "a");
                            match global_env.get(val){
                                Some(var_val) => {
                                    match var_val{
                                        &types::LispItem::Atom(types::LispType::Integer(ref val)) => {
                                            assert_eq!(i, *val);
                                        },
                                        _ => println!("wrong type!")
                                    }
                                },
                                None => println!("symbol \"{}\" is not in the symbol table", val)
                            }
                        },
                        _ => println!("wrong type!")
                    }
                },
                _ => println!("testing: not a list!")
            }
        }
    }

    #[test]
    fn eval_globalvars(){
        let mut global_env : HashMap<String, types::LispItem> = HashMap::new();
        
        // set a variable named "a" to a given integer value, then check if it actually is mapped to that value

        for i in -10000..10000{
            let list = parser::parse_string(format!("(define a {})", i));
            let first = list[0].clone();
            
            match first{
                types::LispItem::List(_, _) => {
                    let res = interpreter::eval(first.clone(), &mut global_env);
                    match res{
                        types::LispItem::Atom(types::LispType::Symbol(ref val)) => {
                            assert_eq!(*val, "a");
                            match global_env.get(val){
                                Some(var_val) => {
                                    match var_val{
                                        &types::LispItem::Atom(types::LispType::Integer(ref val)) => {
                                            assert_eq!(i, *val);
                                        },
                                        _ => println!("wrong type!")
                                    }
                                },
                                None => println!("symbol \"{}\" is not in the symbol table", val)
                            }
                        },
                        _ => println!("wrong type!")
                    }
                },
                _ => println!("testing: not a list!")
            }

            let list = parser::parse_string("(+ a 1)".to_string());
            let first = list[0].clone();
            
            match first{
                types::LispItem::List(_, _) => {
                    let res = interpreter::eval(first.clone(), &mut global_env);
                    match res {
                        types::LispItem::Atom(types::LispType::Integer(ref val)) => assert_eq!(*val, i + 1),
                        _ => println!("wrong type")
                    }
                },
                _ => println!("testing: not a list!")
            }
        }
    }

    #[test]
    fn eval_lambda(){
        let mut global_env : HashMap<String, types::LispItem> = HashMap::new();
        
        // defining a square function and calling it with different integers

        let list = parser::parse_string(format!("(define sqr (lambda (n) (* n n)))"));
        let first = list[0].clone();
            
        match first{
            types::LispItem::List(_, _) => {
                interpreter::eval(first.clone(), &mut global_env);
            },
            _ => println!("testing: not a list!")
        }
        
        for i in -100..100{
            let list = parser::parse_string(format!("(sqr {})", i));
            let first = list[0].clone();
            
            match first{
                types::LispItem::List(_, _) => {
                    let res = interpreter::eval(first.clone(), &mut global_env);
                    match res{
                        types::LispItem::Atom(types::LispType::Integer(ref val)) => assert_eq!(*val, i * i),
                        _ => println!("wrong type")
                    }
                },
                _ => println!("testing: not a list!")
            }
        }
    }
}