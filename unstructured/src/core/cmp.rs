use crate::*;

macro_rules! impl_partial_eq {
    ($($type:ty, $vrnt:ident);*) => {
        $(
            impl<T: UnstructuredDataTrait> PartialEq<$type> for Unstructured<T> {
                fn eq(&self, rhs: & $type) -> bool {
                    match self {
                        Self::$vrnt(i) => i == rhs,
                        _ => false,
                    }
                }
            }
        )*
    };
}
impl_partial_eq! { &str, String; String, String; bool, Bool; char, Char; Number, Number }

macro_rules! impl_partial_eq {
    ($($type:ty, $vrnt:ident);*) => {
        $(
            impl<T: UnstructuredDataTrait> PartialEq<Unstructured<T>> for $type {
                fn eq(&self, rhs: &Unstructured<T>) -> bool {
                    match rhs {
                        Unstructured::<T>::$vrnt(i) => i == self,
                        _ => false,
                    }
                }
            }
        )*
    };
}
impl_partial_eq! { &str, String; String, String; bool, Bool; char, Char }

macro_rules! impl_partial_eq_number {
    ( $( $type:ty )* ) => {
        $(
            impl<T: UnstructuredDataTrait> PartialEq<$type> for Unstructured<T> {
                fn eq(&self, rhs: & $type) -> bool {
                    match self {
                        Self::Number(i) => i == &Number::from(rhs),
                        _ => false,
                    }
                }
            }

            impl<T: UnstructuredDataTrait> PartialEq<Unstructured<T>> for $type {
                fn eq(&self, rhs: &Unstructured<T>) -> bool {
                    match rhs {
                        Unstructured::<T>::Number(i) => i == &Number::from(self),
                        _ => false,
                    }
                }
            }
        )*
    };
}
foreach_numeric_primitive! { impl_partial_eq_number! }

impl<T: UnstructuredDataTrait> PartialEq for Unstructured<T> {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (&Self::Unassigned, &Self::Unassigned) => true,
            (&Self::Null, &Self::Null) => true,
            (&Self::Bool(v0), &Self::Bool(v1)) if v0 == v1 => true,
            (&Self::Number(ref v0), &Self::Number(ref v1)) if v0 == v1 => true,
            (&Self::Char(v0), &Self::Char(v1)) if v0 == v1 => true,
            (&Self::String(ref v0), &Self::String(ref v1)) if v0 == v1 => true,
            (&Self::Option(ref v0), &Self::Option(ref v1)) if v0 == v1 => true,
            (&Self::Newtype(ref v0), &Self::Newtype(ref v1)) if v0 == v1 => true,
            (&Self::Seq(ref v0), &Self::Seq(ref v1)) if v0 == v1 => true,
            (&Self::Map(ref v0), &Self::Map(ref v1)) if v0 == v1 => true,
            (&Self::Bytes(ref v0), &Self::Bytes(ref v1)) if v0 == v1 => true,
            _ => false,
        }
    }
}

impl<T: UnstructuredDataTrait> Eq for Unstructured<T> {}
