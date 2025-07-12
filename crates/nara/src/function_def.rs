use crate::{expression::Expression, statement::Statement, utils};

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionDef {
    pub(crate) name: String,
    pub(crate) params: Vec<String>,
    pub(crate) body: Box<Statement>,
}

impl FunctionDef {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("fn", s)?;
        let (s, _) = utils::extract_whitespace_restrict(s)?;

        let (s, name) = utils::extract_ident(s)?;

        let (s, params) = utils::extract_params(s)?;

        let (s, body) = Statement::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                params,
                body: Box::new(body),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Block;

    #[test]
    fn parse_function_def_with_no_params_and_empty_body() {
        assert_eq!(
            FunctionDef::new("fn nothing() {}"),
            Ok((
                "",
                FunctionDef {
                    name: "nothing".to_string(),
                    params: Vec::new(),
                    body: Box::new(Statement::Expression(Expression::Block(Block {
                        statements: Vec::new()
                    }))),
                }
            ))
        )
    }
}
