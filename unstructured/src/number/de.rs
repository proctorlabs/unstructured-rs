use super::*;

impl<'de> Deserialize<'de> for Number {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Number, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumberVisitor;

        impl<'de> Visitor<'de> for NumberVisitor {
            type Value = Number;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number")
            }

            #[inline]
            fn visit_i8<E>(self, value: i8) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i16<E>(self, value: i16) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i32<E>(self, value: i32) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i128<E>(self, value: i128) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u8<E>(self, value: u8) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u16<E>(self, value: u16) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u32<E>(self, value: u32) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u128<E>(self, value: u128) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_f32<E>(self, value: f32) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Number, E> {
                Ok(value.into())
            }
        }

        deserializer.deserialize_any(NumberVisitor)
    }
}

impl<'de> Deserializer<'de> for Number {
    type Error = crate::de::DeserializerError;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, crate::de::DeserializerError>
    where
        V: Visitor<'de>,
    {
        match self {
            Number::U8(n) => visitor.visit_u8(n),
            Number::U16(n) => visitor.visit_u16(n),
            Number::U32(n) => visitor.visit_u32(n),
            Number::U64(i) => visitor.visit_u64(i),
            Number::U128(n) => visitor.visit_u128(n),
            Number::I8(n) => visitor.visit_i8(n),
            Number::I16(n) => visitor.visit_i16(n),
            Number::I32(n) => visitor.visit_i32(n),
            Number::I64(i) => visitor.visit_i64(i),
            Number::I128(n) => visitor.visit_i128(n),
            Number::F32(f) => visitor.visit_f32(f),
            Number::F64(f) => visitor.visit_f64(f),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de, 'a> Deserializer<'de> for &'a Number {
    type Error = crate::de::DeserializerError;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, crate::de::DeserializerError>
    where
        V: Visitor<'de>,
    {
        match self {
            Number::U8(n) => visitor.visit_u8(*n),
            Number::U16(n) => visitor.visit_u16(*n),
            Number::U32(n) => visitor.visit_u32(*n),
            Number::U64(i) => visitor.visit_u64(*i),
            Number::U128(n) => visitor.visit_u128(*n),
            Number::I8(n) => visitor.visit_i8(*n),
            Number::I16(n) => visitor.visit_i16(*n),
            Number::I32(n) => visitor.visit_i32(*n),
            Number::I64(i) => visitor.visit_i64(*i),
            Number::I128(n) => visitor.visit_i128(*n),
            Number::F32(f) => visitor.visit_f32(*f),
            Number::F64(f) => visitor.visit_f64(*f),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
