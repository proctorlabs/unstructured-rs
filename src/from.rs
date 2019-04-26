use std::borrow::Cow;

use crate::*;

macro_rules! from_integer {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Value {
                fn from(n: $ty) -> Self {
                    Value::Number(n.into())
                }
            }
        )*
    };
}

from_integer! {
    i8 i16 i32 i64 isize
    u8 u16 u32 u64 usize
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        From::from(f as f64)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Number::from_f64(f).map_or(Value::Null, Value::Number)
    }
}

impl From<bool> for Value {
    fn from(f: bool) -> Self {
        Value::Bool(f)
    }
}

impl From<String> for Value {
    fn from(f: String) -> Self {
        Value::Text(f)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(f: &str) -> Self {
        Value::Text(f.to_string())
    }
}

impl<'a> From<Cow<'a, str>> for Value {
    fn from(f: Cow<'a, str>) -> Self {
        Value::Text(f.into_owned())
    }
}

impl From<Map<String, Value>> for Value {
    fn from(f: Map<String, Value>) -> Self {
        Value::Map(f)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(f: Vec<T>) -> Self {
        Value::Array(f.into_iter().map(Into::into).collect())
    }
}

impl<'a, T: Clone + Into<Value>> From<&'a [T]> for Value {
    fn from(f: &'a [T]) -> Self {
        Value::Array(f.iter().cloned().map(Into::into).collect())
    }
}

impl<T: Into<Value>> ::std::iter::FromIterator<T> for Value {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Value::Array(iter.into_iter().map(Into::into).collect())
    }
}
