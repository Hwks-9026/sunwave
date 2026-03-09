mod parser;
mod eval;

use core::result::Result;
use std::ffi::{CStr, CString};
use libc::c_char;
use std::rc::Rc;
use std::cell::RefCell;
use pest::Parser;

use crate::eval::*;
use crate::parser::*;

// Wrap the Environment in an opaque struct for C++
pub struct SunwaveContext {
    env: Rc<RefCell<Environment>>,
}

#[unsafe(no_mangle)]
pub extern "C" fn sunwave_new_context() -> *mut SunwaveContext {
    Box::into_raw(Box::new(SunwaveContext {
        env: Rc::new(RefCell::new(Environment::new())),
    }))
}

#[unsafe(no_mangle)]
pub extern "C" fn sunwave_execute(ctx: *mut SunwaveContext, code: *const c_char) -> *mut c_char {
    let context = unsafe { &mut *ctx };
    let c_str = unsafe { CStr::from_ptr(code) };
    
    let code_str = c_str.to_str().unwrap_or("");
    
    match run_sunwave(code_str, &mut context.env) {
        Ok(()) => {
            let res = CString::new(format!("Ok.")).unwrap();
            res.into_raw()
        }
        Err(e) => {
            let err = CString::new(format!("Error: {}", e)).unwrap();
            err.into_raw()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sunwave_free_context(ctx: *mut SunwaveContext) {
    if !ctx.is_null() {
        unsafe { drop(Box::from_raw(ctx)) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sunwave_free_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe {
        // This takes ownership back and then drops it at the end of the block
        let _ = CString::from_raw(s);
    };
}

fn run_sunwave(code_str: &str, env: &mut Rc<RefCell<Environment>>) -> Result<(), String> {
    let pairs = match MathParser::parse(Rule::file, code_str) {
        Ok(pairs) => {pairs},
        Err(e) => return Err(e.to_string())
    };
    let statements = match parser::parse_math_file(pairs) {
        Ok(stmts) => {stmts},
        Err(e) => return Err(e.to_string())
    };
    eval::run_program(statements, env)?;
    Ok(())
}
