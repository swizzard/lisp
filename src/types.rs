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
