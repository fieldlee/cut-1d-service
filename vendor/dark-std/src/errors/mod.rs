use std::fmt::{self, Debug, Display, Formatter};
use std::io::ErrorKind::UnexpectedEof;
use std::sync::mpsc::RecvError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Error {
    pub inner: String,
}

impl Error {
    pub fn error(&self) -> String {
        self.inner.clone()
    }

    pub fn warp<E>(e: E, info: &str) -> Self
    where
        E: std::fmt::Display,
    {
        Self {
            inner: format!("{}{}", info, e),
        }
    }

    pub fn to_string(&self) -> String {
        self.inner.clone()
    }
}

/// dark_std::errors::Error
#[macro_export]
macro_rules! err {
     ($($arg:tt)*) => {{
         $crate::errors::Error{
             inner: format!($($arg)*)
         }
     }}
}

///new error
#[inline]
pub fn new(text: String) -> Error {
    Error { inner: text }
}

pub trait FromError<T>: Sized {
    fn from_err(_: T) -> Error;
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        if err.kind().eq(&UnexpectedEof) {
            return err!("Eof");
        }
        if err.kind().eq(&std::io::ErrorKind::UnexpectedEof) {
            return err!("UnexpectedEof");
        }
        new(err.to_string())
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(arg: &str) -> Self {
        return new(arg.to_string());
    }
}

impl From<std::string::String> for Error {
    fn from(arg: String) -> Self {
        return new(arg);
    }
}

impl From<&dyn std::error::Error> for Error {
    fn from(arg: &dyn std::error::Error) -> Self {
        return new(arg.to_string());
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(arg: Box<dyn std::error::Error>) -> Self {
        return new(arg.to_string());
    }
}

impl From<&Box<dyn std::error::Error>> for Error {
    fn from(arg: &Box<dyn std::error::Error>) -> Self {
        return new(arg.to_string());
    }
}

impl From<std::sync::mpsc::RecvError> for Error {
    fn from(e: RecvError) -> Self {
        return new(e.to_string());
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for Error {
    fn from(e: std::sync::mpsc::SendError<T>) -> Self {
        return new(e.to_string());
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_error() {
        let e = err!("e");
        assert_eq!(e.to_string(), "e");
    }
}
