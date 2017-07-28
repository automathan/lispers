//mod types; 
use std::collections::HashMap;

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
    Atom(LispType)
}