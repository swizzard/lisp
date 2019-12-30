use std::fmt;

use super::errors::ErrType;

pub type Res = Result<Val, ErrType>;

pub type Args = Vec<Val>;

pub type F = fn(Args) -> Res;


#[derive(Clone, Debug)]
pub enum Val {
    Int(i64),
    Char(char),
    Bool(bool),
    List(Vec<Val>),
    Symbol(String),
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
            Val::List(a) => {
                match other {
                    Val::List(b) => a == b,
                    _ => false
                }
            },
            Val::Symbol(a) => {
                match other {
                    Val::Symbol(b) => a == b,
                    _ => false
                }
            }
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
            Val::Symbol(a) => write!(f, "'{}", a),
            Val::List(a) => {
                let l = a.len() - 1;
                write!(f, "(")?;
                for v in a[0..l].iter() {
                    write!(f, "{}, ", v)?;
                }
                write!(f, "{})", a[l])?;
                Ok(())
            }
        }
    }
}

impl Val {
    const INT_TYPE: &'static str = "int";
    const CHAR_TYPE: &'static str = "char";
    const BOOL_TYPE: &'static str = "bool";
    const LIST_TYPE: &'static str = "list";
    const SYMBOL_TYPE: &'static str = "symbol";

    fn val_type(&self) -> &str {
        match self {
            Val::Int(_) => Val::INT_TYPE,
            Val::Char(_) => Val::CHAR_TYPE,
            Val::Bool(_) => Val::BOOL_TYPE,
            Val::List(_) => Val::LIST_TYPE,
            Val::Symbol(_) => Val::SYMBOL_TYPE,
        }
    }
    pub fn unwrap_int(&self) -> Result<&i64, ErrType> {
        match self {
            Val::Int(a) => Ok(a),
            _ => Err(ErrType::type_error(Val::INT_TYPE, self.val_type()))
        }
    }
    pub fn unwrap_char(&self) -> Result<&char, ErrType> {
        match self {
            Val::Char(a) => Ok(a),
            _ => Err(ErrType::type_error(Val::CHAR_TYPE,  self.val_type())),
        }
    }
    pub fn unwrap_bool(&self) -> Result<&bool, ErrType> {
        match self {
            Val::Bool(a) => Ok(a),
            _ => Err(ErrType::type_error(Val::BOOL_TYPE, self.val_type())),
        }
    }
    pub fn unwrap_list(&self) -> Result<&Vec<Val>, ErrType> {
        match self {
            Val::List(a) => Ok(a),
            _ => Err(ErrType::type_error(Val::LIST_TYPE, self.val_type())),
        }
    }
    pub fn unwrap_symbol(&self) -> Result<&str, ErrType> {
        match self {
            Val::Symbol(a) => Ok(&a),
            _ => Err(ErrType::type_error(Val::SYMBOL_TYPE, self.val_type())),
         }
     }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_symbol() {
        let s = Val::Symbol(String::from("a"));
        assert_eq!(String::from("'a"), s.to_string());
    }

    #[test]
    fn test_display_list() {
        let l = Val::List(vec![Val::Int(1), Val::Int(2), Val::Int(3)]);
        assert_eq!(String::from("(1, 2, 3)"), l.to_string());
    }
}
