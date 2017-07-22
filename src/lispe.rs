use std::env;
use std::collections::HashMap;
mod parser;

// Random experiments while learning rustlang

struct Node {
    next : Option<Box<Node>>,
    symbol : String
}

fn main(){
    parser::linktest("123".to_string());
    
    let mut head: Node = Node {
        next: None,
        symbol: "hello1".to_string()
    };

    let mut test: Node = Node {
        next: None,
        symbol: "hello2".to_string()
    };

    head.next = Some(Box::new(test));

    println!("head.symbol: {}\nhead.next.symbol: {}",head.symbol, head.next.unwrap().symbol);
}

fn old(){
    let mut global_env = HashMap::new();
    let args: Vec<_> = env::args().collect();

    global_env.insert("+", "addition function");
    global_env.insert("-", "subtraction function");

    if args.len() == 2 {
        // shitty interpreter here
        let input = args[1].clone();
        println!("input: \"{}\"", input);
        let tokens: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

        let mut head : String = tokens[0].clone();
        let mut tail : String = tokens.last().unwrap().clone();

        if head.starts_with("("){
            println!("valid list start");
            head = head.chars().skip(1).take(head.len() - 1).collect();
        }

        if tail.ends_with(")"){
            println!("valid list end");
            tail = tail.chars().take(tail.len() - 1).collect();
        }

        /*
        for token in tokens {
            println!("token: {}", token);
        }
        */

        println!("head: {}", head);
        println!("tail: {}", tail);

        let arg1 = tokens[1].parse::<i32>();
        let arg2 = tail.parse::<i32>();

        if arg1.is_ok() && arg2.is_ok() {
            let mut res = 0;

            match head.as_ref() {
                "+" => res = plus(arg1.unwrap(), arg2.unwrap()),
                "-" => res = minus(arg1.unwrap(), arg2.unwrap()),
                _ => println!("undefined function: '{}'", head)
            }

            println!("result: {}", res);
        }else{
            println!("invalid args");
        }
        /*
        let try_double = head.parse::<f64>();
        match try_double{
            Ok(val) =>{
                println!("valid double: {}", val);
            },
            Err(why) => println!("invalid double: {}", why)
        }
        */
    }
}

fn plus(x: i32, y: i32) -> i32 {
    x + y
}

fn minus(x: i32, y: i32) -> i32 {
    x - y
}
