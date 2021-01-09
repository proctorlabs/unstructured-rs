use super::*;

impl Serialize for Number {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            Number::U8(n) => serializer.serialize_u8(*n),
            Number::U16(n) => serializer.serialize_u16(*n),
            Number::U32(n) => serializer.serialize_u32(*n),
            Number::U64(n) => serializer.serialize_u64(*n),
            Number::U128(n) => serializer.serialize_u128(*n),
            Number::I8(n) => serializer.serialize_i8(*n),
            Number::I16(n) => serializer.serialize_i16(*n),
            Number::I32(n) => serializer.serialize_i32(*n),
            Number::I64(n) => serializer.serialize_i64(*n),
            Number::I128(n) => serializer.serialize_i128(*n),
            Number::F32(f) => serializer.serialize_f32(*f),
            Number::F64(f) => serializer.serialize_f64(*f),
        }
    }
}
