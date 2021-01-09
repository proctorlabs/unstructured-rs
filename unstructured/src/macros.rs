#[macro_export]
macro_rules! anyvec {
    ($( $val:expr ,)*) => {
        vec![$($val.into()),*].into_unstructured()
    };
    ($( $val:expr ),*) => {
        anyvec![$($val,)*]
    };
}

#[macro_export]
macro_rules! walk {
    ($us:ident $( / $val:literal )*) => {
        & $us $( [ $val ] )*
    };
}

#[macro_export]
macro_rules! foreach_numeric_primitive {
    ($($impl:tt)*) => {
        $($impl)* { u8 }
        $($impl)* { u16 }
        $($impl)* { u32 }
        $($impl)* { u64 }
        $($impl)* { u128 }
        $($impl)* { i8 }
        $($impl)* { i16 }
        $($impl)* { i32 }
        $($impl)* { i64 }
        $($impl)* { i128 }
        $($impl)* { f32 }
        $($impl)* { f64 }
    };
}
