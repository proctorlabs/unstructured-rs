use serde::{Deserialize, Serialize};
use unstructured::*;

#[test]
fn numeric_indexing_test() {
    let doc = Document::Seq(vec![1u64.into(), 2u64.into(), 3u64.into()]);
    assert_eq!(doc[1], Document::Number(Number::U64(2)));
    assert_eq!(doc[100][9999], Document::Null);
}

const MERGE1: &str = r#"{
    "some": "val",
    "other": 
    {
        "key1": "val1", 
        "key2": "val2",
        "array": [1, 2, 3]
    },
    "overwrite-me": "something"
}"#;

const MERGE2: &str = r#"{
    "some-new": "val-appended",
    "other":
    {
        "key1": "val1-appended",
        "key3": "val3",
        "array": [4, 5, 6]
    },
    "overwrite-me": 10
}"#;

#[test]
fn path_test() {
    let mut doc: Document = serde_json::from_str(MERGE1).unwrap();
    println!("{}", walk!(doc/"other"/"array"));//doc.get_path(&[&"other".into(), &"array".into()]));
    println!(
        "{}",
        doc.get_path(&[
            &Document::String("other".into()),
            &Document::String("array".into())
        ])
    );
    doc.set_path(
        "Set path value",
        &[&"yay".into(), &"for".into(), &"this".into()],
    );
    println!("{}", doc);
}

#[test]
fn dynamic_indexing_test() {
    let mut doc = Document::Null;
    doc["test"][5]["someval"] = true.into();
    println!("{}", doc);
}

#[test]
fn merge_test() {
    let mut doc: Document = serde_json::from_str(MERGE1).unwrap();
    doc.merge(serde_json::from_str(MERGE2).unwrap());
    let res = serde_json::to_string_pretty(&doc).unwrap();
    println!("{}", res);
}

#[test]
fn from_pointer() {
    let doc: Document =
        serde_json::from_str(r#"{"some": {"nested": {"value": "is this value"}}}"#).unwrap();
    let doc_element = doc.select("/some/nested/value").unwrap();
    println!("{}", doc_element);
}

#[test]
fn map_indexing_test() {
    let mut map = Mapping::new();
    map.insert("test".into(), 100u64.into());
    let doc: Document = map.into();
    println!("{}", doc["test"] == Document::Number(Number::U64(100)));
    assert_eq!(doc["test"], Document::Number(Number::U64(100)));
    assert_eq!(doc["test-not-exist"], Document::Null);
    assert_eq!(doc[100][9999], Document::Null);
}

#[test]
fn index_dynamic_mod_test() {
    let mut doc = Document::default();
    doc["test"] = 100u64.into();
    assert_eq!(doc["test"], 100u64);
    assert_eq!(doc["test-not-exist"], Document::Null);
    assert_eq!(doc[100][9999], Document::Null);
}

#[test]
fn de_smoke_test() {
    // some convoluted Document:
    let document = Document::Option(Some(Box::new(Document::Seq(vec![
        8u16.into(),
        Document::Char('a'),
        1.0f32.into(),
        Document::String("hello".into()),
        Document::Map(
            vec![
                (Document::Bool(false), Document::Null),
                (
                    Document::Bool(true),
                    Document::Newtype(Box::new(Document::Bytes(b"hi".as_ref().into()))),
                ),
            ]
            .into_iter()
            .collect(),
        ),
    ]))));

    // assert that the Document: remains unchanged through deserialization
    let document_de = Document::deserialize(document.clone()).unwrap();
    assert_eq!(document_de, document);
}

#[test]
fn ser_smoke_test() {
    #[derive(Serialize)]
    struct Foo {
        a: u32,
        b: String,
        c: Vec<bool>,
    }

    let f = Foo {
        a: 15,
        b: "hello".into(),
        c: vec![true, false],
    };

    let expected = Document::Map(
        vec![
            (Document::String("a".into()), 15u32.into()),
            (
                Document::String("b".into()),
                Document::String("hello".into()),
            ),
            (
                Document::String("c".into()),
                Document::Seq(vec![Document::Bool(true), Document::Bool(false)]),
            ),
        ]
        .into_iter()
        .collect(),
    );

    let document = Document::new(&f).unwrap();
    assert_eq!(expected, document);
}

#[test]
fn deserialize_into_enum() {
    #[derive(Deserialize, Debug, PartialEq, Eq)]
    enum Foo {
        Bar,
        Baz(u8),
    }

    let document = Document::String("Bar".into());
    assert_eq!(Foo::deserialize(document).unwrap(), Foo::Bar);

    let document = Document::Map(vec![("Baz".into(), 1u8.into())].into_iter().collect());
    assert_eq!(Foo::deserialize(document).unwrap(), Foo::Baz(1));
}

#[test]
fn check_assorted_equality() {
    // let docs: Document = anyvec![12, "hello"];
    let d: Document = 1u128.into();
    let e: Number = 1f64.into();
    let f: Number = 5u8.into();
    assert_eq!(e, d);
    assert_eq!(d, e);
    assert_eq!(d, 1);
    assert_eq!(e, 1);
    assert_eq!(1, d);
    assert_eq!(1, e);
    assert_ne!(f, d);
    assert_ne!(d, f);
    assert_ne!(f, e);
    assert_ne!(e, f);

    assert_ne!(d, "hello");
    assert_ne!("hello", d);
    let hello: Document = "hello".into();
    assert_eq!("hello", hello);
    assert_eq!(hello, "hello");
}

#[test]
fn deserialize_newtype() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Foo(i32);

    let input: Document = 5i32.into();
    let f = Foo::deserialize(input).unwrap();
    assert_eq!(f, Foo(5));
}

#[test]
fn deserialize_newtype2() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Foo(i32);

    #[derive(Debug, Deserialize, PartialEq)]
    struct Bar {
        foo: Foo,
    }

    let input = Document::Map(
        vec![(Document::String("foo".to_owned()), 5i32.into())]
            .into_iter()
            .collect(),
    );
    let b = Bar::deserialize(input).unwrap();
    assert_eq!(b, Bar { foo: Foo(5) });
}
