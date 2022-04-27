mod level1 {
    mod level2 {
        #[test]
        fn crate_module() {
            use ::typenaming::TypeNameable;
            #[derive(TypeNameable)]
            struct A {}
            let type_name = dbg!(A::type_info());
            assert_eq!("A", type_name.type_name());
            assert_eq!(Some("typenaming"), type_name.crate_name().as_deref());
            assert_eq!(
                Some("crate_module::level1::level2"),
                type_name.crate_module().as_deref()
            );
            assert_eq!(0, type_name.generics().len());
            assert_eq!(
                &Some(rustc_version::version().unwrap()),
                type_name.rustc_version()
            );
        }
    }
}
