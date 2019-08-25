use std::collections::BTreeMap;
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

impl Index for Document {
    fn index_into<'v>(&self, v: &'v Document) -> Option<&'v Document> {
        match (v, self.as_usize()) {
            (Document::Seq(ref s), Some(i)) => {
                if i >= s.len() {
                    None
                } else {
                    Some(&s[i])
                }
            }
            (Document::Map(ref map), _) => map.get(self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Document) -> Option<&'v mut Document> {
        match (v, self.as_usize()) {
            (Document::Seq(ref mut s), Some(i)) => {
                if i >= s.len() {
                    None
                } else {
                    Some(&mut s[i])
                }
            }
            (Document::Map(ref mut map), _) => map.get_mut(self),
            _ => None,
        }
    }

    fn index_or_insert<'v>(&self, v: &'v mut Document) -> &'v mut Document {
        if self.is_number() && !(v.is_seq() || v.is_map()) {
            *v = Document::Seq(vec![]);
        } else if !self.is_number() && !v.is_map() {
            *v = Document::Map(BTreeMap::default());
        }
        match *v {
            Document::Map(ref mut map) => map.entry(self.clone()).or_insert(Document::Unit),
            Document::Seq(ref mut seq) => {
                if let Some(i) = self.as_usize() {
                    let size = seq.len();
                    if i >= size {
                        seq.push(Document::Unit);
                        &mut seq[size]
                    } else {
                        &mut seq[i]
                    }
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}

macro_rules! impl_index {
    ($( $type:ty ),*) => {

        mod private {
            use super::Document;
            pub trait Sealed {}
            $( impl Sealed for $type {} )*
            impl Sealed for Document {}
            impl<'a, T: ?Sized> Sealed for &'a T where T: Sealed {}
        }

        $(
            impl Index for $type {
                fn index_into<'v>(&self, v: &'v Document) -> Option<&'v Document> {
                    let d: Document = self.into();
                    d.index_into(v)
                }
                fn index_into_mut<'v>(&self, v: &'v mut Document) -> Option<&'v mut Document> {
                    let d: Document = self.into();
                    d.index_into_mut(v)
                }
                fn index_or_insert<'v>(&self, v: &'v mut Document) -> &'v mut Document {
                    let d: Document = self.into();
                    d.index_or_insert(v)
                }
            }
        )*

    };
}

impl_index!(str, String, usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8, f64, f32);

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
