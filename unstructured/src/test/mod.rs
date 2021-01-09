use crate::*;
use serde::Deserialize;

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

            use crate::core::de::*;

            let raw_event = RawEvent::deserialize(deserializer)?;

            // Cannot directly use Document: as Deserializer, since error type needs to be
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
            (Document::String("object".to_owned()), 5u32.into()),
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
            (Document::String("object".to_owned()), 5u8.into()),
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
            (Document::String("object".to_owned()), Document::Null),
        ]
        .into_iter()
        .collect(),
    );
    let _ = Event::deserialize(input).expect_err("expected deserializing bad ADDED event to fail");
}
