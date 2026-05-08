use pest::{iterators::Pair, pratt_parser::{Assoc, Op, PrattParser}};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MathParser;


lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrattParser::new()
            .op(Op::infix(compare_op, Left)) // Lowest
            .op(Op::infix(add_op, Left))
            .op(Op::infix(mul_op, Left))
            .op(Op::infix(pow_op, Right))    // Highest ( this bug took me a long time to find D: )
    };
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(String, Vec<Access>, Expr, bool),
    Export(String),
    Import(String),
    Module(String, Vec<Statement>),
    Equivalence(Expr, Expr, bool),
    Expression(Expr, bool),
}

#[derive(Debug, Clone)]
pub enum Access {
    Name(String),
    Index(usize),
}

impl std::fmt::Display for Access {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name(n) => {write!(f, "{}", n)},
            Self::Index(i) => {write!(f, "{}", i)},
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Tuple(Vec<Expr>),
    MemberAccess(Box<Expr>, Vec<Access>),
    Length(Box<Expr>),
    Block {
        statements: Vec<Statement>,
        ret: Box<Expr>,
    },
    Binary {
        lhs: Box<Expr>,
        op: String,
        rhs: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Lambda {
        args: Vec<String>,
        body: Box<Expr>,
    },
    Ternary {
        condition: Box<Expr>,
        true_branch: Box<Expr>,
        false_branch: Box<Expr>,
    },
    Loop {
        bindings: Vec<(String, Expr)>,
        body: Box<Expr>
    },
    Recur(Vec<Expr>)
}

fn parse_expr(pairs: pest::iterators::Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Expr::Number(primary.as_str().parse().unwrap()),
            Rule::ident => Expr::Variable(primary.as_str().trim().to_string()),
            Rule::primary => {
                let mut inner = primary.into_inner();
                let atom_pair = inner.next().unwrap();
                let actual_atom = if atom_pair.as_rule() == Rule::atom {
                    atom_pair.into_inner().next().unwrap()
                } else {
                    atom_pair
                };
                let mut current_expr = match actual_atom.as_rule() {
                    Rule::number => Expr::Number(actual_atom.as_str().parse().unwrap()),
                    Rule::ident  => Expr::Variable(actual_atom.as_str().trim().trim().to_string()),
                    Rule::expr   => parse_expr(actual_atom.into_inner()),
                    
                    Rule::lambda => {
                        let mut lambda_inner = actual_atom.into_inner();
                        let mut args = Vec::new();
                        let next = lambda_inner.next().unwrap();
                        
                        let body = if next.as_rule() == Rule::id_list {
                            args = next.into_inner().map(|id| id.as_str().to_string()).collect();
                            parse_expr(lambda_inner.next().unwrap().into_inner())
                        } else {
                            parse_expr(next.into_inner())
                        };
                        Expr::Lambda { args, body: Box::new(body) }
                    }
                    Rule::loop_expr => {
                        let loop_inner = actual_atom.into_inner();
                        let mut bindings = Vec::new();
                        let mut body = None;

                        for pair in loop_inner {
                            match pair.as_rule() {
                                Rule::assignment => {
                                    let mut assign_inner = pair.into_inner();
                                    let name = assign_inner.next().unwrap().as_str().trim().to_string();
                                    let val_expr = parse_expr(assign_inner.next().unwrap().into_inner());
                                    bindings.push((name, val_expr));
                                }
                                Rule::expr => {
                                    body = Some(Box::new(parse_expr(pair.into_inner())));
                                }
                                _ => {}
                            }
                        }

                        Expr::Loop { 
                            bindings, 
                            body: body.expect("Loop must have a body expression") 
                        }
                    }
                    Rule::recur_expr => {
                        let args = actual_atom.into_inner()
                            .map(|p| parse_expr(p.into_inner()))
                            .collect();
                        
                        Expr::Recur(args)
                    }
                    Rule::block => {
                        let inner = actual_atom.into_inner();
                        let mut statements = Vec::new();
                        let mut final_expr: Option<Box<Expr>> = None;

                        for pair in inner {
                            match pair.as_rule() {
                                Rule::export | Rule::import => {
                                    match parse_statement(pair) {
                                        None => {},
                                        Some(s) => {statements.push(s);}
                                    }
                                }
                                Rule::assignment => {
                                    let mut assign_inner = pair.into_inner();
                                    
                                    let lvalue_pair = assign_inner.next().unwrap();
                                    let mut lvalue_inner = lvalue_pair.into_inner();
                                    
                                    let root_name = lvalue_inner.next().unwrap().as_str().to_string();
                                    
                                    let mut path = Vec::new();
                                    for access_pair in lvalue_inner {
                                        // access_pair is a Rule::dot_access
                                        let inner = access_pair.into_inner().next().unwrap();
                                        let step = match inner.as_rule() {
                                            Rule::ident => Access::Name(inner.as_str().to_string()),
                                            Rule::tuple_index => {
                                                let idx = inner.as_str().parse().expect("Invalid index");
                                                Access::Index(idx)
                                            },
                                            _ => unreachable!(),
                                        };
                                        path.push(step);
                                    }
                                    
                                    let val_expr = parse_expr(assign_inner.next().unwrap().into_inner());
                                    
                                    statements.push(Statement::Assignment(root_name, path, val_expr, true));
                                }
                                Rule::expr => {
                                    if let Some(prev_expr) = final_expr {
                                        statements.push(Statement::Expression(*prev_expr, true));
                                    }
                                    final_expr = Some(Box::new(parse_expr(pair.into_inner())));
                                }
                                _ => unreachable!("Unexpected rule in block: {:?}", pair.as_rule()),
                            }
                        }

                        Expr::Block {
                            statements,
                            ret: final_expr.unwrap_or_else(|| Box::new(Expr::Number(0.0))),
                        }
                    },
                    Rule::tuple => {
                        let inner = actual_atom.into_inner();
                        let exprs = inner.map(|p| parse_expr(p.into_inner())).collect();
                        Expr::Tuple(exprs)
                    }

                    Rule::length => {
                        let inner = actual_atom.into_inner();
                        let exp_inner = parse_expr(inner);
                        Expr::Length(Box::new(exp_inner))
                    }

                    _ => unreachable!("Unexpected atom: {:?}", actual_atom.as_rule()),
                };
                for suffix in inner {
                    match suffix.as_rule() {
                    Rule::dot_access => {
                        let inner = suffix.into_inner().next().unwrap();
                        let step = match inner.as_rule() {
                            Rule::ident => Access::Name(inner.as_str().to_string()),
                            Rule::tuple_index => Access::Index(inner.as_str().parse().unwrap()),
                            _ => unreachable!(),
                        };
                        current_expr = match current_expr {
                            Expr::MemberAccess(base, mut existing_path) => {
                                existing_path.push(step);
                                Expr::MemberAccess(base, existing_path)
                            }
                            _ => Expr::MemberAccess(Box::new(current_expr), vec![step]),
                        };
                    }
                        Rule::call_args => {
                            let args = suffix.into_inner()
                                .next()
                                .map(|al| al.into_inner().map(|e| parse_expr(e.into_inner())).collect())
                                .unwrap_or_default();

                            current_expr = Expr::Call {
                                callee: Box::new(current_expr),
                                args,
                            };
                        }
                        _ => unreachable!(),
                    }
                }
                current_expr
            }
            Rule::ternary => {
                let mut inner = primary.into_inner();
                let condition = parse_expr(inner.next().unwrap().into_inner());
                let true_branch = parse_expr(inner.next().unwrap().into_inner());
                let false_branch = parse_expr(inner.next().unwrap().into_inner());

                Expr::Ternary {
                    condition: Box::new(condition),
                    true_branch: Box::new(true_branch),
                    false_branch: Box::new(false_branch),
                }
            },
            Rule::expr | Rule::comparison | Rule::sum | Rule::term | Rule::factor => {
                parse_expr(primary.into_inner())
            }
            _ => unreachable!("{:?}", primary.as_rule()),
        })
        .map_infix(|lhs, op, rhs| {
            Expr::Binary {
                lhs: Box::new(lhs),
                op: op.as_str().to_string(),
                rhs: Box::new(rhs),
            }
        })
        .parse(pairs)
}

pub fn parse_statements(pairs: pest::iterators::Pairs<Rule>) -> Vec<Statement> {
    let mut statements = Vec::new();

    for line_pair in pairs {
        match parse_statement(line_pair) {
            Some(statement) => statements.push(statement),
            None => return statements
        }
    }
    statements
}
 
fn parse_statement(line_pair: Pair<Rule>) -> Option<Statement> {
    match line_pair.as_rule() {
        Rule::line => {
            let mut inner_rules = line_pair.into_inner();
            let main_part = inner_rules.next().unwrap();
            
            let is_suppressed = inner_rules.next()
                .map(|p| p.as_rule() == Rule::suppress)
                .unwrap_or(false);

            match main_part.as_rule() {
                Rule::assignment => {
                    let mut inner = main_part.into_inner();
                    let lvalue_pair = inner.next().unwrap();
                    let mut lvalue_inner = lvalue_pair.into_inner();
                    
                    let root_name = lvalue_inner.next().unwrap().as_str().to_string();
                    
                    let mut path = Vec::new();
                    for access_pair in lvalue_inner {
                        let inner_access = access_pair.into_inner().next().unwrap();
                        let step = match inner_access.as_rule() {
                            Rule::ident => Access::Name(inner_access.as_str().to_string()),
                            Rule::tuple_index => {
                                let idx = inner_access.as_str().parse().expect("Invalid tuple index");
                                Access::Index(idx)
                            },
                            _ => unreachable!(),
                        };
                        path.push(step);
                    }
                    
                    let expr = parse_expr(inner.next().unwrap().into_inner());
                    
                    return Some(Statement::Assignment(root_name, path, expr, is_suppressed));
                }
                Rule::export => {
                    let mut inner = main_part.into_inner();
                    let name = inner.next().unwrap().as_str().to_string();
                    return Some(Statement::Export(name));
                }
                Rule::import => {
                    let path = main_part.into_inner()
                        .next().unwrap()
                        .into_inner()
                        .next().unwrap()
                        .as_str()
                        .to_string();
                    return Some(Statement::Import(path));
                }
                Rule::module => {
                    let mut inner = main_part.into_inner();
                    let name = inner.next().unwrap().as_str().to_string();
                    // Recursively parse the body of the module
                    let body = parse_statements(inner); 
                    return Some(Statement::Module(name, body));
                }
                Rule::equivalence => {
                    let mut inner = main_part.into_inner();
                    let lhs = parse_expr(inner.next().unwrap().into_inner());
                    let rhs = parse_expr(inner.next().unwrap().into_inner());
                    return Some(Statement::Equivalence(lhs, rhs, is_suppressed));
                }
                Rule::expr => {
                    let exp = parse_expr(main_part.into_inner());
                    return Some(Statement::Expression(exp, is_suppressed));
                }
                _ => unreachable!(),
            }
        }
        Rule::EOI => None,
        _ => None,
    }
}

pub fn parse_math_file(file: pest::iterators::Pairs<Rule>) -> Result<Vec<Statement>, pest::error::Error<Rule>> {
    let inner = file.into_iter().next().unwrap().into_inner();
    Ok(parse_statements(inner))
}
