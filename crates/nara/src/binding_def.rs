use crate::{
    env::Env,
    expression::Expression,
    utils::{self, tag},
};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BindingDef {
    pub(crate) name: String,
    pub(crate) val: Expression,
}

impl BindingDef {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("val", s)?;
        let (s, _) = utils::extract_whitespace_restrict(s)?;

        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let s = tag("=", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, val) = Expression::new(s)?;

        let (s, _) = utils::extract_semicolon(s);
        let (s, _) = utils::extract_whitespace(s);

        Ok((
            s,
            Self {
                name: name.to_string(),
                val,
            },
        ))
    }

    // Store binding to hashmap
    pub(crate) fn eval(&self, env: &mut Env) -> Result<(), String> {
        env.store_binding(self.name.clone(), self.val.eval(env)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("val x = 10 / 5;    "),
            Ok((
                "",
                BindingDef {
                    name: "x".to_string(),
                    val: Expression::Operation {
                        lhs: Box::new(Expression::Number(Number(10))),
                        rhs: Box::new(Expression::Number(Number(5))),
                        op: Op::Div,
                    }
                }
            ))
        )
    }

    #[test]
    fn cannot_parse_binding_def_without_space_after_val() {
        assert_eq!(
            BindingDef::new("valaaa=1+2"),
            Err("expected whitespace".to_string()),
        );
    }
}
