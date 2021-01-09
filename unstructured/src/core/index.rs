use std::collections::BTreeMap;
use std::ops;

use crate::*;

pub trait Index<T: UnstructuredDataTrait>: private::Sealed {
    #[doc(hidden)]
    fn index_into<'v>(&self, v: &'v Unstructured<T>) -> Option<&'v Unstructured<T>>;

    #[doc(hidden)]
    fn index_into_mut<'v>(&self, v: &'v mut Unstructured<T>) -> Option<&'v mut Unstructured<T>>;

    #[doc(hidden)]
    fn index_or_insert<'v>(&self, v: &'v mut Unstructured<T>) -> &'v mut Unstructured<T>;
}

impl<T: UnstructuredDataTrait> Index<T> for Unstructured<T>
{
    fn index_into<'v>(&self, v: &'v Unstructured<T>) -> Option<&'v Unstructured<T>> {
        match (v, self.as_usize()) {
            (Unstructured::<T>::Seq(ref s), Some(i)) => {
                if i >= s.len() {
                    None
                } else {
                    Some(&s[i])
                }
            }
            (Unstructured::<T>::Map(ref map), _) => map.get(self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Unstructured<T>) -> Option<&'v mut Unstructured<T>> {
        match (v, self.as_usize()) {
            (Unstructured::<T>::Seq(ref mut s), Some(i)) => {
                if i >= s.len() {
                    None
                } else {
                    Some(&mut s[i])
                }
            }
            (Unstructured::<T>::Map(ref mut map), _) => map.get_mut(self),
            _ => None,
        }
    }

    fn index_or_insert<'v>(&self, v: &'v mut Unstructured<T>) -> &'v mut Unstructured<T> {
        if self.is_number()
            && !(v.is::<Sequence<T>>()
                || v.is::<Mapping<T>>())
        {
            *v = Unstructured::<T>::Seq(vec![]);
        } else if !self.is_number() && !v.is::<Mapping<T>>() {
            *v = Unstructured::<T>::Map(BTreeMap::default());
        }
        match *v {
            Unstructured::<T>::Map(ref mut map) => {
                map.entry(self.clone()).or_insert(Unstructured::<T>::Null)
            }
            Unstructured::<T>::Seq(ref mut seq) => {
                if let Some(i) = self.as_usize() {
                    let size = seq.len();
                    if i >= size {
                        seq.push(Unstructured::<T>::Null);
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
            use super::{Unstructured, UnstructuredDataTrait};
            pub trait Sealed {}
            $( impl Sealed for $type {} )*
            impl<T: UnstructuredDataTrait> Sealed for Unstructured<T> {}
            impl<'a, T: ?Sized> Sealed for &'a T where T: Sealed {}
        }

        $(
            impl<T: UnstructuredDataTrait> Index<T> for $type {
                fn index_into<'v>(&self, v: &'v Unstructured<T>) -> Option<&'v Unstructured<T>> {
                    let d: Unstructured<T> = self.into();
                    d.index_into(v)
                }
                fn index_into_mut<'v>(&self, v: &'v mut Unstructured<T>) -> Option<&'v mut Unstructured<T>> {
                    let d: Unstructured<T> = self.into();
                    d.index_into_mut(v)
                }
                fn index_or_insert<'v>(&self, v: &'v mut Unstructured<T>) -> &'v mut Unstructured<T> {
                    let d: Unstructured<T> = self.into();
                    d.index_or_insert(v)
                }
            }
        )*

    };
}

impl_index!(str, String, usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8, f64, f32);

impl<'a, T: ?Sized, Q: UnstructuredDataTrait> Index<Q> for &'a T
where
    T: Index<Q>,
{
    fn index_into<'v>(&self, v: &'v Unstructured<Q>) -> Option<&'v Unstructured<Q>> {
        (**self).index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Unstructured<Q>) -> Option<&'v mut Unstructured<Q>> {
        (**self).index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Unstructured<Q>) -> &'v mut Unstructured<Q> {
        (**self).index_or_insert(v)
    }
}

impl<I, T: UnstructuredDataTrait> ops::Index<I> for Unstructured<T>
where
    I: Index<T>,
{
    type Output = Self;

    fn index(&self, index: I) -> &Self {
        // static NULL: Unstructured<T> = Unstructured::<T>::Null;
        index.index_into(self).unwrap_or(&Unstructured::<T>::Null)
    }
}

impl<I, T: UnstructuredDataTrait> ops::IndexMut<I> for Unstructured<T>
where
    I: Index<T>,
{
    fn index_mut(&mut self, index: I) -> &mut Unstructured<T> {
        index.index_or_insert(self)
    }
}
