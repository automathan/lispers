use std::collections::HashMap;
use types::*;




pub fn eval(item : LispItem, env : &HashMap<String, LispItem>) -> LispItem{
    match item{
        LispItem::Atom(LispType::Integer(ref val)) => return item.clone(),
        LispItem::Atom(LispType::Float(ref val)) => return item.clone(),
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
                        "+" => {
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