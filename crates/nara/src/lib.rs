mod binding_def;
mod env;
mod expression;
mod function_def;
mod interner;
mod statement;
mod utils;
mod val;

pub use env::Env;
pub use interner::StringInterner;
pub use val::Val;

#[derive(Debug)]
pub struct Parse(Vec<statement::Statement>);

impl Parse {
    pub fn eval(&self, env: &mut env::Env) -> Result<Val, String> {
        if self.0.is_empty() {
            return Ok(Val::Unit);
        }

        // Evaluate all statements except the last
        for stmt in &self.0[..self.0.len() - 1] {
            stmt.eval(env)?;
        }

        // Return the result of the last statement
        self.0.last().unwrap().eval(env)
    }
}

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, statements) = utils::sequence(statement::Statement::new, s)?;

    if s.is_empty() {
        if statements.is_empty() {
            Err("expected at least one statement".to_string())
        } else {
            Ok(Parse(statements))
        }
    } else {
        Err("input was not consumed fully by parser".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_string_interning_same_literals() {
        let mut env = Env::default();

        let parse1 = parse("\"hello\"").unwrap();
        let parse2 = parse("\"hello\"").unwrap();

        let val1 = parse1.eval(&mut env).unwrap();
        let val2 = parse2.eval(&mut env).unwrap();

        if let (Val::String(s1), Val::String(s2)) = (val1, val2) {
            assert_eq!(s1, s2);
            assert!(
                Rc::ptr_eq(&s1, &s2),
                "Identical strings should be interned to same location"
            );
        } else {
            panic!("Expected string values");
        }
    }

    #[test]
    fn test_string_interning_concatenation() {
        let mut env = Env::default();

        // Create two identical concatenations
        let parse1 = parse("\"hello\" + \" world\"").unwrap();
        let parse2 = parse("\"hello\" + \" world\"").unwrap();

        let val1 = parse1.eval(&mut env).unwrap();
        let val2 = parse2.eval(&mut env).unwrap();

        if let (Val::String(s1), Val::String(s2)) = (val1, val2) {
            assert_eq!(s1, s2);
            assert!(
                Rc::ptr_eq(&s1, &s2),
                "Identical concatenation results should be interned"
            );
        } else {
            panic!("Expected string values");
        }
    }

    #[test]
    fn test_multi_statement_parsing() {
        let mut env = Env::default();
        let parse_result = parse("val x = 10; x").unwrap();
        let result = parse_result.eval(&mut env).unwrap();
        assert_eq!(result, Val::Number(10));
    }

    #[test]
    fn test_multi_statement_with_operations() {
        let mut env = Env::default();
        let parse_result = parse("val x = 5; val y = 10; x + y").unwrap();
        let result = parse_result.eval(&mut env).unwrap();
        assert_eq!(result, Val::Number(15));
    }
}
