use super::Document;
use std::collections::BTreeMap;

macro_rules! from_imp {
    ( &{ $($ref_ty:ty, $ref_v:ident)* } *{ $($ty:ty, $v:ident)* } ) => {
        $(
            impl From<&$ref_ty> for Document {
                fn from(n: &$ref_ty) -> Self {
                    Document::$ref_v(n.to_owned())
                }
            }
        )*

        $(
            impl From<$ty> for Document {
                fn from(n: $ty) -> Self {
                    Document::$v(n as $ty)
                }
            }
        )*
    };
}

from_imp! {
    &{
        i8,I8 i16,I16 i32,I32 i64,I64 i128,I128
        u8,U8 u16,U16 u32,U32 u64,U64 u128,U128
        f32,F32 f64,F64
        bool,Bool
        char,Char
        String,String str,String
        Vec<u8>,Bytes
        Vec<Document>,Seq
        BTreeMap<Document, Document>,Map
    }

    *{
        i8,I8 i16,I16 i32,I32 i64,I64 i128,I128
        u8,U8 u16,U16 u32,U32 u64,U64 u128,U128
        f32,F32 f64,F64
        bool,Bool
        char,Char
        String,String
        Vec<u8>,Bytes
        Vec<Document>,Seq
        BTreeMap<Document, Document>,Map
    }
}

impl From<&usize> for Document {
    fn from(n: &usize) -> Self {
        Document::U64(*n as u64)
    }
}

impl From<&isize> for Document {
    fn from(n: &isize) -> Self {
        Document::I64(*n as i64)
    }
}
