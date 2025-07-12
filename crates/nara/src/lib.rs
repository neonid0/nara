mod binding_def;
mod env;
mod expression;
mod function_def;
mod statement;
mod utils;
mod val;

pub use env::Env;
pub use val::Val;

#[derive(Debug)]
pub struct Parse(statement::Statement);

impl Parse {
    pub fn eval(&self, env: &mut env::Env) -> Result<Val, String> {
        self.0.eval(env)
    }
}

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, statement) = statement::Statement::new(s)?;

    if s.is_empty() {
        Ok(Parse(statement))
    } else {
        Err("input was not consumed fully by parser".to_string())
    }
}
