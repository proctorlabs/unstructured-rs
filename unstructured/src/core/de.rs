use serde::de;
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

use crate::*;

#[derive(Debug)]
pub enum Unexpected {
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Char(char),
    Str(String),
    Bytes(Vec<u8>),
    Unit,
    Option,
    NewtypeStruct,
    Seq,
    Map,
    Enum,
    UnitVariant,
    NewtypeVariant,
    TupleVariant,
    StructVariant,
    Other(String),
}

impl<'a> From<de::Unexpected<'a>> for Unexpected {
    fn from(unexp: de::Unexpected) -> Unexpected {
        match unexp {
            de::Unexpected::Bool(v) => Unexpected::Bool(v),
            de::Unexpected::Unsigned(v) => Unexpected::Unsigned(v),
            de::Unexpected::Signed(v) => Unexpected::Signed(v),
            de::Unexpected::Float(v) => Unexpected::Float(v),
            de::Unexpected::Char(v) => Unexpected::Char(v),
            de::Unexpected::Str(v) => Unexpected::Str(v.to_owned()),
            de::Unexpected::Bytes(v) => Unexpected::Bytes(v.to_owned()),
            de::Unexpected::Unit => Unexpected::Unit,
            de::Unexpected::Option => Unexpected::Option,
            de::Unexpected::NewtypeStruct => Unexpected::NewtypeStruct,
            de::Unexpected::Seq => Unexpected::Seq,
            de::Unexpected::Map => Unexpected::Map,
            de::Unexpected::Enum => Unexpected::Enum,
            de::Unexpected::UnitVariant => Unexpected::UnitVariant,
            de::Unexpected::NewtypeVariant => Unexpected::NewtypeVariant,
            de::Unexpected::TupleVariant => Unexpected::TupleVariant,
            de::Unexpected::StructVariant => Unexpected::StructVariant,
            de::Unexpected::Other(v) => Unexpected::Other(v.to_owned()),
        }
    }
}

#[allow(clippy::needless_lifetimes)]
impl Unexpected {
    pub fn to_unexpected<'a>(&'a self) -> de::Unexpected<'a> {
        match *self {
            Unexpected::Bool(v) => de::Unexpected::Bool(v),
            Unexpected::Unsigned(v) => de::Unexpected::Unsigned(v),
            Unexpected::Signed(v) => de::Unexpected::Signed(v),
            Unexpected::Float(v) => de::Unexpected::Float(v),
            Unexpected::Char(v) => de::Unexpected::Char(v),
            Unexpected::Str(ref v) => de::Unexpected::Str(v),
            Unexpected::Bytes(ref v) => de::Unexpected::Bytes(v),
            Unexpected::Unit => de::Unexpected::Unit,
            Unexpected::Option => de::Unexpected::Option,
            Unexpected::NewtypeStruct => de::Unexpected::NewtypeStruct,
            Unexpected::Seq => de::Unexpected::Seq,
            Unexpected::Map => de::Unexpected::Map,
            Unexpected::Enum => de::Unexpected::Enum,
            Unexpected::UnitVariant => de::Unexpected::UnitVariant,
            Unexpected::NewtypeVariant => de::Unexpected::NewtypeVariant,
            Unexpected::TupleVariant => de::Unexpected::TupleVariant,
            Unexpected::StructVariant => de::Unexpected::StructVariant,
            Unexpected::Other(ref v) => de::Unexpected::Other(v),
        }
    }
}

#[derive(Debug)]
pub enum DeserializerError {
    Custom(String),
    InvalidType(Unexpected, String),
    InvalidValue(Unexpected, String),
    InvalidLength(usize, String),
    UnknownVariant(String, &'static [&'static str]),
    UnknownField(String, &'static [&'static str]),
    MissingField(&'static str),
    DuplicateField(&'static str),
}

impl de::Error for DeserializerError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        DeserializerError::Custom(msg.to_string())
    }

    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        DeserializerError::InvalidType(unexp.into(), exp.to_string())
    }

    fn invalid_value(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        DeserializerError::InvalidValue(unexp.into(), exp.to_string())
    }

    fn invalid_length(len: usize, exp: &dyn de::Expected) -> Self {
        DeserializerError::InvalidLength(len, exp.to_string())
    }

    fn unknown_variant(field: &str, expected: &'static [&'static str]) -> Self {
        DeserializerError::UnknownVariant(field.into(), expected)
    }

    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        DeserializerError::UnknownField(field.into(), expected)
    }

    fn missing_field(field: &'static str) -> Self {
        DeserializerError::MissingField(field)
    }

    fn duplicate_field(field: &'static str) -> Self {
        DeserializerError::DuplicateField(field)
    }
}

impl DeserializerError {
    pub fn to_error<E: de::Error>(&self) -> E {
        match *self {
            DeserializerError::Custom(ref msg) => E::custom(msg.clone()),
            DeserializerError::InvalidType(ref unexp, ref exp) => {
                E::invalid_type(unexp.to_unexpected(), &&**exp)
            }
            DeserializerError::InvalidValue(ref unexp, ref exp) => {
                E::invalid_value(unexp.to_unexpected(), &&**exp)
            }
            DeserializerError::InvalidLength(len, ref exp) => E::invalid_length(len, &&**exp),
            DeserializerError::UnknownVariant(ref field, exp) => E::unknown_variant(field, exp),
            DeserializerError::UnknownField(ref field, exp) => E::unknown_field(field, exp),
            DeserializerError::MissingField(field) => E::missing_field(field),
            DeserializerError::DuplicateField(field) => E::missing_field(field),
        }
    }

    pub fn into_error<E: de::Error>(self) -> E {
        self.to_error()
    }
}

impl Error for DeserializerError {
    fn description(&self) -> &str {
        "Document deserializer error"
    }
}

impl fmt::Display for DeserializerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeserializerError::Custom(ref msg) => write!(f, "{}", msg),
            DeserializerError::InvalidType(ref unexp, ref exp) => write!(
                f,
                "Invalid type {}. Expected {}",
                unexp.to_unexpected(),
                exp
            ),
            DeserializerError::InvalidValue(ref unexp, ref exp) => write!(
                f,
                "Invalid Value {}. Expected {}",
                unexp.to_unexpected(),
                exp
            ),
            DeserializerError::InvalidLength(len, ref exp) => {
                write!(f, "Invalid length {}. Expected {}", len, exp)
            }
            DeserializerError::UnknownVariant(ref field, exp) => write!(
                f,
                "Unknown variant {}. Expected one of {}",
                field,
                exp.join(", ")
            ),
            DeserializerError::UnknownField(ref field, exp) => write!(
                f,
                "Unknown field {}. Expected one of {}",
                field,
                exp.join(", ")
            ),
            DeserializerError::MissingField(field) => write!(f, "Missing field {}", field),
            DeserializerError::DuplicateField(field) => write!(f, "Duplicate field {}", field),
        }
    }
}

impl From<de::value::Error> for DeserializerError {
    fn from(e: de::value::Error) -> DeserializerError {
        DeserializerError::Custom(e.to_string())
    }
}

pub struct DocumentVisitor<T: UnstructuredDataTrait>(std::marker::PhantomData<T>);

impl<'de, T: UnstructuredDataTrait> de::Visitor<'de> for DocumentVisitor<T> {
    type Value = Unstructured<T>;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("any value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Bool(value))
    }

    fn visit_i8<E>(self, value: i8) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Number(Number::I8(value)))
    }

    fn visit_i16<E>(self, value: i16) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Number(Number::I16(value)))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Number(Number::I32(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Number(Number::I64(value)))
    }

    fn visit_i128<E>(self, value: i128) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Number(Number::I128(value)))
    }

    fn visit_u8<E>(self, value: u8) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Number(Number::U8(value)))
    }

    fn visit_u16<E>(self, value: u16) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Number(Number::U16(value)))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Number(Number::U32(value)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Number(Number::U64(value)))
    }

    fn visit_u128<E>(self, value: u128) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Number(Number::U128(value)))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Number(Number::F32(value)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Number(Number::F64(value)))
    }

    fn visit_char<E>(self, value: char) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Char(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::String(value.into()))
    }

    fn visit_string<E>(self, value: String) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::String(value))
    }

    fn visit_unit<E>(self) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Null)
    }

    fn visit_none<E>(self) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Option(None))
    }

    fn visit_some<D: de::Deserializer<'de>>(self, d: D) -> Result<Unstructured<T>, D::Error> {
        d.deserialize_any(DocumentVisitor::<T>(PhantomData))
            .map(|v| Unstructured::<T>::Option(Some(Box::new(v))))
    }

    fn visit_newtype_struct<D: de::Deserializer<'de>>(
        self,
        d: D,
    ) -> Result<Unstructured<T>, D::Error> {
        d.deserialize_any(DocumentVisitor::<T>(PhantomData))
            .map(|v| Unstructured::<T>::Newtype(Box::new(v)))
    }

    fn visit_seq<V: de::SeqAccess<'de>>(self, mut visitor: V) -> Result<Unstructured<T>, V::Error> {
        let mut documents = Vec::new();
        while let Some(elem) = visitor.next_element()? {
            documents.push(elem);
        }
        Ok(Unstructured::<T>::Seq(documents))
    }

    fn visit_map<V: de::MapAccess<'de>>(self, mut visitor: V) -> Result<Unstructured<T>, V::Error> {
        let mut documents = Mapping::new();
        while let Some((key, document)) = visitor.next_entry()? {
            documents.insert(key, document);
        }
        Ok(Unstructured::<T>::Map(documents))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Bytes(v.into()))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Unstructured<T>, E> {
        Ok(Unstructured::<T>::Bytes(v))
    }
}

impl<'de, T: UnstructuredDataTrait> de::Deserialize<'de> for Unstructured<T> {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(DocumentVisitor::<T>(PhantomData))
    }
}

impl<'de, T: UnstructuredDataTrait> de::IntoDeserializer<'de, DeserializerError>
    for Unstructured<T>
{
    type Deserializer = Unstructured<T>;

    fn into_deserializer(self) -> Unstructured<T> {
        self
    }
}

pub struct DocumentDeserializer<E, T: UnstructuredDataTrait> {
    document: Unstructured<T>,
    error: PhantomData<fn() -> E>,
}

impl<E, T: UnstructuredDataTrait> DocumentDeserializer<E, T> {
    pub fn new(document: Unstructured<T>) -> Self {
        DocumentDeserializer {
            document,
            error: Default::default(),
        }
    }
}

impl<'de, E, T: UnstructuredDataTrait> de::Deserializer<'de> for DocumentDeserializer<E, T>
where
    E: de::Error,
{
    type Error = E;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.document {
            Unstructured::<T>::Bool(v) => visitor.visit_bool(v),
            Unstructured::<T>::Number(v) => Ok(v.deserialize_any(visitor).unwrap()),
            Unstructured::<T>::Char(v) => visitor.visit_char(v),
            Unstructured::<T>::String(v) => visitor.visit_string(v),
            Unstructured::<T>::Null => visitor.visit_unit(),
            Unstructured::<T>::Option(None) => visitor.visit_none(),
            Unstructured::<T>::Option(Some(v)) => visitor.visit_some(DocumentDeserializer::new(*v)),
            Unstructured::<T>::Newtype(v) => {
                visitor.visit_newtype_struct(DocumentDeserializer::new(*v))
            }
            Unstructured::<T>::Seq(v) => visitor.visit_seq(de::value::SeqDeserializer::new(
                v.into_iter().map(DocumentDeserializer::new),
            )),
            Unstructured::<T>::Map(v) => visitor
                .visit_map(de::value::MapDeserializer::new(v.into_iter().map(
                    |(k, v)| (DocumentDeserializer::new(k), DocumentDeserializer::new(v)),
                ))),
            Unstructured::<T>::Bytes(v) => visitor.visit_byte_buf(v),
            Unstructured::<T>::Unassigned => visitor.visit_unit(),
            Unstructured::<T>::Err(e) => {
                Err(DeserializerError::Custom(format!("{}", e)).to_error())
            }
            Unstructured::<T>::Other(..) => {
                Err(DeserializerError::Custom("other".into()).to_error())
            }
        }
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.document {
            Unstructured::<T>::Option(..) => self.deserialize_any(visitor),
            Unstructured::<T>::Null => visitor.visit_unit(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        let (variant, document) = match self.document {
            Unstructured::<T>::Map(document) => {
                let mut iter = document.into_iter();
                let (variant, document) = match iter.next() {
                    Some(v) => v,
                    None => {
                        return Err(de::Error::invalid_value(
                            de::Unexpected::Map,
                            &"map with a single key",
                        ));
                    }
                };
                // enums are encoded as maps with a single key:Document pair
                if iter.next().is_some() {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Map,
                        &"map with a single key",
                    ));
                }
                (variant, Some(document))
            }
            Unstructured::<T>::String(variant) => (Unstructured::<T>::String(variant), None),
            other => {
                return Err(de::Error::invalid_type(
                    other.unexpected(),
                    &"string or map",
                ));
            }
        };

        let d = EnumDeserializer {
            variant,
            document,
            error: Default::default(),
        };
        visitor.visit_enum(d)
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        match self.document {
            Unstructured::<T>::Newtype(v) => {
                visitor.visit_newtype_struct(DocumentDeserializer::new(*v))
            }
            _ => visitor.visit_newtype_struct(self),
        }
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 char str string unit
        seq bytes byte_buf map unit_struct
        tuple_struct struct tuple ignored_any identifier
    }
}

impl<'de, E, T: UnstructuredDataTrait> de::IntoDeserializer<'de, E> for DocumentDeserializer<E, T>
where
    E: de::Error,
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de, T: UnstructuredDataTrait> de::Deserializer<'de> for Unstructured<T> {
    type Error = DeserializerError;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        DocumentDeserializer::new(self).deserialize_any(visitor)
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        DocumentDeserializer::new(self).deserialize_option(visitor)
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        DocumentDeserializer::new(self).deserialize_enum(name, variants, visitor)
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        DocumentDeserializer::new(self).deserialize_newtype_struct(name, visitor)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 char str string unit
        seq bytes byte_buf map unit_struct
        tuple_struct struct tuple ignored_any identifier
    }
}

struct EnumDeserializer<E, T: UnstructuredDataTrait> {
    variant: Unstructured<T>,
    document: Option<Unstructured<T>>,
    error: PhantomData<fn() -> E>,
}

#[allow(clippy::type_complexity)]
impl<'de, E, T: UnstructuredDataTrait> de::EnumAccess<'de> for EnumDeserializer<E, T>
where
    E: de::Error,
{
    type Error = E;
    type Variant = VariantDeserializer<Self::Error, T>;

    fn variant_seed<V>(
        self,
        seed: V,
    ) -> Result<(V::Value, VariantDeserializer<Self::Error, T>), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let visitor = VariantDeserializer {
            document: self.document,
            error: Default::default(),
        };
        seed.deserialize(DocumentDeserializer::new(self.variant))
            .map(|v| (v, visitor))
    }
}

struct VariantDeserializer<E, T: UnstructuredDataTrait> {
    document: Option<Unstructured<T>>,
    error: PhantomData<fn() -> E>,
}

impl<'de, E, T: UnstructuredDataTrait> de::VariantAccess<'de> for VariantDeserializer<E, T>
where
    E: de::Error,
{
    type Error = E;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.document {
            Some(document) => de::Deserialize::deserialize(DocumentDeserializer::new(document)),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<Q>(self, seed: Q) -> Result<Q::Value, Self::Error>
    where
        Q: de::DeserializeSeed<'de>,
    {
        match self.document {
            Some(document) => seed.deserialize(DocumentDeserializer::new(document)),
            None => Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.document {
            Some(Unstructured::<T>::Seq(v)) => de::Deserializer::deserialize_any(
                de::value::SeqDeserializer::new(v.into_iter().map(DocumentDeserializer::new)),
                visitor,
            ),
            Some(other) => Err(de::Error::invalid_type(
                other.unexpected(),
                &"tuple variant",
            )),
            None => Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"tuple variant",
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.document {
            Some(Unstructured::<T>::Map(v)) => de::Deserializer::deserialize_any(
                de::value::MapDeserializer::new(
                    v.into_iter()
                        .map(|(k, v)| (DocumentDeserializer::new(k), DocumentDeserializer::new(v))),
                ),
                visitor,
            ),
            Some(other) => Err(de::Error::invalid_type(
                other.unexpected(),
                &"struct variant",
            )),
            None => Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }
}
