use crate::*;

#[test]
fn numeric_indexing_test() {
    let doc = Document::Seq(vec![Document::U64(1), Document::U64(2), Document::U64(3)]);
    assert_eq!(doc[1], Document::U64(2));
    assert_eq!(doc[100][9999], Document::Unit);
}

#[test]
fn from_pointer() {
    let doc: Document =
        serde_json::from_str("{\"some\": {\"nested\": {\"value\": \"is this value\"}}}").unwrap();
    let doc_element = doc.pointer("/some/nested/value").unwrap();
    println!("{}", doc_element);
}

#[test]
fn map_indexing_test() {
    let mut map = BTreeMap::new();
    map.insert("test".into(), (100 as u64).into());
    let doc: Document = map.into();
    println!("{}", doc["test"] == Document::U64(100));
    assert_eq!(doc["test"], Document::U64(100));
    assert_eq!(doc["test-not-exist"], Document::Unit);
    assert_eq!(doc[100][9999], Document::Unit);
}

#[test]
fn index_dynamic_mod_test() {
    let mut doc = Document::default();
    doc["test"] = Document::U64(100);
    assert_eq!(doc["test"], Document::U64(100));
    assert_eq!(doc["test-not-exist"], Document::Unit);
    assert_eq!(doc[100][9999], Document::Unit);
}

#[test]
fn de_smoke_test() {
    // some convoluted Document
    let document = Document::Option(Some(Box::new(Document::Seq(vec![
        Document::U16(8),
        Document::Char('a'),
        Document::F32(1.0),
        Document::String("hello".into()),
        Document::Map(
            vec![
                (Document::Bool(false), Document::Unit),
                (
                    Document::Bool(true),
                    Document::Newtype(Box::new(Document::Bytes(b"hi".as_ref().into()))),
                ),
            ]
            .into_iter()
            .collect(),
        ),
    ]))));

    // assert that the Document remains unchanged through deserialization
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
            (Document::String("a".into()), Document::U32(15)),
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

    let document = Document::Map(
        vec![(Document::String("Baz".into()), Document::U8(1))]
            .into_iter()
            .collect(),
    );
    assert_eq!(Foo::deserialize(document).unwrap(), Foo::Baz(1));
}

#[test]
fn deserialize_inside_deserialize_impl() {
    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Added(u32),
        Error(u8),
    }

    impl<'de> serde::Deserialize<'de> for Event {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(Deserialize)]
            struct RawEvent {
                kind: String,
                object: Document,
            }

            let raw_event = RawEvent::deserialize(deserializer)?;

            // Cannot directly use Document as Deserializer, since error type needs to be
            // generic D::Error rather than specific serde_Document::DeserializerError
            let object_deserializer = DocumentDeserializer::new(raw_event.object);

            Ok(match &*raw_event.kind {
                "ADDED" => Event::Added(<_>::deserialize(object_deserializer)?),
                "ERROR" => Event::Error(<_>::deserialize(object_deserializer)?),
                kind => return Err(serde::de::Error::unknown_variant(kind, &["ADDED", "ERROR"])),
            })
        }
    }

    let input = Document::Map(
        vec![
            (
                Document::String("kind".to_owned()),
                Document::String("ADDED".to_owned()),
            ),
            (Document::String("object".to_owned()), Document::U32(5)),
        ]
        .into_iter()
        .collect(),
    );
    let event = Event::deserialize(input).expect("could not deserialize ADDED event");
    assert_eq!(event, Event::Added(5));

    let input = Document::Map(
        vec![
            (
                Document::String("kind".to_owned()),
                Document::String("ERROR".to_owned()),
            ),
            (Document::String("object".to_owned()), Document::U8(5)),
        ]
        .into_iter()
        .collect(),
    );
    let event = Event::deserialize(input).expect("could not deserialize ERROR event");
    assert_eq!(event, Event::Error(5));

    let input = Document::Map(
        vec![
            (
                Document::String("kind".to_owned()),
                Document::String("ADDED".to_owned()),
            ),
            (Document::String("object".to_owned()), Document::Unit),
        ]
        .into_iter()
        .collect(),
    );
    let _ = Event::deserialize(input).expect_err("expected deserializing bad ADDED event to fail");
}

#[test]
fn deserialize_newtype() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Foo(i32);

    let input = Document::I32(5);
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
        vec![(Document::String("foo".to_owned()), Document::I32(5))]
            .into_iter()
            .collect(),
    );
    let b = Bar::deserialize(input).unwrap();
    assert_eq!(b, Bar { foo: Foo(5) });
}
