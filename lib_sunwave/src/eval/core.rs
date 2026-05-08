use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;

use crate::parser::*;
use super::value::Value;
use super::environment::Environment;
use super::imports::*;

fn apply_binary_op(left: Value, op: &str, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => match op {
            "+" => Ok(Value::Number(a + b)),
            "-" => Ok(Value::Number(a - b)),
            "*" => Ok(Value::Number(a * b)),
            "/" => Ok(Value::Number(a / b)),
            "^" => Ok(Value::Number(a.powf(b))),
            "==" => Ok(Value::Bool(a == b)),
            "!=" => Ok(Value::Bool(a != b)),
            ">" => Ok(Value::Bool(a > b)),
            "<" => Ok(Value::Bool(a < b)),
            ">=" => Ok(Value::Bool(a >= b)),
            "<=" => Ok(Value::Bool(a <= b)),
            _ => Err(format!("Unknown operator: {}", op)),
        },
        (Value::String(a), Value::String(b)) => match op {
            "==" => Ok(Value::Bool(a == b)),
            "!=" => Ok(Value::Bool(a != b)),
            "+" => Ok(Value::String(a + &b)),
            _ => Err("Cannot perform operation on strings".to_string()),
        }
        (Value::Bool(a), Value::Bool(b)) => match op {
            "==" => Ok(Value::Bool(a == b)),
            "!=" => Ok(Value::Bool(a != b)),
            "|" => Ok(Value::Bool(a || b)),
            _ => Err("Cannot perform operation on booleans".to_string()),
        },
        (Value::Tuple(mut t), Value::Number(n)) => match op {
            "-" => Ok(Value::Tuple({t.remove(n as usize); t})),
            e => Err(format!("Cannot perform operation {} on a tuple and number.", e)),

        }
        (Value::Tuple(mut a), Value::Tuple(mut b)) => match op {
            "==" => Ok(Value::Bool(a == b)),
            "!=" => Ok(Value::Bool(a != b)),
            "+" => Ok(Value::Tuple({a.append(&mut b); a})),
            _ => Err(format!("Cannot perform operation {} on a tuples.", op)),
        },
        (Value::Tuple(mut t), v) => match op {
            "+" => Ok(Value::Tuple({t.push(v); t})),
            _ => Err("Cannot perform operation on a tuple and value.".to_string()),

        },
        (Value::None, Value::None) => match op {
            "==" => Ok(Value::Bool(true)),
            "!=" => Ok(Value::Bool(false)),
            ">" => Ok(Value::Bool(false)),
            "<" => Ok(Value::Bool(false)),
            ">=" => Ok(Value::Bool(true)),
            "<=" => Ok(Value::Bool(true)),
            _ => Err("Cannot perform operation on two 'None' values.".to_string()),
        }
        _ => Err("Type mismatch in binary operation".to_string()),
    }
}

fn eval_expr(expr: &Expr, env: &mut Rc<RefCell<Environment>>) -> Result<Value, String> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Variable(name) => {
            if name == "true" { return Ok(Value::Bool(true)); }
            if name == "false" { return Ok(Value::Bool(false)); }
            if name == "none" { return Ok(Value::None);}
            
            env.borrow().get(name)
                .ok_or(format!("Undefined variable: {}", name))
        }

        Expr::Binary { lhs, op, rhs } => {
            let left = eval_expr(lhs, &mut Rc::clone(&env))?;
            let right = eval_expr(rhs, &mut Rc::clone(&env))?;
            apply_binary_op(left, op, right)
        }

        Expr::Lambda { args, body } => Ok(Value::Function {
            args: args.clone(),
            body: *body.clone(),
            captured_env: Rc::clone(&env), // Capture current scope
        }),

        Expr::Call { callee, args } => {
            let func_val = eval_expr(callee, env)?;

            if let Value::Function { args: arg_names, body, captured_env } = func_val {
                let evaled_args: Vec<Result<Value, String>> = args.iter()
                    .map(|a| eval_expr(a, &mut Rc::clone(&env)))
                    .collect();

                // CREATE A NEW SCOPE
                let mut local_env = Rc::new(RefCell::new(Environment {
                    variables: HashMap::new(),
                    parent: Some(Rc::clone(&captured_env)), // Parent is the captured env
                }));

                // Bind arguments to the local scope
                for (name, val) in arg_names.iter().zip(evaled_args) {
                    local_env.borrow_mut().variables.insert(name.clone(), val?);
                }

                eval_expr(&body, &mut local_env)
            } else {
                Err(format!("{:?} is not a function", callee))
            }
        }
        Expr::Ternary { condition, true_branch, false_branch } => {
            let condition = eval_expr(condition, env)?;
            let condition_is_true: bool = match condition {
                #[allow(unused_variables)]
                Value::Function{args, body, captured_env} => {return Err("Cannot evaluate truthyness of an anonymous function".to_string());},
                Value::Number(n) => {if n == 0.0 {false} else {true}},
                Value::Bool(b) => b,
                Value::None => false,
                Value::String(_) => {return Err("Cannot evaluate truthyness of a String".to_string());}
                Value::Module(_) => {return Err("Cannot evaluate truthyness of a Module".to_string());}
                Value::RecurSignal(_) => {return Err("Cannot evaluate truthyness of a Recur Signal".to_string());}
                Value::Tuple(_) => {return Err("Cannot evaluate truthyness of a Tuple".to_string());} // TODO: Element-Wise comparison

            };
            if condition_is_true {
                return eval_expr(true_branch, env);
            }
            else {
                return eval_expr(false_branch, env);
            }
            
        }
        Expr::MemberAccess(base, path) => {
            let mut current_val = eval_expr(base, env)?;
            
            for step in path {
                current_val = match (current_val, step) {
                    (Value::Module(map), Access::Name(name)) => {
                        map.get(name).cloned().ok_or(format!("No member {}", name))?
                    }
                    (Value::Tuple(elements), Access::Index(idx)) => {
                        elements.get(*idx).cloned().ok_or(format!("Index {} out of bounds", idx))?
                    }
                    (Value::Tuple(elements), Access::Name(name)) => {
                        let value = env.borrow().get(name).clone().ok_or(format!("No identifier {}", name))?;
                        if let Value::Number(n) = value {
                            return elements.get(n as usize).cloned().ok_or(format!("Index {} out of bounds", (n as usize)))
                        }
                        return Err(format!("Identifier {} is not a number", name));

                    }
                    (a, b) => return Err(format!("Invalid member access sequence, {:?}, {:?}", &a, &b))
                };
            }
            
            Ok(current_val)
        }
        Expr::Tuple(contents) => {
            let mut values = Vec::new();
            for c in contents {
                values.push(eval_expr(c, env)?)
            }
            if values.len() == 1 {
                Ok(values[0].clone())
            }
            else {
                Ok(Value::Tuple(values))
            }
        }
        Expr::Recur(args) => {
            let mut evaled = Vec::new();
            for arg in args {
                evaled.push(eval_expr(arg, env)?);
            }
            // We don't return a result, we return a SIGNAL to the nearest loop
            Ok(Value::RecurSignal(evaled))
        }

        Expr::Loop { bindings, body } => {
            // 1. Setup the loop environment
            let mut current_env = Rc::new(RefCell::new(Environment {
                variables: HashMap::new(),
                parent: Some(Rc::clone(env)),
            }));

            // 2. Initial Bindings
            let mut arg_names = Vec::new();
            for (name, init_expr) in bindings {
                let val = eval_expr(init_expr, env)?;
                current_env.borrow_mut().variables.insert(name.clone(), val);
                arg_names.push(name.clone());
            }

            loop {
                match eval_expr(body, &mut current_env) {
                    Ok(Value::RecurSignal(new_vals)) => {
                        if new_vals.len() != arg_names.len() {
                            return Err(format!("recur expected {} args, got {}", arg_names.len(), new_vals.len()));
                        }
                        for (name, val) in arg_names.iter().zip(new_vals) {
                            current_env.borrow_mut().variables.insert(name.clone(), val);
                        }
                    }
                    Ok(final_val) => return Ok(final_val), // Normal exit
                    Err(e) => return Err(e),
                }
            }
        }
        Expr::Block { statements, ret } => {
            let mut block_env = Rc::new(RefCell::new(Environment {
                variables: HashMap::new(),
                parent: Some(Rc::clone(env)),
            }));

            run_program(statements.clone(), &mut block_env)?;

            eval_expr(ret, &mut block_env) 
        }
        Expr::Length(inner) => {
            let val = eval_expr(inner, env)?;
            match val {
                Value::Tuple(v) => Ok(Value::Number(v.len() as f64)),
                Value::Module(m) => Ok(Value::Number(m.len() as f64)),
                Value::String(s) => Ok(Value::Number(s.len() as f64)),
                Value::Function { args, ..} => Ok(Value::Number(args.len() as f64)),
                e => Err(format!("Length operator '#' only works on Tuples, Modules, and Lambdas, not {:?}", e)),
            }
        }
    }
}

pub fn run_program(statements: Vec<Statement>, env: &mut Rc<RefCell<Environment>>) -> Result<HashMap<String,Value>, String> {
    let mut exports = HashMap::new();
    for stmt in statements {
        match stmt {
            Statement::Assignment(name, path, expr, supressed) => {
                let val = eval_expr(&expr, env)?;
                let root_name = name.clone();
                let path = path.clone();
                if path.is_empty() {
                    env.borrow_mut().variables.insert(name.clone(), val);
                }
                else {
                    let mut root_val = env.borrow().get(&root_name)
                        .ok_or(format!("Variable {} not found", root_name))?;

                    update_nested_value(&env, &mut root_val, &path, val)?;

                    env.borrow_mut().variables.insert(root_name, root_val);
                }
                if !supressed {
                    let fmtd = &env.borrow().variables.get(&name).unwrap().format_tree(&name);
                    println!("{} assigned to {}",&name, fmtd);
                }
            }
            Statement::Equivalence(lhs, rhs, supressed) => {
                let left_val = eval_expr(&lhs, env)?;
                let right_val = eval_expr(&rhs, env)?;
                let not = if !left_val.is_equal_to(&right_val) {"not "} else {""};
                if !supressed {
                    let l_label = match &lhs {
                        Expr::Variable(name) => name.as_str(),
                        #[allow(unused_variables)]
                        Expr::MemberAccess(base, parts) => &parts.last().unwrap().to_string(),
                        _ => "Result", // Fallback for raw numbers/lambdas
                    };
                    let r_label = match &rhs {
                        Expr::Variable(name) => name.as_str(),
                        #[allow(unused_variables)]
                        Expr::MemberAccess(base, parts) => &parts.last().unwrap().to_string(),
                        _ => "Result", // Fallback for raw numbers/lambdas
                    };
                    println!("{} is {}equal to {}", left_val.format_tree(l_label), not, right_val.format_tree(r_label));
                }
            }
            Statement::Expression(expr, supressed) => {
                let val = eval_expr(&expr, env)?;
                if !supressed {
                    let label = match &expr {
                        Expr::Variable(name) => name.as_str(),
                        #[allow(unused_variables)]
                        Expr::MemberAccess(base, parts) => &parts.last().unwrap().to_string(),
                        _ => "Result", // Fallback for raw numbers/lambdas
                    };
                    println!("{}", val.format_tree(label) );
                }

            }
            Statement::Module(name, body) => {
                let mut mod_env = Rc::new(RefCell::new(Environment {
                    variables: HashMap::new(),
                    parent: Some(Rc::clone(env))
                }));

                let mod_exports = run_program(body, &mut mod_env)?;
                let mod_value = Value::Module(mod_exports);

                env.borrow_mut().variables.insert(name.clone(), mod_value);
            }
            Statement::Export(name) => {
                let val = env.borrow().get(&name)
                    .ok_or_else(|| format!("Export Error: Cannot export undefined variable '{}'", name))?;
                exports.insert(name.clone(), val);
            }
            Statement::Import(raw_path) => {
                use std::path::Path;
                let base_path = Path::new(&raw_path);
                let raw_dot_sw_path = format!("{}.sw", raw_path);
                let dot_sw_path = Path::new(&raw_dot_sw_path);

                if base_path.is_dir() {
                    let entries = fs::read_dir(base_path)
                        .map_err(|_| format!("Could not read directory: {}", raw_path))?;

                    for entry in entries {
                        let entry = entry.map_err(|_| "Error reading entry".to_string())?;
                        let entry_path = entry.path();

                        if entry_path.is_file() && entry_path.extension().map_or(false, |ext| ext == "sw") {
                            let stem = entry_path.file_stem().unwrap().to_str().unwrap();
                            
                            let mut path_parts: Vec<String> = raw_path.split('/')
                                .filter(|s| !s.is_empty() && *s != ".")
                                .map(|s| s.to_string()).collect();
                            path_parts.push(stem.to_string());

                            let file_exports = process_import_file(entry_path.to_str().unwrap())?;
                            inject_at_path(env, &path_parts, file_exports);
                        }
                    }
                } else if dot_sw_path.exists() {
                    let file_exports = process_import_file(dot_sw_path.to_str().unwrap())?;
                    
                    let path_parts: Vec<String> = raw_path.split('/')
                        .filter(|s| !s.is_empty() && *s != ".")
                        .map(|s| s.to_string()).collect();
                        
                    inject_at_path(env, &path_parts, file_exports);
                } else if base_path.is_file() {
                    let file_exports = process_import_file(raw_path.as_str())? ;
                    
                    let name_without_ext = base_path.file_stem().unwrap().to_str().unwrap();
                    let mut path_parts: Vec<String> = raw_path.split('/')
                        .filter(|s| !s.is_empty() && *s != ".")
                        .map(|s| s.to_string()).collect();
                    if let Some(last) = path_parts.last_mut() {
                        *last = name_without_ext.to_string();
                    }

                    inject_at_path(env, &path_parts, file_exports);
                } else {
                    return Err(format!("Could not find directory or file for: {}", raw_path));
                }
            }
        }
    }
    Ok(exports)
}

fn update_nested_value(env: &Rc<RefCell<Environment>>, current: &mut Value, path: &[Access], new_val: Value) -> Result<(), String> {
    if path.is_empty() {
        *current = new_val;
        return Ok(());
    }

    let (first, rest) = path.split_first().unwrap();
    match (current, first) {
        (Value::Tuple(vec), Access::Index(idx)) => {
            let target = vec.get_mut(*idx).ok_or("Index out of bounds")?;
            update_nested_value(env, target, rest, new_val)
        }
        (Value::Module(map), Access::Name(name)) => {
            let target = map.get_mut(name).ok_or("Member not found")?;
            update_nested_value(env, target, rest, new_val)
        }
        (Value::Tuple(vec), Access::Name(name)) => {
            let env_borrow = env.borrow();
            dbg!(&env_borrow);
            let val: &Value = env_borrow.variables.get(name).ok_or("Access variable not in scope.")?;
            let idx: usize = match val {
                Value::Number(idx) => {*idx as usize},
                _ => {return Err("Invalid variable type for tuple access".to_string())}
            };
            let target = vec.get_mut(idx).ok_or("Index out of bounds")?;
            update_nested_value(env, target, rest, new_val)

        }
        (_, f) => {
            dbg!(f);
            Err("Invalid path for assignment".into())
        },
    }
}
