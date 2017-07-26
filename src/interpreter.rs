use std::collections::HashMap;
use types::*;




pub fn eval(item : LispItem, env : &mut HashMap<String, LispItem>) -> LispItem{
    match item{
        LispItem::Atom(LispType::Integer(_)) => return item.clone(),
        LispItem::Atom(LispType::Float(_)) => return item.clone(),
        LispItem::Atom(LispType::Symbol(ref val)) => {
            match env.get(val){
                Some(var_val) => return var_val.clone(),
                None => println!("symbol \"{}\" is not in the symbol table", val)
            }
        },
        LispItem::List(inner) => {
            let head = inner[0].clone(); // expected to be a symbol/function
            
            match head{
                LispItem::Atom(LispType::Symbol(ref val)) => {
                    match val.as_ref(){
                        "+" => { // this is not pretty, but does the job for now
                            let mut sum = 0;
                            for i in 1..inner.len(){
                                let term = eval(inner[i].clone(), env);
                                match term{
                                    LispItem::Atom(LispType::Integer(ref val)) => sum += *val,
                                    _ => println!("invalid value for addition")
                                }
                            }
                            return LispItem::Atom(LispType::Integer(sum));
                        },
                        "*" => { // this is not pretty, but does the job for now
                            let mut sum = 1;
                            for i in 1..inner.len(){
                                let term = eval(inner[i].clone(), env);
                                match term{
                                    LispItem::Atom(LispType::Integer(ref val)) => sum *= *val,
                                    _ => println!("invalid value for multiplication")
                                }
                            }
                            return LispItem::Atom(LispType::Integer(sum));
                        },
                        "define" => {
                            if inner.len() != 3{
                                println!("wrong number of arguments for function: {}", val);
                            }else{
                                let key = inner[1].clone(); // keep it symbolic, don't eval
                                let entry = eval(inner[2].clone(), env);
                                match key {
                                    LispItem::Atom(LispType::Symbol(ref val)) =>{ 
                                        env.insert(val.clone(), entry);
                                        return LispItem::Atom(LispType::Symbol(val.clone()));
                                    },
                                    _ => println!("invalid symbol")
                                }
                            }
                        },
                        "car" => {
                            if inner.len() != 2{
                                println!("wrong number of arguments for function: {}", val);
                            }else{
                                let param = inner[1].clone(); // keep it symbolic, don't eval
                                match param {
                                    LispItem::List(inner) =>{ 
                                        return inner[0].clone();
                                    },
                                    _ => println!("non-list passed to car")
                                }
                            }
                        },
                        "cdr" => {
                            if inner.len() != 2{
                                println!("wrong number of arguments for function: {}", val);
                            }else{
                                let param = inner[1].clone(); // keep it symbolic, don't eval
                                match param {
                                    LispItem::List(inner) =>{ 
                                        let mut cdr : Vec<LispItem> = Vec::new();
                                        for i in 1..inner.len(){
                                            cdr.push(inner[i].clone());
                                        }
                                        return LispItem::List(cdr);
                                    },
                                    _ => println!("non-list passed to car")
                                }
                            }
                        },
                        _ => println!("undefined function: {}", val)
                    }
                    //println!("A {}", val);
                },
                _ => println!("non-symbol when symbol is expected")    
            }
        }
    }
    LispItem::Atom(LispType::Integer(-1))
}