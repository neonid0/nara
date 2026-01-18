use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Rc<crate::statement::Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Number(i64),
    Float(f64),
    String(Rc<str>),
    Bool(bool),
    Function(Function),
    List(Vec<Val>),
    Unit,
}

impl Val {
    pub fn is_truthy(&self) -> bool {
        match self {
            Val::Bool(b) => *b,
            Val::Number(n) => *n != 0,
            Val::Float(f) => *f != 0.0,
            Val::String(s) => !s.is_empty(),
            Val::Function(_) => true,
            Val::List(items) => !items.is_empty(),
            Val::Unit => false,
        }
    }
}
