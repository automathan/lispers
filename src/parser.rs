use types::*;

/*
    The parser has one responsibility:
    To turn the input source code into a linked list.
    The elements in the list can also be lists.

    EDIT: Linked lists are not pretty in Rust, it seems.
*/


pub fn parse_string(input : String) -> Vec<LispItem>{
    let mut tokens: Vec<String> = Vec::new();
    let in_tokens: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    
    for i in 0..in_tokens.len(){
        let token = in_tokens[i].clone();
        for c in token.chars(){
            if c == '(' {
                tokens.push("(".to_string());
            }
        }
        tokens.push(str::replace(&str::replace(&token, "(", ""), ")", ""));
        for c in token.chars(){
            if c == ')' {
                tokens.push(")".to_string());    
            }       
        }
    }
    parse(tokens)
}

pub fn parse(tokens : Vec<String>) -> Vec<LispItem> {
    let mut list : Vec<LispItem> = Vec::new();

    let mut inc: bool;
    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i].clone();
        inc = true;
        match token.as_ref(){
            "(" => {
                let mut inner_tokens : Vec<String> = Vec::new();
                for j in i + 1..tokens.len(){
                    inner_tokens.push(tokens[j].clone());
                }
                let inner_list = parse(inner_tokens);
                i += skip_count(&inner_list);
                inc = false;
                list.push(LispItem::List(inner_list));    
            },
            ")" => {
                return list;
            },
            _ => {
                list.push(LispItem::Atom(parse_type(token)))
            }
        }
        if inc {
            i += 1;
        }
    }    
    list
}

fn parse_type(atom : String) -> LispType{
    if atom.parse::<i32>().is_ok(){
        return LispType::Integer(atom.parse::<i32>().unwrap())
    }
    if atom.parse::<f32>().is_ok(){
        return LispType::Float(atom.parse::<f32>().unwrap())
    }
    return LispType::Symbol(atom)    
}

fn skip_count(list : &Vec<LispItem>) -> usize {
    let mut sum = 0;
    for item in list{
        match item{
            &LispItem::Atom(_) => sum += 1,
            &LispItem::List(ref inner) => {
                sum += skip_count(&inner);
            }
        }
    }
    return sum + 2; // 2 because parenthesis
}