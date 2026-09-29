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

    fn eval_str(s: &str) -> Result<Val, String> {
        parse(s)?.eval(&mut Env::default())
    }

    #[test]
    fn test_operator_precedence() {
        assert_eq!(eval_str("2 + 3 * 4"), Ok(Val::Number(14)));
        assert_eq!(eval_str("2 * 3 + 4"), Ok(Val::Number(10)));
        assert_eq!(eval_str("1 + 2 < 4 && 5 > 3"), Ok(Val::Bool(true)));
        assert_eq!(eval_str("1 == 1 || 1 == 2"), Ok(Val::Bool(true)));
    }

    #[test]
    fn test_left_associativity() {
        assert_eq!(eval_str("1 + 2 + 3"), Ok(Val::Number(6)));
        assert_eq!(eval_str("10 - 4 - 3"), Ok(Val::Number(3)));
        assert_eq!(eval_str("100 / 10 / 5"), Ok(Val::Number(2)));
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(eval_str("(2 + 3) * 4"), Ok(Val::Number(20)));
        assert_eq!(eval_str("10 - (4 - 3)"), Ok(Val::Number(9)));
        assert_eq!(eval_str("!(1 == 2)"), Ok(Val::Bool(true)));
        assert_eq!(eval_str("-(2 + 3)"), Ok(Val::Number(-5)));
    }

    #[test]
    fn test_if_with_operation_condition() {
        assert_eq!(
            eval_str("val x = 10; if x > 5 { \"big\" } else { \"small\" }"),
            Ok(Val::String(Rc::from("big")))
        );
        assert_eq!(
            eval_str("val x = 3; if x > 5 { 1 } else if x > 1 { 2 } else { 3 }"),
            Ok(Val::Number(2))
        );
    }

    #[test]
    fn test_while_with_operation_condition() {
        assert_eq!(eval_str("while 1 > 2 { 1 }"), Ok(Val::Unit));
    }

    #[test]
    fn test_for_over_function_call() {
        assert_eq!(eval_str("for i in range(4) { i * 2 }"), Ok(Val::Number(6)));
    }

    #[test]
    fn test_nested_function_calls() {
        assert_eq!(
            eval_str("fn add(a, b) { a + b }; add(add(1, 2), 3 * 2)"),
            Ok(Val::Number(9))
        );
        assert_eq!(eval_str("len(range(2 + 3))"), Ok(Val::Number(5)));
    }

    #[test]
    fn test_short_circuit() {
        assert_eq!(eval_str("false && missing"), Ok(Val::Bool(false)));
        assert_eq!(eval_str("true || missing"), Ok(Val::Bool(true)));
    }

    #[test]
    fn test_floor_division() {
        assert_eq!(eval_str("7 // 2"), Ok(Val::Number(3)));
        assert_eq!(eval_str("-7 // 2"), Ok(Val::Number(-4)));
        assert_eq!(eval_str("7.5 // 2.0"), Ok(Val::Float(3.0)));
    }

    #[test]
    fn test_comments() {
        assert_eq!(eval_str("1 + 2 # sum"), Ok(Val::Number(3)));
        assert_eq!(eval_str("val a = 4 # four\na * 2"), Ok(Val::Number(8)));
    }

    #[test]
    fn test_fstring_with_expression() {
        assert_eq!(
            eval_str("val a = 2; f\"{a * 3} items\""),
            Ok(Val::String(Rc::from("6 items")))
        );
    }

    #[test]
    fn test_bool_prefixed_identifier() {
        assert_eq!(eval_str("val trueish = 5; trueish"), Ok(Val::Number(5)));
    }

    #[test]
    fn test_multi_statement_with_operations() {
        let mut env = Env::default();
        let parse_result = parse("val x = 5; val y = 10; x + y").unwrap();
        let result = parse_result.eval(&mut env).unwrap();
        assert_eq!(result, Val::Number(15));
    }
}
