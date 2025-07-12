mod binding_usage;
mod block;

use crate::{env::Env, utils, val::Val};
pub(crate) use binding_usage::BindingUsage;
pub(crate) use block::Block;

// Number struct that holds 64-bit integer
#[derive(Debug, PartialEq)]
pub(crate) struct Number(pub i64);

impl Number {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let (s, number) = utils::extract_digits(s)?;

        Ok((s, Self(number.parse().unwrap())))
    }
}

// Number struct that holds 64-bit floating point number
#[derive(Debug, PartialEq)]
pub(crate) struct Float(pub f64);

impl Float {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let (s, float) = utils::extract_float(s)?;

        Ok((s, Self(float.parse().unwrap())))
    }
}

// operator enum with methods to create an operator from a string
#[derive(Debug, PartialEq)]
pub(crate) enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Floor,
}

impl Op {
    fn new(s: &str) -> Result<(&str, Self), String> {
        utils::tag("+", s)
            .map(|s| (s, Self::Add))
            .or_else(|_| utils::tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| utils::tag("*", s).map(|s| (s, Self::Mul)))
            .or_else(|_| utils::tag("//", s).map(|s| (s, Self::Floor)))
            .or_else(|_| utils::tag("/", s).map(|s| (s, Self::Div)))
            .or_else(|_| {
                Err(format!(
                    "Expected one of the operators: +, -, *, /, //, but found '{}'",
                    s
                ))
            })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Expression {
    Number(Number),
    Float(Float),
    Operation { lhs: Number, rhs: Number, op: Op },
    BindingUsage(BindingUsage),
    Block(Block),
}

impl Expression {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_operation(s)
            .or_else(|_| Self::new_float(s))
            .or_else(|_| Self::new_number(s))
            .or_else(|_| {
                BindingUsage::new(s)
                    .map(|(s, binding_usage)| (s, Self::BindingUsage(binding_usage)))
            })
            .or_else(|_| Block::new(s).map(|(s, block)| (s, Self::Block(block))))
    }

    fn new_operation(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = Number::new(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, op) = Op::new(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, rhs) = Number::new(s)?;

        Ok((s, Self::Operation { lhs, rhs, op }))
    }

    fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    fn new_float(s: &str) -> Result<(&str, Self), String> {
        Float::new(s).map(|(s, float)| (s, Self::Float(float)))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        match self {
            Self::Float(Float(n)) => Ok(Val::Float(*n)),
            Self::Number(Number(n)) => Ok(Val::Number(*n)),
            Self::Operation { lhs, rhs, op } => {
                let Number(lhs) = lhs;
                let Number(rhs) = rhs;

                let result = match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => {
                        if rhs == &0 {
                            return Err("Division by zero".to_string());
                        }
                        lhs / rhs
                    }
                    Op::Floor => {
                        if rhs == &0 {
                            return Err("Division by zero".to_string());
                        }
                        lhs / rhs // floor division is the same as integer division in rust
                    }
                };

                Ok(Val::Number(result))
            }
            Self::BindingUsage(binding_usage) => binding_usage.eval(env),
            Self::Block(block) => block.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{env::Env, statement::Statement};

    #[test]
    fn parse_num() {
        assert_eq!(Number::new("321312"), Ok(("", Number(321312))))
    }

    #[test]
    fn parse_number_as_expression() {
        assert_eq!(
            Expression::new("475"),
            Ok(("", Expression::Number(Number(475))))
        )
    }
    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), Ok(("", Op::Add)));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), Ok(("", Op::Sub)));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), Ok(("", Op::Mul)));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), Ok(("", Op::Div)));
    }

    #[test]
    fn parse_floor_op() {
        assert_eq!(Op::new("//"), Ok(("", Op::Floor)));
    }

    #[test]
    fn parse_single_int() {
        assert_eq!(
            Expression::new("50"),
            Ok(("", Expression::Number(Number(50))))
        )
    }

    #[test]
    fn parse_single_float() {
        assert_eq!(
            Expression::new(" 3.14"),
            Ok(("", Expression::Float(Float(3.14))))
        )
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(20),
                rhs: Number(10),
                op: Op::Add,
            }
            .eval(&Env::default()),
            Ok(Val::Number(30))
        )
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(5),
                rhs: Number(10),
                op: Op::Sub,
            }
            .eval(&Env::default()),
            Ok(Val::Number(-5))
        )
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(20),
                rhs: Number(10),
                op: Op::Mul,
            }
            .eval(&Env::default()),
            Ok(Val::Number(200))
        )
    }

    // there will be expressions for division and floor division
    #[test]
    fn eval_div() {
        assert_eq!(
            Expression::Operation {
                // lhs: Float(3.0),
                lhs: Number(12),
                rhs: Number(3),
                op: Op::Div,
            }
            .eval(&Env::default()),
            Ok(Val::Number(4))
        )
    }

    #[test]
    fn eval_floor() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(20),
                rhs: Number(3),
                op: Op::Floor,
            }
            .eval(&Env::default()),
            Ok(Val::Number(6))
        )
    }

    #[test]
    fn parse_expression_with_whitespace() {
        assert_eq!(
            Expression::new("3   //     4"),
            Ok((
                "",
                Expression::Operation {
                    lhs: Number(3),
                    op: Op::Floor,
                    rhs: Number(4),
                }
            ))
        )
    }

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            Expression::new("bar"),
            Ok((
                "",
                Expression::BindingUsage(BindingUsage {
                    name: "bar".to_string(),
                }),
            ))
        )
    }
    #[test]
    fn eval_binding_usage() {
        let mut env = Env::default();
        env.store_binding("ten".to_string(), Val::Number(10));

        assert_eq!(
            Expression::BindingUsage(BindingUsage {
                name: "ten".to_string(),
            })
            .eval(&env),
            Ok(Val::Number(10)),
        )
    }

    #[test]
    fn parse_block() {
        assert_eq!(
            Expression::new("{ 300 }"),
            Ok((
                "",
                Expression::Block(Block {
                    statements: vec![Statement::Expression(Expression::Number(Number(300)))]
                })
            ))
        )
    }

    #[test]
    fn eval_block() {
        assert_eq!(
            Expression::Block(Block {
                statements: vec![Statement::Expression(Expression::Number(Number(10)))],
            })
            .eval(&Env::default()),
            Ok(Val::Number(10))
        )
    }
}
