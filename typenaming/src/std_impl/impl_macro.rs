/// This is an unstable helper macro, which is used to implement TypeNameable for several types
#[doc(hidden)]
#[macro_export]
macro_rules! implementing {
    ( $t:ident ) => {
        #[allow(non_snake_case)]
        #[doc(hidden)]
        pub mod $t {
            use crate::{TypeInfo, TypeNameable};

            type T = $t;
            const TT: &'static str = stringify!($t);
            fn info() -> TypeInfo {
                TypeInfo::new(
                    TT.to_owned(),
                    Some("core".to_owned()),
                    Some("core".to_owned()),
                    Some(crate::Version::new(1, 0, 0)),
                    Some(crate::Version::new(1, 0, 0)),
                    vec![],
                )
            }

            impl TypeNameable for T {
                fn type_info() -> TypeInfo {
                    info()
                }
            }

            impl<'a> TypeNameable for &'a T {
                fn type_info() -> TypeInfo {
                    info()
                }
            }
        }
    };
}

/// This is an unstable helper macro, which is used to implement TypeNameable for several types
#[doc(hidden)]
#[macro_export]
macro_rules! implementing_unsized {
    ( $t:ident ) => {
        #[allow(non_snake_case)]
        #[doc(hidden)]
        pub mod $t {
            use crate::{TypeInfo, TypeNameable};

            type T = $t;
            const TT: &'static str = stringify!($t);
            fn info() -> TypeInfo {
                TypeInfo::new(
                    TT.to_owned(),
                    Some("core".to_owned()),
                    Some("core".to_owned()),
                    Some(crate::Version::new(1, 0, 0)),
                    Some(crate::Version::new(1, 0, 0)),
                    vec![],
                )
            }

            impl<'a> TypeNameable for &'a T {
                fn type_info() -> TypeInfo {
                    info()
                }
            }
        }
    };
}

/// This is an unstable helper macro, which is used to implement TypeNameable for several types
#[doc(hidden)]
#[macro_export]
macro_rules! implementing_generic {
    ( $t:ident, $($g:ident),* ) => {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub mod $t {
            use crate::{TypeNameable, TypeInfo};

            const TT: &'static str = stringify!($t);
            fn info($($g: TypeInfo,)*) -> TypeInfo {
                TypeInfo::new(
                    TT.to_owned(),
                    Some("core".to_owned()),
                    Some("core".to_owned()),
                    Some(crate::Version::new(1, 0, 0)),
                    Some(crate::Version::new(1, 0, 0)),
                    vec![$($g,)*],
                )
            }

            impl<$($g,)*> TypeNameable for $t<$($g,)*>
            where
                $($g: TypeNameable,)*
            {
                fn type_info() -> TypeInfo {
                    info($($g::type_info(),)*)
                }
            }

            impl<'a, $($g,)*> TypeNameable for &'a $t<$($g,)*>
            where
                $($g: TypeNameable,)*
            {
                fn type_info() -> TypeInfo {
                    info($($g::type_info(),)*)
                }
            }
        }
    };
}
