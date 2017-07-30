use std::io;
use std::io::prelude::*;

mod types;
mod parser;
mod interpreter;

fn main() {
    println!("Lispe.rs v0.18");
    let mut stdout = io::stdout();
    
    //let mut global_env : HashMap<String, types::LispItem> = HashMap::new();
    let mut global_env = types::Environment::new(None);

    global_env.insert(&"pi".to_string(), &types::LispItem::Atom(types::LispType::Float(3.1416)));
    global_env.insert(&"e".to_string(), &types::LispItem::Atom(types::LispType::Float(2.7183)));
    global_env.insert(&"t".to_string(), &types::LispItem::Atom(types::LispType::Bool(true)));
    global_env.insert(&"nil".to_string(), &types::LispItem::Atom(types::LispType::Bool(false)));
    

    loop { // REPL
        write!(&mut stdout, "lispers>").ok();
        
        stdout.flush()
        .ok()
        .expect("Failed");
        
        let mut input = String::new();

        io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed");

        //input.pop(); // remove '\n'

        let list = parser::parse_string(input);
        let first = list[0].clone(); // because the input is actually a list with 1 element, the input list (or something else)
        
        match first{
            types::LispItem::List(_, _) => print_item(interpreter::eval(first.clone(), &mut global_env)),
            types::LispItem::Atom(types::LispType::Symbol(ref val)) => {
                match global_env.get(val){
                    Some(ref var_val) => {
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
            /*
            types::LispItem::Atom(types::LispType::Integer(ref val)) => print!(" {}:i32 ", val),
            types::LispItem::Atom(types::LispType::Bool(ref val)) => print!(" {}:bool ", if *val {"t"} else {"nil"}),
            types::LispItem::Atom(types::LispType::Float(ref val)) => print!(" {}:f32 ", val),
            types::LispItem::Atom(types::LispType::Symbol(ref val)) => print!(" \"{}\":sym ", val),
            */
            types::LispItem::List(inner, _) => {
                print!(" ( ");
                print_list(inner);
                print!(" ) ");
            },
            _ => println!("{}", item)
            /*
            types::LispItem::Atom(types::LispType::Function(_, _)) => print!("?"),
            types::LispItem::Error(ref msg, ref src) => println!("Error: {}, source: {}", msg, src)
            */
        }
    }
}

#[allow(dead_code)]
fn print_item(item : types::LispItem){
    match item{
        /*
        types::LispItem::Atom(types::LispType::Integer(ref val)) => print!(" {}:i32 ", val),
        types::LispItem::Atom(types::LispType::Bool(ref val)) => print!(" {}:bool ", if *val {"t"} else {"nil"}),
        types::LispItem::Atom(types::LispType::Float(ref val)) => print!(" {}:f32 ", val),
        types::LispItem::Atom(types::LispType::Symbol(ref val)) => print!(" \"{}\":sym ", val),
        */
        types::LispItem::List(inner, dm) => {
            print!(" ( ");
            if dm {
                print!(" dm ");
            }
            print_list(inner);
            print!(" ) ");
        },
        _ => println!("{}", item)
        /*
        types::LispItem::Atom(types::LispType::Function(_, _)) => print!("?"),
        types::LispItem::Error(ref msg, ref src) => println!("Error: {}, source: {}", msg, src)
        */
    }
}

#[cfg(test)]
mod basic_functions{
    use super::*;

    #[test]
    fn eval_add(){
        // adding values to themselves and check if the result is twice the value of i
        
        let mut global_env = types::Environment::new(None);
        for i in -10000..10000{
            let list = parser::parse_string(format!("(+ {} {})", i, i));
            let res = interpreter::eval(list[0].clone(), &mut global_env);
            match res{
                types::LispItem::Atom(types::LispType::Integer(ref val)) => assert_eq!(*val, i * 2),
                _ => println!("wrong type")
            }
        }
    }

    #[test]
    fn eval_mul(){
        // calculating square numbers

        let mut global_env = types::Environment::new(None);
        for i in -1000..1000{
            let list = parser::parse_string(format!("(* {} {})", i, i));
            let res = interpreter::eval(list[0].clone(), &mut global_env);
            match res{
                types::LispItem::Atom(types::LispType::Integer(ref val)) => assert_eq!(*val, i * i),
                _ => println!("wrong type")
            }
        }
    }

    #[test]
    fn eval_greater(){
        // comparing numbers

        let mut global_env = types::Environment::new(None);
        for i in -10000..10000{
            let list = parser::parse_string(format!("(> {} {})", i, -i));
            let res = interpreter::eval(list[0].clone(), &mut global_env);
            match res{
                types::LispItem::Atom(types::LispType::Bool(ref val)) => assert_eq!(*val, (i > -i)),
                _ => println!("wrong type")
            }
        }
    }

    #[test]
    fn eval_lesser(){
        // comparing numbers

        let mut global_env = types::Environment::new(None);
        for i in -10000..10000{
            let list = parser::parse_string(format!("(< {} {})", i, -i));
            let res = interpreter::eval(list[0].clone(), &mut global_env);
            match res{
                types::LispItem::Atom(types::LispType::Bool(ref val)) => assert_eq!(*val, (i < -i)),
                _ => println!("wrong type")
            }
        }
    }

    #[test]
    fn eval_equal(){
        // comparing numbers

        let mut global_env = types::Environment::new(None);
        for i in -10000..10000{
            let list = parser::parse_string(format!("(= {} {})", i, -i));
            let res = interpreter::eval(list[0].clone(), &mut global_env);
            match res{
                types::LispItem::Atom(types::LispType::Bool(ref val)) => assert_eq!(*val, (i == -i)),
                _ => println!("wrong type")
            }
        }

        for i in -10000..10000{
            let list = parser::parse_string(format!("(= {} {})", i, i));
            let res = interpreter::eval(list[0].clone(), &mut global_env);
            match res{
                types::LispItem::Atom(types::LispType::Bool(ref val)) => assert_eq!(*val, (i == i)),
                _ => println!("wrong type")
            }
        }
    }

    #[test]
    fn eval_define(){
        // set a variable named "a" to a given integer value, then check if it actually is mapped to that value
        
        let mut global_env = types::Environment::new(None);
        for i in -10000..10000{
            let list = parser::parse_string(format!("(define a {})", i));
            let res = interpreter::eval(list[0].clone(), &mut global_env);
            match res{
                types::LispItem::Atom(types::LispType::Symbol(ref val)) => {
                    assert_eq!(*val, "a");
                    match global_env.get(val){
                        Some(var_val) => {
                            match var_val{
                                types::LispItem::Atom(types::LispType::Integer(ref val)) => {
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
        }
    }

    #[test]
    fn eval_globalvars(){
        // set a variable named "a" to a given integer value, then perform arithmetics with that value

        let mut global_env = types::Environment::new(None);
        for i in -10000..10000{
            let list = parser::parse_string(format!("(define a {})", i));
            let res = interpreter::eval(list[0].clone(), &mut global_env);
            match res{
                types::LispItem::Atom(types::LispType::Symbol(ref val)) => {
                    assert_eq!(*val, "a");
                    match global_env.get(val){
                        Some(var_val) => {
                            match var_val{
                                types::LispItem::Atom(types::LispType::Integer(ref val)) => {
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

            let list = parser::parse_string("(+ a 1)".to_string());
            let res = interpreter::eval(list[0].clone(), &mut global_env);
            match res {
                types::LispItem::Atom(types::LispType::Integer(ref val)) => assert_eq!(*val, i + 1),
                _ => println!("wrong type")
            }
        }
    }

    #[test]
    fn eval_lambda(){
        // defining a square function and calling it with different integers
        
        let mut global_env = types::Environment::new(None);
        let list = parser::parse_string(format!("(define sqr (lambda (n) (* n n)))"));
        interpreter::eval(list[0].clone(), &mut global_env);    
        
        for i in -100..100{
            let list = parser::parse_string(format!("(sqr {})", i));
            let res = interpreter::eval(list[0].clone(), &mut global_env);
            match res{
                types::LispItem::Atom(types::LispType::Integer(ref val)) => assert_eq!(*val, i * i),
                _ => println!("wrong type")
            }
        }
    }

    #[test]
    fn eval_factorial(){
        // factorial, also known as recursion 101
        
        let mut global_env = types::Environment::new(None);
        let list = parser::parse_string(format!("(define fact (lambda (n) (* n (cond (< n 2) 1 (fact (- n 1)))))))"));
        interpreter::eval(list[0].clone(), &mut global_env);
        
        for i in 1..11{
            let list = parser::parse_string(format!("(fact {})", i));
            let res = interpreter::eval(list[0].clone(), &mut global_env);
            match res{
                types::LispItem::Atom(types::LispType::Integer(ref val)) =>{
                    let mut iter_fact = 1;
                    for j in 1..(i + 1){
                        iter_fact *= j;
                    }
                    assert_eq!(*val, iter_fact);
                },
                _ => println!("wrong type")
            }
        }
    }
    
    #[test]
    fn eval_list_iteration(){
        // factorial, also known as recursion 101
        
        let mut global_env = types::Environment::new(None);
        let list = parser::parse_string(format!("(define lsum (lambda (li) (+ (car li) (cond (cdr li) (lsum (cdr li)) 0))))"));
        interpreter::eval(list[0].clone(), &mut global_env);
        
        for i in 1..50{ // without tail recursion the call stack will explode on larger values of i
            let mut expr : String = "(lsum '(".to_string();
            for _ in 0..i{
                expr.push_str(&format!("{} ", i));
            }
            expr.push_str("))");
            let list = parser::parse_string(expr);
            let res = interpreter::eval(list[0].clone(), &mut global_env);
            match res{
                types::LispItem::Atom(types::LispType::Integer(ref val)) =>{
                    assert_eq!(*val, i * i);
                },
                _ => println!("wrong type")
            }
        }
    }
}