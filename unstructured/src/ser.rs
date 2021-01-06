use serde::ser;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::Document;

#[derive(Debug)]
pub enum SerializerError {
    Custom(String),
}

impl fmt::Display for SerializerError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SerializerError::Custom(ref s) => fmt.write_str(s),
        }
    }
}

impl Error for SerializerError {
    fn description(&self) -> &str {
        "Document serializer error"
    }
}

impl ser::Error for SerializerError {
    fn custom<T: fmt::Display>(msg: T) -> SerializerError {
        SerializerError::Custom(msg.to_string())
    }
}

impl ser::Serialize for Document {
    fn serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match *self {
            Document::Bool(v) => s.serialize_bool(v),
            Document::U8(v) => s.serialize_u8(v),
            Document::U16(v) => s.serialize_u16(v),
            Document::U32(v) => s.serialize_u32(v),
            Document::U64(v) => s.serialize_u64(v),
            Document::U128(v) => s.serialize_u128(v),
            Document::I8(v) => s.serialize_i8(v),
            Document::I16(v) => s.serialize_i16(v),
            Document::I32(v) => s.serialize_i32(v),
            Document::I64(v) => s.serialize_i64(v),
            Document::I128(v) => s.serialize_i128(v),
            Document::F32(v) => s.serialize_f32(v),
            Document::F64(v) => s.serialize_f64(v),
            Document::Char(v) => s.serialize_char(v),
            Document::String(ref v) => s.serialize_str(v),
            Document::Unit => s.serialize_unit(),
            Document::Option(None) => s.serialize_none(),
            Document::Option(Some(ref v)) => s.serialize_some(v),
            Document::Newtype(ref v) => s.serialize_newtype_struct("", v),
            Document::Seq(ref v) => v.serialize(s),
            Document::Map(ref v) => v.serialize(s),
            Document::Bytes(ref v) => s.serialize_bytes(v),
            Document::Unassigned => s.serialize_unit(),
            Document::Err(ref e) => s.serialize_str(e.to_string().as_str()),
        }
    }
}

pub struct Serializer;

impl ser::Serializer for Serializer {
    type Ok = Document;
    type Error = SerializerError;
    type SerializeSeq = SerializeSeq;
    type SerializeTuple = SerializeTuple;
    type SerializeTupleStruct = SerializeTupleStruct;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeStruct;
    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Document::I8(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Document::I16(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Document::I32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Document::I64(v))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Ok(Document::I128(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Document::U8(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Document::U16(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Document::U32(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Document::U64(v))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(Document::U128(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Document::F32(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Document::F64(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Document::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Bytes(v.to_vec()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Option(None))
    }

    fn serialize_some<T: ?Sized>(self, document: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        document
            .serialize(Serializer)
            .map(|v| Document::Option(Some(Box::new(v))))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Unit)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Unit)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Unit)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        document: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        document
            .serialize(Serializer)
            .map(|v| Document::Newtype(Box::new(v)))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        document: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        document
            .serialize(Serializer)
            .map(|v| Document::Newtype(Box::new(v)))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeSeq(vec![]))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializeTuple(vec![]))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializeTupleStruct(vec![]))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariant(vec![]))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap {
            map: BTreeMap::new(),
            key: None,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeStruct(BTreeMap::new()))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeStructVariant(BTreeMap::new()))
    }
}

pub struct SerializeSeq(Vec<Document>);

impl ser::SerializeSeq for SerializeSeq {
    type Ok = Document;
    type Error = SerializerError;

    fn serialize_element<T: ?Sized>(&mut self, document: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let document = document.serialize(Serializer)?;
        self.0.push(document);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Seq(self.0))
    }
}

pub struct SerializeTuple(Vec<Document>);

impl ser::SerializeTuple for SerializeTuple {
    type Ok = Document;
    type Error = SerializerError;

    fn serialize_element<T: ?Sized>(&mut self, document: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let document = document.serialize(Serializer)?;
        self.0.push(document);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Seq(self.0))
    }
}

pub struct SerializeTupleStruct(Vec<Document>);

impl ser::SerializeTupleStruct for SerializeTupleStruct {
    type Ok = Document;
    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(&mut self, document: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let document = document.serialize(Serializer)?;
        self.0.push(document);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Seq(self.0))
    }
}

pub struct SerializeTupleVariant(Vec<Document>);

impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Document;
    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(&mut self, document: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let document = document.serialize(Serializer)?;
        self.0.push(document);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Seq(self.0))
    }
}

pub struct SerializeMap {
    map: BTreeMap<Document, Document>,
    key: Option<Document>,
}

impl ser::SerializeMap for SerializeMap {
    type Ok = Document;
    type Error = SerializerError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let key = key.serialize(Serializer)?;
        self.key = Some(key);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.map.insert(self.key.take().unwrap(), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Map(self.map))
    }
}

pub struct SerializeStruct(BTreeMap<Document, Document>);

impl ser::SerializeStruct for SerializeStruct {
    type Ok = Document;
    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        document: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let key = Document::String(key.to_string());
        let document = document.serialize(Serializer)?;
        self.0.insert(key, document);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Map(self.0))
    }
}

pub struct SerializeStructVariant(BTreeMap<Document, Document>);

impl ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Document;
    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        document: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let key = Document::String(key.to_string());
        let document = document.serialize(Serializer)?;
        self.0.insert(key, document);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Document::Map(self.0))
    }
}
