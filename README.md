<p align="center">

[![Crate](https://img.shields.io/crates/v/unstructured.svg)](https://crates.io/crates/unstructured)
[![Documentation](https://img.shields.io/badge/docs-current-important.svg)](https://docs.rs/unstructured/)
[![MIT License](https://img.shields.io/github/license/proctorlabs/unstructured-rs.svg)](LICENSE)

</p>

# Unstructured Documents

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

In addition to many of the futures provided by the above libraries, unstructured also provides:

- Easy usage of comparisons with primitive types, e.g. ```Document::U64(100) == 100 as u64```
- Easy merging of multiple documents: ```doc1.merge(doc2)``` or ```doc = doc1 + doc2```
- Selectors for retrieving nested values within a document without cloning: ```doc.select(".path.to.key")```
- Filters to create new documents from an array of input documents: ```docs.filter("[0].path.to.key | [1].path.to.array[0:5]")```
- Convenience methods for is_type(), as_type(), take_type()
- Most of the From implementation for easy document creation

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

```rust
use unstructured::Document;

let doc: Document =
    serde_json::from_str("{\"some\": {\"nested\": {\"value\": \"is this value\"}}}").unwrap();
let doc_element = doc.select("/some/nested/value").unwrap(); // Returns an Option<Document>, None if not found
let expected: Document = "is this value".into();
assert_eq!(*doc_element, expected);
```

In addition to selectors, filters can be used to create new documents from an array of input documents.

- Document selection: "[0]", "[1]", "*"
- Path navigation: "[0].path.to.key" "[0] /path/to/key" r#" [0] .["path"].["to"].["key"] "#
- Index selection: "[0] .array.[0]"
- Sequence selection: "[0] .array.[0:0]" "[0] .array.[:]" "[0] .array.[:5]"
- Filtering multiple docs: "[0].key | [1].key"
- Merging docs: "*" "[0].key.to.merge | [1].add.this.key.too | [2].key.to.merge"

```rust
use unstructured::Document;

let docs: Vec<Document> = vec![
    serde_json::from_str(r#"{"some": {"nested": {"vals": [1,2,3]}}}"#).unwrap(),
    serde_json::from_str(r#"{"some": {"nested": {"vals": [4,5,6]}}}"#).unwrap(),
];
let result = Document::filter(&docs, "[0].some.nested.vals | [1].some.nested.vals").unwrap();
assert_eq!(result["some"]["nested"]["vals"][4], Document::U64(5));
```
