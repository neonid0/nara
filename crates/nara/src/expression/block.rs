use crate::{env::Env, statement::Statement, utils, val::Val};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Block {
    pub(crate) statements: Vec<Statement>,
}

impl Block {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, statements) = utils::sequence(Statement::new, s)?;

        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("}", s)?;

        // optional extract semicolon
        let (s, _) = utils::extract_semicolon(s);

        Ok((s, Block { statements }))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        if self.statements.is_empty() {
            return Ok(Val::Unit);
        }

        // Doesn’t compile because Env doesn’t implement Clone.
        let mut child_env = env.create_child();

        let stmts_except_last = &self.statements[..self.statements.len() - 1];
        for stmt in stmts_except_last {
            stmt.eval(&mut child_env)?;
        }

        // can unwrap safely because I checked that the block is not empty.
        self.statements.last().unwrap().eval(&mut child_env)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Expression, Number, Op};
    use super::*;
    use crate::binding_def::BindingDef;
    use crate::expression::binding_usage::BindingUsage;

    #[test]
    fn parse_empty_block() {
        assert_eq!(
            Block::new("{}"),
            Ok((
                "",
                Block {
                    statements: Vec::new()
                }
            ))
        )
    }

    #[test]
    fn eval_empty_block() {
        assert_eq!(
            Block {
                statements: Vec::new()
            }
            .eval(&Env::default()),
            Ok(Val::Unit),
        )
    }

    #[test]
    fn parse_empty_block_with_whitespace_adn_semicolon() {
        assert_eq!(
            Block::new("{    };"),
            Ok((
                "",
                Block {
                    statements: Vec::new()
                }
            ))
        )
    }

    #[test]
    fn parse_block_in_one_line() {
        assert_eq!(
            Block::new("{val one=1;one}"),
            Ok((
                "",
                Block {
                    statements: vec![
                        Statement::BindingDef(BindingDef {
                            name: "one".to_string(),
                            val: Expression::Number(Number(1)),
                        }),
                        Statement::Expression(Expression::BindingUsage(BindingUsage {
                            name: "one".to_string(),
                        })),
                    ]
                }
            ))
        )
    }

    #[test]
    fn parse_block_with_single_statement() {
        assert_eq!(
            Block::new("{3+5}"),
            Ok((
                "",
                Block {
                    statements: vec![Statement::Expression(Expression::Operation {
                        lhs: Box::new(Expression::Number(Number(3))),
                        rhs: Box::new(Expression::Number(Number(5))),
                        op: Op::Add,
                    })]
                }
            ))
        )
    }

    #[test]
    fn eval_block_with_one_expression() {
        assert_eq!(
            Block {
                statements: vec![Statement::Expression(Expression::Operation {
                    lhs: Box::new(Expression::Number(Number(3))),
                    rhs: Box::new(Expression::Number(Number(5))),
                    op: Op::Mul,
                })]
            }
            .eval(&Env::default()),
            Ok(Val::Number(15))
        )
    }
    #[test]
    fn eval_block_with_multi_statements() {
        assert_eq!(
            Block {
                statements: vec![
                    Statement::BindingDef(BindingDef {
                        name: "one".to_string(),
                        val: Expression::Number(Number(1)),
                    }),
                    Statement::Expression(Expression::BindingUsage(BindingUsage {
                        name: "one".to_string(),
                    })),
                ],
            }
            .eval(&Env::default()),
            Ok(Val::Number(1))
        )
    }

    #[test]
    fn eval_block_with_multiple_binding_defs() {
        assert_eq!(
            Block {
                statements: vec![
                    Statement::BindingDef(BindingDef {
                        name: "foo".to_string(),
                        val: Expression::Number(Number(5)),
                    }),
                    Statement::BindingDef(BindingDef {
                        name: "bar".to_string(),
                        val: Expression::Number(Number(4)),
                    }),
                    Statement::BindingDef(BindingDef {
                        name: "baz".to_string(),
                        val: Expression::Number(Number(3)),
                    }),
                ],
            }
            .eval(&Env::default()),
            Ok(Val::Unit),
        );
    }

    #[test]
    fn eval_block_with_multiple_expressions() {
        assert_eq!(
            Block {
                statements: vec![
                    Statement::Expression(Expression::Number(Number(100))),
                    Statement::Expression(Expression::Number(Number(30))),
                    Statement::Expression(Expression::Operation {
                        lhs: Box::new(Expression::Number(Number(10))),
                        rhs: Box::new(Expression::Number(Number(7))),
                        op: Op::Sub,
                    }),
                ],
            }
            .eval(&Env::default()),
            Ok(Val::Number(3)),
        );
    }

    #[test]
    fn parse_block_with_multi_statements() {
        assert_eq!(
            Block::new(
                "{
                    val x = 10;
                    val y = x;
                    x
                }"
            ),
            Ok((
                "",
                Block {
                    statements: vec![
                        Statement::BindingDef(BindingDef {
                            name: "x".to_string(),
                            val: Expression::Number(Number(10)),
                        }),
                        Statement::BindingDef(BindingDef {
                            name: "y".to_string(),
                            val: Expression::BindingUsage(BindingUsage {
                                name: "x".to_string(),
                            }),
                        }),
                        Statement::Expression(Expression::BindingUsage(BindingUsage {
                            name: "x".to_string(),
                        })) // Statement::Expression(Expression::Operation {
                            //     lhs: Expression::BindingUsage(BindingUsage {
                            //         name: "x".to_string(),
                            //     }),
                            //     rhs: Expression::BindingUsage(BindingUsage {
                            //         name: "y".to_string(),
                            //     }),
                            //     op: Op::Add,
                            // }),
                    ],
                },
            )),
        );
    }

    #[test]
    fn eval_block_using_bindings_from_parent_env() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(2));

        assert_eq!(
            Block {
                statements: vec![
                    Statement::BindingDef(BindingDef {
                        name: "baz".to_string(),
                        val: Expression::BindingUsage(BindingUsage {
                            name: "foo".to_string(),
                        }),
                    }),
                    Statement::Expression(Expression::BindingUsage(BindingUsage {
                        name: "baz".to_string(),
                    })),
                ],
            }
            .eval(&env),
            Ok(Val::Number(2)),
        );
    }
}
