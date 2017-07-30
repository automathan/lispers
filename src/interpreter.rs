use types::*;

pub fn eval(item : LispItem, env : &mut Environment) -> LispItem{
    match item{
        LispItem::Error(_, _) => return item.clone(),
        LispItem::Atom(LispType::Integer(_)) => return item.clone(),
        LispItem::Atom(LispType::Float(_)) => return item.clone(),
        LispItem::Atom(LispType::Bool(_)) => return item.clone(),
        LispItem::Atom(LispType::Symbol(ref val)) => {
            match env.get(val){
                Some(var_val) => return var_val.clone(),
                None => return LispItem::Error("Not in symtab".to_string(), Box::new(item.clone()))
            }
        },
        LispItem::Atom(LispType::Function(_, _)) => {
            // this part becomes relevant when functions can be used as arguments 
            // let mut local_env : HashMap<String, LispItem> = HashMap::new();
            return item;
        },
        LispItem::List(inner, dm) => {
            if inner.len() == 0 {
                return LispItem::Atom(LispType::Bool(false));   
            }
            if dm {
                return LispItem::List(inner, dm);
            }else{
                let head = inner[0].clone(); // expected to be a symbol/function, immediate lambdas are not supported yet
                
                match head{
                    LispItem::Atom(LispType::Symbol(ref val)) => {
                        return apply(val, inner, env);
                    },
                    _ => return LispItem::Error("non-symbol at head".to_string(), Box::new(head))
                }
            }
        }
    }
}

fn apply(val : &String, inner : Vec<LispItem>, env : &mut Environment) -> LispItem{ // The name of this function is technically wrong.
    match env.get(val){
        Some(var_val) => {
            match var_val {
                LispItem::Atom(LispType::Function(ref bindings, ref body)) => {
                    let mut local_env = Environment::new(Some(&env));

                    if bindings.len() != inner.len() - 1 {
                        return lisp_error("wrong number of args", &var_val);
                    }else{
                        for i in 0..bindings.len(){
                            let item = eval(inner[1 + i].clone(), &mut local_env);
                            local_env.insert(&bindings[i].clone(), &item);
                        }
                        return eval(LispItem::List(body.clone(), false), &mut local_env);
                    }
                },
                _ => return lisp_error("symtab entry is not function", &var_val)
            }
        },
        None => {}
    }
        match val.as_ref(){
            "+" => { // this is not pretty, but does the job for now
                let mut sum = 0;
                for i in 1..inner.len(){
                    let term = eval(inner[i].clone(), env);
                    match term{
                        LispItem::Atom(LispType::Integer(ref val)) => sum += *val,
                        _ => return lisp_error("invalid value for addition", &term)
                    }
                }
                return LispItem::Atom(LispType::Integer(sum));
            },
            "-" => {
                if inner.len() == 2{ // unary minus
                    let term = eval(inner[1].clone(), env);
                    match term{
                        LispItem::Atom(LispType::Integer(ref val)) => return LispItem::Atom(LispType::Integer(-(*val))),
                        _ => return lisp_error("invalid value for negation", &term)
                    }
                }else{
                    let mut sum = 0;
                    for i in 1..inner.len(){
                        let term = eval(inner[i].clone(), env);
                        match term{
                            LispItem::Atom(LispType::Integer(ref val)) => sum -= if i == 1 {-(*val)} else {*val},
                            _ => return lisp_error("invalid value for subtraction", &term)
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
                        _ => return lisp_error("invalid value for multiplication", &term)
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
                    return lisp_error_str("wrong number of arguments", val);
                }else{
                    let key = inner[1].clone(); // keep it symbolic, don't eval
                    let entry = eval(inner[2].clone(), env);
                    match key {
                        LispItem::Atom(LispType::Symbol(ref val)) =>{ 
                            env.insert(&val.clone(), &entry);
                            return LispItem::Atom(LispType::Symbol(val.clone()));
                        },
                        _ => return lisp_error("invalid symbol", &key)
                    }
                }
            },
            "car" => {
                if inner.len() != 2{
                    return lisp_error_str("wrong number of arguments", val);
                }else{
                    let param = eval(inner[1].clone(), env);
                    match param.clone() {
                        LispItem::List(inner, dm) =>{
                            if dm { 
                                return inner[0].clone();
                            }else{
                                return lisp_error("non-dm-list passed to car", &param);
                            }
                        },
                        _ => return lisp_error("non-list passed to car", &param)
                    }
                }
            },
            "cdr" => {
                if inner.len() != 2 {
                    return lisp_error_str("wrong number of arguments", val);
                }else{
                    let param = eval(inner[1].clone(), env);
                    match param.clone() {
                        LispItem::List(inner, dm) =>{ 
                            if dm {
                                if inner.len() == 0{
                                    return LispItem::Atom(LispType::Bool(false));
                                }
                                let mut cdr : Vec<LispItem> = Vec::new();
                                for i in 1..inner.len(){
                                    cdr.push(inner[i].clone());
                                }
                                return LispItem::List(cdr, true);
                            }else{
                                return lisp_error("non-dm-list passed to cdr", &param);
                            }
                        },
                        _ => return lisp_error("non-list passed to cdr", &param)
                    }
                }
            },
            "cons" => { // (cons e l) return element e prepended to list l (l.dm == t)
                if inner.len() != 3 {
                    return lisp_error_str("wrong number of arguments", val);
                }else{
                    let el = eval(inner[1].clone(), env);
                    let li = eval(inner[2].clone(), env);
                    match (el.clone(), li.clone()){
                        (LispItem::Atom(_), LispItem::List(inner, true)) => {
                            let mut out : Vec<LispItem> = Vec::new();
                            out.push(el.clone());
                            for i in 0..inner.len(){
                                out.push(inner[i].clone());
                            }
                            return LispItem::List(out, true);
                        },
                        (_, _) => return lisp_error_str("bad cons input", &format!("el: {}, li: {}", el, li))
                    }
                }
            },
            "eval" => { // eval a datamode list (override datamode, basically)
                 if inner.len() != 2 {
                    return lisp_error_str("wrong number of arguments", val);
                }else{
                    let param = eval(inner[1].clone(), env);
                    match param.clone() {
                        LispItem::List(inner, dm) =>{ 
                            if dm {
                                if inner.len() == 0{
                                    return LispItem::Atom(LispType::Bool(false));
                                }
                                return eval(LispItem::List(inner, false), env);
                            }else{
                                return lisp_error("non-dm-list passed to eval", &param);
                            }
                        },
                        _ => return lisp_error("non-list passed to eval", &param)
                    }
                }
            },
            "<" => {
                if inner.len() != 3 {
                    return lisp_error_str("wrong number of arguments", val);
                }else{
                    let param1 = eval(inner[1].clone(), env);
                    let param2 = eval(inner[2].clone(), env); 
                    match (param1.clone(),param2.clone()){
                        (LispItem::Atom(LispType::Integer(ref val1)),LispItem::Atom(LispType::Integer(ref val2))) => return LispItem::Atom(LispType::Bool(val1 < val2)),
                        _ => return lisp_error_str("can not compare!", &format!("{} < {}", param1, param2))
                    }
                }
            },
            ">" => {
                if inner.len() != 3 {
                    return lisp_error_str("wrong number of arguments", val);
                }else{
                    let param1 = eval(inner[1].clone(), env);
                    let param2 = eval(inner[2].clone(), env); 
                    match (param1.clone(),param2.clone()){
                        (LispItem::Atom(LispType::Integer(ref val1)),LispItem::Atom(LispType::Integer(ref val2))) => return LispItem::Atom(LispType::Bool(val1 > val2)),
                        _ => return lisp_error_str("can not compare!", &format!("{} > {}", param1, param2))
                    }
                }
            },
            "=" => {
                if inner.len() != 3 {
                    return lisp_error_str("wrong number of arguments", val);
                }else{
                    let param1 = eval(inner[1].clone(), env);
                    let param2 = eval(inner[2].clone(), env); 
                    match (param1.clone(),param2.clone()){
                        (LispItem::Atom(LispType::Integer(ref val1)),LispItem::Atom(LispType::Integer(ref val2))) => return LispItem::Atom(LispType::Bool(val1 == val2)),
                        _ => return lisp_error_str("can not compare!", &format!("{} > {}", param1, param2))
                    }
                }
            },
            "cond" => {
                if inner.len() != 4 { // (cond statement do else)
                    return lisp_error_str("wrong number of arguments", val);
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
                        LispItem::List(li, _) => {
                            if li.len() > 0 {
                                return eval(inner[2].clone(), env);
                            }else{
                                return eval(inner[3].clone(), env);
                            }
                        },
                        _ => return lisp_error("not a boolean", &statement)
                    }
                }
            },
            "lambda" => {
                if inner.len() != 3 { // (lambda bindings body)
                    return lisp_error_str("wrong number of arguments", val);
                }else{
                    let bind = inner[1].clone();
                    let body = inner[2].clone(); 
                    match (bind.clone(), body.clone()){
                        (LispItem::List(a, _), LispItem::List(b, _)) => {
                            let mut tmp : Vec<String> = Vec::new();
                            for sym in a.clone() {
                                match sym{
                                    LispItem::Atom(LispType::Symbol(ref val)) => tmp.push(val.clone()),
                                    _ => return lisp_error("non-sym found in bindings list", &sym)          
                                }
                            }
                            if tmp.len() == a.len(){
                                return LispItem::Atom(LispType::Function(tmp, b));
                            }
                        },
                        (_, _) => return lisp_error_str("bad lambda args", &format!("bind: {}, body {}", bind, body))
                    }
                }
            },
            _ => return lisp_error_str("undefined function", val)
        }
    lisp_error("error, reached end of apply with no return", &LispItem::Atom(LispType::Symbol(val.clone())))
}