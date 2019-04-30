use std::collections::BTreeMap;
use std::fmt;
use std::ops;

use super::Document;

pub trait Index: private::Sealed {
    #[doc(hidden)]
    fn index_into<'v>(&self, v: &'v Document) -> Option<&'v Document>;

    #[doc(hidden)]
    fn index_into_mut<'v>(&self, v: &'v mut Document) -> Option<&'v mut Document>;

    #[doc(hidden)]
    fn index_or_insert<'v>(&self, v: &'v mut Document) -> &'v mut Document;
}

impl Index for usize {
    fn index_into<'v>(&self, v: &'v Document) -> Option<&'v Document> {
        match *v {
            Document::Seq(ref vec) => vec.get(*self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Document) -> Option<&'v mut Document> {
        match *v {
            Document::Seq(ref mut vec) => vec.get_mut(*self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Document) -> &'v mut Document {
        match *v {
            Document::Seq(ref mut vec) => {
                let len = vec.len();
                vec.get_mut(*self).unwrap_or_else(|| {
                    panic!(
                        "cannot access index {} of Document array of length {}",
                        self, len
                    )
                })
            }
            _ => panic!("cannot access index {} of Document {}", self, Type(v)),
        }
    }
}

impl Index for str {
    fn index_into<'v>(&self, v: &'v Document) -> Option<&'v Document> {
        match *v {
            Document::Map(ref map) => map.get(&Document::String(self.to_owned())),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Document) -> Option<&'v mut Document> {
        match *v {
            Document::Map(ref mut map) => map.get_mut(&Document::String(self.to_owned())),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Document) -> &'v mut Document {
        if let Document::Unit = *v {
            *v = Document::Map(BTreeMap::default());
        }
        match *v {
            Document::Map(ref mut map) => map
                .entry(Document::String(self.to_owned()))
                .or_insert(Document::Unit),
            _ => panic!("cannot access key {:?} in Document {}", self, Type(v)),
        }
    }
}

impl Index for String {
    fn index_into<'v>(&self, v: &'v Document) -> Option<&'v Document> {
        self[..].index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Document) -> Option<&'v mut Document> {
        self[..].index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Document) -> &'v mut Document {
        self[..].index_or_insert(v)
    }
}

impl<'a, T: ?Sized> Index for &'a T
where
    T: Index,
{
    fn index_into<'v>(&self, v: &'v Document) -> Option<&'v Document> {
        (**self).index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Document) -> Option<&'v mut Document> {
        (**self).index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Document) -> &'v mut Document {
        (**self).index_or_insert(v)
    }
}

mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl<'a, T: ?Sized> Sealed for &'a T where T: Sealed {}
}

struct Type<'a>(&'a Document);

impl<'a> fmt::Display for Type<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Document::Unit => formatter.write_str("unit"),
            Document::Bool(_) => formatter.write_str("boolean"),
            Document::U8(_) => formatter.write_str("u8"),
            Document::U16(_) => formatter.write_str("u16"),
            Document::U32(_) => formatter.write_str("u32"),
            Document::U64(_) => formatter.write_str("u64"),
            Document::I8(_) => formatter.write_str("i8"),
            Document::I16(_) => formatter.write_str("i16"),
            Document::I32(_) => formatter.write_str("i32"),
            Document::I64(_) => formatter.write_str("i64"),
            Document::F32(_) => formatter.write_str("f32"),
            Document::F64(_) => formatter.write_str("f64"),
            Document::String(_) => formatter.write_str("string"),
            Document::Seq(_) => formatter.write_str("seq"),
            Document::Map(_) => formatter.write_str("map"),
            Document::Char(_) => formatter.write_str("char"),
            Document::Option(_) => formatter.write_str("option"),
            Document::Newtype(_) => formatter.write_str("newtype"),
            Document::Bytes(_) => formatter.write_str("bytes"),
        }
    }
}

impl<I> ops::Index<I> for Document
where
    I: Index,
{
    type Output = Document;

    fn index(&self, index: I) -> &Document {
        static NULL: Document = Document::Unit;
        index.index_into(self).unwrap_or(&NULL)
    }
}

impl<I> ops::IndexMut<I> for Document
where
    I: Index,
{
    fn index_mut(&mut self, index: I) -> &mut Document {
        index.index_or_insert(self)
    }
}
