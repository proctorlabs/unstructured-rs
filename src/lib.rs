#![cfg_attr(feature = "cargo-clippy", allow(clippy::cast_lossless))]

#[macro_use]
extern crate serde;

#[cfg(test)]
mod test;

mod de;
mod error;
mod from;
mod index;
mod map;
mod number;
mod partial_eq;
mod ser;

pub use self::index::Index;
use self::ser::Serializer;
pub use error::Error;
pub use map::Map;
pub use number::Number;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::fmt::{self, Debug};
use std::mem;
use std::str;

#[derive(Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    Text(String),
    Array(Vec<Value>),
    Map(Map<String, Value>),
}

impl Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Null => formatter.debug_tuple("Null").finish(),
            Value::Bool(v) => formatter.debug_tuple("Bool").field(&v).finish(),
            Value::Number(ref v) => Debug::fmt(v, formatter),
            Value::Text(ref v) => formatter.debug_tuple("Text").field(v).finish(),
            Value::Array(ref v) => formatter.debug_tuple("Array").field(v).finish(),
            Value::Map(ref v) => formatter.debug_tuple("Object").field(v).finish(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Null => f.debug_tuple("Null").finish(),
            Value::Bool(v) => f.debug_tuple("Bool").field(&v).finish(),
            Value::Number(ref v) => Debug::fmt(v, f),
            Value::Text(ref v) => f.debug_tuple("Text").field(v).finish(),
            Value::Array(ref v) => f.debug_tuple("Array").field(v).finish(),
            Value::Map(ref v) => f.debug_tuple("Object").field(v).finish(),
        }
    }
}

fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}

impl Value {
    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }

    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {
        index.index_into_mut(self)
    }

    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    pub fn as_object(&self) -> Option<&Map<String, Value>> {
        match *self {
            Value::Map(ref map) => Some(map),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut Map<String, Value>> {
        match *self {
            Value::Map(ref mut map) => Some(map),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::Array(ref array) => Some(&*array),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match *self {
            Value::Array(ref mut list) => Some(list),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::Text(ref s) => Some(s),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_i64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_i64(),
            _ => false,
        }
    }

    pub fn is_u64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_u64(),
            _ => false,
        }
    }

    pub fn is_f64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_f64(),
            _ => false,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::Number(ref n) => n.as_i64(),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::Number(ref n) => n.as_u64(),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::Number(ref n) => n.as_f64(),
            _ => None,
        }
    }

    pub fn is_boolean(&self) -> bool {
        self.as_bool().is_some()
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    pub fn as_null(&self) -> Option<()> {
        match *self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    pub fn pointer<'a>(&'a self, pointer: &str) -> Option<&'a Value> {
        if pointer == "" {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        let tokens = pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"));
        let mut target = self;

        for token in tokens {
            let target_opt = match *target {
                Value::Map(ref map) => map.get(&token),
                Value::Array(ref list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => return None,
            };
            if let Some(t) = target_opt {
                target = t;
            } else {
                return None;
            }
        }
        Some(target)
    }

    pub fn pointer_mut<'a>(&'a mut self, pointer: &str) -> Option<&'a mut Value> {
        if pointer == "" {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        let tokens = pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"));
        let mut target = self;

        for token in tokens {
            // borrow checker gets confused about `target` being mutably borrowed too many times because of the loop
            // this once-per-loop binding makes the scope clearer and circumvents the error
            let target_once = target;
            let target_opt = match *target_once {
                Value::Map(ref mut map) => map.get_mut(&token),
                Value::Array(ref mut list) => {
                    parse_index(&token).and_then(move |x| list.get_mut(x))
                }
                _ => return None,
            };
            if let Some(t) = target_opt {
                target = t;
            } else {
                return None;
            }
        }
        Some(target)
    }

    pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
    }

    pub fn try_into<T>(self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        T::deserialize(self)
    }

    pub fn try_from<T>(value: T) -> Result<Value, Error>
    where
        T: Serialize,
    {
        value.serialize(Serializer)
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::Null
    }
}
