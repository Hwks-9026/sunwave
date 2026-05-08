use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs;

use pest::Parser;

use crate::parser::*;
use super::value::Value;
use super::environment::Environment;
use super::core::run_program;

pub fn process_import_file(filename: &str) -> Result<HashMap<String, Value>, String> {
    let content = fs::read_to_string(filename)
        .map_err(|_| format!("Could not find file: {}", filename))?;

    let pairs = MathParser::parse(Rule::file, &content)
        .map_err(|e| format!("Error parsing {}: {}", filename, e))?;
    
    let imported_stmts = parse_math_file(pairs)
        .map_err(|e| format!("AST Error in {}: {}", filename, e))?;

    let mut import_env = Rc::new(RefCell::new(Environment::new()));
    run_program(imported_stmts, &mut import_env)
}


pub fn inject_at_path(env: &Rc<RefCell<Environment>>, path_parts: &[String], exports: HashMap<String, Value>) {
    if path_parts.is_empty() { return; }

    let current_name = &path_parts[0];
    let mut env_borrow = env.borrow_mut();

    if path_parts.len() == 1 {
        env_borrow.variables.insert(current_name.clone(), Value::Module(exports));
    } else {
        let sub_mod_value = env_borrow.variables.entry(current_name.clone())
            .or_insert_with(|| Value::Module(HashMap::new()));

        if let Value::Module(sub_exports) = sub_mod_value {
            let sub_env = Rc::new(RefCell::new(Environment {
                variables: sub_exports.clone(),
                parent: None,
            }));

            inject_at_path(&sub_env, &path_parts[1..], exports);

            *sub_exports = sub_env.borrow().variables.clone();
        }
    }
}
