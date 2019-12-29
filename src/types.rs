use std::collections::HashMap;
use std::fmt;


pub type F = fn(Vec<Val>) -> Val;


#[derive(Debug)]
pub enum Val {
    Int(i64),
    Char(char),
    Bool(bool),
}

impl PartialEq for Val {
    fn eq(&self, other: &Val) -> bool {
        match self {
            Val::Int(a) => {
                match other {
                    Val::Int(b) => a == b,
                    _ => false
                }
            },
            Val::Char(a) => {
                match other {
                    Val::Char(b) => a == b,
                    _ => false
                }
            },
            Val::Bool(a) => {
                match other {
                    Val::Bool(b) => a == b,
                    _ => false
                }
            },
        }
    }
}

impl Eq for Val {}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::Int(a) => write!(f, "{}", a),
            Val::Char(a) => write!(f, "{}", a),
            Val::Bool(a) => write!(f, "{}", a),
        }
    }
}

impl Val {
    pub fn unwrap_int(&self) -> Option<&i64> {
        match self {
            Val::Int(a) => Some(a),
            _ => None
        }
    }
    pub fn unwrap_char(&self) -> Option<&char> {
        match self {
            Val::Char(a) => Some(a),
            _ => None
        }
    }
    pub fn unwrap_bool(&self) -> Option<&bool> {
        match self {
            Val::Bool(a) => Some(a),
            _ => None
        }
    }
}


#[cfg(tests)]
mod test_val {
    use super::*;

    #[test]
    fn test_is_func() {
        let v = Val::Int(1);
        assert!(!v.is_func());
        let v2 = Val::Func(Func::new(NoArgs, || 0));
        assert!(v2.is_func());
    }

    #[test]
    fn test_add_arity() {
        assert!(!Val::Int(1).add_arity(NoArgs, || 0));
        let v = Val::new_func(NoArgs, || 0);
        assert!(v.add_arity(VarArgs, |args| args.iter().fold(0, |acc, x| acc + x)));
    }
}

#[cfg(test)]
mod test_func {
    use super::*;

    #[test]
    fn test_get_f() -> Result<(), String> {
        let v = Val::new_func(
            Arity::SomeArgs(2),
            |args| Val::Int(
                args[0].unwrap_int().unwrap() + args[1].unwrap_int().unwrap())
            ).unwrap_func().unwrap();
        assert!(v.get_f(Arity::SomeArgs(2)).is_some());
        Ok(())
    }

    #[test]
    fn test_get_f_varargs() {
        let v = Val::new_func(
            Arity::VarArgs,
            |args| args.iter().fold(Val::Int(0), |acc, x| Val::Int(
                    acc.unwrap_int().unwrap() + x.unwrap_int().unwrap()))
            ).unwrap_func().unwrap();
        assert!(v.get_f(Arity::SomeArgs(2)).is_some());
    }

    #[test]
    fn test_add_arity() {
        let f = Val::new_func(Arity::NoArgs, |_| Val::Int(0));
        let v = f.unwrap_func().unwrap();
        v.add_arity(
            Arity::SomeArgs(2),
            |args| Val::Int(args[0].unwrap_int().unwrap() + args[1].unwrap_int().unwrap())
        );
        assert!(v.get_f(Arity::SomeArgs(2)).is_some());
    }
}
