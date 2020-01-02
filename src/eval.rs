use crate::errors::ErrType;
use crate::env::*;
use crate::types::*;


pub fn eval_fn(env: &Env, f: &dyn Fn) -> F {
    
