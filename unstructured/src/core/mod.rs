mod convert;
pub(crate) mod de;
mod from;
mod index;
pub(crate) mod ser;
mod cmp;

use std::mem;
use std::collections::BTreeMap;
use de::*;
use ser::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use crate::Number;

pub use convert::*;


#[derive(Debug, Clone)]
pub struct UnstructuredType;

impl UnstructuredDataTrait for UnstructuredType {
    type ErrorType = UnstructuredError;
    type OtherType = DefaultOther;
}

pub type Document = Unstructured<UnstructuredType>;

#[derive(Clone, Debug)]
pub struct DefaultOther;

impl std::fmt::Display for DefaultOther {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum UnstructuredError {
    Serializer,
    Deserializer,
}

impl std::error::Error for UnstructuredError {}

impl std::fmt::Display for UnstructuredError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait UnstructuredDataTrait: Clone {
    type ErrorType: std::error::Error + Clone + Send + Sync;
    type OtherType: std::fmt::Display + Clone + Send + Sync;
}

pub type Sequence<T> = Vec<Unstructured<T>>;
pub type Mapping<T> = BTreeMap<Unstructured<T>, Unstructured<T>>;

#[derive(Clone, Debug)]
pub enum Unstructured<T: UnstructuredDataTrait>
{
    Unassigned,
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Char(char),
    Bytes(Vec<u8>),
    Seq(Sequence<T>),
    Map(Mapping<T>),
    Option(Option<Box<Unstructured<T>>>),
    Newtype(Box<Unstructured<T>>),
    Err(T::ErrorType),
    Other(T::OtherType),
}

impl<T: UnstructuredDataTrait> Hash for Unstructured<T> {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.discriminant().hash(hasher);
        match *self {
            Self::Bool(v) => v.hash(hasher),
            Self::Number(ref n) => n.hash(hasher),
            Self::Char(v) => v.hash(hasher),
            Self::String(ref v) => v.hash(hasher),
            Self::Null => ().hash(hasher),
            Self::Option(ref v) => v.hash(hasher),
            Self::Newtype(ref v) => v.hash(hasher),
            Self::Seq(ref v) => v.hash(hasher),
            Self::Map(ref v) => v.hash(hasher),
            Self::Bytes(ref v) => v.hash(hasher),
            Self::Unassigned => ().hash(hasher),
            Self::Err(ref e) => format!("{}", e).hash(hasher),
            Self::Other(..) => 100.hash(hasher),
        }
    }
}

impl<T: UnstructuredDataTrait> PartialOrd for Unstructured<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<T: UnstructuredDataTrait> Default for Unstructured<T> {
    fn default() -> Self {
        Self::Unassigned
    }
}

impl<T: UnstructuredDataTrait> std::ops::Add<Unstructured<T>> for Unstructured<T>
{
    type Output = Unstructured<T>;

    fn add(mut self, rhs: Unstructured<T>) -> Unstructured<T> {
        self.merge(rhs);
        self
    }
}

impl<T: UnstructuredDataTrait> Ord for Unstructured<T> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (&Self::Bool(v0), &Self::Bool(ref v1)) => v0.cmp(v1),
            (&Self::Number(ref v0), &Self::Number(ref v1)) => v0.cmp(v1),
            (&Self::Char(v0), &Self::Char(ref v1)) => v0.cmp(v1),
            (&Self::String(ref v0), &Self::String(ref v1)) => v0.cmp(v1),
            (&Self::Null, &Self::Null) => Ordering::Equal,
            (&Self::Option(ref v0), &Self::Option(ref v1)) => v0.cmp(v1),
            (&Self::Newtype(ref v0), &Self::Newtype(ref v1)) => v0.cmp(v1),
            (&Self::Seq(ref v0), &Self::Seq(ref v1)) => v0.cmp(v1),
            (&Self::Map(ref v0), &Self::Map(ref v1)) => v0.cmp(v1),
            (&Self::Bytes(ref v0), &Self::Bytes(ref v1)) => v0.cmp(v1),
            (ref v0, ref v1) => v0.discriminant().cmp(&v1.discriminant()),
        }
    }
}

impl<T: UnstructuredDataTrait> Unstructured<T> {
    pub fn get_path(&self, path: &[&Self]) -> &Self
    where
        Self: index::Index<T>,
    {
        let mut temp = self;
        for p in path.iter() {
            temp = &temp[p];
        }
        temp
    }

    pub fn set_path<U: Into<Self>>(&mut self, val: U, path: &[&Self])
    where
        Self: index::Index<T>,
    {
        let mut temp = self;
        for p in path.iter() {
            temp = &mut temp[p];
        }
        *temp = val.into();
    }

    pub fn set<U: Into<Self>>(&mut self, val: U) {
        *self = val.into();
    }

    pub fn replace<U: Into<Self>>(&mut self, new_val: U) -> Self {
        mem::replace(self, new_val.into())
    }

    pub fn take(&mut self) -> Self {
        mem::replace(self, Self::Unassigned)
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    /// Returns true if the value is any signed integer (i8, i16, i32, i64)
    pub fn is_signed(&self) -> bool {
        matches!(self, Self::Number(n) if n.is_signed())
    }

    /// Returns true if the value is any unsigned integer (u8, u16, u32, u64)
    pub fn is_unsigned(&self) -> bool {
        matches!(self, Self::Number(n) if n.is_unsigned())
    }

    /// Returns true if the value is any float (f32, f64)
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Number(n) if n.is_float())
    }

    pub fn as_usize(&self) -> Option<usize>
    {
        match self {
            Self::Number(n) => Self::from(n.clone()).cast::<usize>(),
            _ => None,
        }
    }

    fn discriminant(&self) -> usize {
        match *self {
            Self::Bool(..) => 0,
            Self::Number(ref n) => n.discriminant(),
            Self::Char(..) => 13,
            Self::String(..) => 14,
            Self::Null => 15,
            Self::Option(..) => 16,
            Self::Newtype(..) => 17,
            Self::Seq(..) => 18,
            Self::Map(..) => 19,
            Self::Bytes(..) => 20,
            Self::Unassigned => 21,
            Self::Err(..) => 22,
            Self::Other(..) => 23,
        }
    }

    #[allow(clippy::cast_lossless)]
    fn unexpected(&self) -> serde::de::Unexpected {
        match *self {
            Self::Bool(b) => serde::de::Unexpected::Bool(b),
            Self::Number(ref n) => n.unexpected(),
            Self::Char(c) => serde::de::Unexpected::Char(c),
            Self::String(ref s) => serde::de::Unexpected::Str(s),
            Self::Null => serde::de::Unexpected::Unit,
            Self::Option(_) => serde::de::Unexpected::Option,
            Self::Newtype(_) => serde::de::Unexpected::NewtypeStruct,
            Self::Seq(_) => serde::de::Unexpected::Seq,
            Self::Map(_) => serde::de::Unexpected::Map,
            Self::Bytes(ref b) => serde::de::Unexpected::Bytes(b),
            Self::Unassigned => serde::de::Unexpected::Other("Unassigned"),
            Self::Err(_) => serde::de::Unexpected::Other("Err"),
            Self::Other(..) => serde::de::Unexpected::Other("Other"),
        }
    }

    /// This attempts to deserialize the document into a type that implements Deserialize
    pub fn try_into<'de, Q: Deserialize<'de>>(self) -> Result<Q, DeserializerError> {
        Q::deserialize(self)
    }

    /// This creates a new document from a type that implements Serialize
    pub fn new<Q: Serialize>(value: Q) -> Result<Self, SerializerError> {
        value.serialize(Serializer::new())
    }

    /// Merge another document into this one, consuming both documents into the result.
    /// If this document is not a map or seq, it will be overwritten.
    /// If this document is a seq and the other is also a seq, the other seq will be
    /// appended to the end of this one. If the other document is not a seq, then it will
    /// be appended to the end of the sequence in this one.
    /// If this document is a map and the other document is also be a map, merging
    /// maps will cause values from the other document to overwrite this one.
    /// Otherwise, the value from the other document will overwrite this one.
    pub fn merge(&mut self, mut other: Self)
    {
        match self {
            Self::Seq(s) => {
                if let Self::Seq(ref mut o) = other {
                    s.append(o);
                } else {
                    s.push(other);
                }
            }
            Self::Map(ref mut m) => {
                if let Self::Map(o) = other {
                    for (key, val) in o.into_iter() {
                        if let Some(loc) = m.get_mut(&key) {
                            loc.merge(val);
                        } else {
                            m.insert(key, val.clone());
                        }
                    }
                } else {
                    *self = other
                }
            }
            _ => *self = other,
        }
    }
}

impl<T: UnstructuredDataTrait> fmt::Display for Unstructured<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Null => fmt.write_str("<null>"),
            Self::Bool(b) => b.fmt(fmt),
            Self::Number(n) => n.fmt(fmt),
            Self::Char(c) => c.fmt(fmt),
            Self::String(ref s) => s.fmt(fmt),
            Self::Newtype(t) => t.fmt(fmt),
            Self::Bytes(_) => fmt.write_str("b[...]"),
            Self::Unassigned => fmt.write_str("(Unassigned)"),
            Self::Err(e) => e.fmt(fmt),
            Self::Other(o) => o.fmt(fmt),
            Self::Option(o) => o
                .as_ref()
                .map(|v| v.fmt(fmt))
                .unwrap_or_else(|| fmt.write_str("None")),
            Self::Seq(s) => {
                fmt.write_str("[")?;
                fmt.write_str(
                    &s.iter()
                        .map(|doc| doc.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                )?;
                fmt.write_str("]")
            }
            Self::Map(m) => {
                fmt.write_str("{")?;
                fmt.write_str(
                    &m.iter()
                        .map(|(k, v)| format!("{} => {}", k, v))
                        .collect::<Vec<String>>()
                        .join(","),
                )?;
                fmt.write_str("}")
            }
        }
    }
}
