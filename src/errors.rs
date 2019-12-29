use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct TypeError<'a> {
    name: &'a str,
    actual: &'a str,
    expected: &'a str
}

impl<'a> Display for TypeError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Wrong type for {}: expected {}, got {}",
               self.name, self.expected, self.actual)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LookupError<'a> {
    name: &'a str
}

impl<'a> Display for LookupError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} not found", self.name)
    }
}

impl<'a> Error for LookupError<'a> {}

#[derive(Debug, PartialEq, Eq)]
pub struct NotAFunctionError<'a> {
    name: &'a str
}

impl<'a> Display for NotAFunctionError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt:: Result {
        write!(f, "{} is not a function", self.name)
    }
}

impl<'a> Error for NotAFunctionError<'a> {}

#[derive(Debug, PartialEq, Eq)]
pub struct ArityMismatchError<'a> {
    name: &'a str,
}

impl<'a> Display for ArityMismatchError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Wrong number of args for {}", self.name)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrType<'a> {
    TypeError(TypeError<'a>),
    LookupError(LookupError<'a>),
    NotAFunctionError(NotAFunctionError<'a>),
    ArityMismatchError(ArityMismatchError<'a>),
}

impl<'a> Display for ErrType<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrType::TypeError(e) => write!(f, "{}", e),
            ErrType::LookupError(e) => write!(f, "{}", e),
            ErrType::NotAFunctionError(e) => write!(f, "{}", e),
            ErrType::ArityMismatchError(e) => write!(f, "{}", e),
        }
    }
}

impl<'a> ErrType<'a> {
    pub fn type_error(name: &'a str, expected: &'a str, actual: &'a str) -> ErrType<'a> {
        ErrType::TypeError(TypeError { name, expected, actual })
    }
    pub fn lookup(name: &'a str) -> ErrType {
        ErrType::LookupError(LookupError { name })
    }
    pub fn not_a_function(name: &'a str) -> ErrType {
        ErrType::NotAFunctionError(NotAFunctionError { name })
    }
    pub fn arity_mismatch(name: &'a str) -> ErrType {
        ErrType::ArityMismatchError(ArityMismatchError { name })
    }
}
