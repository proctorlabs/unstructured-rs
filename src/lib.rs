#[macro_use]
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod test;

use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use self::ser::Serializer;

pub use de::*;
pub use ser::*;

mod de;
mod from;
mod index;
mod ser;

#[derive(Clone, Debug)]
pub enum Document {
    Bool(bool),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),

    Char(char),
    String(String),

    Unit,
    Option(Option<Box<Document>>),
    Newtype(Box<Document>),
    Seq(Vec<Document>),
    Map(BTreeMap<Document, Document>),
    Bytes(Vec<u8>),
}

impl Hash for Document {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.discriminant().hash(hasher);
        match *self {
            Document::Bool(v) => v.hash(hasher),
            Document::U8(v) => v.hash(hasher),
            Document::U16(v) => v.hash(hasher),
            Document::U32(v) => v.hash(hasher),
            Document::U64(v) => v.hash(hasher),
            Document::I8(v) => v.hash(hasher),
            Document::I16(v) => v.hash(hasher),
            Document::I32(v) => v.hash(hasher),
            Document::I64(v) => v.hash(hasher),
            Document::F32(v) => OrderedFloat(v).hash(hasher),
            Document::F64(v) => OrderedFloat(v).hash(hasher),
            Document::Char(v) => v.hash(hasher),
            Document::String(ref v) => v.hash(hasher),
            Document::Unit => ().hash(hasher),
            Document::Option(ref v) => v.hash(hasher),
            Document::Newtype(ref v) => v.hash(hasher),
            Document::Seq(ref v) => v.hash(hasher),
            Document::Map(ref v) => v.hash(hasher),
            Document::Bytes(ref v) => v.hash(hasher),
        }
    }
}

impl PartialEq for Document {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (&Document::Bool(v0), &Document::Bool(v1)) if v0 == v1 => true,
            (&Document::U8(v0), &Document::U8(v1)) if v0 == v1 => true,
            (&Document::U16(v0), &Document::U16(v1)) if v0 == v1 => true,
            (&Document::U32(v0), &Document::U32(v1)) if v0 == v1 => true,
            (&Document::U64(v0), &Document::U64(v1)) if v0 == v1 => true,
            (&Document::I8(v0), &Document::I8(v1)) if v0 == v1 => true,
            (&Document::I16(v0), &Document::I16(v1)) if v0 == v1 => true,
            (&Document::I32(v0), &Document::I32(v1)) if v0 == v1 => true,
            (&Document::I64(v0), &Document::I64(v1)) if v0 == v1 => true,
            (&Document::F32(v0), &Document::F32(v1)) if OrderedFloat(v0) == OrderedFloat(v1) => {
                true
            }
            (&Document::F64(v0), &Document::F64(v1)) if OrderedFloat(v0) == OrderedFloat(v1) => {
                true
            }
            (&Document::Char(v0), &Document::Char(v1)) if v0 == v1 => true,
            (&Document::String(ref v0), &Document::String(ref v1)) if v0 == v1 => true,
            (&Document::Unit, &Document::Unit) => true,
            (&Document::Option(ref v0), &Document::Option(ref v1)) if v0 == v1 => true,
            (&Document::Newtype(ref v0), &Document::Newtype(ref v1)) if v0 == v1 => true,
            (&Document::Seq(ref v0), &Document::Seq(ref v1)) if v0 == v1 => true,
            (&Document::Map(ref v0), &Document::Map(ref v1)) if v0 == v1 => true,
            (&Document::Bytes(ref v0), &Document::Bytes(ref v1)) if v0 == v1 => true,
            _ => false,
        }
    }
}

impl Ord for Document {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (&Document::Bool(v0), &Document::Bool(ref v1)) => v0.cmp(v1),
            (&Document::U8(v0), &Document::U8(ref v1)) => v0.cmp(v1),
            (&Document::U16(v0), &Document::U16(ref v1)) => v0.cmp(v1),
            (&Document::U32(v0), &Document::U32(ref v1)) => v0.cmp(v1),
            (&Document::U64(v0), &Document::U64(ref v1)) => v0.cmp(v1),
            (&Document::I8(v0), &Document::I8(ref v1)) => v0.cmp(v1),
            (&Document::I16(v0), &Document::I16(ref v1)) => v0.cmp(v1),
            (&Document::I32(v0), &Document::I32(ref v1)) => v0.cmp(v1),
            (&Document::I64(v0), &Document::I64(ref v1)) => v0.cmp(v1),
            (&Document::F32(v0), &Document::F32(v1)) => OrderedFloat(v0).cmp(&OrderedFloat(v1)),
            (&Document::F64(v0), &Document::F64(v1)) => OrderedFloat(v0).cmp(&OrderedFloat(v1)),
            (&Document::Char(v0), &Document::Char(ref v1)) => v0.cmp(v1),
            (&Document::String(ref v0), &Document::String(ref v1)) => v0.cmp(v1),
            (&Document::Unit, &Document::Unit) => Ordering::Equal,
            (&Document::Option(ref v0), &Document::Option(ref v1)) => v0.cmp(v1),
            (&Document::Newtype(ref v0), &Document::Newtype(ref v1)) => v0.cmp(v1),
            (&Document::Seq(ref v0), &Document::Seq(ref v1)) => v0.cmp(v1),
            (&Document::Map(ref v0), &Document::Map(ref v1)) => v0.cmp(v1),
            (&Document::Bytes(ref v0), &Document::Bytes(ref v1)) => v0.cmp(v1),
            (ref v0, ref v1) => v0.discriminant().cmp(&v1.discriminant()),
        }
    }
}

impl Document {
    fn discriminant(&self) -> usize {
        match *self {
            Document::Bool(..) => 0,
            Document::U8(..) => 1,
            Document::U16(..) => 2,
            Document::U32(..) => 3,
            Document::U64(..) => 4,
            Document::I8(..) => 5,
            Document::I16(..) => 6,
            Document::I32(..) => 7,
            Document::I64(..) => 8,
            Document::F32(..) => 9,
            Document::F64(..) => 10,
            Document::Char(..) => 11,
            Document::String(..) => 12,
            Document::Unit => 13,
            Document::Option(..) => 14,
            Document::Newtype(..) => 15,
            Document::Seq(..) => 16,
            Document::Map(..) => 17,
            Document::Bytes(..) => 18,
        }
    }

    #[allow(clippy::cast_lossless)]
    fn unexpected(&self) -> serde::de::Unexpected {
        match *self {
            Document::Bool(b) => serde::de::Unexpected::Bool(b),
            Document::U8(n) => serde::de::Unexpected::Unsigned(n as u64),
            Document::U16(n) => serde::de::Unexpected::Unsigned(n as u64),
            Document::U32(n) => serde::de::Unexpected::Unsigned(n as u64),
            Document::U64(n) => serde::de::Unexpected::Unsigned(n),
            Document::I8(n) => serde::de::Unexpected::Signed(n as i64),
            Document::I16(n) => serde::de::Unexpected::Signed(n as i64),
            Document::I32(n) => serde::de::Unexpected::Signed(n as i64),
            Document::I64(n) => serde::de::Unexpected::Signed(n),
            Document::F32(n) => serde::de::Unexpected::Float(n as f64),
            Document::F64(n) => serde::de::Unexpected::Float(n),
            Document::Char(c) => serde::de::Unexpected::Char(c),
            Document::String(ref s) => serde::de::Unexpected::Str(s),
            Document::Unit => serde::de::Unexpected::Unit,
            Document::Option(_) => serde::de::Unexpected::Option,
            Document::Newtype(_) => serde::de::Unexpected::NewtypeStruct,
            Document::Seq(_) => serde::de::Unexpected::Seq,
            Document::Map(_) => serde::de::Unexpected::Map,
            Document::Bytes(ref b) => serde::de::Unexpected::Bytes(b),
        }
    }

    pub fn try_into<'de, T: Deserialize<'de>>(self) -> Result<T, DeserializerError> {
        T::deserialize(self)
    }

    pub fn new<T>(value: T) -> Result<Self, SerializerError>
    where
        T: Serialize,
    {
        value.serialize(Serializer)
    }
}

impl Eq for Document {}
impl PartialOrd for Document {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Default for Document {
    fn default() -> Self {
        Document::Unit
    }
}

impl fmt::Display for Document {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Document::Bool(b) => b.fmt(fmt),
            Document::U8(n) => n.fmt(fmt),
            Document::U16(n) => n.fmt(fmt),
            Document::U32(n) => n.fmt(fmt),
            Document::U64(n) => n.fmt(fmt),
            Document::I8(n) => n.fmt(fmt),
            Document::I16(n) => n.fmt(fmt),
            Document::I32(n) => n.fmt(fmt),
            Document::I64(n) => n.fmt(fmt),
            Document::F32(n) => n.fmt(fmt),
            Document::F64(n) => n.fmt(fmt),
            Document::Char(c) => c.fmt(fmt),
            Document::String(ref s) => s.fmt(fmt),
            Document::Unit => fmt.write_str("()"),
            Document::Option(_) => fmt.write_str("()"),
            Document::Newtype(t) => t.fmt(fmt),
            Document::Seq(_) => fmt.write_str("()"),
            Document::Map(_) => fmt.write_str("()"),
            Document::Bytes(_) => fmt.write_str("()"),
        }
    }
}
