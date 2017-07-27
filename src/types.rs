//mod types;

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