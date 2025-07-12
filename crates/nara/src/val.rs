#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Number(i64),
    Float(f64),
    Unit,
}
