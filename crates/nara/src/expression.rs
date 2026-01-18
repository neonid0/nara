mod binding_usage;
mod block;

use crate::{env::Env, utils, val::Val};
pub(crate) use binding_usage::BindingUsage;
pub(crate) use block::Block;

// Number struct that holds 64-bit integer
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Number(pub i64);

impl Number {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let (s, number) = utils::extract_digits(s)?;

        Ok((s, Self(number.parse().unwrap())))
    }
}

// Boolean literal
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BoolLiteral(pub bool);

impl BoolLiteral {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);

        utils::tag("true", s)
            .map(|s| (s, Self(true)))
            .or_else(|_| utils::tag("false", s).map(|s| (s, Self(false))))
    }
}

// Number struct that holds 64-bit floating point number
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Float(pub f64);

impl Float {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let (s, float) = utils::extract_float(s)?;

        Ok((s, Self(float.parse().unwrap())))
    }
}

// String struct that holds string literals
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct StringLiteral(pub String);

impl StringLiteral {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let (s, string_content) = utils::extract_string_literal(s)?;

        Ok((s, Self(string_content)))
    }
}

// operator enum with methods to create an operator from a string
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Op {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Floor,
    // Comparison
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    // Logical
    And,
    Or,
}

impl Op {
    fn new(s: &str) -> Result<(&str, Self), String> {
        // Try multi-character operators first
        utils::tag("==", s)
            .map(|s| (s, Self::Eq))
            .or_else(|_| utils::tag("!=", s).map(|s| (s, Self::NotEq)))
            .or_else(|_| utils::tag("<=", s).map(|s| (s, Self::LtEq)))
            .or_else(|_| utils::tag(">=", s).map(|s| (s, Self::GtEq)))
            .or_else(|_| utils::tag("&&", s).map(|s| (s, Self::And)))
            .or_else(|_| utils::tag("||", s).map(|s| (s, Self::Or)))
            .or_else(|_| utils::tag("//", s).map(|s| (s, Self::Floor)))
            // Then single-character operators
            .or_else(|_| utils::tag("+", s).map(|s| (s, Self::Add)))
            .or_else(|_| utils::tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| utils::tag("*", s).map(|s| (s, Self::Mul)))
            .or_else(|_| utils::tag("/", s).map(|s| (s, Self::Div)))
            .or_else(|_| utils::tag("<", s).map(|s| (s, Self::Lt)))
            .or_else(|_| utils::tag(">", s).map(|s| (s, Self::Gt)))
            .map_err(|_| {
                format!(
                    "Expected an operator, but found '{}'",
                    s.chars().take(10).collect::<String>()
                )
            })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct IfExpr {
    pub(crate) condition: Box<Expression>,
    pub(crate) then_branch: Box<Expression>,
    pub(crate) else_branch: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct FunctionCall {
    pub(crate) name: String,
    pub(crate) args: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct WhileLoop {
    pub(crate) condition: Box<Expression>,
    pub(crate) body: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct ForLoop {
    pub(crate) var: String,
    pub(crate) iterable: Box<Expression>,
    pub(crate) body: Box<Expression>,
}

impl ForLoop {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("for", s)?;
        let (s, _) = utils::extract_whitespace_restrict(s)?;

        // Parse variable name
        let (s, var) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace(s);

        // Parse 'in' keyword
        let s = utils::tag("in", s)?;
        let (s, _) = utils::extract_whitespace_restrict(s)?;

        // Parse iterable
        let (s, iterable) = Expression::new_operand(s)?;
        let (s, _) = utils::extract_whitespace(s);

        // Parse body (must be a block)
        let (s, body_block) = Block::new(s)?;
        let body = Box::new(Expression::Block(body_block));

        Ok((
            s,
            Self {
                var: var.to_string(),
                iterable: Box::new(iterable),
                body,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct ListLiteral {
    pub(crate) elements: Vec<Expression>,
}

impl ListLiteral {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);

        if !s.starts_with('[') {
            return Err("expected '[' for list literal".to_string());
        }

        let s = &s[1..];
        let mut elements = Vec::new();
        let mut remaining = s;

        loop {
            let (rest, _) = utils::extract_whitespace(remaining);

            // Check for closing bracket
            if rest.starts_with(']') {
                return Ok((&rest[1..], Self { elements }));
            }

            // Parse element
            let (rest, element) = Expression::new(rest)?;
            elements.push(element);

            let (rest, _) = utils::extract_whitespace(rest);

            // Check for comma or closing bracket
            if rest.starts_with(',') {
                remaining = &rest[1..];
            } else if rest.starts_with(']') {
                return Ok((&rest[1..], Self { elements }));
            } else {
                return Err(format!(
                    "expected ',' or ']' in list literal, got '{}'",
                    rest.chars().take(10).collect::<String>()
                ));
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Expression {
    Number(Number),
    Float(Float),
    String(StringLiteral),
    Bool(BoolLiteral),
    FString(Vec<utils::FStringPart>),
    List(ListLiteral),
    Operation {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        op: Op,
    },
    UnaryOp {
        operand: Box<Expression>,
        op: UnaryOp,
    },
    If(IfExpr),
    While(WhileLoop),
    For(ForLoop),
    FunctionCall(FunctionCall),
    BindingUsage(BindingUsage),
    Block(Block),
}

impl IfExpr {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("if", s)?;
        let (s, _) = utils::extract_whitespace_restrict(s)?;

        // Parse condition (any expression that's not a block)
        let (s, condition) = Expression::new_operand(s)?;
        let (s, _) = utils::extract_whitespace(s);

        // Parse then branch (must be a block)
        let (s, then_block) = Block::new(s)?;
        let then_branch = Box::new(Expression::Block(then_block));

        let (s, _) = utils::extract_whitespace(s);

        // Check for else branch
        let (s, else_branch) = if let Ok(s) = utils::tag("else", s) {
            let (s, _) = utils::extract_whitespace(s);

            // Check if it's another if (else if)
            if let Ok((s, if_expr)) = Self::new(s) {
                (s, Some(Box::new(Expression::If(if_expr))))
            } else {
                // Otherwise it should be a block
                let (s, else_block) = Block::new(s)?;
                (s, Some(Box::new(Expression::Block(else_block))))
            }
        } else {
            (s, None)
        };

        Ok((
            s,
            Self {
                condition: Box::new(condition),
                then_branch,
                else_branch,
            },
        ))
    }
}

impl FunctionCall {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace(s);

        // Must have parentheses for function call
        if !s.starts_with('(') {
            return Err("expected '(' for function call".to_string());
        }

        let (s, args_str) = utils::extract_paranthesis(s)?;

        // Parse comma-separated arguments
        let args = if args_str.trim().is_empty() {
            Vec::new()
        } else {
            let mut args = Vec::new();
            let mut remaining = args_str;

            loop {
                let (rest, arg) = Expression::new(remaining)?;
                args.push(arg);

                let (rest, _) = utils::extract_whitespace(rest);
                if rest.starts_with(',') {
                    remaining = &rest[1..];
                    let (rest, _) = utils::extract_whitespace(remaining);
                    remaining = rest;
                } else if rest.is_empty() {
                    break;
                } else {
                    return Err(format!("expected ',' or end of arguments, got '{}'", rest));
                }
            }

            args
        };

        Ok((
            s,
            Self {
                name: name.to_string(),
                args,
            },
        ))
    }
}

impl WhileLoop {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("while", s)?;
        let (s, _) = utils::extract_whitespace_restrict(s)?;

        // Parse condition
        let (s, condition) = Expression::new_operand(s)?;
        let (s, _) = utils::extract_whitespace(s);

        // Parse body (must be a block)
        let (s, body_block) = Block::new(s)?;
        let body = Box::new(Expression::Block(body_block));

        Ok((
            s,
            Self {
                condition: Box::new(condition),
                body,
            },
        ))
    }
}

impl Expression {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_operation(s)
            .or_else(|_| Self::new_unary_op(s))
            .or_else(|_| IfExpr::new(s).map(|(s, if_expr)| (s, Self::If(if_expr))))
            .or_else(|_| WhileLoop::new(s).map(|(s, while_loop)| (s, Self::While(while_loop))))
            .or_else(|_| ForLoop::new(s).map(|(s, for_loop)| (s, Self::For(for_loop))))
            .or_else(|_| FunctionCall::new(s).map(|(s, call)| (s, Self::FunctionCall(call))))
            .or_else(|_| ListLiteral::new(s).map(|(s, list)| (s, Self::List(list))))
            .or_else(|_| Self::new_bool(s))
            .or_else(|_| Self::new_float(s))
            .or_else(|_| Self::new_number(s))
            .or_else(|_| Self::new_fstring(s))
            .or_else(|_| Self::new_string(s))
            .or_else(|_| {
                BindingUsage::new(s)
                    .map(|(s, binding_usage)| (s, Self::BindingUsage(binding_usage)))
            })
            .or_else(|_| Block::new(s).map(|(s, block)| (s, Self::Block(block))))
    }

    fn new_operation(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = Self::new_operand(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, op) = Op::new(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, rhs) = Self::new_operand(s)?;

        Ok((
            s,
            Self::Operation {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op,
            },
        ))
    }

    fn new_operand(s: &str) -> Result<(&str, Self), String> {
        Self::new_bool(s)
            .or_else(|_| Self::new_float(s))
            .or_else(|_| Self::new_number(s))
            .or_else(|_| Self::new_string(s))
            .or_else(|_| ListLiteral::new(s).map(|(s, list)| (s, Self::List(list))))
            .or_else(|_| {
                BindingUsage::new(s)
                    .map(|(s, binding_usage)| (s, Self::BindingUsage(binding_usage)))
            })
            .or_else(|_| Block::new(s).map(|(s, block)| (s, Self::Block(block))))
    }

    fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    fn new_float(s: &str) -> Result<(&str, Self), String> {
        Float::new(s).map(|(s, float)| (s, Self::Float(float)))
    }

    fn new_bool(s: &str) -> Result<(&str, Self), String> {
        BoolLiteral::new(s).map(|(s, bool_lit)| (s, Self::Bool(bool_lit)))
    }

    fn new_unary_op(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);

        let (s, op) = utils::tag("!", s)
            .map(|s| (s, UnaryOp::Not))
            .or_else(|_| utils::tag("-", s).map(|s| (s, UnaryOp::Neg)))?;

        let (s, _) = utils::extract_whitespace(s);
        let (s, operand) = Self::new_operand(s)?;

        Ok((
            s,
            Self::UnaryOp {
                operand: Box::new(operand),
                op,
            },
        ))
    }

    fn new_string(s: &str) -> Result<(&str, Self), String> {
        StringLiteral::new(s).map(|(s, string)| (s, Self::String(string)))
    }

    fn new_fstring(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let (s, parts) = utils::extract_fstring(s)?;
        Ok((s, Self::FString(parts)))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        match self {
            Self::Float(Float(n)) => Ok(Val::Float(*n)),
            Self::Number(Number(n)) => Ok(Val::Number(*n)),
            Self::String(StringLiteral(s)) => Ok(Val::String(env.intern(s))),
            Self::Bool(BoolLiteral(b)) => Ok(Val::Bool(*b)),
            Self::FString(parts) => {
                if parts.is_empty() {
                    return Ok(Val::String(env.intern("")));
                }

                let mut result = String::new();

                for part in parts {
                    match part {
                        utils::FStringPart::Text(text) => result.push_str(text),
                        utils::FStringPart::Interpolation(expr_str) => {
                            let (remaining, expr) = Expression::new(expr_str)?;
                            if !remaining.is_empty() {
                                return Err(format!(
                                    "f-string interpolation didn't consume all input: '{}'",
                                    remaining
                                ));
                            }

                            let val = expr.eval(env)?;

                            let str_repr = match val {
                                Val::Number(n) => n.to_string(),
                                Val::Float(f) => f.to_string(),
                                Val::String(s) => s.to_string(),
                                Val::Bool(b) => b.to_string(),
                                Val::Function(_) => String::from("<function>"),
                                Val::List(items) => {
                                    let strs: Vec<String> = items
                                        .iter()
                                        .map(|v| match v {
                                            Val::Number(n) => n.to_string(),
                                            Val::Float(f) => f.to_string(),
                                            Val::String(s) => format!("\"{}\"", s),
                                            Val::Bool(b) => b.to_string(),
                                            Val::Function(_) => String::from("<function>"),
                                            Val::List(_) => String::from("[...]"),
                                            Val::Unit => String::from("()"),
                                        })
                                        .collect();
                                    format!("[{}]", strs.join(", "))
                                }
                                Val::Unit => String::from("()"),
                            };

                            result.push_str(&str_repr);
                        }
                    }
                }

                Ok(Val::String(env.intern(&result)))
            }
            Self::List(list_lit) => {
                let mut elements = Vec::new();
                for elem_expr in &list_lit.elements {
                    elements.push(elem_expr.eval(env)?);
                }
                Ok(Val::List(elements))
            }
            Self::Operation { lhs, rhs, op } => {
                let lhs_val = lhs.eval(env)?;
                let rhs_val = rhs.eval(env)?;

                match (lhs_val, rhs_val, op) {
                    // Arithmetic: Number operations
                    (Val::Number(l), Val::Number(r), Op::Add) => Ok(Val::Number(l + r)),
                    (Val::Number(l), Val::Number(r), Op::Sub) => Ok(Val::Number(l - r)),
                    (Val::Number(l), Val::Number(r), Op::Mul) => Ok(Val::Number(l * r)),
                    (Val::Number(l), Val::Number(r), Op::Div) => {
                        if r == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Val::Number(l / r))
                        }
                    }
                    (Val::Number(l), Val::Number(r), Op::Floor) => {
                        if r == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Val::Number(l / r))
                        }
                    }

                    // Arithmetic: Float operations
                    (Val::Float(l), Val::Float(r), Op::Add) => Ok(Val::Float(l + r)),
                    (Val::Float(l), Val::Float(r), Op::Sub) => Ok(Val::Float(l - r)),
                    (Val::Float(l), Val::Float(r), Op::Mul) => Ok(Val::Float(l * r)),
                    (Val::Float(l), Val::Float(r), Op::Div) => Ok(Val::Float(l / r)),

                    // String concatenation
                    (Val::String(l), Val::String(r), Op::Add) => {
                        let concatenated = format!("{}{}", l, r);
                        Ok(Val::String(env.intern(&concatenated)))
                    }

                    // Comparison: Numbers
                    (Val::Number(l), Val::Number(r), Op::Eq) => Ok(Val::Bool(l == r)),
                    (Val::Number(l), Val::Number(r), Op::NotEq) => Ok(Val::Bool(l != r)),
                    (Val::Number(l), Val::Number(r), Op::Lt) => Ok(Val::Bool(l < r)),
                    (Val::Number(l), Val::Number(r), Op::LtEq) => Ok(Val::Bool(l <= r)),
                    (Val::Number(l), Val::Number(r), Op::Gt) => Ok(Val::Bool(l > r)),
                    (Val::Number(l), Val::Number(r), Op::GtEq) => Ok(Val::Bool(l >= r)),

                    // Comparison: Floats
                    (Val::Float(l), Val::Float(r), Op::Eq) => Ok(Val::Bool(l == r)),
                    (Val::Float(l), Val::Float(r), Op::NotEq) => Ok(Val::Bool(l != r)),
                    (Val::Float(l), Val::Float(r), Op::Lt) => Ok(Val::Bool(l < r)),
                    (Val::Float(l), Val::Float(r), Op::LtEq) => Ok(Val::Bool(l <= r)),
                    (Val::Float(l), Val::Float(r), Op::Gt) => Ok(Val::Bool(l > r)),
                    (Val::Float(l), Val::Float(r), Op::GtEq) => Ok(Val::Bool(l >= r)),

                    // Comparison: Strings
                    (Val::String(l), Val::String(r), Op::Eq) => Ok(Val::Bool(l == r)),
                    (Val::String(l), Val::String(r), Op::NotEq) => Ok(Val::Bool(l != r)),

                    // Comparison: Bools
                    (Val::Bool(l), Val::Bool(r), Op::Eq) => Ok(Val::Bool(l == r)),
                    (Val::Bool(l), Val::Bool(r), Op::NotEq) => Ok(Val::Bool(l != r)),

                    // Logical operators
                    (Val::Bool(l), Val::Bool(r), Op::And) => Ok(Val::Bool(l && r)),
                    (Val::Bool(l), Val::Bool(r), Op::Or) => Ok(Val::Bool(l || r)),

                    // Type errors
                    (l, r, op) => Err(format!(
                        "Type error: cannot apply operator {:?} to {:?} and {:?}",
                        op, l, r
                    )),
                }
            }
            Self::UnaryOp { operand, op } => {
                let val = operand.eval(env)?;
                match (op, val) {
                    (UnaryOp::Not, Val::Bool(b)) => Ok(Val::Bool(!b)),
                    (UnaryOp::Neg, Val::Number(n)) => Ok(Val::Number(-n)),
                    (UnaryOp::Neg, Val::Float(f)) => Ok(Val::Float(-f)),
                    (op, val) => Err(format!(
                        "Type error: cannot apply unary operator {:?} to {:?}",
                        op, val
                    )),
                }
            }
            Self::If(if_expr) => {
                let condition_val = if_expr.condition.eval(env)?;

                if condition_val.is_truthy() {
                    if_expr.then_branch.eval(env)
                } else if let Some(else_branch) = &if_expr.else_branch {
                    else_branch.eval(env)
                } else {
                    Ok(Val::Unit)
                }
            }
            Self::While(while_loop) => {
                let mut result = Val::Unit;

                loop {
                    let condition_val = while_loop.condition.eval(env)?;
                    if !condition_val.is_truthy() {
                        break;
                    }

                    result = while_loop.body.eval(env)?;
                }

                Ok(result)
            }
            Self::For(for_loop) => {
                let iterable_val = for_loop.iterable.eval(env)?;
                let mut result = Val::Unit;

                match iterable_val {
                    Val::List(items) => {
                        for item in items {
                            let mut loop_env = env.create_child();
                            loop_env.store_binding(for_loop.var.clone(), item);
                            result = for_loop.body.eval(&mut loop_env)?;
                        }
                        Ok(result)
                    }
                    _ => Err("for loop requires an iterable (list)".to_string()),
                }
            }
            Self::FunctionCall(call) => {
                // Check for built-in functions first
                if call.name == "print" {
                    for arg in &call.args {
                        let val = arg.eval(env)?;
                        let output = match val {
                            Val::Number(n) => n.to_string(),
                            Val::Float(f) => f.to_string(),
                            Val::String(s) => s.to_string(),
                            Val::Bool(b) => b.to_string(),
                            Val::Function(_) => String::from("<function>"),
                            Val::List(items) => {
                                let strs: Vec<String> = items
                                    .iter()
                                    .map(|v| match v {
                                        Val::Number(n) => n.to_string(),
                                        Val::Float(f) => f.to_string(),
                                        Val::String(s) => s.to_string(),
                                        Val::Bool(b) => b.to_string(),
                                        Val::Function(_) => String::from("<function>"),
                                        Val::List(_) => String::from("[...]"),
                                        Val::Unit => String::from("()"),
                                    })
                                    .collect();
                                format!("[{}]", strs.join(", "))
                            }
                            Val::Unit => String::from("()"),
                        };
                        println!("{}", output);
                    }
                    return Ok(Val::Unit);
                }

                if call.name == "len" {
                    if call.args.len() != 1 {
                        return Err(format!("len() expects 1 argument, got {}", call.args.len()));
                    }
                    let val = call.args[0].eval(env)?;
                    let length = match val {
                        Val::String(s) => s.len() as i64,
                        Val::List(items) => items.len() as i64,
                        _ => return Err("len() requires a string or list".to_string()),
                    };
                    return Ok(Val::Number(length));
                }

                if call.name == "range" {
                    if call.args.len() != 1 && call.args.len() != 2 {
                        return Err(format!(
                            "range() expects 1 or 2 arguments, got {}",
                            call.args.len()
                        ));
                    }

                    let start = if call.args.len() == 1 {
                        0
                    } else {
                        match call.args[0].eval(env)? {
                            Val::Number(n) => n,
                            _ => return Err("range() arguments must be numbers".to_string()),
                        }
                    };

                    let end = match call.args[if call.args.len() == 1 { 0 } else { 1 }].eval(env)? {
                        Val::Number(n) => n,
                        _ => return Err("range() arguments must be numbers".to_string()),
                    };

                    let items: Vec<Val> = (start..end).map(Val::Number).collect();
                    return Ok(Val::List(items));
                }

                // Get the function from environment
                let func_val = env.get_binding_value_restrict(&call.name)?;

                match func_val {
                    Val::Function(func) => {
                        // Check parameter count
                        if func.params.len() != call.args.len() {
                            return Err(format!(
                                "Function '{}' expects {} arguments, got {}",
                                call.name,
                                func.params.len(),
                                call.args.len()
                            ));
                        }

                        // Evaluate arguments
                        let mut arg_vals = Vec::new();
                        for arg in &call.args {
                            arg_vals.push(arg.eval(env)?);
                        }

                        // Create child environment
                        let mut func_env = env.create_child();

                        // Bind parameters to argument values
                        for (param, arg_val) in func.params.iter().zip(arg_vals.iter()) {
                            func_env.store_binding(param.clone(), arg_val.clone());
                        }

                        // Evaluate function body
                        func.body.eval(&mut func_env)
                    }
                    _ => Err(format!("'{}' is not a function", call.name)),
                }
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
    use std::rc::Rc;

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
            Expression::new(" 2.5"),
            Ok(("", Expression::Float(Float(2.5))))
        )
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Number(Number(20))),
                rhs: Box::new(Expression::Number(Number(10))),
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
                lhs: Box::new(Expression::Number(Number(5))),
                rhs: Box::new(Expression::Number(Number(10))),
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
                lhs: Box::new(Expression::Number(Number(20))),
                rhs: Box::new(Expression::Number(Number(10))),
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
                lhs: Box::new(Expression::Number(Number(12))),
                rhs: Box::new(Expression::Number(Number(3))),
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
                lhs: Box::new(Expression::Number(Number(20))),
                rhs: Box::new(Expression::Number(Number(3))),
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
                    lhs: Box::new(Expression::Number(Number(3))),
                    rhs: Box::new(Expression::Number(Number(4))),
                    op: Op::Floor,
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

    #[test]
    fn parse_string_literal() {
        assert_eq!(
            Expression::new("\"hello world\""),
            Ok((
                "",
                Expression::String(StringLiteral("hello world".to_string()))
            ))
        )
    }

    #[test]
    fn parse_string_with_escapes() {
        assert_eq!(
            Expression::new("\"hello\\nworld\""),
            Ok((
                "",
                Expression::String(StringLiteral("hello\nworld".to_string()))
            ))
        )
    }

    #[test]
    fn eval_string() {
        assert_eq!(
            Expression::String(StringLiteral("test".to_string())).eval(&Env::default()),
            Ok(Val::String(Rc::from("test")))
        )
    }

    #[test]
    fn eval_string_concatenation() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::String(StringLiteral("hello".to_string()))),
                rhs: Box::new(Expression::String(StringLiteral(" world".to_string()))),
                op: Op::Add,
            }
            .eval(&Env::default()),
            Ok(Val::String(Rc::from("hello world")))
        )
    }

    #[test]
    fn parse_string_concatenation() {
        assert_eq!(
            Expression::new("\"hello\" + \" world\""),
            Ok((
                "",
                Expression::Operation {
                    lhs: Box::new(Expression::String(StringLiteral("hello".to_string()))),
                    rhs: Box::new(Expression::String(StringLiteral(" world".to_string()))),
                    op: Op::Add,
                }
            ))
        )
    }

    #[test]
    fn type_error_number_plus_string() {
        let result = Expression::Operation {
            lhs: Box::new(Expression::Number(Number(10))),
            rhs: Box::new(Expression::String(StringLiteral("test".to_string()))),
            op: Op::Add,
        }
        .eval(&Env::default());

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type error"));
    }

    #[test]
    fn type_error_string_minus_string() {
        let result = Expression::Operation {
            lhs: Box::new(Expression::String(StringLiteral("hello".to_string()))),
            rhs: Box::new(Expression::String(StringLiteral("world".to_string()))),
            op: Op::Sub,
        }
        .eval(&Env::default());

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type error"));
    }

    #[test]
    fn parse_simple_fstring() {
        assert_eq!(
            Expression::new("f\"hello\""),
            Ok((
                "",
                Expression::FString(vec![utils::FStringPart::Text("hello".to_string())])
            ))
        )
    }

    #[test]
    fn eval_simple_fstring() {
        assert_eq!(
            Expression::FString(vec![utils::FStringPart::Text("hello".to_string())])
                .eval(&Env::default()),
            Ok(Val::String(Rc::from("hello")))
        )
    }

    #[test]
    fn parse_fstring_with_interpolation() {
        assert_eq!(
            Expression::new("f\"Hello {name}!\""),
            Ok((
                "",
                Expression::FString(vec![
                    utils::FStringPart::Text("Hello ".to_string()),
                    utils::FStringPart::Interpolation("name".to_string()),
                    utils::FStringPart::Text("!".to_string()),
                ])
            ))
        )
    }

    #[test]
    fn eval_fstring_with_variable() {
        let mut env = Env::default();
        env.store_binding("name".to_string(), Val::String(Rc::from("World")));

        assert_eq!(
            Expression::FString(vec![
                utils::FStringPart::Text("Hello ".to_string()),
                utils::FStringPart::Interpolation("name".to_string()),
                utils::FStringPart::Text("!".to_string()),
            ])
            .eval(&env),
            Ok(Val::String(Rc::from("Hello World!")))
        )
    }

    #[test]
    fn eval_fstring_with_expression() {
        assert_eq!(
            Expression::FString(vec![
                utils::FStringPart::Text("Result: ".to_string()),
                utils::FStringPart::Interpolation("10 + 20".to_string()),
            ])
            .eval(&Env::default()),
            Ok(Val::String(Rc::from("Result: 30")))
        )
    }

    #[test]
    fn eval_fstring_with_number() {
        assert_eq!(
            Expression::FString(vec![
                utils::FStringPart::Text("Number: ".to_string()),
                utils::FStringPart::Interpolation("42".to_string()),
            ])
            .eval(&Env::default()),
            Ok(Val::String(Rc::from("Number: 42")))
        )
    }

    // ========== Boolean Tests ==========
    
    #[test]
    fn parse_true_literal() {
        assert_eq!(
            Expression::new("true"),
            Ok(("", Expression::Bool(BoolLiteral(true))))
        )
    }

    #[test]
    fn parse_false_literal() {
        assert_eq!(
            Expression::new("false"),
            Ok(("", Expression::Bool(BoolLiteral(false))))
        )
    }

    #[test]
    fn eval_true() {
        assert_eq!(
            Expression::Bool(BoolLiteral(true)).eval(&Env::default()),
            Ok(Val::Bool(true))
        )
    }

    #[test]
    fn eval_false() {
        assert_eq!(
            Expression::Bool(BoolLiteral(false)).eval(&Env::default()),
            Ok(Val::Bool(false))
        )
    }

    #[test]
    fn test_truthiness_bool() {
        assert_eq!(Val::Bool(true).is_truthy(), true);
        assert_eq!(Val::Bool(false).is_truthy(), false);
    }

    #[test]
    fn test_truthiness_numbers() {
        assert_eq!(Val::Number(0).is_truthy(), false);
        assert_eq!(Val::Number(1).is_truthy(), true);
        assert_eq!(Val::Number(-1).is_truthy(), true);
    }

    #[test]
    fn test_truthiness_strings() {
        assert_eq!(Val::String(Rc::from("")).is_truthy(), false);
        assert_eq!(Val::String(Rc::from("hello")).is_truthy(), true);
    }

    #[test]
    fn test_truthiness_lists() {
        assert_eq!(Val::List(vec![]).is_truthy(), false);
        assert_eq!(Val::List(vec![Val::Number(1)]).is_truthy(), true);
    }

    // ========== Comparison Operator Tests ==========

    #[test]
    fn parse_eq_operator() {
        assert_eq!(Op::new("=="), Ok(("", Op::Eq)));
    }

    #[test]
    fn parse_neq_operator() {
        assert_eq!(Op::new("!="), Ok(("", Op::NotEq)));
    }

    #[test]
    fn parse_lt_operator() {
        assert_eq!(Op::new("<"), Ok(("", Op::Lt)));
    }

    #[test]
    fn parse_gt_operator() {
        assert_eq!(Op::new(">"), Ok(("", Op::Gt)));
    }

    #[test]
    fn parse_lte_operator() {
        assert_eq!(Op::new("<="), Ok(("", Op::LtEq)));
    }

    #[test]
    fn parse_gte_operator() {
        assert_eq!(Op::new(">="), Ok(("", Op::GtEq)));
    }

    #[test]
    fn eval_number_equality() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Number(Number(5))),
                rhs: Box::new(Expression::Number(Number(5))),
                op: Op::Eq,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(true))
        )
    }

    #[test]
    fn eval_number_inequality() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Number(Number(5))),
                rhs: Box::new(Expression::Number(Number(3))),
                op: Op::NotEq,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(true))
        )
    }

    #[test]
    fn eval_number_less_than() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Number(Number(3))),
                rhs: Box::new(Expression::Number(Number(5))),
                op: Op::Lt,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(true))
        )
    }

    #[test]
    fn eval_number_greater_than() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Number(Number(10))),
                rhs: Box::new(Expression::Number(Number(5))),
                op: Op::Gt,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(true))
        )
    }

    #[test]
    fn eval_number_lte() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Number(Number(5))),
                rhs: Box::new(Expression::Number(Number(5))),
                op: Op::LtEq,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(true))
        )
    }

    #[test]
    fn eval_number_gte() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Number(Number(10))),
                rhs: Box::new(Expression::Number(Number(5))),
                op: Op::GtEq,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(true))
        )
    }

    #[test]
    fn eval_string_equality() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::String(StringLiteral("hello".to_string()))),
                rhs: Box::new(Expression::String(StringLiteral("hello".to_string()))),
                op: Op::Eq,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(true))
        )
    }

    #[test]
    fn eval_bool_equality() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Bool(BoolLiteral(true))),
                rhs: Box::new(Expression::Bool(BoolLiteral(true))),
                op: Op::Eq,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(true))
        )
    }

    // ========== Logical Operator Tests ==========

    #[test]
    fn parse_and_operator() {
        assert_eq!(Op::new("&&"), Ok(("", Op::And)));
    }

    #[test]
    fn parse_or_operator() {
        assert_eq!(Op::new("||"), Ok(("", Op::Or)));
    }

    #[test]
    fn eval_and_true_true() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Bool(BoolLiteral(true))),
                rhs: Box::new(Expression::Bool(BoolLiteral(true))),
                op: Op::And,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(true))
        )
    }

    #[test]
    fn eval_and_true_false() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Bool(BoolLiteral(true))),
                rhs: Box::new(Expression::Bool(BoolLiteral(false))),
                op: Op::And,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(false))
        )
    }

    #[test]
    fn eval_or_false_true() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Bool(BoolLiteral(false))),
                rhs: Box::new(Expression::Bool(BoolLiteral(true))),
                op: Op::Or,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(true))
        )
    }

    #[test]
    fn eval_or_false_false() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Bool(BoolLiteral(false))),
                rhs: Box::new(Expression::Bool(BoolLiteral(false))),
                op: Op::Or,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(false))
        )
    }

    // ========== Unary Operator Tests ==========

    #[test]
    fn parse_not_operator() {
        assert_eq!(
            Expression::new("!true"),
            Ok((
                "",
                Expression::UnaryOp {
                    operand: Box::new(Expression::Bool(BoolLiteral(true))),
                    op: UnaryOp::Not,
                }
            ))
        )
    }

    #[test]
    fn eval_not_true() {
        assert_eq!(
            Expression::UnaryOp {
                operand: Box::new(Expression::Bool(BoolLiteral(true))),
                op: UnaryOp::Not,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(false))
        )
    }

    #[test]
    fn eval_not_false() {
        assert_eq!(
            Expression::UnaryOp {
                operand: Box::new(Expression::Bool(BoolLiteral(false))),
                op: UnaryOp::Not,
            }
            .eval(&Env::default()),
            Ok(Val::Bool(true))
        )
    }

    #[test]
    fn eval_negate_number() {
        assert_eq!(
            Expression::UnaryOp {
                operand: Box::new(Expression::Number(Number(42))),
                op: UnaryOp::Neg,
            }
            .eval(&Env::default()),
            Ok(Val::Number(-42))
        )
    }

    #[test]
    fn eval_negate_float() {
        assert_eq!(
            Expression::UnaryOp {
                operand: Box::new(Expression::Float(Float(3.14))),
                op: UnaryOp::Neg,
            }
            .eval(&Env::default()),
            Ok(Val::Float(-3.14))
        )
    }

    // ========== If/Else Tests ==========

    #[test]
    fn parse_if_expression() {
        assert_eq!(
            Expression::new("if true { 42 }"),
            Ok((
                "",
                Expression::If(IfExpr {
                    condition: Box::new(Expression::Bool(BoolLiteral(true))),
                    then_branch: Box::new(Expression::Block(Block {
                        statements: vec![Statement::Expression(Expression::Number(Number(42)))]
                    })),
                    else_branch: None,
                })
            ))
        )
    }

    #[test]
    fn parse_if_else_expression() {
        let result = Expression::new("if false { 1 } else { 2 }");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        match expr {
            Expression::If(if_expr) => {
                assert!(if_expr.else_branch.is_some());
            }
            _ => panic!("Expected If expression"),
        }
    }

    #[test]
    fn eval_if_true() {
        assert_eq!(
            Expression::If(IfExpr {
                condition: Box::new(Expression::Bool(BoolLiteral(true))),
                then_branch: Box::new(Expression::Block(Block {
                    statements: vec![Statement::Expression(Expression::Number(Number(42)))]
                })),
                else_branch: None,
            })
            .eval(&Env::default()),
            Ok(Val::Number(42))
        )
    }

    #[test]
    fn eval_if_false_no_else() {
        assert_eq!(
            Expression::If(IfExpr {
                condition: Box::new(Expression::Bool(BoolLiteral(false))),
                then_branch: Box::new(Expression::Block(Block {
                    statements: vec![Statement::Expression(Expression::Number(Number(42)))]
                })),
                else_branch: None,
            })
            .eval(&Env::default()),
            Ok(Val::Unit)
        )
    }

    #[test]
    fn eval_if_else_true() {
        assert_eq!(
            Expression::If(IfExpr {
                condition: Box::new(Expression::Bool(BoolLiteral(true))),
                then_branch: Box::new(Expression::Block(Block {
                    statements: vec![Statement::Expression(Expression::Number(Number(1)))]
                })),
                else_branch: Some(Box::new(Expression::Block(Block {
                    statements: vec![Statement::Expression(Expression::Number(Number(2)))]
                }))),
            })
            .eval(&Env::default()),
            Ok(Val::Number(1))
        )
    }

    #[test]
    fn eval_if_else_false() {
        assert_eq!(
            Expression::If(IfExpr {
                condition: Box::new(Expression::Bool(BoolLiteral(false))),
                then_branch: Box::new(Expression::Block(Block {
                    statements: vec![Statement::Expression(Expression::Number(Number(1)))]
                })),
                else_branch: Some(Box::new(Expression::Block(Block {
                    statements: vec![Statement::Expression(Expression::Number(Number(2)))]
                }))),
            })
            .eval(&Env::default()),
            Ok(Val::Number(2))
        )
    }

    // ========== While Loop Tests ==========

    #[test]
    fn parse_while_loop() {
        let result = Expression::new("while false { 1 }");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        match expr {
            Expression::While(_) => {}
            _ => panic!("Expected While expression"),
        }
    }

    #[test]
    fn eval_while_false() {
        assert_eq!(
            Expression::While(WhileLoop {
                condition: Box::new(Expression::Bool(BoolLiteral(false))),
                body: Box::new(Expression::Block(Block {
                    statements: vec![Statement::Expression(Expression::Number(Number(1)))]
                })),
            })
            .eval(&Env::default()),
            Ok(Val::Unit)
        )
    }

    // ========== For Loop Tests ==========

    #[test]
    fn parse_for_loop() {
        let result = Expression::new("for i in [1, 2, 3] { i }");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        match expr {
            Expression::For(_) => {}
            _ => panic!("Expected For expression"),
        }
    }

    #[test]
    fn eval_for_loop_empty_list() {
        assert_eq!(
            Expression::For(ForLoop {
                var: "i".to_string(),
                iterable: Box::new(Expression::List(ListLiteral { elements: vec![] })),
                body: Box::new(Expression::Block(Block {
                    statements: vec![Statement::Expression(Expression::Number(Number(1)))]
                })),
            })
            .eval(&Env::default()),
            Ok(Val::Unit)
        )
    }

    #[test]
    fn eval_for_loop_with_items() {
        let result = Expression::For(ForLoop {
            var: "x".to_string(),
            iterable: Box::new(Expression::List(ListLiteral {
                elements: vec![
                    Expression::Number(Number(1)),
                    Expression::Number(Number(2)),
                ]
            })),
            body: Box::new(Expression::Block(Block {
                statements: vec![Statement::Expression(Expression::BindingUsage(
                    BindingUsage {
                        name: "x".to_string()
                    }
                ))]
            })),
        })
        .eval(&Env::default());
        
        // Last iteration returns 2
        assert_eq!(result, Ok(Val::Number(2)));
    }

    // ========== List Tests ==========

    #[test]
    fn parse_empty_list() {
        assert_eq!(
            Expression::new("[]"),
            Ok(("", Expression::List(ListLiteral { elements: vec![] })))
        )
    }

    #[test]
    fn parse_list_with_numbers() {
        assert_eq!(
            Expression::new("[1, 2, 3]"),
            Ok((
                "",
                Expression::List(ListLiteral {
                    elements: vec![
                        Expression::Number(Number(1)),
                        Expression::Number(Number(2)),
                        Expression::Number(Number(3)),
                    ]
                })
            ))
        )
    }

    #[test]
    fn parse_list_with_spaces() {
        assert_eq!(
            Expression::new("[ 1 , 2 , 3 ]"),
            Ok((
                "",
                Expression::List(ListLiteral {
                    elements: vec![
                        Expression::Number(Number(1)),
                        Expression::Number(Number(2)),
                        Expression::Number(Number(3)),
                    ]
                })
            ))
        )
    }

    #[test]
    fn eval_empty_list() {
        assert_eq!(
            Expression::List(ListLiteral { elements: vec![] }).eval(&Env::default()),
            Ok(Val::List(vec![]))
        )
    }

    #[test]
    fn eval_list_with_numbers() {
        assert_eq!(
            Expression::List(ListLiteral {
                elements: vec![
                    Expression::Number(Number(1)),
                    Expression::Number(Number(2)),
                    Expression::Number(Number(3)),
                ]
            })
            .eval(&Env::default()),
            Ok(Val::List(vec![
                Val::Number(1),
                Val::Number(2),
                Val::Number(3)
            ]))
        )
    }

    #[test]
    fn eval_list_with_mixed_types() {
        assert_eq!(
            Expression::List(ListLiteral {
                elements: vec![
                    Expression::Number(Number(42)),
                    Expression::String(StringLiteral("hello".to_string())),
                    Expression::Bool(BoolLiteral(true)),
                ]
            })
            .eval(&Env::default()),
            Ok(Val::List(vec![
                Val::Number(42),
                Val::String(Rc::from("hello")),
                Val::Bool(true)
            ]))
        )
    }

    // ========== Function Tests ==========

    #[test]
    fn parse_function_call_no_args() {
        let result = Expression::new("foo()");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        match expr {
            Expression::FunctionCall(call) => {
                assert_eq!(call.name, "foo");
                assert_eq!(call.args.len(), 0);
            }
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn parse_function_call_with_args() {
        let result = Expression::new("add(1, 2)");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        match expr {
            Expression::FunctionCall(call) => {
                assert_eq!(call.name, "add");
                assert_eq!(call.args.len(), 2);
            }
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn eval_function_call_builtin_print() {
        let result = Expression::FunctionCall(FunctionCall {
            name: "print".to_string(),
            args: vec![Expression::Number(Number(42))],
        })
        .eval(&Env::default());
        
        assert_eq!(result, Ok(Val::Unit));
    }

    #[test]
    fn eval_function_call_builtin_len_string() {
        assert_eq!(
            Expression::FunctionCall(FunctionCall {
                name: "len".to_string(),
                args: vec![Expression::String(StringLiteral("hello".to_string()))],
            })
            .eval(&Env::default()),
            Ok(Val::Number(5))
        )
    }

    #[test]
    fn eval_function_call_builtin_len_list() {
        assert_eq!(
            Expression::FunctionCall(FunctionCall {
                name: "len".to_string(),
                args: vec![Expression::List(ListLiteral {
                    elements: vec![
                        Expression::Number(Number(1)),
                        Expression::Number(Number(2)),
                        Expression::Number(Number(3)),
                    ]
                })],
            })
            .eval(&Env::default()),
            Ok(Val::Number(3))
        )
    }

    #[test]
    fn eval_function_call_builtin_range_one_arg() {
        assert_eq!(
            Expression::FunctionCall(FunctionCall {
                name: "range".to_string(),
                args: vec![Expression::Number(Number(5))],
            })
            .eval(&Env::default()),
            Ok(Val::List(vec![
                Val::Number(0),
                Val::Number(1),
                Val::Number(2),
                Val::Number(3),
                Val::Number(4),
            ]))
        )
    }

    #[test]
    fn eval_function_call_builtin_range_two_args() {
        assert_eq!(
            Expression::FunctionCall(FunctionCall {
                name: "range".to_string(),
                args: vec![Expression::Number(Number(2)), Expression::Number(Number(5))],
            })
            .eval(&Env::default()),
            Ok(Val::List(vec![
                Val::Number(2),
                Val::Number(3),
                Val::Number(4),
            ]))
        )
    }

    #[test]
    fn eval_user_defined_function() {
        use crate::function_def::FunctionDef;
        
        let mut env = Env::default();
        
        // Define: fn double(x) { x + x }
        let func_def = FunctionDef {
            name: "double".to_string(),
            params: vec!["x".to_string()],
            body: Box::new(Statement::Expression(Expression::Operation {
                lhs: Box::new(Expression::BindingUsage(BindingUsage {
                    name: "x".to_string(),
                })),
                rhs: Box::new(Expression::BindingUsage(BindingUsage {
                    name: "x".to_string(),
                })),
                op: Op::Add,
            })),
        };
        
        // Store function
        let func_val = Val::Function(crate::val::Function {
            params: func_def.params.clone(),
            body: Rc::new(*func_def.body.clone()),
        });
        env.store_binding("double".to_string(), func_val);
        
        // Call: double(21)
        let result = Expression::FunctionCall(FunctionCall {
            name: "double".to_string(),
            args: vec![Expression::Number(Number(21))],
        })
        .eval(&env);
        
        assert_eq!(result, Ok(Val::Number(42)));
    }

    #[test]
    fn test_function_parameter_binding() {
        use crate::function_def::FunctionDef;
        
        let mut env = Env::default();
        
        // Define: fn add(a, b) { a + b }
        let func_def = FunctionDef {
            name: "add".to_string(),
            params: vec!["a".to_string(), "b".to_string()],
            body: Box::new(Statement::Expression(Expression::Operation {
                lhs: Box::new(Expression::BindingUsage(BindingUsage {
                    name: "a".to_string(),
                })),
                rhs: Box::new(Expression::BindingUsage(BindingUsage {
                    name: "b".to_string(),
                })),
                op: Op::Add,
            })),
        };
        
        let func_val = Val::Function(crate::val::Function {
            params: func_def.params.clone(),
            body: Rc::new(*func_def.body.clone()),
        });
        env.store_binding("add".to_string(), func_val);
        
        // Call: add(10, 32)
        let result = Expression::FunctionCall(FunctionCall {
            name: "add".to_string(),
            args: vec![Expression::Number(Number(10)), Expression::Number(Number(32))],
        })
        .eval(&env);
        
        assert_eq!(result, Ok(Val::Number(42)));
    }

    // ========== Child Environment Tests ==========

    #[test]
    fn test_child_environment_scoping() {
        let mut parent = Env::default();
        parent.store_binding("x".to_string(), Val::Number(10));
        
        let mut child = parent.create_child();
        child.store_binding("y".to_string(), Val::Number(20));
        
        // Child can access parent's binding
        assert_eq!(
            child.get_binding_value_restrict("x"),
            Ok(Val::Number(10))
        );
        
        // Child has its own binding
        assert_eq!(
            child.get_binding_value_restrict("y"),
            Ok(Val::Number(20))
        );
    }

    #[test]
    fn test_child_environment_shadowing() {
        let mut parent = Env::default();
        parent.store_binding("x".to_string(), Val::Number(10));
        
        let mut child = parent.create_child();
        child.store_binding("x".to_string(), Val::Number(20));
        
        // Child shadows parent's binding
        assert_eq!(
            child.get_binding_value_restrict("x"),
            Ok(Val::Number(20))
        );
    }
}
