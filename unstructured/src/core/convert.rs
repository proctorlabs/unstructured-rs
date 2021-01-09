use crate::*;

pub trait DocumentConvertible<T: UnstructuredDataTrait>: Sized {
    fn is(val: &Unstructured<T>) -> bool;

    fn into_unstructured(self) -> Unstructured<T>;

    fn into_native(val: Unstructured<T>) -> Option<Self>;

    fn castable(val: &Unstructured<T>) -> bool;

    fn cast(val: Unstructured<T>) -> Option<Self>;
}

impl<Q: UnstructuredDataTrait> Unstructured<Q> {
    pub fn is<T: DocumentConvertible<Q>>(&self) -> bool {
        T::is(self)
    }

    pub fn unwrap<T: DocumentConvertible<Q>>(self) -> T {
        T::into_native(self).unwrap()
    }

    pub fn cast<T: DocumentConvertible<Q>>(self) -> Option<T> {
        T::cast(self)
    }
}

macro_rules! impl_document_convertible {
    ( $( $t:ty : $variant:ident ( $u:ty ) => $( $variant2:ident ( $variant2_ty:ident ) )* , )* ) => {
        $(
            impl<T: UnstructuredDataTrait> DocumentConvertible<T> for $t {
                fn into_unstructured(self) -> Unstructured<T> {
                    Unstructured::<T>::Number(Number::$variant(self as $u))
                }

                fn into_native(val: Unstructured<T>) -> Option<Self> {
                    match val {
                        Unstructured::<T>::Number(Number::$variant(v)) => Some(v as $t),
                        _ => None,
                    }
                }

                fn is(val: &Unstructured<T>) -> bool {
                    match val {
                        Unstructured::<T>::Number(Number::$variant(_)) => true,
                        _ => false,
                    }
                }

                #[allow(clippy::float_cmp)]
                fn castable(val: &Unstructured<T>) -> bool {
                    match val {
                        $( Unstructured::<T>::Number(Number::$variant2(v)) => *v == (*v as $t) as $variant2_ty, )*
                        Unstructured::<T>::String(s) => match s.parse::< $t >() { Ok(_) => true, Err(_) => false },
                        Unstructured::<T>::Option(inner) => match inner { Some(v) => <$t>::castable(v), None => false },
                        Unstructured::<T>::Newtype(inner) => <$t>::castable(inner),
                        _ => false,
                    }
                }

                #[allow(clippy::float_cmp)]
                fn cast(val: Unstructured<T>) -> Option<Self> {
                    match val {
                        $( Unstructured::<T>::Number(Number::$variant2(v)) => if v == (v as $t) as $variant2_ty { Some(v as $t) } else { None }, )*
                        Unstructured::<T>::String(s) => match s.parse::< $t >() { Ok(v) => Some(v), Err(_) => None },
                        Unstructured::<T>::Option(inner) => match inner { Some(v) => v.cast(), None => None },
                        Unstructured::<T>::Newtype(inner) => inner.cast(),
                        _ => None,
                    }
                }
            }
        )*
    };

    ( $( $t:ty : $variant:ident  => $( $variant2:ident ( $variant2_ty:ident ) )* , )* ) => {
        $(
            impl_document_convertible!($t : $variant ( $t )  => $( $variant2 ( $variant2_ty ) )* , );
        )*
    };
}

impl_document_convertible! {
    i8:I8 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
    i16:I16 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
    i32:I32 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
    i64:I64 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
    i128:I128 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
    u8:U8 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
    u16:U16 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
    u32:U32 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
    u64:U64 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
    u128:U128 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
    f32:F32 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
    f64:F64 => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
}

impl_document_convertible! {
    isize:I64(i64) => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
    usize:U64(u64) => I8(i8) I16(i16) I32(i32) I64(i64) I128(i128) U8(u8) U16(u16) U32(u32) U64(u64) U128(u128) F32(f32) F64(f64),
}

impl<T: UnstructuredDataTrait> DocumentConvertible<T>
    for Mapping<T>
{
    fn into_unstructured(self) -> Unstructured<T> {
        Unstructured::<T>::Map(self)
    }

    fn into_native(val: Unstructured<T>) -> Option<Self> {
        match val {
            Unstructured::<T>::Map(v) => Some(v),
            _ => None,
        }
    }

    fn is(val: &Unstructured<T>) -> bool {
        matches!(val, Unstructured::<T>::Map(_))
    }

    fn castable(val: &Unstructured<T>) -> bool {
        matches!(val, Unstructured::<T>::Map(_))
    }

    fn cast(val: Unstructured<T>) -> Option<Self> {
        match val {
            Unstructured::<T>::Map(m) => Some(m),
            _ => None,
        }
    }
}

impl<T: UnstructuredDataTrait> DocumentConvertible<T> for Sequence<T> {
    fn into_unstructured(self) -> Unstructured<T> {
        Unstructured::<T>::Seq(self)
    }

    fn into_native(val: Unstructured<T>) -> Option<Self> {
        match val {
            Unstructured::<T>::Seq(v) => Some(v),
            _ => None,
        }
    }

    fn is(val: &Unstructured<T>) -> bool {
        matches!(val, Unstructured::<T>::Seq(_))
    }

    fn castable(val: &Unstructured<T>) -> bool {
        matches!(val, Unstructured::<T>::Seq(_))
    }

    fn cast(val: Unstructured<T>) -> Option<Self> {
        match val {
            Unstructured::<T>::Seq(m) => Some(m),
            _ => None,
        }
    }
}

impl<T: UnstructuredDataTrait> DocumentConvertible<T> for String {
    fn into_unstructured(self) -> Unstructured<T> {
        Unstructured::<T>::String(self)
    }

    fn into_native(val: Unstructured<T>) -> Option<Self> {
        match val {
            Unstructured::<T>::String(v) => Some(v),
            _ => None,
        }
    }

    fn is(val: &Unstructured<T>) -> bool {
        matches!(val, Unstructured::<T>::String(_))
    }

    fn castable(val: &Unstructured<T>) -> bool {
        matches!(val, Unstructured::<T>::String(_))
    }

    fn cast(val: Unstructured<T>) -> Option<Self> {
        match val {
            Unstructured::<T>::String(m) => Some(m),
            _ => None,
        }
    }
}

impl<T: UnstructuredDataTrait> DocumentConvertible<T> for bool {
    fn into_unstructured(self) -> Unstructured<T> {
        Unstructured::<T>::Bool(self)
    }

    fn into_native(val: Unstructured<T>) -> Option<Self> {
        match val {
            Unstructured::<T>::Bool(v) => Some(v),
            _ => None,
        }
    }

    fn is(val: &Unstructured<T>) -> bool {
        matches!(val, Unstructured::<T>::Bool(_))
    }

    fn castable(val: &Unstructured<T>) -> bool {
        matches!(val, Unstructured::<T>::Bool(_))
    }

    fn cast(val: Unstructured<T>) -> Option<Self> {
        match val {
            Unstructured::<T>::Bool(m) => Some(m),
            _ => None,
        }
    }
}
