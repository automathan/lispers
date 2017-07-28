use types::*;

pub fn eval(item : LispItem, env : &mut Environment) -> LispItem{
    match item{
        LispItem::Atom(LispType::Integer(_)) => return item.clone(),
        LispItem::Atom(LispType::Float(_)) => return item.clone(),
        LispItem::Atom(LispType::Bool(_)) => return item.clone(),
        LispItem::Atom(LispType::Symbol(ref val)) => {
            match env.get(val){
                Some(var_val) => return var_val.clone(),
                None => println!("symbol \"{}\" is not in the symbol table", val)
            }
        },
        LispItem::Atom(LispType::Function(_, _)) => {
            println!("kek");
            // this part becomes relevant when functions can be used as arguments 
            // let mut local_env : HashMap<String, LispItem> = HashMap::new();
        },
        LispItem::List(inner, dm) => {
            if dm{
                return LispItem::List(inner, dm);
            }else{
                let head = inner[0].clone(); // expected to be a symbol/function, immediate lambdas are not supported yet
                
                match head{
                    LispItem::Atom(LispType::Symbol(ref val)) => {
                        return apply(val, inner, env);
                    },
                    _ => println!("non-symbol when symbol is expected")    
                }
            }
        }
    }
    LispItem::Atom(LispType::Integer(-1)) // until I make a real system for error handling 
}

fn apply(val : &String, inner : Vec<LispItem>, env : &mut Environment) -> LispItem{ // The name of this function is technically wrong.
    let mut found = false;
    match env.get(val){
        Some(var_val) => {
            match var_val {
                LispItem::Atom(LispType::Function(ref bindings, ref body)) => {
                    found = true;
                    //let mut local_env : HashMap<String, LispItem> = HashMap::new();
                    let mut local_env = Environment::new(Some(&env));

                    if bindings.len() != inner.len() - 1 {
                        println!("invalid number of args for function {}", val);
                    }else{
                        for i in 0..bindings.len(){
                            let item = eval(inner[1 + i].clone(), &mut local_env);
                            local_env.insert(&bindings[i].clone(), &item);//&inner[1 + i].clone());
                        }
                        return eval(LispItem::List(body.clone(), false), &mut local_env);
                    }
                },
                _ => println!("symbol \"{}\" is not a function", val)
            }
        },
        None => {}
    }
    if !found {
        match val.as_ref(){
            "+" => { // this is not pretty, but does the job for now
                let mut sum = 0;
                for i in 1..inner.len(){
                    let term = eval(inner[i].clone(), env);
                    match term{
                        LispItem::Atom(LispType::Integer(ref val)) => {println!("val: {}", val);
                     sum += *val},
                        _ => println!("invalid value for addition")
                    }
                }
                return LispItem::Atom(LispType::Integer(sum));
            },
            "-" => {
                if inner.len() == 2{ // unary minus
                    let term = eval(inner[1].clone(), env);
                    match term{
                        LispItem::Atom(LispType::Integer(ref val)) => return LispItem::Atom(LispType::Integer(-(*val))),
                        _ => println!("invalid value for subtraction")
                    }
                }else{
                    let mut sum = 0;
                    for i in 1..inner.len(){
                        let term = eval(inner[i].clone(), env);
                        match term{
                            LispItem::Atom(LispType::Integer(ref val)) => sum -= if i == 1 {-(*val)} else {*val},
                            _ => println!("invalid value for subtraction")
                        }
                    }
                    return LispItem::Atom(LispType::Integer(sum));
                }
            },
            "*" => {
                let mut prod = 1;
                for i in 1..inner.len(){
                    let term = eval(inner[i].clone(), env);
                    match term{
                        LispItem::Atom(LispType::Integer(ref val)) => prod *= *val,
                        _ => println!("invalid value for multiplication")
                    }
                }
                return LispItem::Atom(LispType::Integer(prod));
            },
            "list" => {
                let mut list : Vec<LispItem> = Vec::new();
                for i in 1..inner.len(){
                    let term = eval(inner[i].clone(), env);
                    list.push(term);
                }
                return LispItem::List(list, true);
            },
            "define" => {
                if inner.len() != 3{
                    println!("wrong number of arguments for function: {}", val);
                }else{
                    let key = inner[1].clone(); // keep it symbolic, don't eval
                    let entry = eval(inner[2].clone(), env);
                    match key {
                        LispItem::Atom(LispType::Symbol(ref val)) =>{ 
                            env.insert(&val.clone(), &entry);
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
                    let param = eval(inner[1].clone(), env);
                    match param {
                        LispItem::List(inner, dm) =>{
                            if dm { 
                                return inner[0].clone();
                            }else{
                                println!("non-dm-list passed to car");
                            }
                        },
                        _ => println!("non-list passed to car")
                    }
                }
            },
            "cdr" => {
                if inner.len() != 2 {
                    println!("wrong number of arguments for function: {}", val);
                }else{
                    let param = eval(inner[1].clone(), env);
                    match param {
                        LispItem::List(inner, dm) =>{ 
                            if dm {
                                let mut cdr : Vec<LispItem> = Vec::new();
                                for i in 1..inner.len(){
                                    cdr.push(inner[i].clone());
                                }
                                return LispItem::List(cdr, true);
                            }else{
                                println!("non-dm-list passed to cdr");
                            }
                        },
                        _ => println!("non-list passed to cdr")
                    }
                }
            },
            "<" => {
                if inner.len() != 3 {
                    println!("wrong number of arguments for function: {}", val);
                }else{
                    let param1 = eval(inner[1].clone(), env);
                    let param2 = eval(inner[2].clone(), env); 
                    match (param1,param2){
                        (LispItem::Atom(LispType::Integer(ref val1)),LispItem::Atom(LispType::Integer(ref val2))) => return LispItem::Atom(LispType::Bool(val1 < val2)),
                        _ => println!("can't compare!")
                    }
                }
            },
            ">" => {
                if inner.len() != 3 {
                    println!("wrong number of arguments for function: {}", val);
                }else{
                    let param1 = eval(inner[1].clone(), env);
                    let param2 = eval(inner[2].clone(), env); 
                    match (param1,param2){
                        (LispItem::Atom(LispType::Integer(ref val1)),LispItem::Atom(LispType::Integer(ref val2))) => return LispItem::Atom(LispType::Bool(val1 > val2)),
                        _ => println!("can't compare!")
                    }
                }
            },
            "cond" => {
                if inner.len() != 4 { // (cond statement do else)
                    println!("wrong number of arguments for function: {}", val);
                }else{
                    let statement = eval(inner[1].clone(), env);
                    match statement{
                        LispItem::Atom(LispType::Bool(ref val)) => {
                            if *val {
                                return eval(inner[2].clone(), env);
                            }else{
                                return eval(inner[3].clone(), env);
                            }
                        },
                        _ => println!("not a boolean")
                    }
                }
            },
            "lambda" => {
                if inner.len() != 3 { // (lambda bindings body)
                    println!("wrong number of arguments for function: {}", val);    
                }else{
                    let bind = inner[1].clone();
                    let body = inner[2].clone(); 
                    match (bind, body){
                        (LispItem::List(a, _), LispItem::List(b, _)) => {
                            let mut tmp : Vec<String> = Vec::new();
                            for sym in a.clone() {
                                match sym{
                                    LispItem::Atom(LispType::Symbol(ref val)) => tmp.push(val.clone()),
                                    _ => println!("non-sym found in bindings list")          
                                }
                            }
                            if tmp.len() == a.len(){
                                return LispItem::Atom(LispType::Function(tmp, b));
                            }
                        },
                        (_, _) => println!("bad lambda args")
                    }
                }
            },
            _ => println!("undefined function: {}", val)
        }
    }
    LispItem::Atom(LispType::Integer(-1))
}