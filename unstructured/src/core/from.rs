use crate::*;

macro_rules! from_imp {
    ( &{ $($ref_ty:ty, $ref_v:ident)* } *{ $($ty:ty, $v:ident)* } ) => {
        $(
            impl<T: UnstructuredDataTrait> From<&$ref_ty> for Unstructured<T> {
                fn from(n: &$ref_ty) -> Self {
                    Unstructured::<T>::$ref_v(n.to_owned())
                }
            }
        )*

        $(
            impl<T: UnstructuredDataTrait> From<$ty> for Unstructured<T> {
                fn from(n: $ty) -> Self {
                    Unstructured::<T>::$v(n as $ty)
                }
            }
        )*
    };
}

impl<T: Into<Number>, Q: UnstructuredDataTrait> From<T> for Unstructured<Q> {
    fn from(n: T) -> Self {
        Self::Number(n.into())
    }
}

from_imp! {
    &{
        bool,Bool
        char,Char
        String,String str,String
        Vec<u8>,Bytes
        Sequence<T>,Seq
        Mapping<T>,Map
        Option<Box<Unstructured<T>>>,Option
        Box<Unstructured<T>>,Newtype
    }

    *{
        bool,Bool
        char,Char
        String,String
        Vec<u8>,Bytes
        Sequence<T>,Seq
        Mapping<T>,Map
        Option<Box<Unstructured<T>>>,Option
        Box<Unstructured<T>>,Newtype
    }
}
