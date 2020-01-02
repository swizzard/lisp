use lisp::errors::ErrType;
use lisp::env::*;
use lisp::stdlib::core::core;
use lisp::types::*;


fn test() -> Res {
    let core = core();
    let mut p = Env::new(Some(&core));
    let a = Val::Int(1);
    let b = Val::Int(2);
    p.register("a", a);
    p.register("b", b);
    /* (defn f (c) (let (a 1 b 2) (+ a b c))) */
    let f = |args: Args| {
        let name = "+";
        let fu = p.lookup(name)?;
        let fu = fu.unwrap_func().ok_or(ErrType::not_a_function(name))?;
        let fu = fu.lookup(Arity::VarArgs).ok_or(ErrType::lookup(name))?;
        let a = p.lookup("a").and_then(|v| v.unwrap_val().ok_or(ErrType::lookup("a")))?;
        let b = p.lookup("b").and_then(|v| v.unwrap_val().ok_or(ErrType::lookup("b")))?;
        let c = args[0];
        Ok(fu(vec![*a, *b, c]))
    };
    let func_name = "f";
    p.register_func(func_name, Arity::SomeArgs(1), f);
    /* (f 3) */
    let v = Val::Int(3);
    let func = p.lookup(func_name)?;
    let func = func.unwrap_func().ok_or(ErrType::lookup(func_name))?;



}

fn main() {
    println!("Hello, world!");
}
