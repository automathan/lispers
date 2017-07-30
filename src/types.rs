use std::collections::HashMap;
use std::fmt;

//#[derive(Clone)]
pub struct Environment<'a>{
    bindings: HashMap<String, LispItem>,
    parent: Option<Box<&'a Environment<'a>>>
}

impl<'a> Environment<'a>{
    pub fn new(parent : Option<&'a Environment>) -> Environment<'a> {
        match parent {
            Some(env) => {
                return Environment {
                    bindings: HashMap::new(),
                    parent: Some(Box::new(env))
                }
            },
            None => {
                return Environment {
                    bindings: HashMap::new(),
                    parent: None
                }
            }
        }
    }
    pub fn get(&self, key: &String) -> Option<LispItem>{
        match self.bindings.get(key){
            Some(val) => return Some(val.clone()),
            None => {
                match self.parent{
                    Some(ref val) => return val.get(key).clone(),
                    None => return None
                }
            }
        }
    }
    pub fn insert(&mut self, key: &String, value: &LispItem){
        self.bindings.insert(key.clone(), value.clone());
    }
}

#[derive(Clone)]
pub enum LispType{
    Integer(i32),
    Float(f32),
    Symbol(String),
    Function(Vec<String>, Vec<LispItem>), // (bindings,body)
    Bool(bool)
}

#[derive(Clone)]
pub enum LispItem{
    List(Vec<LispItem>,bool), // items, datamode
    Atom(LispType),
    Error(String, Box<LispItem>) // (msg, source) 
}

pub fn lisp_error(msg : &str, src : &LispItem) -> LispItem{
    return LispItem::Error(msg.to_string(), Box::new(src.clone()));
}

pub fn lisp_error_str(msg : &str, src : &String) -> LispItem{ // for unwrapped symbol atoms
    return LispItem::Error(msg.to_string(), Box::new(LispItem::Atom(LispType::Symbol(src.clone()))));
}

impl fmt::Display for LispItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            &LispItem::Atom(LispType::Integer(ref val)) => return write!(f, " {}:i32 ", val),
            &LispItem::Atom(LispType::Bool(ref val)) => return write!(f, " {}:bool ", if *val {"t"} else {"nil"}),
            &LispItem::Atom(LispType::Float(ref val)) => return write!(f, " {}:f32 ", val),
            &LispItem::Atom(LispType::Symbol(ref val)) => return write!(f, " \"{}\":sym ", val),
            &LispItem::List(ref inner, ref dm) => return write!(f, "list, len = {}, dm = {}", inner.len(), dm),
            &LispItem::Atom(LispType::Function(_, _)) => return write!(f, "?"),
            &LispItem::Error(ref msg, ref src) => return write!(f, "[error: {}, source: {}]", msg, src)
        }
    }
}