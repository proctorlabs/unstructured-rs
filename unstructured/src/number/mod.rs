use crate::*;
use ordered_float::OrderedFloat;
use std::fmt;
use serde::{Deserialize, Serialize};
use serde::{
    de::{Deserializer, Visitor},
    ser::Serializer,
};

mod cmp;
mod de;
mod from;
mod ser;

#[derive(Clone, Debug)]
pub enum Number {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
}

impl fmt::Display for Number {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::U8(n) => n.fmt(fmt),
            Number::U16(n) => n.fmt(fmt),
            Number::U32(n) => n.fmt(fmt),
            Number::U64(n) => n.fmt(fmt),
            Number::U128(n) => n.fmt(fmt),
            Number::I8(n) => n.fmt(fmt),
            Number::I16(n) => n.fmt(fmt),
            Number::I32(n) => n.fmt(fmt),
            Number::I64(n) => n.fmt(fmt),
            Number::I128(n) => n.fmt(fmt),
            Number::F32(n) => n.fmt(fmt),
            Number::F64(n) => n.fmt(fmt),
        }
    }
}

impl Number {
    pub(crate) fn discriminant(&self) -> usize {
        match *self {
            Number::U8(..) => 1,
            Number::U16(..) => 2,
            Number::U32(..) => 3,
            Number::U64(..) => 4,
            Number::U128(..) => 5,
            Number::I8(..) => 6,
            Number::I16(..) => 7,
            Number::I32(..) => 8,
            Number::I64(..) => 9,
            Number::I128(..) => 10,
            Number::F32(..) => 11,
            Number::F64(..) => 12,
        }
    }

    #[allow(clippy::cast_lossless)]
    pub(crate) fn unexpected(&self) -> serde::de::Unexpected {
        match *self {
            Number::U8(n) => serde::de::Unexpected::Unsigned(n as u64),
            Number::U16(n) => serde::de::Unexpected::Unsigned(n as u64),
            Number::U32(n) => serde::de::Unexpected::Unsigned(n as u64),
            Number::U64(n) => serde::de::Unexpected::Unsigned(n),
            Number::U128(n) => serde::de::Unexpected::Unsigned(n as u64),
            Number::I8(n) => serde::de::Unexpected::Signed(n as i64),
            Number::I16(n) => serde::de::Unexpected::Signed(n as i64),
            Number::I32(n) => serde::de::Unexpected::Signed(n as i64),
            Number::I64(n) => serde::de::Unexpected::Signed(n),
            Number::I128(n) => serde::de::Unexpected::Signed(n as i64),
            Number::F32(n) => serde::de::Unexpected::Float(n as f64),
            Number::F64(n) => serde::de::Unexpected::Float(n),
        }
    }

    pub fn is_signed(&self) -> bool {
        matches!(self, Number::I8(_) | Number::I16(_) | Number::I32(_) | Number::I64(_) | Number::I128(_))
    }

    /// Returns true if the value is any unsigned integer (u8, u16, u32, u64)
    pub fn is_unsigned(&self) -> bool {
        matches!(self, Number::U8(_) | Number::U16(_) | Number::U32(_) | Number::U64(_) | Number::U128(_))
    }

    /// Returns true if the value is any float (f32, f64)
    pub fn is_float(&self) -> bool {
        matches!(self, Number::F32(_) | Number::F64(_))
    }
}
