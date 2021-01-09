use super::*;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

impl Hash for Number {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.discriminant().hash(hasher);
        match *self {
            Number::U8(v) => v.hash(hasher),
            Number::U16(v) => v.hash(hasher),
            Number::U32(v) => v.hash(hasher),
            Number::U64(v) => v.hash(hasher),
            Number::U128(v) => v.hash(hasher),
            Number::I8(v) => v.hash(hasher),
            Number::I16(v) => v.hash(hasher),
            Number::I32(v) => v.hash(hasher),
            Number::I64(v) => v.hash(hasher),
            Number::I128(v) => v.hash(hasher),
            Number::F32(v) => OrderedFloat(v).hash(hasher),
            Number::F64(v) => OrderedFloat(v).hash(hasher),
        }
    }
}

impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Number {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (Number::I128(i), n) => i.cmp(&i128::from(n)),
            (Number::U128(i), n) => i.cmp(&u128::from(n)),
            (Number::F64(i), n) => OrderedFloat(*i).cmp(&OrderedFloat(f64::from(n))),
            (Number::I64(i), n) => i.cmp(&i64::from(n)),
            (Number::U64(i), n) => i.cmp(&u64::from(n)),
            (Number::F32(i), n) => OrderedFloat(*i).cmp(&OrderedFloat(f32::from(n))),
            (Number::I32(i), n) => i.cmp(&i32::from(n)),
            (Number::U32(i), n) => i.cmp(&u32::from(n)),
            (Number::I16(i), n) => i.cmp(&i16::from(n)),
            (Number::U16(i), n) => i.cmp(&u16::from(n)),
            (Number::I8(i), n) => i.cmp(&i8::from(n)),
            (Number::U8(i), n) => i.cmp(&u8::from(n)),
        }
    }
}

impl std::ops::Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::I128(i), n) => (i + i128::from(n)).into(),
            (Number::U128(i), n) => (i + u128::from(n)).into(),
            (Number::F64(i), n) => (i + f64::from(n)).into(),
            (Number::I64(i), n) => (i + i64::from(n)).into(),
            (Number::U64(i), n) => (i + u64::from(n)).into(),
            (Number::F32(i), n) => (i + f32::from(n)).into(),
            (Number::I32(i), n) => (i + i32::from(n)).into(),
            (Number::U32(i), n) => (i + u32::from(n)).into(),
            (Number::I16(i), n) => (i + i16::from(n)).into(),
            (Number::U16(i), n) => (i + u16::from(n)).into(),
            (Number::I8(i), n) => (i + i8::from(n)).into(),
            (Number::U8(i), n) => (i + u8::from(n)).into(),
        }
    }
}

impl PartialEq<Number> for Number {
    fn eq(&self, rhs: &Number) -> bool {
        match (self, rhs) {
            (Number::I128(i), n) => i == &i128::from(n),
            (Number::U128(i), n) => i == &u128::from(n),
            (Number::F64(i), n) => OrderedFloat(*i) == OrderedFloat(f64::from(n)),
            (Number::I64(i), n) => i == &i64::from(n),
            (Number::U64(i), n) => i == &u64::from(n),
            (Number::F32(i), n) => OrderedFloat(*i) == OrderedFloat(f32::from(n)),
            (Number::I32(i), n) => i == &i32::from(n),
            (Number::U32(i), n) => i == &u32::from(n),
            (Number::I16(i), n) => i == &i16::from(n),
            (Number::U16(i), n) => i == &u16::from(n),
            (Number::I8(i), n) => i == &i8::from(n),
            (Number::U8(i), n) => i == &u8::from(n),
        }
    }
}

impl<T: UnstructuredDataTrait> PartialEq<Unstructured<T>> for Number {
    fn eq(&self, rhs: &Unstructured<T>) -> bool {
        match (rhs, self) {
            (Unstructured::<T>::Number(n1), n2) => n1 == n2,
            (Unstructured::<T>::Option(Some(n1)), n2) => &**n1 == n2,
            (Unstructured::<T>::Newtype(n1), n2) => &**n1 == n2,
            _ => false,
        }
    }
}

macro_rules! impl_partial_eq_number {
    ( $( $type:ty )* ) => {
        $(
            impl PartialEq<Number> for $type {
                fn eq(&self, rhs: &Number) -> bool {
                    &Number::from(self) == rhs
                }
            }

            impl PartialEq<$type> for Number {
                fn eq(&self, rhs: & $type) -> bool {
                    &Number::from(rhs) == self
                }
            }
        )*
    };
}
impl_partial_eq_number! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 }
