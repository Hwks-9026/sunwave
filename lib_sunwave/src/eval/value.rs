use crate::parser::Expr;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::environment::Environment;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Tuple(Vec<Value>),
    Function { args: Vec<String>, body: Expr, captured_env: Rc<RefCell<Environment>> },
    Module(HashMap<String, Value>),
    RecurSignal(Vec<Value>),
    None
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Tuple(a), Value::Tuple(b)) => a == b,
            _ => false, 
        }
    }
}

impl Value {
    pub fn is_equal_to(&self, other: &Value) -> bool {
        self == other
    }

    fn inner_str(&self) -> String {
        match self {
            Self::Number(n) => format!("{}", n),
            Self::Bool(b) => format!("{}", b),
            Self::String(s) => format!("\"{}\"", s),
            Self::Tuple(elements) => {
                let mut s = String::from("(");
                for (i, val) in elements.iter().enumerate() {
                    if i == elements.len() - 1 {
                        s.push_str(&format!("{}", val.inner_str()));
                    }
                    else {
                        s.push_str(&format!("{}, ", val.inner_str()));
                    }
                }
                s.push_str(")");
                s
            }
            #[allow(unused_variables)]
            Self::Function { args, body, captured_env } => {
                format!("Lambda({:?})", args)
            }
            Self::Module(values) => {
                format!("Module[len({})]", values.len())
            }
            Self::RecurSignal(values) => {
                format!("Recur[num_vars({})]", values.len())
            }
            Self::None => {
                "None".to_string()
            }
        }
    }

    pub fn format_tree(&self, name: &str) -> String {
        let mut out = String::new();
        self.write(name, &mut out, &mut Vec::new(), true, true);
        out
    }

    fn write(
        &self,
        name: &str,
        out: &mut String,
        prefix: &mut Vec<bool>,
        is_last: bool,
        is_root: bool,
    ) {
        // draw ancestor guides
        for &has_more in prefix.iter() {
            if has_more {
                out.push_str("│  ");
            } else {
                out.push_str("   ");
            }
        }

        // draw branch (not for root)
        if !is_root {
            if is_last {
                out.push_str("└─ ");
            } else {
                out.push_str("├─ ");
            }
        }

        match self {
            Value::Module(map) => {
                out.push_str(&format!("Module '{}'\n", name));

                let mut keys: Vec<_> = map.keys().collect();
                keys.sort();

                let len = keys.len();

                // extend prefix for children
                if !is_root {
                    prefix.push(!is_last);
                }

                for (i, key) in keys.iter().enumerate() {
                    let val = &map[*key];
                    let child_last = i == len - 1;

                    val.write(key, out, prefix, child_last, false);
                }

                if !is_root {
                    prefix.pop();
                }
            }

            Value::Number(n) => {
                out.push_str(&format!("[{}: {}]\n", name, n));
            }

            Value::Bool(b) => {
                out.push_str(&format!("[{}: {}]\n", name, b));
            }
            Value::String(s) => {
                out.push_str(&format!("[{}: {}]\n", name, s));
            }

            Value::Function { args, .. } => {
                out.push_str(&format!(
                    "[{}: Lambda({})]\n",
                    name,
                    args.join(", ")
                ));
            }

            Value::RecurSignal(_) => {
                out.push_str(&format!("[{}: RecurSignal]\n", name));
            }

            Value::Tuple(elements) => {
                let mut s = String::from("(");
                for (i, val) in elements.iter().enumerate() {
                    if i == elements.len() - 1 {
                        s.push_str(&format!("{}", val.inner_str()));
                    }
                    else {
                        s.push_str(&format!("{}, ", val.inner_str()));
                    }
                }
                s.push_str(")");
                out.push_str(&format!("{}", s))
            }
            Value::None => {
                out.push_str("None");
            }
        }
    }
} 
