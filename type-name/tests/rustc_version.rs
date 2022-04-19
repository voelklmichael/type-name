#[test]
fn rustc_version() {
    use ::type_name::TypeName;
    #[derive(TypeName)]
    struct A {}
    let type_name = dbg!(A::type_name_static());
    assert_eq!("A", type_name.type_name());
    assert_eq!(Some("type-name"), type_name.crate_name().as_deref());
    assert_eq!(0, type_name.generics().len());
    assert_eq!(
        &Some(rustc_version::version().unwrap()),
        type_name.rustc_version()
    );
}
