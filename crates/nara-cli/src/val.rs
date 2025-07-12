use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Number(i64),
    Float(f64),
    Unit,
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Unit => write!(f, "Unit"),
            _ => todo!("Display not implemented for this value type"),
        }
    }
}
