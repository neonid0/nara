use crate::binding_def::BindingDef;
use crate::env::Env;
use crate::expression::Expression;
use crate::function_def::FunctionDef;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub(crate) enum Statement {
    BindingDef(BindingDef),
    FunctionDef(FunctionDef),
    Expression(Expression),
}

impl Statement {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            .or_else(|_| {
                FunctionDef::new(s).map(|(s, function_def)| (s, Self::FunctionDef(function_def)))
            })
            .or_else(|_| {
                Expression::new(s).map(|(s, expression)| (s, Self::Expression(expression)))
            })
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Val, String> {
        match self {
            Self::BindingDef(binding_def) => {
                binding_def.eval(env)?;
                Ok(Val::Unit)
            }
            Self::Expression(expression) => expression.eval(env),
            _ => todo!("Function definitions are not yet supported in eval"),
            // Self::FunctionDef(function_def) => {
            //     env.store_function(function_def.name.clone(), function_def.clone());
            //     Ok(Val::Unit)
            // }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        expression::{BindingUsage, Block, Number, Op},
        function_def::FunctionDef,
    };

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Statement::new("val x = 15;"),
            Ok((
                "",
                Statement::BindingDef(BindingDef {
                    name: "x".to_string(),
                    val: Expression::Number(Number(15))
                }),
            ))
        )
    }

    #[test]
    fn eval_binding_def() {
        assert_eq!(
            Statement::BindingDef(BindingDef {
                name: "some_variable".to_string(),
                val: Expression::Number(Number(10))
            })
            .eval(&mut Env::default()),
            Ok(Val::Unit)
        )
    }

    #[test]
    fn parse_expression() {
        assert_eq!(
            Statement::new("10 + 10"),
            Ok((
                "",
                Statement::Expression(Expression::Operation {
                    lhs: Number(10),
                    rhs: Number(10),
                    op: Op::Add
                })
            ))
        )
    }

    #[test]
    fn eval_expression() {
        assert_eq!(
            Statement::Expression(Expression::Number(Number(5))).eval(&mut Env::default()),
            Ok(Val::Number(5))
        )
    }

    #[test]
    fn parse_nested_functions() {
        assert_eq!(
            Statement::new(
                "fn outer() {
                    fn inner() {3 + 2}
                    inner
                }"
            ),
            Ok((
                "",
                Statement::FunctionDef(FunctionDef {
                    name: "outer".to_string(),
                    params: vec![],
                    body: Box::new(Statement::Expression(Expression::Block(Block {
                        statements: vec![
                            Statement::FunctionDef(FunctionDef {
                                name: "inner".to_string(),
                                params: vec![],
                                body: Box::new(Statement::Expression(Expression::Block(Block {
                                    statements: vec![Statement::Expression(
                                        Expression::Operation {
                                            lhs: Number(3),
                                            rhs: Number(2),
                                            op: Op::Add
                                        }
                                    )]
                                }))),
                            }),
                            Statement::Expression(Expression::BindingUsage(BindingUsage {
                                name: "inner".to_string()
                            }))
                        ],
                    })))
                })
            ))
        )
    }

    // return statement is not yet supported in eval
    #[test]
    fn parse_function_def() {
        assert_eq!(
            Statement::new(
                "fn semihkedy(param1, param2) {
                    val one = 1;
                    return one
                    it should return one regardless to statement order
                }"
            ),
            Ok((
                "",
                Statement::FunctionDef(FunctionDef {
                    name: "semihkedy".to_string(),
                    params: vec!["param1".to_string(), "param2".to_string()],
                    body: Box::new(Statement::Expression(Expression::Block(Block {
                        statements: vec![
                            Statement::BindingDef(BindingDef {
                                name: "one".to_string(),
                                val: Expression::Number(Number(1)),
                            }),
                            Statement::Expression(Expression::BindingUsage(BindingUsage {
                                name: "one".to_string(),
                            })),
                        ],
                    })))
                })
            ))
        )
    }

    #[test]
    fn parse_function_def_with_operation() {
        assert_eq!(
            Statement::new("fn operation(par1, par2) 4 + 3"),
            Ok((
                "",
                Statement::FunctionDef(FunctionDef {
                    name: "operation".to_string(),
                    params: vec!["par1".to_string(), "par2".to_string()],
                    body: Box::new(Statement::Expression(Expression::Operation {
                        lhs: Number(4),
                        rhs: Number(3),
                        op: Op::Add
                    },))
                })
            ))
        )
    }

    #[test]
    fn parse_function_def_with_number() {
        assert_eq!(
            Statement::new("fn number() 42"),
            Ok((
                "",
                Statement::FunctionDef(FunctionDef {
                    name: "number".to_string(),
                    params: vec![],
                    body: Box::new(Statement::Expression(Expression::Number(Number(42)))),
                }),
            ))
        );
    }
}
