<p align="center">

[![Crate](https://img.shields.io/crates/v/unstructured.svg)](https://crates.io/crates/unstructured)
[![Documentation](https://img.shields.io/badge/docs-current-important.svg)](https://docs.rs/unstructured/)
[![MIT License](https://img.shields.io/github/license/proctorlabs/unstructured-rs.svg)](LICENSE)

</p>

# Unstructured Data in Rust

This library provides types for usage with unstructured data. This is based on functionality from both
[serde_json](https://github.com/serde-rs/json) and [serde_value](https://github.com/arcnmx/serde-value). Depending
on your use case, it may make sense to use one of those instead.

These structures for serialization and deserialization into an intermediate container with serde and manipulation
of this data while in this intermediate state.

## Purpose

So why not use one of the above libraries?

- **serde_json::value::Value** is coupled with JSON serialization/deserialization pretty strongly. The purpose is to have
  an intermediate format for usage specifically with JSON. This can be a problem if you need something more generic (e.g.
  you need to support features that JSON does not) or do not wish to require dependence on JSON libraries. Document supports
  serialization to/from JSON without being limited to usage with JSON libraries.
- **serde_value::Value** provides an intermediate format for serialization and deserialization like Document, however it does
  not provide as many options for manipulating the data such as indexing and easy type conversion.

## Example Usage

The primary struct used in this repo is ```Document```. Document provides methods for easy type conversion and manipulation.

```rust
use unstructured::Document;
use std::collections::BTreeMap;

let mut map = BTreeMap::new(); // Will be inferred as BTreeMap<Document, Document> though root element can be any supported type
map.insert("test".into(), (100 as u64).into()); // From<> is implement for most basic data types
let doc: Document = map.into(); // Create a new Document where the root element is the map defined above
assert_eq!(doc["test"], Document::U64(100));
```

Document implements serialize and deserialize so that it can be easily used where the data format is unknown and manipulated
after it has been received.

```rust
#[macro_use]
extern crate serde_derive;
use unstructured::Document;

#[derive(Deserialize, Serialize)]
struct SomeStruct {
    key: String,
}

fn main() {
    let from_service = "{\"key\": \"value\"}";
    let doc: Document = serde_json::from_str(from_service).unwrap();
    assert_eq!(doc["key"], "value".into());

    let some_struct: SomeStruct = doc.try_into().unwrap();
    assert_eq!(some_struct.key, "value");

    let another_doc = Document::new(some_struct).unwrap();
    assert_eq!(another_doc["key"], "value".into());
}
```

[JSON Pointer syntax](https://tools.ietf.org/html/rfc6901) can be used as well to quickly get a nested value. This will work
regardless of the format that you deserialized from, so this syntax can be used to easily retrieve, for example, nested YAML values.

```rust
use unstructured::Document;

let doc: Document =
    serde_json::from_str("{\"some\": {\"nested\": {\"value\": \"is this value\"}}}").unwrap();
let doc_element = doc.pointer("/some/nested/value").unwrap(); // Returns an Option<Document>, None if not found
assert_eq!(*doc_element, "is this value".into());
```

Below are the Document enum types available:

```rust
use std::collections::BTreeMap;
pub enum Document {
    // Boolean
    Bool(bool),

    // Unsigned
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    // Signed
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    // Floats
    F32(f32),
    F64(f64),

    // Char/String
    Char(char),
    String(String),
    // Effectively 'Null'
    Unit,
    // Options
    Option(Option<Box<Document>>),
    // Newtypes
    Newtype(Box<Document>),
    // Arrays
    Seq(Vec<Document>),
    // Maps
    Map(BTreeMap<Document, Document>),
    // Raw data
    Bytes(Vec<u8>),
}
```

