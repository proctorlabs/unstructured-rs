use super::*;

macro_rules! from_imp {
    ( $($ty:ty, $v:ident)* ) => {
        $(
            impl From<&$ty> for Number {
                fn from(n: &$ty) -> Self {
                    Number::$v(n.to_owned())
                }
            }

            impl From<&Number> for $ty {
                fn from(n: &Number) -> Self {
                    match n {
                        Number::U8(n) => *n as $ty,
                        Number::U16(n) => *n as $ty,
                        Number::U32(n) => *n as $ty,
                        Number::U64(n) => *n as $ty,
                        Number::U128(n) => *n as $ty,
                        Number::I8(n) => *n as $ty,
                        Number::I16(n) => *n as $ty,
                        Number::I32(n) => *n as $ty,
                        Number::I64(n) => *n as $ty,
                        Number::I128(n) => *n as $ty,
                        Number::F32(n) => *n as $ty,
                        Number::F64(n) => *n as $ty,
                    }
                }
            }

            impl From<$ty> for Number {
                fn from(n: $ty) -> Self {
                    Number::$v(n as $ty)
                }
            }

            impl From<Number> for $ty {
                fn from(n: Number) -> Self {
                    match n {
                        Number::U8(n) => n as $ty,
                        Number::U16(n) => n as $ty,
                        Number::U32(n) => n as $ty,
                        Number::U64(n) => n as $ty,
                        Number::U128(n) => n as $ty,
                        Number::I8(n) => n as $ty,
                        Number::I16(n) => n as $ty,
                        Number::I32(n) => n as $ty,
                        Number::I64(n) => n as $ty,
                        Number::I128(n) => n as $ty,
                        Number::F32(n) => n as $ty,
                        Number::F64(n) => n as $ty,
                    }
                }
            }
        )*
    };
}

from_imp! {
    i8,I8 i16,I16 i32,I32 i64,I64 i128,I128
    u8,U8 u16,U16 u32,U32 u64,U64 u128,U128
    f32,F32 f64,F64
}

impl From<usize> for Number {
    fn from(n: usize) -> Self {
        Number::U64(n as u64)
    }
}

impl From<&usize> for Number {
    fn from(n: &usize) -> Self {
        Number::U64(*n as u64)
    }
}

impl From<isize> for Number {
    fn from(n: isize) -> Self {
        Number::I64(n as i64)
    }
}

impl From<&isize> for Number {
    fn from(n: &isize) -> Self {
        Number::I64(*n as i64)
    }
}
