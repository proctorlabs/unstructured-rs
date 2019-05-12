use super::Document;
use std::collections::BTreeMap;

macro_rules! from_imp {
    ($($ty:ident, $v:ident)*) => {
        $(
            impl From<$ty> for Document {
                fn from(n: $ty) -> Self {
                    Document::$v(n)
                }
            }
        )*
    };
}

from_imp! {
    i8,I8 i16,I16 i32,I32 i64,I64
    u8,U8 u16,U16 u32,U32 u64,U64
    f32,F32 f64,F64
    bool,Bool char,Char
    String,String
}

impl From<&str> for Document {
    fn from(n: &str) -> Self {
        Document::String(n.into())
    }
}

impl From<Vec<u8>> for Document {
    fn from(n: Vec<u8>) -> Self {
        Document::Bytes(n)
    }
}

impl From<Vec<Document>> for Document {
    fn from(n: Vec<Document>) -> Self {
        Document::Seq(n)
    }
}

impl From<BTreeMap<Document, Document>> for Document {
    fn from(n: BTreeMap<Document, Document>) -> Self {
        Document::Map(n)
    }
}
