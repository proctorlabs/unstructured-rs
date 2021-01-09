use serde::ser;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

use crate::*;

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
        "Unstructured::<T>: serializer error"
    }
}

impl ser::Error for SerializerError {
    fn custom<T: fmt::Display>(msg: T) -> SerializerError {
        SerializerError::Custom(msg.to_string())
    }
}

impl<T: UnstructuredDataTrait> ser::Serialize for Unstructured<T> {
    fn serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match *self {
            Unstructured::<T>::Bool(v) => s.serialize_bool(v),
            Unstructured::<T>::Number(ref v) => v.serialize(s),
            Unstructured::<T>::Char(v) => s.serialize_char(v),
            Unstructured::<T>::String(ref v) => s.serialize_str(v),
            Unstructured::<T>::Null => s.serialize_unit(),
            Unstructured::<T>::Option(None) => s.serialize_none(),
            Unstructured::<T>::Option(Some(ref v)) => s.serialize_some(v),
            Unstructured::<T>::Newtype(ref v) => s.serialize_newtype_struct("", v),
            Unstructured::<T>::Seq(ref v) => v.serialize(s),
            Unstructured::<T>::Map(ref v) => v.serialize(s),
            Unstructured::<T>::Bytes(ref v) => s.serialize_bytes(v),
            Unstructured::<T>::Unassigned => s.serialize_unit(),
            Unstructured::<T>::Err(ref e) => s.serialize_str(e.to_string().as_str()),
            Unstructured::<T>::Other(..) => s.serialize_str("other"),
        }
    }
}

pub struct Serializer<T: UnstructuredDataTrait>(std::marker::PhantomData<T>);

impl<T: UnstructuredDataTrait> Serializer<T> {
    pub fn new() -> Self {
        Serializer(PhantomData)
    }
}

impl<T: UnstructuredDataTrait> ser::Serializer for Serializer<T> {
    type Ok = Unstructured<T>;
    type Error = SerializerError;
    type SerializeSeq = SerializeSeq<T>;
    type SerializeTuple = SerializeTuple<T>;
    type SerializeTupleStruct = SerializeTupleStruct<T>;
    type SerializeTupleVariant = SerializeTupleVariant<T>;
    type SerializeMap = SerializeMap<T>;
    type SerializeStruct = SerializeStruct<T>;
    type SerializeStructVariant = SerializeStructVariant<T>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Number(Number::I8(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Number(Number::I16(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Number(Number::I32(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Number(Number::I64(v)))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Number(Number::I128(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Number(Number::U8(v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Number(Number::U16(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Number(Number::U32(v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Number(Number::U64(v)))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Number(Number::U128(v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Number(Number::F32(v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Number(Number::F64(v)))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Bytes(v.to_vec()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Option(None))
    }

    fn serialize_some<Q: ?Sized>(self, document: &Q) -> Result<Self::Ok, Self::Error>
    where
        Q: ser::Serialize,
    {
        document
            .serialize(Serializer(PhantomData))
            .map(|v| Unstructured::<T>::Option(Some(Box::new(v))))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Null)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Null)
    }

    fn serialize_newtype_struct<Q: ?Sized>(
        self,
        _name: &'static str,
        document: &Q,
    ) -> Result<Self::Ok, Self::Error>
    where
        Q: ser::Serialize,
    {
        document
            .serialize(Serializer(PhantomData))
            .map(|v| Unstructured::<T>::Newtype(Box::new(v)))
    }

    fn serialize_newtype_variant<Q: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        document: &Q,
    ) -> Result<Self::Ok, Self::Error>
    where
        Q: ser::Serialize,
    {
        document
            .serialize(Serializer(PhantomData))
            .map(|v| Unstructured::<T>::Newtype(Box::new(v)))
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

pub struct SerializeSeq<T: UnstructuredDataTrait>(Sequence<T>);

impl<T: UnstructuredDataTrait> ser::SerializeSeq for SerializeSeq<T> {
    type Ok = Unstructured<T>;
    type Error = SerializerError;

    fn serialize_element<Q: ?Sized>(&mut self, document: &Q) -> Result<(), Self::Error>
    where
        Q: ser::Serialize,
    {
        let document = document.serialize(Serializer(PhantomData))?;
        self.0.push(document);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Seq(self.0))
    }
}

pub struct SerializeTuple<T: UnstructuredDataTrait>(Sequence<T>);

impl<T: UnstructuredDataTrait> ser::SerializeTuple for SerializeTuple<T> {
    type Ok = Unstructured<T>;
    type Error = SerializerError;

    fn serialize_element<Q: ?Sized>(&mut self, document: &Q) -> Result<(), Self::Error>
    where
        Q: ser::Serialize,
    {
        let document = document.serialize(Serializer(PhantomData))?;
        self.0.push(document);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Seq(self.0))
    }
}

pub struct SerializeTupleStruct<T: UnstructuredDataTrait>(Sequence<T>);

impl<T: UnstructuredDataTrait> ser::SerializeTupleStruct for SerializeTupleStruct<T> {
    type Ok = Unstructured<T>;
    type Error = SerializerError;

    fn serialize_field<Q: ?Sized>(&mut self, document: &Q) -> Result<(), Self::Error>
    where
        Q: ser::Serialize,
    {
        let document = document.serialize(Serializer(PhantomData))?;
        self.0.push(document);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<T>::Seq(self.0))
    }
}

pub struct SerializeTupleVariant<T: UnstructuredDataTrait>(Sequence<T>);

impl<Q: UnstructuredDataTrait> ser::SerializeTupleVariant for SerializeTupleVariant<Q> {
    type Ok = Unstructured<Q>;
    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(&mut self, document: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let document = document.serialize(Serializer(PhantomData))?;
        self.0.push(document);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<Q>::Seq(self.0))
    }
}

pub struct SerializeMap<T: UnstructuredDataTrait> {
    map: Mapping<T>,
    key: Option<Unstructured<T>>,
}

impl<R: UnstructuredDataTrait> ser::SerializeMap for SerializeMap<R> {
    type Ok = Unstructured<R>;
    type Error = SerializerError;

    fn serialize_key<Q: ?Sized>(&mut self, key: &Q) -> Result<(), Self::Error>
    where
        Q: ser::Serialize,
    {
        let key = key.serialize(Serializer(PhantomData))?;
        self.key = Some(key);
        Ok(())
    }

    fn serialize_value<Q: ?Sized>(&mut self, value: &Q) -> Result<(), Self::Error>
    where
        Q: ser::Serialize,
    {
        let value = value.serialize(Serializer(PhantomData))?;
        self.map.insert(self.key.take().unwrap(), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<R>::Map(self.map))
    }
}

pub struct SerializeStruct<T: UnstructuredDataTrait>(Mapping<T>);

impl<Q: UnstructuredDataTrait> ser::SerializeStruct for SerializeStruct<Q> {
    type Ok = Unstructured<Q>;
    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        document: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let key = Unstructured::<Q>::String(key.to_string());
        let document = document.serialize(Serializer(PhantomData))?;
        self.0.insert(key, document);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<Q>::Map(self.0))
    }
}

pub struct SerializeStructVariant<T: UnstructuredDataTrait>(
    Mapping<T>,
);

impl<Q: UnstructuredDataTrait> ser::SerializeStructVariant for SerializeStructVariant<Q> {
    type Ok = Unstructured<Q>;
    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        document: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let key = Unstructured::<Q>::String(key.to_string());
        let document = document.serialize(Serializer(PhantomData))?;
        self.0.insert(key, document);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Unstructured::<Q>::Map(self.0))
    }
}
