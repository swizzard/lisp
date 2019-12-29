use std::collections::HashMap;
use super::types::{F, Val};
use super::errors::ErrType;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Arity {
    NoArgs,
    SomeArgs(u8),
    VarArgs
}

#[derive(Debug)]
struct Func(HashMap<Arity, F>);

impl Func {
    pub fn lookup(&self, arity: Arity) -> Option<&F> {
        self.0.get(&arity).or_else(|| self.0.get(&Arity::VarArgs))
    }
    pub fn new(arity: Arity, f: F) -> Self {
        let mut m = HashMap::new();
        m.insert(arity, f);
        Self(m)
    }
    pub fn add_arity(&mut self, f: F, arity: Arity) {
        self.0.insert(arity, f);
    }
}

#[derive(Debug)]
enum Entry {
    Val(Val),
    Func(Func),
}

pub struct Scope<'a>(HashMap<&'a str, Entry>);

pub struct Env<'a> {
    scope: Scope<'a>,
    parent: Option<&'a Env<'a>>,
}

impl<'a, 'b> Env<'a> {
    pub fn register(&mut self, name: &'a str, v: Val) {
        self.scope.0.insert(name, Entry::Val(v));
    }
    pub fn register_func(&mut self, name: &'a str, arity: Arity, f: F) -> Option<ErrType> {
        if let Some(ref mut existing) = self.scope.0.get_mut(name) {

    pub fn register_func(&mut self, name: &'a str, f: Func) -> Option<ErrType> {
        if let Some(ref mut existing) = self.scope.0.get_mut(name) {
            if existing.add_arity(f) {
                None
            } else {
                Some(ErrType::not_a_function(name))
            }
        } else {
            self.scope.0.insert(name, Entry::Func(f));
            None
        }
    }
    pub fn lookup(&'a self, name: &'a str) -> Result<&'a Entry, ErrType> {
        if let Some(v) = self.scope.0.get(name) {
            Ok(v)
        } else {
            self.parent.map_or(Err(ErrType::lookup(name)), |p| p.lookup(name))
        }
    }
    pub fn new(parent: Option<&'a Env<'a>>) -> Env<'a> {
        Env {
            scope: Scope(HashMap::new()),
            parent
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::ErrType;
    use crate::types::*;

    #[test]
    fn test_lookup() {
        let mut m = HashMap::new();
        m.insert("a", Val::Int(0));
        let e = Env {
            parent: None,
            scope: Scope(m)
        };
        if let Ok(v) = e.lookup("a") {
            assert_eq!(*v, Val::Int(0));
        } else {
            panic!("test_lookup");
        }
    }

    #[test]
    fn test_lookup_parent() {
        let mut p_m = HashMap::new();
        p_m.insert("a", Val::Int(0));
        let m = HashMap::new();
        let parent = Env {
            parent: None,
            scope: p_m,
        };
        let e = Env {
            parent: Some(&parent),
            scope: m
        };
        if let Ok(v) = e.lookup("a") {
            assert_eq!(*v, Val::Int(0));
        } else {
            panic!("test_lookup_parent");
        }
    }

    #[test]
    fn test_shadow() {
        let mut p_m = HashMap::new();
        p_m.insert("a", Val::Int(0));
        let mut m = HashMap::new();
        m.insert("a", Val::Int(1));
        let parent = Env {
            parent: None,
            scope: p_m,
        };
        let e = Env {
            parent: Some(&parent),
            scope: m
        };
        if let Ok(v) = e.lookup("a") {
            assert_eq!(*v, Val::Int(1));
        } else {
            panic!("test_shadow");
        }
    }

    #[test]
    fn test_lookup_error() {
        let mut m = HashMap::new();
        let a = "a";
        m.insert(a, Val::Int(0));
        let e = Env {
            parent: None,
            scope: m
        };
        let expected_err = ErrType::lookup("b");
        if let Err(err) = e.lookup("b") {
            assert_eq!(err, expected_err);
        } else {
            panic!("test_lookup_error");
        }
    }

    #[test]
    fn test_lookup_parent_error() {
        let mut p_m = HashMap::new();
        let a = "a";
        p_m.insert(a, Val::Int(0));
        let mut m = HashMap::new();
        let b = "b";
        m.insert(b, Val::Int(1));
        let parent = Env {
            parent: None,
            scope: p_m,
        };
        let e = Env {
            parent: Some(&parent),
            scope: m
        };
        let expected_err = ErrType::lookup("c");
        if let Err(err) = e.lookup("c") {
            assert_eq!(err, expected_err);
        } else {
            panic!("test_lookup_parent_error");
        }
    }
}
