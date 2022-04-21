/// This is an unstable helper macro, which is used to implement TypeName for several types
#[doc(hidden)]
#[macro_export]
macro_rules! implementing {
    ( $t:ident ) => {
        #[allow(non_snake_case)]
        #[doc(hidden)]
        pub mod $t {
            use crate::{TypeName, TypeNameData};

            type T = $t;
            const TT: &'static str = stringify!($t);
            fn info() -> TypeNameData {
                TypeNameData::new(
                    TT.to_owned(),
                    Some("core".to_owned()),
                    Some("core".to_owned()),
                    Some(crate::Version::new(1, 0, 0)),
                    Some(crate::Version::new(1, 0, 0)),
                    vec![],
                )
            }

            impl TypeName for T {
                fn type_name(&self) -> TypeNameData {
                    info()
                }

                fn type_name_static() -> TypeNameData
                where
                    Self: Sized,
                {
                    info()
                }
            }

            impl<'a> TypeName for &'a T {
                fn type_name(&self) -> TypeNameData {
                    info()
                }

                fn type_name_static() -> TypeNameData
                where
                    Self: Sized,
                {
                    info()
                }
            }
        }
    };
}

/// This is an unstable helper macro, which is used to implement TypeName for several types
#[doc(hidden)]
#[macro_export]
macro_rules! implementing_unsized {
    ( $t:ident ) => {
        #[allow(non_snake_case)]
        #[doc(hidden)]
        pub mod $t {
            use crate::{TypeName, TypeNameData};

            type T = $t;
            const TT: &'static str = stringify!($t);
            fn info() -> TypeNameData {
                TypeNameData::new(
                    TT.to_owned(),
                    Some("core".to_owned()),
                    Some("core".to_owned()),
                    Some(crate::Version::new(1, 0, 0)),
                    Some(crate::Version::new(1, 0, 0)),
                    vec![],
                )
            }

            impl<'a> TypeName for &'a T {
                fn type_name(&self) -> TypeNameData {
                    info()
                }

                fn type_name_static() -> TypeNameData
                where
                    Self: Sized,
                {
                    info()
                }
            }
        }
    };
}

/// This is an unstable helper macro, which is used to implement TypeName for several types
#[doc(hidden)]
#[macro_export]
macro_rules! implementing_generic {
    ( $t:ident, $($g:ident),* ) => {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub mod $t {
            use crate::{TypeName, TypeNameData};

            const TT: &'static str = stringify!($t);
            fn info($($g: TypeNameData,)*) -> TypeNameData {
                TypeNameData::new(
                    TT.to_owned(),
                    Some("core".to_owned()),
                    Some("core".to_owned()),
                    Some(crate::Version::new(1, 0, 0)),
                    Some(crate::Version::new(1, 0, 0)),
                    vec![$($g,)*],
                )
            }

            impl<$($g,)*> TypeName for $t<$($g,)*>
            where
                $($g: TypeName,)*
            {
                fn type_name(&self) -> TypeNameData {
                    info($($g::type_name_static(),)*)
                }

                fn type_name_static() -> TypeNameData
                where
                    Self: Sized,
                {
                    info($($g::type_name_static(),)*)
                }
            }

            impl<'a, $($g,)*> TypeName for &'a $t<$($g,)*>
            where
                $($g: TypeName,)*
            {
                fn type_name(&self) -> TypeNameData {
                    info($($g::type_name_static(),)*)
                }

                fn type_name_static() -> TypeNameData
                where
                    Self: Sized,
                {
                    info($($g::type_name_static(),)*)
                }
            }
        }
    };
}
