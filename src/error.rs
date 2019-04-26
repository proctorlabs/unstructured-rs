use std::error;
use std::fmt::{self, Debug, Display};

use serde::de;
use serde::ser;

pub struct Error {
    err: Box<ErrorImpl>,
}

struct ErrorImpl {
    code: ErrorCode,
}

#[doc(hidden)]
pub enum ErrorCode {
    Message(Box<str>),
    KeyMustBeAString,
}

impl Error {
    #[doc(hidden)]
    #[cold]
    pub fn syntax(code: ErrorCode) -> Self {
        Error {
            err: Box::new(ErrorImpl { code }),
        }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCode::Message(msg) => f.write_str(msg),
            ErrorCode::KeyMustBeAString => f.write_str("key must be a string"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.err.code {
            _ => {
                // If you want a better message, use Display::fmt or to_string().
                "JSON error"
            }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.err.code {
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&*self.err, f)
    }
}

impl Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error({:?})", self.err.code.to_string())
    }
}

impl de::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Message(msg.to_string().into_boxed_str()),
            }),
        }
    }

    #[cold]
    fn invalid_type(unexp: de::Unexpected, exp: &de::Expected) -> Self {
        if let de::Unexpected::Unit = unexp {
            Error::custom(format_args!("invalid type: null, expected {}", exp))
        } else {
            Error::custom(format_args!("invalid type: {}, expected {}", unexp, exp))
        }
    }
}

impl ser::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Message(msg.to_string().into_boxed_str()),
            }),
        }
    }
}
