/*!
This library provides types for usage with unstructured data. This is based on functionality from both
[serde_json](https://github.com/serde-rs/json) and [serde_value](https://github.com/arcnmx/serde-value). Depending
on your use case, it may make sense to use one of those instead.

These structures for serialization and deserialization into an intermediate container with serde and manipulation
of this data while in this intermediate state.

# Purpose

So why not use one of the above libraries?

- **serde_json::value::Value** is coupled with JSON serialization/deserialization pretty strongly. The purpose is to have
  an intermediate format for usage specifically with JSON. This can be a problem if you need something more generic (e.g.
  you need to support features that JSON does not) or do not wish to require dependence on JSON libraries. Document supports
  serialization to/from JSON without being limited to usage with JSON libraries.
- **serde_value::Value** provides an intermediate format for serialization and deserialization like Document, however it does
  not provide as many options for manipulating the data such as indexing and easy type conversion.

In addition to many of the futures provided by the above libraries, unstructured also provides:

- Easy usage of comparisons with primitive types, e.g. ```Document::U64(100) == 100 as u64```
- Easy merging of multiple documents: ```doc1.merge(doc2)``` or ```doc = doc1 + doc2```
- Selectors for retrieving nested values within a document without cloning: ```doc.select(".path.to.key")```
- Filters to create new documents from an array of input documents: ```docs.filter("[0].path.to.key | [1].path.to.array[0:5]")```
- Convenience methods for is_type(), as_type(), take_type()
- Most of the From implementation for easy document creation

# Example Usage

The primary struct used in this repo is ```Document```. Document provides methods for easy type conversion and manipulation.

```
use unstructured::Document;
use std::collections::BTreeMap;

let mut map = BTreeMap::new(); // Will be inferred as BTreeMap<Document, Document> though root element can be any supported type
map.insert("test".into(), (100 as u64).into()); // From<> is implement for most basic data types
let doc: Document = map.into(); // Create a new Document where the root element is the map defined above
assert_eq!(doc["test"], Document::U64(100));
```

Document implements serialize and deserialize so that it can be easily used where the data format is unknown and manipulated
after it has been received.

```
#[macro_use]
extern crate serde;
use unstructured::Document;

#[derive(Deserialize, Serialize)]
struct SomeStruct {
    key: String,
}

fn main() {
    let from_service = "{\"key\": \"value\"}";
    let doc: Document = serde_json::from_str(from_service).unwrap();
    let expected: Document = "value".into();
    assert_eq!(doc["key"], expected);

    let some_struct: SomeStruct = doc.try_into().unwrap();
    assert_eq!(some_struct.key, "value");

    let another_doc = Document::new(some_struct).unwrap();
    assert_eq!(another_doc["key"], expected);
}
```

Selectors can be used to retrieve a reference to nested values, regardless of the incoming format.

- [JSON Pointer syntax](https://tools.ietf.org/html/rfc6901): ```doc.select("/path/to/key")```
- A JQ inspired syntax: ```doc.select(".path.to.[\"key\"")```

```
use unstructured::Document;

let doc: Document =
    serde_json::from_str("{\"some\": {\"nested\": {\"value\": \"is this value\"}}}").unwrap();
let doc_element = doc.select("/some/nested/value").unwrap(); // Returns an Option<Document>, None if not found
let expected: Document = "is this value".into();
assert_eq!(*doc_element, expected);
```

In addition to selectors, filters can be used to create new documents from an array of input documents.

- Document selection: ```"[0]", "[1]", "*"```
- Path navigation: ```"[0].path.to.key" "[0] /path/to/key" r#" [0] .["path"].["to"].["key"] "#```
- Index selection: ```"[0] .array.[0]"```
- Sequence selection: ```"[0] .array.[0:0]" "[0] .array.[:]" "[0] .array.[:5]"```
- Filtering multiple docs: ```"[0].key | [1].key"```
- Merging docs: ```"*" "[0].key.to.merge | [1].add.this.key.too | [2].key.to.merge"```

```
use unstructured::Document;

let docs: Vec<Document> = vec![
    serde_json::from_str(r#"{"some": {"nested": {"vals": [1,2,3]}}}"#).unwrap(),
    serde_json::from_str(r#"{"some": {"nested": {"vals": [4,5,6]}}}"#).unwrap(),
];
let result = Document::filter(&docs, "[0].some.nested.vals | [1].some.nested.vals").unwrap();
assert_eq!(result["some"]["nested"]["vals"][4], Document::U64(5));
```
*/

#[macro_use]
extern crate serde;

#[cfg(test)]
mod test;

use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::sync::Arc;

use de::*;
use ser::*;

mod de;
mod from;
mod index;
mod selector;
mod ser;

#[derive(Clone, Debug)]
pub enum Document {
    Bool(bool),

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

    Char(char),
    String(String),

    Unit,
    Option(Option<Box<Document>>),
    Newtype(Box<Document>),
    Seq(Vec<Document>),
    Map(BTreeMap<Document, Document>),
    Bytes(Vec<u8>),

    Unassigned,
    Err(Arc<dyn std::error::Error + Send + Sync>),
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
            Document::U128(v) => v.hash(hasher),
            Document::I8(v) => v.hash(hasher),
            Document::I16(v) => v.hash(hasher),
            Document::I32(v) => v.hash(hasher),
            Document::I64(v) => v.hash(hasher),
            Document::I128(v) => v.hash(hasher),
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
            Document::Unassigned => ().hash(hasher),
            Document::Err(ref e) => format!("{}", e).hash(hasher),
        }
    }
}

impl std::ops::Add<Document> for Document {
    type Output = Document;

    fn add(mut self, rhs: Document) -> Document {
        self.merge(rhs);
        self
    }
}

macro_rules! impl_partial_eq {
    ($($type:ty, $vrnt:ident);*) => {
        $(
            impl PartialEq<$type> for Document {
                fn eq(&self, rhs: & $type) -> bool {
                    match self {
                        Document::$vrnt(i) => i == rhs,
                        _ => false,
                    }
                }
            }
        )*
    };
}

impl_partial_eq! {
    str, String;
    String, String;
    bool, Bool;
    u8, U8;
    u16, U16;
    u32, U32;
    u64, U64;
    i8, I8;
    i16, I16;
    i32, I32;
    i64, I64;
    f32, F32;
    f64, F64;
    char, Char
}

impl PartialEq for Document {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (&Document::Bool(v0), &Document::Bool(v1)) if v0 == v1 => true,
            (&Document::U8(v0), &Document::U8(v1)) if v0 == v1 => true,
            (&Document::U16(v0), &Document::U16(v1)) if v0 == v1 => true,
            (&Document::U32(v0), &Document::U32(v1)) if v0 == v1 => true,
            (&Document::U64(v0), &Document::U64(v1)) if v0 == v1 => true,
            (&Document::U128(v0), &Document::U128(v1)) if v0 == v1 => true,
            (&Document::I8(v0), &Document::I8(v1)) if v0 == v1 => true,
            (&Document::I16(v0), &Document::I16(v1)) if v0 == v1 => true,
            (&Document::I32(v0), &Document::I32(v1)) if v0 == v1 => true,
            (&Document::I64(v0), &Document::I64(v1)) if v0 == v1 => true,
            (&Document::I128(v0), &Document::I128(v1)) if v0 == v1 => true,
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

macro_rules! impl_is_as {
    ($($is:ident, $as:ident, $take:ident, $v:ident, $r:ty);*) => {
        $(
            /// Check whether this Document is a $r
            pub fn $is(&self) -> bool {
                match self {
                    Document::$v(_) => true,
                    _ => false,
                }
            }

            /// Retrieve the value of this Document as $r .
            /// This will return None if the document type is not Document::$v
            pub fn $as(&self) -> Option<$r> {
                match self {
                    Document::$v(r) => Some(r.to_owned()),
                    _ => None,
                }
            }

            /// Move the value of this document out of the object if is is an $r.
            /// This will return None and leave the Document unchanged if the type does not match.
            /// When the value is moved out, a Document::Unit is left in its place.
            pub fn $take(&mut self) -> Option<$r> {
                if self.$is() {
                    let r = mem::replace(self, Document::Unit);
                    if let Document::$v(res) = r {
                        return Some(res);
                    }
                }
                None
            }
        )*
    };
}

macro_rules! impl_cast {
    ($( $name:ident : $t:ty => $( $variant:ident ( $variant_ty:ident ) )* , )*) => {
        $(
            pub fn $name (self) -> Option< $t > {
                match self {
                    $( Document::$variant(v) => if v == (v as $t) as $variant_ty { Some(v as $t) } else { None }, )*
                    _ => None,
                }
            }
        )*
    };
}

impl Document {
    impl_cast! {
        cast_i8 : i8 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128),
        cast_i16 : i16 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128),
        cast_i32 : i32 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128),
        cast_i64 : i64 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128),
        cast_i128 : i128 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128),
        cast_isize : isize => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128),
        cast_u8 : u8 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128),
        cast_u16 : u16 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128),
        cast_u32 : u32 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128),
        cast_u64 : u64 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128),
        cast_u128 : u128 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128),
        cast_usize : usize => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128),
        // cast_f32 : f32 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
        // cast_f64 : f64 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
    }

    impl_is_as! {
        is_i8,      as_i8,      take_i8,      I8,         i8;
        is_i16,     as_i16,     take_i16,     I16,        i16;
        is_i32,     as_i32,     take_i32,     I32,        i32;
        is_i64,     as_i64,     take_i64,     I64,        i64;
        is_i128,    as_i128,    take_i128,    I128,       i128;
        is_u8,      as_u8,      take_u8,      U8,         u8;
        is_u16,     as_u16,     take_u16,     U16,        u16;
        is_u32,     as_u32,     take_u32,     U32,        u32;
        is_u64,     as_u64,     take_u64,     U64,        u64;
        is_u128,    as_u128,    take_u128,    U128,       u128;
        is_f32,     as_f32,     take_f32,     F32,        f32;
        is_f64,     as_f64,     take_f64,     F64,        f64;
        is_bool,    as_bool,    take_bool,    Bool,       bool;
        is_char,    as_char,    take_char,    Char,       char;
        is_string,  as_string,  take_string,  String,     String;
        is_map,     as_map,     take_map,     Map,        BTreeMap<Document, Document>;
        is_option,  as_option,  take_option,  Option,     Option<Box<Document>>;
        is_bytes,   as_bytes,   take_bytes,   Bytes,      Vec<u8>;
        is_seq,     as_seq,     take_seq,     Seq,        Vec<Document>;
        is_newtype, as_newtype, take_newtype, Newtype,    Box<Document>
    }

    pub fn get_path<T: index::Index>(&self, path: &[&T]) -> &Document {
        let mut temp = self;
        for p in path.iter() {
            temp = &temp[p];
        }
        temp
    }

    pub fn set_path<T: index::Index, U: Into<Document>>(&mut self, val: U, path: &[&T]) {
        let mut temp = self;
        for p in path.iter() {
            temp = &mut temp[p];
        }
        *temp = val.into();
    }

    pub fn set<U: Into<Document>>(&mut self, val: U) {
        *self = val.into();
    }

    pub fn replace(&mut self, new_val: Document) -> Self {
        mem::replace(self, new_val)
    }

    pub fn take(&mut self) -> Self {
        mem::replace(self, Document::Unit)
    }

    pub fn is_unit(&self) -> bool {
        matches!(self, Document::Unit)
    }

    pub fn is_number(&self) -> bool {
        matches!(self,
            Document::U8(_)
            | Document::U16(_)
            | Document::U32(_)
            | Document::U64(_)
            | Document::U128(_)
            | Document::I8(_)
            | Document::I16(_)
            | Document::I32(_)
            | Document::I128(_)
            | Document::I64(_)
            | Document::F32(_)
            | Document::F64(_)
        )
    }

    /// Returns true if the value is any signed integer (i8, i16, i32, i64)
    pub fn is_signed(&self) -> bool {
        matches!(self,
            Document::I8(_)
            | Document::I16(_)
            | Document::I32(_)
            | Document::I64(_)
            | Document::I128(_)
        )
    }

    /// Returns true if the value is any unsigned integer (u8, u16, u32, u64)
    pub fn is_unsigned(&self) -> bool {
        matches!(self,
            Document::U8(_)
            | Document::U16(_)
            | Document::U32(_)
            | Document::U64(_)
            | Document::U128(_)
        )
    }

    /// Returns true if the value is any float (f32, f64)
    pub fn is_float(&self) -> bool {
        matches!(self, Document::F32(_) | Document::F64(_))
    }

    pub fn as_usize(&self) -> Option<usize> {
        match self {
            Document::U8(i) => Some(*i as usize),
            Document::U16(i) => Some(*i as usize),
            Document::U32(i) => Some(*i as usize),
            Document::U64(i) => Some(*i as usize),
            Document::U128(i) => Some(*i as usize),
            Document::I8(i) => Some(*i as usize),
            Document::I16(i) => Some(*i as usize),
            Document::I32(i) => Some(*i as usize),
            Document::I64(i) => Some(*i as usize),
            Document::I128(i) => Some(*i as usize),
            Document::F32(i) => Some(*i as usize),
            Document::F64(i) => Some(*i as usize),
            Document::Char(i) => Some(*i as usize),
            _ => None,
        }
    }

    fn discriminant(&self) -> usize {
        match *self {
            Document::Bool(..) => 0,
            Document::U8(..) => 1,
            Document::U16(..) => 2,
            Document::U32(..) => 3,
            Document::U64(..) => 4,
            Document::U128(..) => 5,
            Document::I8(..) => 6,
            Document::I16(..) => 7,
            Document::I32(..) => 8,
            Document::I64(..) => 9,
            Document::I128(..) => 10,
            Document::F32(..) => 11,
            Document::F64(..) => 12,
            Document::Char(..) => 13,
            Document::String(..) => 14,
            Document::Unit => 15,
            Document::Option(..) => 16,
            Document::Newtype(..) => 17,
            Document::Seq(..) => 18,
            Document::Map(..) => 19,
            Document::Bytes(..) => 20,
            Document::Unassigned => 21,
            Document::Err(..) => 22,
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
            Document::U128(n) => serde::de::Unexpected::Unsigned(n as u64),
            Document::I8(n) => serde::de::Unexpected::Signed(n as i64),
            Document::I16(n) => serde::de::Unexpected::Signed(n as i64),
            Document::I32(n) => serde::de::Unexpected::Signed(n as i64),
            Document::I64(n) => serde::de::Unexpected::Signed(n),
            Document::I128(n) => serde::de::Unexpected::Signed(n as i64),
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
            Document::Unassigned => serde::de::Unexpected::Other("Unassigned"),
            Document::Err(_) => serde::de::Unexpected::Other("Err"),
        }
    }

    /// This attempts to deserialize the document into a type that implements Deserialize
    pub fn try_into<'de, T: Deserialize<'de>>(self) -> Result<T, DeserializerError> {
        T::deserialize(self)
    }

    /// This creates a new document from a type that implements Serialize
    pub fn new<T: Serialize>(value: T) -> Result<Self, SerializerError> {
        value.serialize(Serializer)
    }

    /// Merge another document into this one, consuming both documents into the result.
    /// If this document is not a map or seq, it will be overwritten.
    /// If this document is a seq and the other is also a seq, the other seq will be
    /// appended to the end of this one. If the other document is not a seq, then it will
    /// be appended to the end of the sequence in this one.
    /// If this document is a map and the other document is also be a map, merging
    /// maps will cause values from the other document to overwrite this one.
    /// Otherwise, the value from the other document will overwrite this one.
    pub fn merge(&mut self, mut other: Document) {
        match self {
            Document::Seq(s) => {
                if let Document::Seq(ref mut o) = other {
                    s.append(o);
                } else {
                    s.push(other);
                }
            }
            Document::Map(ref mut m) => {
                if let Document::Map(o) = other {
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
            Document::U128(n) => n.fmt(fmt),
            Document::I8(n) => n.fmt(fmt),
            Document::I16(n) => n.fmt(fmt),
            Document::I32(n) => n.fmt(fmt),
            Document::I64(n) => n.fmt(fmt),
            Document::I128(n) => n.fmt(fmt),
            Document::F32(n) => n.fmt(fmt),
            Document::F64(n) => n.fmt(fmt),
            Document::Char(c) => c.fmt(fmt),
            Document::String(ref s) => s.fmt(fmt),
            Document::Unit => fmt.write_str("()"),
            Document::Option(o) => o
                .as_ref()
                .map(|v| v.fmt(fmt))
                .unwrap_or_else(|| fmt.write_str("None")),
            Document::Newtype(t) => t.fmt(fmt),
            Document::Seq(s) => {
                fmt.write_str("[")?;
                fmt.write_str(
                    &s.iter()
                        .map(|doc| doc.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                )?;
                fmt.write_str("]")
            }
            Document::Map(m) => {
                fmt.write_str("{")?;
                fmt.write_str(
                    &m.iter()
                        .map(|(k, v)| format!("{} => {}", k, v))
                        .collect::<Vec<String>>()
                        .join(","),
                )?;
                fmt.write_str("}")
            }
            Document::Bytes(_) => fmt.write_str("b[...]"),
            Document::Unassigned => fmt.write_str("(Unassigned)"),
            Document::Err(e) => e.fmt(fmt),
        }
    }
}
