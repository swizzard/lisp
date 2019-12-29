use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct TypeError {
    actual: String,
    expected: String,
}

impl Display for TypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Wrong type: expected {}, got {}",
               self.expected, self.actual)
    }
}

impl Error for TypeError {}

#[derive(Debug, PartialEq, Eq)]
pub struct LookupError {
    name: String
}

impl Display for LookupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} not found", self.name)
    }
}

impl Error for LookupError {}

#[derive(Debug, PartialEq, Eq)]
pub struct NotAFunctionError {
    name: String
}

impl Display for NotAFunctionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt:: Result {
        write!(f, "{} is not a function", self.name)
    }
}

impl Error for NotAFunctionError {}

#[derive(Debug, PartialEq, Eq)]
pub struct ArityMismatchError {
    name: String,
}

impl Display for ArityMismatchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Wrong number of args for {}", self.name)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrType {
    TypeError(TypeError),
    LookupError(LookupError),
    NotAFunctionError(NotAFunctionError),
    ArityMismatchError(ArityMismatchError),
}

impl Display for ErrType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrType::TypeError(e) => write!(f, "{}", e),
            ErrType::LookupError(e) => write!(f, "{}", e),
            ErrType::NotAFunctionError(e) => write!(f, "{}", e),
            ErrType::ArityMismatchError(e) => write!(f, "{}", e),
        }
    }
}

impl ErrType {
    pub fn type_error(expected: &str, actual: &str) -> ErrType {
        ErrType::TypeError(TypeError { expected: String::from(expected), actual: String::from(actual) })
    }
    pub fn lookup(name: &str) -> ErrType {
        ErrType::LookupError(LookupError { name: String::from(name) })
    }
    pub fn not_a_function(name: &str) -> ErrType {
        ErrType::NotAFunctionError(NotAFunctionError { name: String::from(name) })
    }
    pub fn arity_mismatch(name: &str) -> ErrType {
        ErrType::ArityMismatchError(ArityMismatchError { name: String::from(name) })
    }
}
